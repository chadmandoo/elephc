# Phase 08 — CSE, LICM, and Inlining

> **For agentic workers:** Add global IR-level optimizations: common subexpression elimination, loop-invariant code motion, and small-function inlining. Expected cumulative gain: 10–20% on loop-heavy workloads on top of Phase 07.

**Goal:** Implement three optimizations that need basic-block reasoning and dominance information — operations that AST-level passes cannot do.

**Architecture:** Three new IR passes plus a dominance analysis. All passes are *opt-in* via the existing `PassConfig` from Phase 07 and run in the fixed-point pipeline.

**Tech Stack:** Rust, existing IR module. No new dependencies.

---

## File Structure

- Create: `src/ir_passes/dominance.rs` — dominator tree, dominance frontier, immediate-dominator table
- Create: `src/ir_passes/cse.rs` — common subexpression elimination
- Create: `src/ir_passes/licm.rs` — loop-invariant code motion
- Create: `src/ir_passes/loops.rs` — natural loop detection (back edges, headers, loop bodies)
- Create: `src/ir_passes/inliner.rs` — inline candidate analysis and inlining transformation
- Modify: `src/ir_passes/mod.rs` — re-export
- Modify: `src/ir_passes/pass_driver.rs` — add Phase 08 passes to `PassConfig`
- Modify: `src/pipeline.rs` — call the inliner before the rest of the pass pipeline

---

## Task 1: Dominance analysis

**Files:**
- Create: `src/ir_passes/dominance.rs`
- Test: `src/ir_passes/tests/dominance_test.rs`

- [ ] **Step 1: Failing test**

```rust
#[test]
fn entry_dominates_all_blocks() {
    let f = build_diamond_cfg();
    let dom = compute_dominance(&f);
    let entry = f.entry;
    for b in &f.blocks {
        assert!(dom.dominates(entry, b.id));
    }
}

#[test]
fn diamond_blocks_dominator_is_entry() {
    let f = build_diamond_cfg();
    let dom = compute_dominance(&f);
    let merge = f.blocks.last().unwrap().id;
    assert_eq!(dom.immediate_dominator(merge), Some(f.entry));
}
```

- [ ] **Step 2: Implement**

Use the Lengauer-Tarjan algorithm or the simpler iterative Cooper-Harvey-Kennedy method. The latter is ~30 LOC and good enough for our small CFGs.

```rust
//! Purpose:
//! Computes the immediate-dominator table and offers dominance queries.
//!
//! Called from:
//! - `crate::ir_passes::cse::run`
//! - `crate::ir_passes::licm::run`
//!
//! Key details:
//! - Cooper-Harvey-Kennedy iterative dominators; small CFG so cost is negligible.

use std::collections::HashMap;

use crate::ir::{BlockId, Function};

pub struct Dominance {
    pub idom: HashMap<BlockId, Option<BlockId>>,
    pub depth: HashMap<BlockId, u32>,
}

impl Dominance {
    pub fn immediate_dominator(&self, b: BlockId) -> Option<BlockId> {
        self.idom.get(&b).copied().flatten()
    }
    pub fn dominates(&self, a: BlockId, b: BlockId) -> bool {
        if a == b { return true; }
        let mut cur = self.idom.get(&b).copied().flatten();
        while let Some(c) = cur {
            if c == a { return true; }
            cur = self.idom.get(&c).copied().flatten();
        }
        false
    }
}

pub fn compute_dominance(func: &Function) -> Dominance {
    unimplemented!()
}
```

- [ ] **Step 3 / 4 / 5: Test, pass, commit**

```bash
git commit -m "feat(ir_passes): dominance analysis"
```

---

## Task 2: Loop detection

**Files:**
- Create: `src/ir_passes/loops.rs`
- Test: `src/ir_passes/tests/loops_test.rs`

- [ ] **Step 1: Failing tests**: detect a simple `while` loop and a nested loop.

- [ ] **Step 2: Implement**

Identify back edges (edges `u -> v` where `v` dominates `u`). Each back edge defines a natural loop: the set of blocks from which the source `u` is reachable without going through `v`.

```rust
pub struct LoopInfo {
    pub header: BlockId,
    pub body: HashSet<BlockId>,
    pub back_edges: Vec<(BlockId, BlockId)>,
    pub preheader: Option<BlockId>, // synthesized by LICM if needed
}
```

- [ ] **Step 3 / 4 / 5**: Test and commit.

---

## Task 3: Common Subexpression Elimination

**Files:**
- Create: `src/ir_passes/cse.rs`
- Test: `src/ir_passes/tests/cse_test.rs`

CSE finds two instructions that produce the same value and replaces uses of the second with the first.

