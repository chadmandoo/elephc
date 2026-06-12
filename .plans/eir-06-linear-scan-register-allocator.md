# Phase 06 — Linear-Scan Register Allocator

> **For agentic workers:** Replace the "every SSA value gets a stack slot" placement from Phase 04 with a real register allocator. This phase produces the first significant performance gain — expect 15–30% on compute benchmarks.

**Goal:** Implement linear-scan register allocation (Poletto & Sarkar, 1999) over EIR functions, producing per-instruction register assignments. The IR backend reads the allocation result and emits ASM that places values in registers when possible and spills only when register pressure demands.

**Architecture:** Three new modules under `src/ir_passes/`: liveness analysis, interval construction, allocator core. Output is an `Allocation` table keyed by `ValueId` indicating *register*, *stack slot*, or both (for spilled values). The `codegen_ir` backend reads `Allocation` and emits accordingly.

**Tech Stack:** Rust. Existing IR module. No new dependencies.

---

## File Structure

- Create: `src/ir_passes/mod.rs` — module entry, pass registry
- Create: `src/ir_passes/liveness.rs` — backward dataflow, per-block live-in/live-out, per-instruction live ranges
- Create: `src/ir_passes/intervals.rs` — `LiveInterval { value, start, end, type }`, interval list construction
- Create: `src/ir_passes/regalloc.rs` — linear-scan algorithm with separate int and float register pools
- Create: `src/ir_passes/allocation.rs` — `Allocation` table consumed by the backend
- Create: `src/ir_passes/tests/` — unit tests on hand-built EIR
- Modify: `src/lib.rs` — add `pub mod ir_passes;`
- Modify: `src/codegen_ir/value_placement.rs` — consume `Allocation` if present, fall back to old behavior if not
- Modify: `src/codegen_ir/lower_inst/*` — when emitting, ask the placement: "where is value X?" instead of always loading from stack
- Modify: `src/codegen_ir/lower_term.rs` — block-parameter parallel move uses the allocator's per-value answer
- Modify: `src/pipeline.rs` — invoke the pass on each function after lowering, before codegen

---

## Task 1: Implement liveness analysis

**Files:**
- Create: `src/ir_passes/mod.rs`
- Create: `src/ir_passes/liveness.rs`
- Test: `src/ir_passes/tests/liveness_test.rs`

- [ ] **Step 1: Module scaffold**

```rust
// src/ir_passes/mod.rs
//! Purpose:
//! IR-level analyses and transformations. Currently holds liveness, intervals,
//! and the linear-scan register allocator.
//!
//! Called from:
//! - `crate::pipeline::compile()` when `--ir-backend` is in use.
//!
//! Key details:
//! - Passes are read-only or produce sidecar tables (e.g., `Allocation`).
//!   They do not mutate `Function` directly.

mod allocation;
mod intervals;
mod liveness;
mod regalloc;

#[cfg(test)]
mod tests;

pub use allocation::{Allocation, Placement};
pub use intervals::{IntervalIndex, LiveInterval};
pub use liveness::{LivenessInfo, compute_liveness};
pub use regalloc::allocate_registers;
```

- [ ] **Step 2: Failing test**

```rust
// src/ir_passes/tests/liveness_test.rs
#[test]
fn liveness_basic_sequence() {
    // Build:
    //   v0 = const_i64 1
    //   v1 = const_i64 2
    //   v2 = iadd v0, v1
    //   return v2
    let func = build_basic_add_function();
    let liveness = compute_liveness(&func);
    let block0 = func.blocks[0].id;
    let live_out = liveness.live_out_of(block0);
    assert!(live_out.is_empty(), "no values escape an entry-only function");

    let live_after_v0 = liveness.live_after_instruction_in_block(block0, 0);
    assert!(live_after_v0.contains(&v0_id()), "v0 alive after its def, until v2");
}
```

- [ ] **Step 3: Implement backward dataflow**

