# Phase 09 — Legacy Cleanup and Documentation Consolidation

> **For agentic workers:** Final phase. Remove the legacy AST → ASM backend, consolidate documentation, audit naming, and freeze the IR contract for the v1.0 release.

**Goal:** Delete `src/codegen/` paths that the IR backend has replaced. Rename `src/codegen_ir/` to `src/codegen/`. Move historical docs aside. Finalize `docs/internals/the-ir.md` as the canonical codegen explanation alongside `the-codegen.md`.

**Architecture:** Pure cleanup. No new code, no functional change. The IR backend is already the default since Phase 05.

**Tech Stack:** Rust, git, project documentation tooling.

---

## File Structure

Heavy delete/rename phase. Approximate map:

- Delete: `src/codegen/expr.rs`, `src/codegen/expr/` (replaced by `src/codegen_ir/lower_inst/`)
- Delete: `src/codegen/stmt.rs`, `src/codegen/stmt/` (replaced by `src/codegen_ir/lower_inst/` + `src/codegen_ir/lower_term.rs`)
- Delete: `src/codegen/builtins/` *if* every builtin already routes through the IR backend; otherwise keep until parity is full
- Delete: `src/codegen/class_methods.rs`, `src/codegen/functions/` (replaced by IR function emission)
- Keep and merge into new home: `src/codegen/abi/`, `src/codegen/runtime/`, `src/codegen/emit.rs`, `src/codegen/data_section.rs`, `src/codegen/platform.rs`, `src/codegen/ffi.rs`
- Rename: `src/codegen_ir/` → `src/codegen/` (after the legacy delete)
- Move historical doc: `docs/internals/legacy-codegen.md` (created from the current `the-codegen.md` content describing the AST-walker, preserved for historical reference)
- Refresh: `docs/internals/the-codegen.md` to describe the IR-based pipeline as the only pipeline
- Refresh: `docs/internals/the-ir.md` to remove "preview" / "phase" language and present EIR as the canonical IR

---

## Task 1: Verify no consumers of legacy paths

**Files:**
- Inspect: `src/`, `tests/`

- [ ] **Step 1: Find every reference to the legacy backend**

Run:
```bash
grep -rn "generate_user_asm\|--ast-backend\|crate::codegen::expr\|crate::codegen::stmt" src/ tests/ | grep -v codegen_ir
```

Expected: only the deprecation shim from Phase 05, the `--ast-backend` CLI flag (which is about to be removed), and possibly some test helpers.

If any production path still requires the legacy code, document it in `docs/internals/legacy-codegen.md` and defer that path's removal to a follow-up task.

- [ ] **Step 2: Remove `--ast-backend` from CLI**

In `src/cli.rs`, remove the `--ast-backend` flag. The argument parser should error with `unknown flag: --ast-backend; the IR backend is the only backend since v0.26.0`.

- [ ] **Step 3: Run full gate**

```bash
cargo test
cargo test -- --include-ignored
```

Expected: green. (Some tests may use `--ast-backend` — update them or remove if redundant.)

- [ ] **Step 4: Commit**

```bash
git add src/cli.rs
git commit -m "feat(cli): remove deprecated --ast-backend flag"
```

---

## Task 2: Delete the AST emitter

**Files:**
- Delete: `src/codegen/expr/`, `src/codegen/expr.rs`
- Delete: `src/codegen/stmt/`, `src/codegen/stmt.rs`
- Delete: `src/codegen/class_methods.rs`
- Delete: `src/codegen/functions/`
- Delete: `src/codegen/main_emission.rs`
- Delete: `src/codegen/prescan.rs`, `src/codegen/program_usage/`, `src/codegen/program_usage.rs`
- Delete: `src/codegen/function_variants.rs`
- Delete: `src/codegen/interface_wrappers.rs`
- Delete: `src/codegen/driver_support.rs`
- Modify: `src/codegen/mod.rs` to remove deleted modules
- Modify: anything that referenced deleted symbols

- [ ] **Step 1: Make the deletion**

```bash
git rm -r src/codegen/expr src/codegen/expr.rs
git rm -r src/codegen/stmt src/codegen/stmt.rs
git rm src/codegen/class_methods.rs
git rm -r src/codegen/functions
git rm src/codegen/main_emission.rs
git rm src/codegen/prescan.rs src/codegen/program_usage.rs
git rm -r src/codegen/program_usage
git rm src/codegen/function_variants.rs src/codegen/interface_wrappers.rs src/codegen/driver_support.rs
```

