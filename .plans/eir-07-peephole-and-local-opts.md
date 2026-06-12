# Phase 07 — IR-Level Peephole and Local Optimizations

> **For agentic workers:** Add a set of small, local, fixed-point IR optimizations that pay off measurably. Expected gain: 5–10% on top of Phase 06.

**Goal:** Implement on-the-IR optimizations that the AST-level passes cannot reach: redundant move elimination, dead store elimination, identity arithmetic folding, branch chain shortening, and per-block constant propagation.

**Architecture:** Each optimization is a `Pass` over a `Function` producing a new `Function` (or mutating in place where safe). All passes run inside a fixed-point loop until no pass reports changes. Validation runs after every pass during testing.

**Tech Stack:** Rust, EIR module. No new dependencies.

---

## File Structure

- Create: `src/ir_passes/peephole.rs` — peephole patterns
- Create: `src/ir_passes/dead_inst.rs` — DCE on instructions whose result is unused and effect-free
- Create: `src/ir_passes/dead_store.rs` — eliminate stores to locals never re-read
- Create: `src/ir_passes/branch_simplify.rs` — chain-of-branches collapse, constant-cond folding
- Create: `src/ir_passes/identity_fold.rs` — `x + 0`, `x * 1`, `x | 0`, `x ^ 0`, `x - x`, etc.
- Create: `src/ir_passes/move_elimination.rs` — remove `Move`/`Borrow` no-ops that the allocator left untouched
- Create: `src/ir_passes/pass_driver.rs` — fixed-point driver
- Modify: `src/ir_passes/mod.rs` — re-export
- Modify: `src/pipeline.rs` — invoke pass pipeline after lowering, before register allocation

---

## Task 1: Pass driver

**Files:**
- Create: `src/ir_passes/pass_driver.rs`
- Test: `src/ir_passes/tests/pass_driver_test.rs`

- [ ] **Step 1: Failing test**

```rust
#[test]
fn driver_runs_to_fixed_point() {
    let mut func = build_function_with_redundant_pattern();
    let report = run_pass_pipeline(&mut func, PassConfig::default());
    assert!(report.iterations >= 1);
    assert!(report.changed);
    assert!(no_redundant_patterns(&func));
}
```

- [ ] **Step 2: Implement driver**

```rust
//! Purpose:
//! Drives the fixed-point loop over IR optimization passes.
//!
//! Called from:
//! - `crate::pipeline::compile()` after lowering, before register allocation
//!
//! Key details:
//! - Passes report `Changed::Yes` if they modified the function; the driver
//!   re-runs the pipeline until all passes report `Changed::No` in one round.
//! - Capped at `MAX_ITERATIONS` to prevent oscillation bugs.

use crate::ir::Function;

const MAX_ITERATIONS: usize = 16;

#[derive(Debug, Default)]
pub struct PassConfig {
    pub run_peephole: bool,
    pub run_identity_fold: bool,
    pub run_branch_simplify: bool,
    pub run_dead_inst: bool,
    pub run_dead_store: bool,
    pub run_move_elimination: bool,
}

impl PassConfig {
    pub fn all_phase_07() -> Self {
        Self {
            run_peephole: true,
            run_identity_fold: true,
            run_branch_simplify: true,
            run_dead_inst: true,
            run_dead_store: true,
            run_move_elimination: true,
        }
    }
}

pub struct PassReport {
    pub iterations: usize,
    pub changed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Changed { Yes, No }

pub fn run_pass_pipeline(func: &mut Function, cfg: PassConfig) -> PassReport {
    let mut total_changed = false;
    for i in 0..MAX_ITERATIONS {
        let mut iter_changed = false;
        if cfg.run_move_elimination {
            iter_changed |= matches!(super::move_elimination::run(func), Changed::Yes);
        }
        if cfg.run_identity_fold {
            iter_changed |= matches!(super::identity_fold::run(func), Changed::Yes);
        }
        if cfg.run_peephole {
            iter_changed |= matches!(super::peephole::run(func), Changed::Yes);
        }
        if cfg.run_branch_simplify {
            iter_changed |= matches!(super::branch_simplify::run(func), Changed::Yes);
        }
        if cfg.run_dead_inst {
            iter_changed |= matches!(super::dead_inst::run(func), Changed::Yes);
        }
        if cfg.run_dead_store {
            iter_changed |= matches!(super::dead_store::run(func), Changed::Yes);
        }
        total_changed |= iter_changed;
        crate::ir::validate_function(func).expect("invariant broken by IR pass");
        if !iter_changed {
            return PassReport { iterations: i + 1, changed: total_changed };
        }
    }
    panic!("IR pass pipeline did not converge in {} iterations", MAX_ITERATIONS);
}
```

