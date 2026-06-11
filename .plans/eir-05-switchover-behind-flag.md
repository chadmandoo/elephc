# Phase 05 - Switch EIR Backend to Default

> **For agentic workers:** This phase ships the EIR backend as the user-facing
> default. Keep the legacy AST backend reachable through an explicit
> `--ast-backend` fallback while the old emitter remains in-tree. No
> optimization work in this phase.

**Goal:** Move the default backend to EIR now that parity gates are green. Keep
the legacy backend available as an escape hatch until real-world validation is
complete. Remove it in Phase 09.

**Architecture:** `src/cli.rs` owns backend selection, and `src/pipeline.rs`
uses that selection to choose AST -> ASM or AST -> EIR -> ASM. CI should run
the EIR backend on the normal path and keep explicit legacy fallback coverage.

**Tech Stack:** Rust, existing CLI argument parser, existing CI configuration.

---

## File Structure

No new modules. Edits to:

- Modify: `src/cli.rs` - backend flags and default
- Modify: `src/pipeline.rs` - backend selection plumbing if needed
- Modify: `docs/internals/the-codegen.md` - document EIR as the default backend
- Modify: `docs/internals/the-ir.md` - promote from preview language to default-backend language
- Modify: `.github/workflows/*.yml` or equivalent - CI coverage for default EIR and legacy fallback
- Modify: `Cargo.toml` - version bump once Phase 05 ships
- Modify: `ROADMAP.md` - tick off Phase 05 entries

---

## Task 1: Make EIR the default backend

**Files:**
- Modify: `src/cli.rs`
- Modify: `src/pipeline.rs`
- Modify: `docs/internals/the-ir.md`

- [ ] **Step 1: Change backend selection defaults**

In `src/cli.rs`, make EIR the default backend. Keep:

- `--ir-backend`: explicit selection of the default EIR backend
- `--ast-backend`: explicit fallback to the legacy AST backend

If both flags are set, fail with:

```text
cannot use --ir-backend and --ast-backend together
```

- [ ] **Step 2: Document backend selection**

Add a "Backend selection" section to `docs/internals/the-ir.md`:

```markdown
## Backend selection

The compiler currently supports two backends:

- EIR backend (default): lowers the AST to EIR first, then emits ASM from EIR
- `--ast-backend`: legacy fallback that walks the AST directly and emits ASM

Use `--ir-backend` to select the default explicitly. Use `--ast-backend` only
as a temporary fallback while the legacy emitter remains in-tree.

The EIR backend is feature-complete against the supported test matrix. It
currently does not register-allocate; register allocation is a later v0.24.x
task.
```

- [ ] **Step 3: Commit**

```bash
git add src/cli.rs src/pipeline.rs docs/internals/the-ir.md
git commit -m "feat: switch default backend to eir"
```

---

## Task 2: CI default-EIR and legacy fallback coverage

**Files:**
- Modify: `.github/workflows/ci.yml`

- [ ] **Step 1: Audit existing CI**

Run `find .github/workflows -name '*.yml'`, or check the repository's
equivalent CI configuration.

- [ ] **Step 2: Run ordinary CI with the default EIR backend**

Ensure the normal CI path runs `cargo test` without selecting the legacy
backend. This makes EIR the default gate.

- [ ] **Step 3: Add explicit legacy fallback coverage**

Add a smaller explicit legacy job or matrix entry that invokes the compiler
with `--ast-backend` for the codegen/parity surfaces that still need fallback
coverage.

- [ ] **Step 4: Add an EIR benchmark job**

Benchmarks already exist. Add a job that runs them with the default EIR backend
and stores results in `benchmarks/results/ir/`.

- [ ] **Step 5: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: make eir the default backend gate"
```

---

## Task 3: Mark legacy AST backend deprecated

**Files:**
- Modify: `src/cli.rs`
- Modify: `src/codegen/mod.rs`
- Modify: `docs/internals/the-codegen.md`

- [ ] **Step 1: Emit a deprecation note**

When `--ast-backend` is passed, print to stderr:

```text
warning: --ast-backend is deprecated and will be removed in v0.26.0. The EIR
backend is now the default. See docs/internals/the-ir.md for details.
```

Only emit once per compilation. Do not fail.

- [ ] **Step 2: Add `#[deprecated]` to crate-internal entry points used only by the legacy backend**

Only on public-ish entry points such as `codegen::generate_user_asm` and
`codegen::generate`. Internal helpers stay untouched.

- [ ] **Step 3: Update `the-codegen.md`**

At the top of `docs/internals/the-codegen.md`, explain that EIR is the default
backend and that the legacy AST backend remains temporarily documented only as
a fallback implementation.

- [ ] **Step 4: Commit**

```bash
git add src/cli.rs src/codegen/mod.rs docs/internals/the-codegen.md
git commit -m "chore: mark ast backend deprecated"
```

---

## Task 4: Release notes and version

**Files:**
- Modify: `Cargo.toml`
- Modify: release notes location, if present
- Modify: `ROADMAP.md`

- [ ] **Step 1: Write release notes**

Add a "Backend rework - what changed and why" section to the v0.24.0 release
notes. Explain:

- EIR is now the default backend
- `--ir-backend` remains accepted as an explicit default selection
- `--ast-backend` is the temporary legacy fallback
- register allocation and IR optimization are still future work

- [ ] **Step 2: Bump version**

```toml
[package]
version = "0.24.0"
```

- [ ] **Step 3: Update ROADMAP**

Mark EIR parity and the default switch completed under v0.24.x. Leave register
allocation and register-pressure mitigation as the remaining v0.24.x work.

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml ROADMAP.md
git commit -m "chore: prepare v0.24 eir default"
```

---

## Exit Criteria

- EIR backend is the default backend.
- `--ir-backend` still works as an explicit default selection.
- `--ast-backend` still works as a legacy fallback and warns on use.
- Normal CI uses the EIR backend.
- Legacy fallback has explicit targeted CI coverage.
- Benchmark suite has an EIR baseline.
- Documentation reflects the new default.
- ROADMAP is up to date.