- [ ] **Step 2: Fix `src/codegen/mod.rs`**

Remove `mod` lines for deleted modules. Remove `pub use` re-exports of deleted symbols.

- [ ] **Step 3: Build until clean**

```bash
cargo build
```

Resolve every error, one at a time. Each one is a leftover caller of a deleted symbol. The IR backend should replace it (in most cases, the path was already through the IR backend; the leftover is a stale `pub use` or test helper).

- [ ] **Step 4: Run full gate**

```bash
cargo test
cargo test -- --include-ignored
./scripts/test-linux-x86_64.sh
./scripts/test-linux-arm64.sh
```

- [ ] **Step 5: Commit**

```bash
git commit -m "refactor: delete legacy AST → ASM emitter"
```

This will be one large commit (likely 10k+ lines deleted, including tests that exercised the AST emitter specifically). That is acceptable for a delete-only change.

---

## Task 3: Rename `codegen_ir` to `codegen`

**Files:**
- Rename: `src/codegen_ir/` → (moved into `src/codegen/`)
- Modify: every `use crate::codegen_ir::*` → `use crate::codegen::*`

- [ ] **Step 1: Decide the new layout**

The new `src/codegen/` mixes:
- IR consumer (was `src/codegen_ir/lower_inst/`, `lower_term.rs`, `block_emit.rs`, `frame.rs`, `value_placement.rs`, `context.rs`)
- Preserved shared infrastructure (`abi/`, `runtime/`, `emit.rs`, `data_section.rs`, `platform.rs`, `ffi.rs`)

New layout:

```
src/codegen/
├── mod.rs              # entry: generate_user_asm(module, ...)
├── abi/                # unchanged
├── runtime/            # unchanged
├── emit.rs             # unchanged
├── data_section.rs     # unchanged
├── platform.rs         # unchanged
├── ffi.rs              # unchanged
├── frame.rs            # was codegen_ir/frame.rs
├── block_emit.rs       # was codegen_ir/block_emit.rs
├── context.rs          # was codegen_ir/context.rs
├── value_placement.rs  # was codegen_ir/value_placement.rs
├── lower_inst/         # was codegen_ir/lower_inst/
│   ├── arithmetic.rs
│   ├── arrays.rs
│   ├── calls.rs
│   ├── ...
└── lower_term.rs       # was codegen_ir/lower_term.rs
```

- [ ] **Step 2: Execute the moves**

```bash
git mv src/codegen_ir/frame.rs src/codegen/frame.rs
git mv src/codegen_ir/block_emit.rs src/codegen/block_emit.rs
git mv src/codegen_ir/context.rs src/codegen/context.rs
git mv src/codegen_ir/value_placement.rs src/codegen/value_placement.rs
git mv src/codegen_ir/lower_inst src/codegen/lower_inst
git mv src/codegen_ir/lower_term.rs src/codegen/lower_term.rs
git rm src/codegen_ir/mod.rs
rmdir src/codegen_ir
```

- [ ] **Step 3: Fix imports**

```bash
grep -rln "crate::codegen_ir" src/ tests/ | xargs sed -i '' 's/crate::codegen_ir/crate::codegen/g'   # macOS
# or on Linux:
# grep -rln "crate::codegen_ir" src/ tests/ | xargs sed -i 's/crate::codegen_ir/crate::codegen/g'
```

Edit `src/codegen/mod.rs` to expose the renamed submodules.

Edit `src/lib.rs` to remove `pub mod codegen_ir`.

- [ ] **Step 4: Build**

```bash
cargo build
```

Fix the remaining stragglers.

- [ ] **Step 5: Full test gate**

```bash
cargo test
cargo test -- --include-ignored
./scripts/test-linux-x86_64.sh
./scripts/test-linux-arm64.sh
```

- [ ] **Step 6: Commit**

```bash
git commit -m "refactor: rename codegen_ir to codegen"
```

---

## Task 4: Documentation consolidation

**Files:**
- Modify: `docs/internals/the-codegen.md`
- Move: `docs/internals/legacy-codegen.md` (from old `the-codegen.md` contents)
- Modify: `docs/internals/the-ir.md`
- Modify: `docs/internals/architecture.md`
- Modify: `docs/internals/how-elephc-works.md`
- Modify: `docs/README.md`