- [ ] **Step 3 / 4 / 5: Test and commit**

```bash
git commit -m "feat(ir_passes): pass driver with fixed-point loop"
```

---

## Task 2: Identity fold

**Files:**
- Create: `src/ir_passes/identity_fold.rs`
- Test: `src/ir_passes/tests/identity_fold_test.rs`

Patterns to fold:

- `IAdd(x, 0)` / `IAdd(0, x)` → `x`
- `ISub(x, 0)` → `x`
- `IMul(x, 1)` / `IMul(1, x)` → `x`
- `IMul(x, 0)` / `IMul(0, x)` → `ConstI64(0)`
- `IBitAnd(x, 0)` → `ConstI64(0)`
- `IBitOr(x, 0)` → `x`
- `IBitXor(x, x)` → `ConstI64(0)`
- `IShl(x, 0)` → `x`, `IShrA(x, 0)` → `x`
- `FMul(x, 1.0)` → `x`; do NOT fold `FAdd(x, 0.0)` (signed zero / -0.0)
- `ICmp(Eq, x, x)` → `ConstI64(1)` if `x` is not `F64` (NaN-aware)

Each pattern is one match arm.

- [ ] **Step 1**: failing tests for at least 8 patterns above.
- [ ] **Step 2 / 3 / 4**: implement and verify.
- [ ] **Step 5: Commit**

```bash
git commit -m "feat(ir_passes): identity-fold pass"
```

---

## Task 3: Peephole patterns

**Files:**
- Create: `src/ir_passes/peephole.rs`
- Test: `src/ir_passes/tests/peephole_test.rs`

Patterns to match:

- **Redundant load after store**: `StoreLocal(slot, v); LoadLocal(slot)` → use `v` directly.
- **Box-unbox cancellation**: `MixedBox(v); MixedUnbox(_, tag=v.tag)` → `v`.
- **Acquire-Release pairs with no use between**: cancel if `v` is not observed otherwise.
- **String literal concat folding**: `StrConcat(ConstStr a, ConstStr b)` → `ConstStr "ab"` (interning into data pool).
- **Coalesced casts**: `IToF; FToI` → identity (only when round-trip is exact, e.g., integer literals).

Each pattern is one helper in `peephole.rs`. The driver calls them all and reports whether anything matched.

- [ ] **Step 1: Failing tests per pattern**
- [ ] **Step 2 / 3 / 4: Implement**
- [ ] **Step 5: Commit**

```bash
git commit -m "feat(ir_passes): peephole pass with five core patterns"
```

---

## Task 4: Branch simplification

**Files:**
- Create: `src/ir_passes/branch_simplify.rs`
- Test: `src/ir_passes/tests/branch_simplify_test.rs`

Patterns:

- **Constant-condition `CondBr`**: if the condition is `ConstI64(0)`, replace with `Br(else_block)`. If `ConstI64(non-zero)`, replace with `Br(then_block)`.
- **Trivially equivalent CondBr branches**: if `then_block == else_block` (and args match), replace with `Br(then_block)`.
- **Empty-block jump-thread**: a block that has no instructions and an unconditional `Br(next)` is collapsed: predecessors branch directly to `next`. Safe only when the block has no parameters whose values come from multiple distinct predecessor sources (otherwise parameter substitution must be applied).
- **Dead block removal**: after simplification, blocks unreachable from the entry are removed.

- [ ] **Step 1 / 2 / 3 / 4 / 5**: TDD pattern.

```bash
git commit -m "feat(ir_passes): branch simplification pass"
```

---

## Task 5: Dead instruction elimination

**Files:**
- Create: `src/ir_passes/dead_inst.rs`
- Test: `src/ir_passes/tests/dead_inst_test.rs`