```rust
// src/ir_passes/liveness.rs
//! Purpose:
//! Backward dataflow per-block liveness analysis on EIR functions. Computes
//! live-in / live-out / per-instruction live ranges.
//!
//! Called from:
//! - `crate::ir_passes::intervals::build_intervals`
//!
//! Key details:
//! - SSA-lite with block params: a block param is "defined" at block entry;
//!   block arguments at a branch are "uses" at the terminator point.
//! - Iterative until fixed point; CFG is small so worklist is plenty.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::ir::{BlockId, Function, Terminator, ValueId};

pub struct LivenessInfo {
    pub live_in: HashMap<BlockId, HashSet<ValueId>>,
    pub live_out: HashMap<BlockId, HashSet<ValueId>>,
}

impl LivenessInfo {
    pub fn live_out_of(&self, block: BlockId) -> &HashSet<ValueId> {
        self.live_out.get(&block).expect("missing block in liveness")
    }

    pub fn live_after_instruction_in_block(
        &self,
        block: BlockId,
        inst_index_in_block: u32,
    ) -> HashSet<ValueId> {
        // Per-instruction liveness is computed on demand from block live-out
        // by replaying the block backward up to the requested instruction.
        // Implementation detail in `liveness.rs`.
        unimplemented!()
    }
}

pub fn compute_liveness(func: &Function) -> LivenessInfo {
    // Initialize live_in[B] = uses(B) - defs_before_use(B); live_out[B] = empty
    // Loop until no changes:
    //   live_out[B] = union of live_in over successors (with branch args
    //                 substituted for block-param defs)
    //   live_in[B] = uses(B) U (live_out[B] - defs(B))
    let mut info = LivenessInfo {
        live_in: HashMap::new(),
        live_out: HashMap::new(),
    };
    for b in &func.blocks {
        info.live_in.insert(b.id, HashSet::new());
        info.live_out.insert(b.id, HashSet::new());
    }

    let succs = compute_successors(func);

    let mut worklist: VecDeque<BlockId> = func.blocks.iter().map(|b| b.id).collect();
    while let Some(b) = worklist.pop_front() {
        let block = &func.blocks[b.as_raw() as usize];
        let mut new_live_out = HashSet::new();
        for s in succs.get(&b).into_iter().flatten() {
            let s_live_in = info.live_in.get(s).unwrap().clone();
            new_live_out.extend(s_live_in);
        }
        // Subtract: block-param "defs" at successor s correspond to branch
        // args at this block's terminator. Replace them.
        new_live_out = substitute_branch_args(func, b, new_live_out);

        let mut new_live_in = uses_of_block(func, b);
        for v in &new_live_out {
            if !defs_of_block(func, b).contains(v) {
                new_live_in.insert(*v);
            }
        }

        let changed = info.live_in.get(&b) != Some(&new_live_in)
            || info.live_out.get(&b) != Some(&new_live_out);
        info.live_in.insert(b, new_live_in);
        info.live_out.insert(b, new_live_out);
        if changed {
            for &pred in &predecessors_of(func, b, &succs) {
                worklist.push_back(pred);
            }
        }
    }
    info
}

// Helper signatures only — full implementation goes here.
fn compute_successors(_f: &Function) -> HashMap<BlockId, Vec<BlockId>> { unimplemented!() }
fn substitute_branch_args(_f: &Function, _b: BlockId, set: HashSet<ValueId>) -> HashSet<ValueId> { set }
fn uses_of_block(_f: &Function, _b: BlockId) -> HashSet<ValueId> { unimplemented!() }
fn defs_of_block(_f: &Function, _b: BlockId) -> HashSet<ValueId> { unimplemented!() }
fn predecessors_of(_f: &Function, _b: BlockId, _succs: &HashMap<BlockId, Vec<BlockId>>) -> Vec<BlockId> { unimplemented!() }
```

- [ ] **Step 4: Implement helpers and run tests**

Fill in the helpers and run `cargo test --lib ir_passes::tests::liveness_test`. Expect pass.

- [ ] **Step 5: Commit**

```bash
git add src/ir_passes/mod.rs src/ir_passes/liveness.rs src/ir_passes/tests/liveness_test.rs src/lib.rs
git commit -m "feat(ir_passes): implement liveness analysis"
```

---

## Task 2: Build live intervals

**Files:**
- Create: `src/ir_passes/intervals.rs`
- Test: `src/ir_passes/tests/intervals_test.rs`

- [ ] **Step 1: Failing test**

```rust
#[test]
fn intervals_for_simple_add() {
    let func = build_basic_add_function();
    let intervals = build_intervals(&func, &compute_liveness(&func));
    assert_eq!(intervals.len(), 3);  // v0, v1, v2
    let v0 = find_interval_for(&intervals, ValueId::from_raw(0));
    let v2 = find_interval_for(&intervals, ValueId::from_raw(2));
    assert!(v0.end <= v2.start, "v0 dies before v2 is defined? no — at iadd v0 is used");
}
```

- [ ] **Step 2: Implement**

```rust
// src/ir_passes/intervals.rs
//! Purpose:
//! Builds linear program-order live intervals from per-block liveness.
//!
//! Called from:
//! - `crate::ir_passes::regalloc::allocate_registers`
//!
//! Key details:
//! - Program order is the topological order used by the backend for block
//!   emission; intervals refer to this order, not the SSA def position.

use crate::ir::{Function, IrType, ValueId};
use crate::ir_passes::liveness::LivenessInfo;

#[derive(Debug, Clone)]
pub struct LiveInterval {
    pub value: ValueId,
    pub ir_type: IrType,
    pub start: u32,   // program-order index of the first use OR def
    pub end: u32,     // program-order index of the last use
}

pub type IntervalIndex = u32;

pub fn build_intervals(func: &Function, liveness: &LivenessInfo) -> Vec<LiveInterval> {
    // Walk blocks in program order. For each value, track the
    // [first_def_pos, last_use_pos] across all blocks where the value is
    // live. Block-param defs are at the block's first program position;
    // branch-arg uses are at the terminator's program position.
    unimplemented!()
}
```