Conservative version (per-block):
- Within each block, hash instructions by `(op, operands, immediate)`.
- If a later instruction matches an earlier one *and* its effects are `PURE`, replace its result with the earlier one and drop the later instruction.

Global version (cross-block, dominance-aware):
- Same hashing.
- When two matching instructions are in different blocks, only replace if the earlier dominates the later.
- Effects must remain `PURE` (or contain no observable bits).

Start with per-block CSE; add cross-block CSE in a follow-up commit.

- [ ] **Step 1**: failing tests for at least these patterns:
  - `v1 = load_local slot[a]; v2 = load_local slot[a]` → `v2` becomes `v1`. (`LoadLocal` is `READS_LOCAL`, not pure — but it is safe if no `StoreLocal slot[a]` lies between def of `v1` and use point. The pass tracks per-block stores.)
  - `v1 = iadd p0, p1; v2 = iadd p0, p1` → `v2` becomes `v1`.
  - `v1 = array_get arr, 0; v2 = array_get arr, 0` — NOT CSE'd (effect = `READS_HEAP | MAY_FATAL`); careful here.

- [ ] **Step 2 / 3 / 4: Implement**

- [ ] **Step 5: Commit**

```bash
git commit -m "feat(ir_passes): common subexpression elimination"
```

---

## Task 4: Loop-Invariant Code Motion

**Files:**
- Create: `src/ir_passes/licm.rs`
- Test: `src/ir_passes/tests/licm_test.rs`

LICM hoists `PURE` instructions out of loops when:
1. The instruction's operands are all loop-invariant (defined outside the loop or loop-invariant themselves).
2. The instruction has no side effects (`Effects::PURE`).
3. The instruction dominates all loop exits *OR* it is safe to execute speculatively (always true for pure instructions).

The pass needs a *preheader* block — a block on the loop's entry edge with no other predecessors — to hoist into. If the loop doesn't have one, the pass synthesizes one.