An instruction is dead when:

1. Its result is unused by any other instruction or terminator.
2. Its effects are `PURE` or limited to `READS_LOCAL` (which is also safe to drop since no observable state changes).

Iterate until fixed point. (The driver gives us fixed point for free; one pass per iteration suffices.)

Patterns to be careful about:
- Calls with unused returns may still have side effects. Check `Effects::may_mutate() || may_observe()` — if either, keep the call.
- `Acquire`/`Release` ops change refcounts; never remove individually. The move-elimination pass handles paired removal.

- [ ] **Step 1 / 2 / 3 / 4 / 5**

```bash
git commit -m "feat(ir_passes): dead instruction elimination"
```

---

## Task 6: Dead store elimination

**Files:**
- Create: `src/ir_passes/dead_store.rs`
- Test: `src/ir_passes/tests/dead_store_test.rs`

A `StoreLocal(slot, v)` is dead when:

1. There is no `LoadLocal(slot)` reachable from this store before either another `StoreLocal(slot, _)` (no reads in between) or end-of-function.
2. The slot is not aliased through `Global`/`Static`/`ByRef` semantics.

Care:
- Slots backing `Global` / `Static` PHP locals can be observed via `LoadGlobal`. The dead-store pass must ignore these slots.
- Slots passed to closures via capture: also not dead — the closure may read them later.

Implementation: per-slot, walk the CFG; mark stores reachable to a load as live; the remaining stores are dead.

- [ ] **Step 1 / 2 / 3 / 4 / 5**

```bash
git commit -m "feat(ir_passes): dead store elimination"
```

---

## Task 7: Move elimination

**Files:**
- Create: `src/ir_passes/move_elimination.rs`
- Test: `src/ir_passes/tests/move_elimination_test.rs`

After register allocation, the lowering may emit `mov reg_a, reg_b` because the allocator placed both ends in different registers. Some of those can be removed:

- `mov reg, reg` (same register) — always remove.
- Paired `Acquire(v); Release(v)` with no intervening use of `v` — remove both.
- `Move(v)` IR ops are no-ops at codegen; remove them from the IR after they've served their semantic-validation purpose. This is a "make it tidy" pass; impact on perf is negligible.

This pass runs both *before* register allocation (to remove pure `Move`/`Borrow` ops that influence interval computation) and *after* (to clean up identity `mov`s the allocator left).

- [ ] **Step 1 / 2 / 3 / 4 / 5**

```bash
git commit -m "feat(ir_passes): move elimination"
```

---

## Task 8: Integrate and benchmark

**Files:**
- Modify: `src/pipeline.rs`
- Test: existing benchmark suite

- [ ] **Step 1: Run the pipeline**

```rust
let mut func = func;  // mutable
ir_passes::pass_driver::run_pass_pipeline(&mut func, PassConfig::all_phase_07());
let alloc = ir_passes::allocate_registers(&func, target);
```

Apply to every function in the module.

- [ ] **Step 2: Full test suite**

```bash
cargo test
cargo test -- --include-ignored
./scripts/test-linux-x86_64.sh
./scripts/test-linux-arm64.sh
```

Expected: all green.

- [ ] **Step 3: Benchmark gate**

Run benchmarks. Expected: **5–10% improvement** over Phase 06 baseline on compute benchmarks, smaller (~2–3%) on benchmarks dominated by builtin calls or I/O.

If less than 3% improvement, investigate which pass is paying off:
1. Run the pipeline with `PassConfig` selectively disabling each pass.
2. Identify the pass with the smallest contribution; ensure it's actually firing on real workloads (instrument the pass driver with per-pass change counts).

- [ ] **Step 4: Commit**

```bash
git commit -m "feat(ir_passes): integrate phase 07 optimization pipeline"
```

---

## Exit criteria

- All passes integrated and gated by validator
- Pipeline converges in ≤16 iterations on every test fixture
- Test suite green
- Benchmark suite shows additional ≥3% gain over Phase 06 baseline (cumulatively, ≥18% vs Phase 04)
- Per-pass change counts logged so future tuning has a baseline
- Documentation: `docs/internals/the-ir.md` gains "Optimization passes" section listing each pass and what it does