- [ ] **Step 1: Preserve the historical doc**

```bash
cp docs/internals/the-codegen.md docs/internals/legacy-codegen.md
```

Open `legacy-codegen.md`. Edit the frontmatter:
- title: "Legacy AST → ASM emitter (historical)"
- description: "Description of the AST-walking emitter that was the default through v0.23. Retained for historical reference."
- sidebar.order: 100 (last)

Add a top-line note:

```markdown
> **Status:** Removed in v0.26.0. The current codegen pipeline is documented in
> [the-codegen.md](the-codegen.md) and [the-ir.md](the-ir.md). This page is
> retained for historical context only.
```

- [ ] **Step 2: Rewrite `the-codegen.md`**

The new `the-codegen.md` describes the IR-based pipeline:
1. EIR functions arrive from `crate::ir_lower`.
2. IR-level optimization passes run via the pass pipeline.
3. Linear-scan register allocation produces an `Allocation`.
4. Block-by-block emission lowers each instruction to ASM using the same ABI helpers and runtime calls as before.

- [ ] **Step 3: Cross-link**

In `docs/internals/the-ir.md`, add a "See also" footer linking to `the-codegen.md`. In `the-codegen.md`, add the same back-link.

- [ ] **Step 4: Update `architecture.md`**

The pipeline diagram now includes "AST → EIR" and "EIR → ASM" boxes instead of one monolithic "AST → ASM" box.

- [ ] **Step 5: Update `docs/README.md`**

Add `the-ir.md` to the internals index. Add `legacy-codegen.md` under a "Historical" subsection.

- [ ] **Step 6: Commit**

```bash
git add docs/
git commit -m "docs: consolidate codegen documentation around the IR pipeline"
```

---

## Task 5: Final verification

**Files:** the whole repo

- [ ] **Step 1: Run every gate**

```bash
cargo build
cargo build --release
cargo test
cargo test -- --include-ignored
./scripts/test-linux-x86_64.sh
./scripts/test-linux-arm64.sh
./scripts/run-benchmarks.sh
```

- [ ] **Step 2: Audit zero-warning policy**

`cargo build` must produce no warnings. Fix or `#[allow]` any that surface.

- [ ] **Step 3: Audit assembly-comment policy**

`CLAUDE.md` requires every `emitter.instruction(...)` to have a `//` comment at column 81. Run the audit script from `CLAUDE.md`:

```bash
python3 -c "
import os
for root, _, files in os.walk('src'):
    for fn in files:
        if not fn.endswith('.rs'): continue
        path = os.path.join(root, fn)
        with open(path) as f:
            for i, line in enumerate(f, 1):
                if 'emitter.instruction' in line and '//' in line:
                    pos = line.rstrip().index('//')
                    if pos != 80 and len(line[:pos].rstrip()) < 80:
                        print(f'{path}:{i}: // at col {pos+1}')
"
```

Fix any drifted lines.

- [ ] **Step 4: Audit file-size policy**

```bash
find src -name '*.rs' -exec wc -l {} \; | sort -nr | head -20
```

Any file >500 LOC that mixes responsibilities should be split. Cohesive single-feature leaves above the threshold are allowed (per `CLAUDE.md` policy).

- [ ] **Step 5: Audit Rust module preamble policy**

Every `*.rs` file in `src/` must start with a `//!` preamble. Run:

```bash
for f in $(find src -name '*.rs'); do
    head -1 "$f" | grep -q '^//!' || echo "missing preamble: $f"
done
```

Fix any misses.

- [ ] **Step 6: Bench result archive**

Take a clean benchmark run on macos-aarch64 and linux-x86_64. Save the JSON outputs as `benchmarks/results/v0.26.0-final.json`. Commit.

- [ ] **Step 7: Update ROADMAP**

Mark all v0.24, v0.25, v0.26 items complete. The next milestone is the v1.0 freeze.

- [ ] **Step 8: Final release commit**

```bash
git commit -m "chore: finalize v0.26.0 (IR backend default, legacy removed)"
```

Tag and release per project convention.

---

## Exit criteria

- Legacy AST emitter completely removed
- `src/codegen_ir/` renamed to `src/codegen/`
- Documentation reflects the IR-based pipeline as the only pipeline
- All audit checks pass: zero warnings, assembly-comment alignment, module preambles, file-size policy
- Benchmark results archived
- ROADMAP up to date
- The project is ready for the v1.0 freeze pass