- [ ] **Step 3 / 4: Test, pass, commit**

---

## Task 3: Implement linear scan

**Files:**
- Create: `src/ir_passes/regalloc.rs`
- Create: `src/ir_passes/allocation.rs`
- Test: `src/ir_passes/tests/regalloc_test.rs`

- [ ] **Step 1: Define `Allocation`**

```rust
// src/ir_passes/allocation.rs
//! Purpose:
//! The output of register allocation: per-ValueId placement and per-function
//! spill slot table.
//!
//! Called from:
//! - `crate::codegen_ir::lower_inst::*` to find each operand's location.
//!
//! Key details:
//! - `Placement::Register(name)` means the value lives in that register
//!   between its def and use range.
//! - `Placement::Spilled(offset)` means the value lives at the named stack
//!   offset and must be loaded into a register for each use.

use std::collections::HashMap;

use crate::ir::ValueId;

#[derive(Debug, Clone)]
pub enum Placement {
    Register(&'static str),
    Spilled { offset: i32 },
    SpilledAndReloaded { offset: i32, reload_to: &'static str },
}

pub struct Allocation {
    pub placement: HashMap<ValueId, Placement>,
    pub spill_slot_bytes: usize,
    pub callee_saved_used: Vec<&'static str>,
}
```

- [ ] **Step 2: Linear-scan core**

```rust
// src/ir_passes/regalloc.rs
//! Purpose:
//! Poletto-Sarkar linear-scan register allocator with separate int and float
//! register pools, callee-saved register preservation, and spill heuristics.
//!
//! Called from:
//! - `crate::pipeline::compile()` after AST → IR lowering
//!
//! Key details:
//! - Pools per architecture come from `crate::codegen::abi::registers`.
//!   Reserve scratch registers used by lowering (`x9..x11`) — those stay
//!   excluded from the allocator pool.
//! - Spill heuristic: spill the interval with the largest end position.

use std::collections::BTreeSet;

use crate::codegen::abi::registers;
use crate::codegen::platform::{Arch, Target};
use crate::ir::{Function, IrType};
use crate::ir_passes::allocation::{Allocation, Placement};
use crate::ir_passes::intervals::{build_intervals, LiveInterval};
use crate::ir_passes::liveness::compute_liveness;

pub fn allocate_registers(func: &Function, target: Target) -> Allocation {
    let liveness = compute_liveness(func);
    let mut intervals = build_intervals(func, &liveness);
    intervals.sort_by_key(|iv| iv.start);

    let int_pool = int_register_pool(target);
    let float_pool = float_register_pool(target);
    let mut int_free: BTreeSet<&'static str> = int_pool.iter().copied().collect();
    let mut float_free: BTreeSet<&'static str> = float_pool.iter().copied().collect();
    let mut active: Vec<(LiveInterval, Placement)> = Vec::new();
    let mut alloc = Allocation {
        placement: Default::default(),
        spill_slot_bytes: 0,
        callee_saved_used: Vec::new(),
    };

    for iv in intervals {
        expire_old_intervals(&mut active, iv.start, &mut int_free, &mut float_free);
        let pool = if iv.ir_type.is_float() { &mut float_free } else { &mut int_free };
        if let Some(reg) = pool.iter().next().copied() {
            pool.remove(reg);
            let placement = Placement::Register(reg);
            alloc.placement.insert(iv.value, placement.clone());
            active.push((iv, placement));
            mark_callee_saved_if_needed(&mut alloc, reg, target);
        } else {
            // Spill heuristic: spill the active interval with the largest
            // end position.
            spill_at_interval(&mut active, &mut alloc, &iv);
        }
    }
    alloc
}

fn expire_old_intervals(/* ... */) { unimplemented!() }
fn spill_at_interval(/* ... */) { unimplemented!() }
fn int_register_pool(target: Target) -> Vec<&'static str> {
    match target.arch {
        Arch::AArch64 => vec!["x19","x20","x21","x22","x23","x24","x25","x26","x27","x28", "x12","x13","x14","x15"],
        Arch::X86_64 => vec!["rbx","r12","r13","r14","r15"],
    }
    // Note: x0-x7 / rdi-r9 are reserved for argument passing during calls;
    // the allocator may use them outside live ranges crossing a call.
    // Refinement deferred to a follow-up commit if pressure benchmarks justify.
}
fn float_register_pool(target: Target) -> Vec<&'static str> {
    match target.arch {
        Arch::AArch64 => vec!["d8","d9","d10","d11","d12","d13","d14","d15"],
        Arch::X86_64 => vec![/* XMM scratch set, deferred */],
    }
}
fn mark_callee_saved_if_needed(_alloc: &mut Allocation, _reg: &'static str, _target: Target) {}
```