Patterns this catches that AST-level passes miss:
- Loop-invariant arithmetic: `for ($i = 0; $i < count($a); $i++) { $x = $a[0] + 1; ... }` — the `+ 1` and the `array_get a[0]` (if proven side-effect-free for the loop's array) are hoisted.
- Loop-invariant `count($a)`: when the loop doesn't mutate `$a`, the array length read is hoisted.

Care:
- `count()` reads heap — not strictly pure. Adding a "safe within loop body" predicate based on memory dependencies is more sophisticated than the basic pass. For Phase 08, only hoist `PURE` operations. Memory-aware LICM is a v0.26 follow-up.

- [ ] **Step 1: Failing tests** (3-5 patterns)
- [ ] **Step 2 / 3 / 4: Implement**

```rust
//! Purpose:
//! Hoists pure loop-invariant instructions out of natural loops.
//!
//! Called from:
//! - `crate::ir_passes::pass_driver` when `run_licm` is enabled.
//!
//! Key details:
//! - Synthesizes a preheader if the loop entry edge has multiple predecessors.
//! - Only hoists `Effects::PURE` instructions; memory-effect-aware LICM is a
//!   v0.26 follow-up.

pub fn run(func: &mut crate::ir::Function) -> super::pass_driver::Changed {
    // 1. compute dominance
    // 2. detect loops
    // 3. for each loop, find invariant instructions
    // 4. ensure preheader exists; if not, synthesize one
    // 5. move invariants to preheader, in dependency order
    unimplemented!()
}
```

- [ ] **Step 5: Commit**

```bash
git commit -m "feat(ir_passes): loop-invariant code motion"
```

---

## Task 5: Small-function inliner

**Files:**
- Create: `src/ir_passes/inliner.rs`
- Test: `src/ir_passes/tests/inliner_test.rs`

The inliner picks **small callees with no recursion and no exception handlers** and substitutes their bodies into the call site. Benefits:

- Eliminates call/return overhead.
- Exposes additional CSE/LICM opportunities post-inline.
- Allows the register allocator to see the inlined body as part of the caller's live ranges.

Cost model:

- **Size threshold**: callee instruction count ≤ `INLINE_THRESHOLD` (start at 24 instructions).
- **No-recursion**: callee does not reach itself transitively in the call graph.
- **No exception handlers**: callee has no `Throw`, no `Try`. (Inlining a function that may throw makes the caller's stack-unwinding contract more complex; defer this case.)
- **No generator / fiber bodies**: not inlinable.
- **No closures with hidden environment unless the captures are loop-invariant or constant**: defer to later.

The pass runs *once* before the rest of the pipeline (not per fixed-point iteration), because each inlining substantially increases function size and IR pass cost.

- [ ] **Step 1: Failing test**

```rust
#[test]
fn inlines_small_pure_callee() {
    // function double($x) { return $x * 2; }
    // function compute() { return double(7); }
    // After inline, compute() should contain "imul 7, 2" directly.
    let module = build_two_function_module();
    inline_pass(&mut module, InlineConfig::default());
    let compute = &module.functions[1];
    let has_call = compute.instructions.iter().any(|i| matches!(i.op, crate::ir::Op::Call));
    assert!(!has_call);
}
```

- [ ] **Step 2: Implement**

```rust
//! Purpose:
//! Substitutes small callees into call sites to eliminate call overhead and
//! expose downstream optimization.
//!
//! Called from:
//! - `crate::pipeline::compile()` once after AST → IR lowering, before the
//!   IR pass pipeline.
//!
//! Key details:
//! - Inlining renumbers ValueIds, BlockIds, slot indices. The transformation
//!   must rebuild internal tables consistently.

use crate::ir::{Function, Module};

pub struct InlineConfig {
    pub size_threshold: usize,
    pub max_inline_depth: usize,
}

impl Default for InlineConfig {
    fn default() -> Self {
        Self { size_threshold: 24, max_inline_depth: 3 }
    }
}

pub fn inline_pass(module: &mut Module, cfg: InlineConfig) {
    let call_graph = build_call_graph(module);
    let candidates = select_inline_candidates(module, &call_graph, &cfg);
    for cand in candidates {
        inline_at_callsites(module, cand);
    }
}

fn build_call_graph(_m: &Module) -> CallGraph { unimplemented!() }
fn select_inline_candidates(_m: &Module, _cg: &CallGraph, _cfg: &InlineConfig) -> Vec<InlineCandidate> { unimplemented!() }
fn inline_at_callsites(_m: &mut Module, _c: InlineCandidate) { unimplemented!() }

struct CallGraph;
struct InlineCandidate;
```

- [ ] **Step 3 / 4: Test and verify**

- [ ] **Step 5: Commit**

```bash
git commit -m "feat(ir_passes): inline small pure functions"
```

---

## Task 6: Integration and benchmark gate

**Files:**
- Modify: `src/pipeline.rs`
- Modify: `src/ir_passes/pass_driver.rs`

- [ ] **Step 1: Wire passes into pipeline**

```rust
// Phase 08 pipeline shape
let mut module = ir_lower::lower_program(/* ... */);
ir_passes::inliner::inline_pass(&mut module, InlineConfig::default());

for func in module.functions.iter_mut().chain(module.class_methods.iter_mut()) {
    ir_passes::pass_driver::run_pass_pipeline(func, PassConfig::all_phase_08());
}

// Phase 06 register allocation runs as before.
```

`PassConfig::all_phase_08()` enables Phase 07 passes plus CSE and LICM.

- [ ] **Step 2: Full test gate**

```bash
cargo test
cargo test -- --include-ignored
./scripts/test-linux-x86_64.sh
./scripts/test-linux-arm64.sh
```

Expected: all green.

- [ ] **Step 3: Benchmark gate**

```bash
./scripts/run-benchmarks.sh --backend ir --baseline phase7
```

Expected gains:
- **Tight loop benchmarks** (fibonacci, prime sieve, mandelbrot): ≥10% over Phase 07.
- **Function-call-heavy benchmarks**: ≥15% over Phase 07 (inlining payoff).
- **I/O-bound or builtin-bound benchmarks**: ≤5% over Phase 07 (CSE/LICM payoff small here).

If gain is below target, prioritize debugging:
1. Verify each pass is firing — pass_driver instrumentation per-pass change counts.
2. Look at `--emit-asm` output of a hot benchmark function before and after to ensure inlining and hoisting actually happened.
3. Check the register allocator isn't getting *worse* (more spills) on inlined bodies. If so, raise the int pool or refine the inline threshold.

- [ ] **Step 4: Commit**

```bash
git commit -m "feat(ir_passes): integrate phase 08 pipeline (CSE, LICM, inlining)"
```

---

## Task 7: Documentation

**Files:**
- Modify: `docs/internals/the-ir.md`

- [ ] **Step 1: Document passes**

Add an "Optimization passes" section listing each pass, its purpose, the patterns it catches, and the expected gain. Cross-reference `src/ir_passes/` files.

- [ ] **Step 2: Commit**

```bash
git commit -m "docs: document IR optimization passes"
```

---

## Exit criteria

- CSE, LICM, inlining integrated
- All tests green on macOS and Docker Linux gates
- Benchmark suite shows ≥10% additional improvement on loop-heavy / call-heavy benchmarks vs Phase 07 baseline
- Cumulative improvement vs Phase 04 baseline: ≥30% on compute benchmarks
- Documentation updated
- Zero compiler warnings
