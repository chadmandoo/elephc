#!/usr/bin/env python3
"""Mechanically move registry builtin backend wrappers behind typed EIR targets.

The script is intentionally narrow: it only rewrites builtin homes whose lowering
function is a thin call into ``crate::codegen``. It generates small backend dispatch
groups, a stable target enum, and replaces each ``lower:`` field with transitional
semantic metadata. Re-running after the migration is a no-op.
"""

from __future__ import annotations

import importlib.util
import re
from dataclasses import dataclass
from pathlib import Path


REPO = Path(__file__).resolve().parents[1]
AUDIT_PATH = REPO / "scripts" / "audit_builtin_eir_boundary.py"
GROUP_SIZE = 35


def load_audit_module():
    """Load the sibling inventory module without requiring a Python package."""
    spec = importlib.util.spec_from_file_location("builtin_boundary_audit", AUDIT_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {AUDIT_PATH}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


AUDIT = load_audit_module()


@dataclass(frozen=True)
class Migration:
    """One builtin wrapper and the generated target metadata replacing it."""

    name: str
    variant: str
    area: str
    home: Path
    lower_name: str
    body: str
    strategy: str


def variant_name(name: str) -> str:
    """Convert a canonical PHP function name into a stable Rust enum variant."""
    parts = re.findall(r"[A-Za-z0-9]+", name)
    variant = "".join(part[:1].upper() + part[1:] for part in parts)
    if not variant or variant[0].isdigit():
        variant = f"Builtin{variant}"
    return variant


def function_extent(source: str, name: str) -> tuple[int, int, str]:
    """Return the docblock-inclusive extent and body of one thin wrapper function."""
    match = re.search(
        rf"(?m)^(?:pub(?:\([^)]*\))?\s+)?fn\s+{re.escape(name)}\s*\(",
        source,
    )
    if match is None:
        raise RuntimeError(f"missing lowering function {name}")
    brace = source.find("{", match.end())
    if brace < 0:
        raise RuntimeError(f"missing body for lowering function {name}")
    depth = 0
    end = -1
    for index in range(brace, len(source)):
        if source[index] == "{":
            depth += 1
        elif source[index] == "}":
            depth -= 1
            if depth == 0:
                end = index + 1
                break
    if end < 0:
        raise RuntimeError(f"unterminated body for lowering function {name}")
    start = match.start()
    line_start = source.rfind("\n", 0, start) + 1
    cursor = line_start
    while cursor > 0:
        previous_end = cursor - 1
        previous_start = source.rfind("\n", 0, previous_end) + 1
        line = source[previous_start:previous_end].strip()
        if line.startswith("///") or line.startswith("#[") or not line:
            cursor = previous_start
            continue
        break
    body = source[brace + 1 : end - 1].strip()
    return cursor, end, body


def strategy_variant(strategy: str) -> str:
    """Map inventory strategy labels onto registry target metadata variants."""
    mapping = {
        "eir_primitive": "EirPrimitive",
        "eir_graph": "EirGraph",
        "typed_runtime_call": "RuntimeCall",
        "conditional": "Conditional",
    }
    try:
        return mapping[strategy]
    except KeyError as error:
        raise RuntimeError(f"unknown target strategy {strategy}") from error


def collect_migrations() -> list[Migration]:
    """Collect every remaining thin backend wrapper and its audited strategy."""
    inventory = AUDIT.build_inventory()
    strategies = {
        record["canonical_name"]: record["lowering"]["target_strategy"]
        for record in inventory["registry_builtins"]
    }
    migrations: list[Migration] = []
    for home in sorted((REPO / "src" / "builtins").rglob("*.rs")):
        source = home.read_text()
        block = AUDIT.builtin_macro_block(source)
        lower_name = AUDIT.field_value(block, "lower")
        if lower_name is None:
            continue
        name_match = re.search(r'(?m)^\s*name:\s*"([^"]+)"', block)
        area_match = re.search(r"(?m)^\s*area:\s*([A-Za-z0-9_]+)", block)
        if name_match is None or area_match is None:
            raise RuntimeError(f"cannot read builtin identity from {home}")
        name = name_match.group(1)
        _, _, body = function_extent(source, lower_name)
        if "crate::codegen::" not in body:
            raise RuntimeError(f"{home}: {lower_name} is not a thin backend wrapper")
        migrations.append(
            Migration(
                name=name,
                variant=variant_name(name),
                area=area_match.group(1).lower(),
                home=home,
                lower_name=lower_name,
                body=body,
                strategy=strategies[name.lower()],
            )
        )
    variants = [migration.variant for migration in migrations]
    if len(variants) != len(set(variants)):
        duplicates = sorted({variant for variant in variants if variants.count(variant) > 1})
        raise RuntimeError(f"generated target variant collision: {duplicates}")
    return migrations


def render_target_enum(migrations: list[Migration]) -> str:
    """Render the backend-neutral enum used by typed EIR runtime calls."""
    variants = "\n".join(f"    {migration.variant}," for migration in migrations)
    names = "\n".join(
        f'            BuiltinRuntimeTarget::{migration.variant} => "{migration.name}",'
        for migration in migrations
    )
    return f'''//! Purpose:
//! Defines stable typed identities for registry builtin backend operations.
//! This file is generated by `scripts/migrate_builtin_backend_targets.py`.
//!
//! Called from:
//! - Registry semantic descriptors and typed EIR `RuntimeCall` instructions.
//! - Target backend dispatch groups under `codegen/lower_inst/builtin_runtime_targets/`.
//!
//! Key details:
//! - Variants are semantic identities; backend dispatch never infers behavior from PHP names.
//! - Physical registers, helper symbols, and platform branches remain downstream in codegen.

/// Stable semantic identity for one registry builtin backend operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinRuntimeTarget {{
{variants}
}}

impl BuiltinRuntimeTarget {{
    /// Returns the stable textual EIR spelling for diagnostics and snapshots.
    pub fn as_eir(self) -> &'static str {{
        match self {{
{names}
        }}
    }}
}}
'''


def render_dispatch_group(index: int, migrations: list[Migration]) -> str:
    """Render one bounded backend dispatch group from the extracted wrapper bodies."""
    uses_php_type = any("PhpType" in migration.body for migration in migrations)
    php_import = "use crate::types::PhpType;\n" if uses_php_type else ""
    arms = "\n".join(
        f"        BuiltinRuntimeTarget::{migration.variant} => Some({{\n"
        + "\n".join(f"            {line}" for line in migration.body.splitlines())
        + "\n        }),"
        for migration in migrations
    )
    return f'''//! Purpose:
//! Dispatches one bounded generated group of typed builtin runtime targets.
//! This file is generated by `scripts/migrate_builtin_backend_targets.py`.
//!
//! Called from:
//! - `super::lower()` while lowering typed EIR runtime calls.
//!
//! Key details:
//! - Dispatch is by enum identity, never by PHP function-name strings.
//! - Extracted bodies remain thin calls into target-aware backend emitters.

use crate::codegen::context::FunctionContext;
use crate::codegen::Result;
use crate::ir::{{BuiltinRuntimeTarget, Instruction}};
{php_import}
/// Lowers a target owned by generated dispatch group {index:02d}, or returns `None`.
pub(super) fn lower(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    target: BuiltinRuntimeTarget,
) -> Option<Result<()>> {{
    match target {{
{arms}
        _ => None,
    }}
}}
'''


def render_dispatch_root(group_count: int) -> str:
    """Render the small dispatcher that probes generated bounded groups."""
    mods = "\n".join(f"mod group_{index:02d};" for index in range(group_count))
    probes = "\n".join(
        f"    if let Some(result) = group_{index:02d}::lower(ctx, inst, target) {{\n"
        "        return result;\n"
        "    }"
        for index in range(group_count)
    )
    return f'''//! Purpose:
//! Routes typed registry builtin runtime targets to bounded backend dispatch groups.
//! This file is generated by `scripts/migrate_builtin_backend_targets.py`.
//!
//! Called from:
//! - `crate::codegen::lower_inst::runtime_calls` for typed builtin operations.
//!
//! Key details:
//! - No PHP-name lookup participates in backend dispatch.
//! - Group files retain existing target-aware emitters while semantic migration continues.

use crate::codegen::context::FunctionContext;
use crate::codegen::{{CodegenIrError, Result}};
use crate::ir::{{BuiltinRuntimeTarget, Instruction}};

{mods}

/// Lowers one typed registry builtin target through its generated backend adapter.
pub(super) fn lower(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    target: BuiltinRuntimeTarget,
) -> Result<()> {{
{probes}
    Err(CodegenIrError::invalid_module(format!(
        "missing backend dispatch for typed builtin target {{}}",
        target.as_eir(),
    )))
}}
'''


def rewrite_home(migration: Migration) -> None:
    """Replace one backend hook with transitional typed semantic metadata."""
    source = migration.home.read_text()
    block = AUDIT.builtin_macro_block(source)
    lower_pattern = re.compile(
        rf"(?m)^(?P<indent>\s*)lower:\s*{re.escape(migration.lower_name)},\s*$"
    )
    replacement = (
        r"\g<indent>semantics: crate::builtins::semantics::backend_target_adapter(\n"
        + f"            crate::ir::BuiltinRuntimeTarget::{migration.variant},\n"
        + "            crate::builtins::semantics::BuiltinTargetStrategy::"
        + strategy_variant(migration.strategy)
        + ",\n"
        + r"\g<indent>),"
    )
    rewritten_block, count = lower_pattern.subn(replacement, block, count=1)
    if count != 1:
        raise RuntimeError(f"cannot replace lower field in {migration.home}")
    source = source.replace(block, rewritten_block, 1)
    start, end, _ = function_extent(source, migration.lower_name)
    source = source[:start] + source[end:]
    source = source.replace("use crate::codegen::context::FunctionContext;\n", "")
    source = source.replace("use crate::codegen::CodegenIrError;\n", "")
    source = source.replace("use crate::ir::Instruction;\n", "")
    source = source.replace(
        "    codegen::{context::FunctionContext, CodegenIrError},\n",
        "",
    )
    source = source.replace("    ir::Instruction,\n", "")
    source = source.replace("declaration and lowering", "declaration and semantic metadata")
    source = source.replace("its declaration and lowering", "its declaration and semantic metadata")
    migration.home.write_text(source.rstrip() + "\n")


def main() -> int:
    """Generate typed targets and mechanically rewrite every remaining builtin home."""
    migrations = collect_migrations()
    if not migrations:
        print("No legacy backend hooks remain; nothing to rewrite.")
        return 0
    target_path = REPO / "src" / "ir" / "builtin_runtime_target.rs"
    target_path.write_text(render_target_enum(migrations))

    dispatch_root = REPO / "src" / "codegen" / "lower_inst" / "builtin_runtime_targets.rs"
    dispatch_dir = dispatch_root.with_suffix("")
    dispatch_dir.mkdir(exist_ok=True)
    groups = [
        migrations[index : index + GROUP_SIZE]
        for index in range(0, len(migrations), GROUP_SIZE)
    ]
    dispatch_root.write_text(render_dispatch_root(len(groups)))
    for index, group in enumerate(groups):
        (dispatch_dir / f"group_{index:02d}.rs").write_text(render_dispatch_group(index, group))

    for migration in migrations:
        rewrite_home(migration)
    print(
        f"Migrated {len(migrations)} backend hooks into {len(groups)} typed dispatch groups."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