- [ ] **Step 3: Tests**

Hand-build several EIR functions:
1. A two-add function where everything fits in two registers.
2. A function that requires spilling (10+ live values simultaneously).
3. A function with mixed int and float values.
4. A function with a call mid-live-range (must reserve callee-saved registers across calls).

Verify the produced `Allocation` matches expected placements.

- [ ] **Step 4: Commit**

```bash
git add src/ir_passes/regalloc.rs src/ir_passes/allocation.rs src/ir_passes/tests/regalloc_test.rs
git commit -m "feat(ir_passes): implement linear-scan register allocator"
```

---

## Task 4: Wire allocator into the backend

**Files:**
- Modify: `src/pipeline.rs`
- Modify: `src/codegen_ir/value_placement.rs`
- Modify: `src/codegen_ir/lower_inst/*` (read placement instead of always stack)
- Modify: `src/codegen_ir/lower_term.rs` (parallel move with placement awareness)

- [ ] **Step 1: Invoke allocator after lowering**

In `src/pipeline.rs`:

```rust
let ir_module = ir_lower::lower_program(/* ... */);
let allocations: HashMap<&Function, Allocation> = ir_module
    .functions.iter().chain(ir_module.class_methods.iter())
    .map(|f| (f, ir_passes::allocate_registers(f, target)))
    .collect();
let user_asm = codegen_ir::generate_user_asm_from_ir_with_alloc(&ir_module, &allocations, ...);
```

- [ ] **Step 2: Backend reads `Allocation`**

Rewrite the helper that produces the "where is this value?" answer for the lowering code. If `Allocation` says register, the lowering emits `mov`/`fmov` from that register where the AST emitter would `ldr`. If `Allocation` says spilled, fall back to the Phase 04 stack-slot behavior for that value.

- [ ] **Step 3: Parallel-move scheduling at terminators**

Block-parameter moves are now register-to-register where possible. The parallel-move scheduler must handle cycles (`a -> b, b -> a`) with one scratch register. Implementation: standard "topological sort, break cycles by saving to scratch" pattern.

- [ ] **Step 4: Run full test suite**

Run:
```bash
cargo test
cargo test -- --include-ignored
./scripts/test-linux-x86_64.sh
./scripts/test-linux-arm64.sh
```

Expected: all green. Behavior is unchanged; only the assembly is leaner.

- [ ] **Step 5: Benchmark gate**

Run the benchmark suite:
```bash
./scripts/run-benchmarks.sh --backend ir --baseline phase4
```

Expected: **at least 15% improvement on compute-heavy benchmarks** (fibonacci, mandelbrot, tight numeric loops). If less, diagnose:
- Are spills happening when they shouldn't? Print the `Allocation` and inspect.
- Is the parallel-move scheduler creating redundant `mov`s? Check with `--emit-asm` on a known hot function.
- Is the int pool too small? Try `x12-x15` additions.

- [ ] **Step 6: Commit**

```bash
git add src/pipeline.rs src/codegen_ir/
git commit -m "feat(codegen_ir): consume linear-scan allocation for value placement"
```

---

## Task 5: Refinements (only if benchmarks below target)

**Files:** as needed

If Phase 06 benchmarks are below the ≥15% target, consider these refinements in order:

- [ ] **Refinement A: Reuse caller-saved registers when live ranges don't cross a call**

The default-conservative pool excludes `x0..x7`. Many SSA values never cross a call. Annotate intervals with "crosses call" flag, allow caller-saved regs for non-crossing intervals.

- [ ] **Refinement B: Coalesce moves**

When an SSA value is the target of a single `Mov` op (from Phase 03 ownership ops) and the source's interval doesn't conflict, assign them the same register. Eliminates the `Mov` at lowering.

- [ ] **Refinement C: Better spill heuristic**

Replace "spill the interval with largest end" with "spill the interval with the lowest use frequency in the upcoming live range". Cheap to compute, often better.

Each refinement is its own task with its own benchmark check. Don't bundle them — that makes it hard to attribute the gain.

Commit per refinement:
```bash
git commit -m "perf(ir_passes): use caller-saved regs for non-call-crossing intervals"
```

---

## Exit criteria

- Linear-scan allocator integrated and producing valid placements
- Full test suite green on both backends; IR backend now demonstrates ≥15% speedup on compute benchmarks vs Phase 04 baseline
- Allocator handles all edge cases tested: spilling, float pool, callee-saved across calls, parallel moves at terminators
- Documentation: `docs/internals/the-ir.md` gains a "Register allocation" section
- Zero compiler warnings
