//! Purpose:
//! Home of the PHP `mb_strpos` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - Mirrors `strpos`: the declared signature carries the full golden param list
//!   (`haystack`, `needle`, `offset`), but `max_args: 2` caps `check_arity` so the
//!   offset/encoding arguments are rejected — UTF-8 is assumed (the default encoding)
//!   and the code-point offset argument is a follow-up.
//! - `check` returns `PhpType::Union([Int, Bool])` (code-point position, or `false`).
//! - `lower` reuses the shared `lower_string_position` emitter, passing the
//!   `__rt_mb_strpos` runtime helper, which returns a UTF-8 code-point index.
//! - EIR-only (no frozen legacy-backend emitter), so it joins `mb_strlen` in the
//!   callable-dispatch wrapper exclusion.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::errors::CompileError;
use crate::ir::Instruction;
use crate::types::PhpType;

builtin! {
    name: "mb_strpos",
    area: String,
    params: [haystack: Str, needle: Str, offset: Int = DefaultSpec::Int(0)],
    max_args: 2,
    returns: Mixed,
    check: check,
    lower: lower,
    summary: "Finds the code-point position of the first occurrence of a substring (multibyte-aware).",
    php_manual: "https://www.php.net/manual/en/function.mb-strpos.php",
}

/// Returns `PhpType::Union([Int, Bool])` for an `mb_strpos` call (position, or `false`).
///
/// A check hook is required because the `builtin!` macro cannot express a union return
/// type inline. Argument types are inferred by the common registry dispatch path before
/// this hook fires; arity (capped to 2 via `max_args`) is validated by the registry.
fn check(_cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    Ok(PhpType::Union(vec![PhpType::Int, PhpType::Bool]))
}

/// Lowers an `mb_strpos` call by dispatching to the shared string-position emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::strings::lower_string_position(
        ctx,
        inst,
        "mb_strpos",
        "__rt_mb_strpos",
    )
}
