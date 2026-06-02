# Phase 05 — Switchover Behind a Flag, Then by Default

> **For agentic workers:** This phase ships the IR backend to users. First behind the explicit `--ir-backend` flag, then as default, then with the legacy backend marked deprecated. No optimization work in this phase.

**Goal:** Move the user-facing default to the IR backend. Keep the legacy backend reachable for one minor version as an escape hatch. Remove it in Phase 09.

**Architecture:** Single point of control in `src/pipeline.rs` selects backend. CI gates run both backends in parallel until the default switches; after that, CI runs the IR backend on every commit and the legacy backend on a nightly schedule until removal.

**Tech Stack:** Rust, existing CLI argument parser, existing CI configuration.

---

## File Structure

No new modules. Edits to:

- Modify: `src/pipeline.rs` — backend selection
- Modify: `src/cli.rs` — CLI flag default flip
- Modify: `Cargo.toml` — version bump to 0.24.0 once Phase 05 ships
- Modify: `docs/internals/the-codegen.md` — note the dual-backend transition
- Modify: `docs/internals/the-ir.md` — promote from "preview" to "default"
- Modify: `.github/workflows/*.yml` (or equivalent) — CI matrix for both backends during transition
- Modify: `ROADMAP.md` — tick off Phase 05 entries

---

## Task 1: Make `--ir-backend` opt-in, well-documented, and stable

**Files:**
- Modify: `src/cli.rs`
- Modify: `docs/internals/the-ir.md`

- [ ] **Step 1: Document `--ir-backend`**

Add a "Using `--ir-backend`" section to `docs/internals/the-ir.md`:

```markdown
## Using the IR backend

The compiler currently supports two backends:

- `--ast-backend` (default through v0.23.x): walks the AST directly and emits ASM
- `--ir-backend` (opt-in in v0.24.x, default in v0.25.x): lowers the AST to EIR
  first, then emits ASM from EIR

In v0.24.x the IR backend is feature-complete and tested against the full
test suite. It currently does not register-allocate (Phase 06 introduces
that). Performance is comparable to the legacy backend; pick either based on
your needs:

- Use `--ir-backend` to validate the new pipeline on your code
- Use `--ast-backend` if you hit an `--ir-backend` regression and need a fast
  rollback
```

- [ ] **Step 2: Add a CLI conflict check**

If both `--ir-backend` and `--ast-backend` are set, error with `cannot use --ir-backend and --ast-backend together`.

- [ ] **Step 3: Commit**

```bash
git add src/cli.rs docs/internals/the-ir.md
git commit -m "feat(cli): document --ir-backend as opt-in stable in v0.24"
```

---

## Task 2: CI dual-backend matrix

**Files:**
- Modify: `.github/workflows/ci.yml`

- [ ] **Step 1: Audit existing CI**

Run: `find .github/workflows -name '*.yml'` (or check `gitlab-ci.yml` / wherever CI lives). Read the existing matrix.

- [ ] **Step 2: Add backend dimension**

Add a `backend: [ast, ir]` axis to the existing matrix. The job invokes `cargo test` with a `BACKEND` env var that the test harness uses to pick the flag.

For test runners that compile inline source via `compile_and_run`, change the helper to read `BACKEND` and pass `--ast-backend` or `--ir-backend` accordingly. Default to `ast` so local `cargo test` keeps working without surprise.

- [ ] **Step 3: Add a separate IR-only benchmark job**

Benchmarks already exist (`v0.19.x` benchmark suite). Add a new job that runs them with `--ir-backend` and stores results in `benchmarks/results/ir/`. The existing benchmark gate continues to use the AST backend until Phase 05 default switch.

- [ ] **Step 4: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: add dual-backend test matrix"
```

---

## Task 3: Stabilize for one minor version (v0.24.x)

**Files:**
- Modify: `Cargo.toml` (version)
- Modify: `ROADMAP.md`

- [ ] **Step 1: Bump version to 0.24.0**

```toml
# Cargo.toml
[package]
version = "0.24.0"
```

- [ ] **Step 2: Update ROADMAP**

Mark Phase 01–04 items completed under v0.24.x. Leave the default-switch and Phase 06 items as the remaining work for v0.24.x.

- [ ] **Step 3: Tag release**

Coordinate with the project's release procedure (see `scripts/release.sh` or equivalent). The 0.24.0 release ships the IR backend as opt-in.

- [ ] **Step 4: Soak period**

Spend at least **two weeks** on 0.24.0 collecting issues from any users who try `--ir-backend`. During this window, fix anything reported but do *not* flip the default. The soak period exists because the IR backend changes the pipeline shape, and issue patterns from real users will surface differently from the internal test corpus.

If significant issues surface, do not proceed to default switch until they are fixed and a 0.24.1 release lands cleanly.

---

## Task 4: Flip the default

**Files:**
- Modify: `src/pipeline.rs` (default value)
- Modify: `src/cli.rs` (CLI default)
- Modify: `docs/internals/the-codegen.md`
- Modify: `docs/internals/the-ir.md`

- [ ] **Step 1: Change default to IR**

In the argument parser, change the default value of the backend flag from `ast` to `ir`. Keep `--ast-backend` working as an escape hatch.

- [ ] **Step 2: Update documentation**

```markdown
# docs/internals/the-codegen.md (top of file)
## Backend selection

elephc has two backends:
- `--ir-backend` (default since v0.25.0): EIR → ASM
- `--ast-backend` (deprecated, removal planned in v0.26.0): AST → ASM

This page documents the IR backend. The legacy AST backend is documented
historically in `docs/internals/legacy-codegen.md` (created in the v0.26
cycle as part of removal).
```

- [ ] **Step 3: Run full gate**

```bash
cargo build
cargo test
cargo test -- --include-ignored
./scripts/test-linux-x86_64.sh
./scripts/test-linux-arm64.sh
```

All must pass with the IR backend as default.

- [ ] **Step 4: Benchmark gate**

Run the benchmark suite. Verify the IR backend (now default) is within the gate's tolerance. If Phase 06 has shipped before this switch, the IR backend should already be *faster* than the AST backend; the gate ensures no regression.

- [ ] **Step 5: Commit**

```bash
git add src/pipeline.rs src/cli.rs docs/internals/the-codegen.md docs/internals/the-ir.md
git commit -m "feat: switch default backend to --ir-backend"
```

- [ ] **Step 6: Bump version, release 0.25.0**

```toml
version = "0.25.0"
```

---

## Task 5: Mark legacy AST backend deprecated

**Files:**
- Modify: `src/cli.rs`
- Modify: `src/codegen/mod.rs`

- [ ] **Step 1: Emit a deprecation note**

When `--ast-backend` is passed, print to stderr:

```
warning: --ast-backend is deprecated and will be removed in v0.26.0. The IR
backend is now the default. See docs/internals/the-ir.md for details.
```

Only emit once per compilation. Do *not* fail.

- [ ] **Step 2: Add `#[deprecated]` to crate-internal entry points used only by the legacy backend**

Only on the public-ish entry points (`codegen::generate_user_asm`, `codegen::generate`). Internal helpers stay untouched.

- [ ] **Step 3: Update `ROADMAP.md`**

Mark v0.25.x complete and add v0.26.x "Remove legacy backend" as the next milestone (covered in detail by Phase 09 plan).

- [ ] **Step 4: Commit**

```bash
git add src/cli.rs src/codegen/mod.rs ROADMAP.md
git commit -m "chore: mark --ast-backend deprecated in v0.25"
```

---

## Task 6: Communication

This task is non-code but mandatory.

- [ ] **Step 1: Write a release notes entry**

Add a "Backend rework — what changed and why" section to the v0.25.0 release notes (in whatever location the project uses for release notes; probably `CHANGELOG.md` or a `docs/changes/0.25.0.md` page).

- [ ] **Step 2: Reddit/HN post**

Optional but valuable: the project has a Reddit launch tracked in memory at `reference_reddit.md`. A short post explaining "elephc 0.25: new IR backend ships, AST backend deprecated" reaches the audience that benchmarks the project.

---

## Exit criteria

- v0.24.0 shipped with `--ir-backend` opt-in
- v0.24 soak period showed no blocking issues
- v0.25.0 shipped with IR backend as default
- `--ast-backend` still works but warns on use
- Benchmark suite shows no regression vs v0.23 baseline
- Documentation reflects the new default
- ROADMAP updated
