# Phase 04 — IR → ASM Backend (1:1, no optimization)

> **Current status:** Historical phase plan. EIR is now the default and only
> active implementation backend. References below to keeping the legacy backend
> working were Phase 04 parity scaffolding, not current feature-work policy.

> **For agentic workers:** Build a new backend that takes an EIR `Module` and produces assembly. The goal is **semantic equivalence** to the current `src/codegen/` backend on every `tests/codegen/` fixture, NOT byte-identical assembly. Zero new optimizations.

**Goal:** A new module `src/codegen_ir/` that walks EIR and emits ASM identical-in-behavior to today's AST → ASM path. After this phase, choosing the IR backend produces correct programs; choosing the legacy backend continues to work. No default switch yet (that's Phase 05).

**Architecture:** Mirror the structure of `src/codegen/` but consume `Function` instances instead of AST. Reuse `src/codegen/abi/` register helpers and `src/codegen/runtime/` runtime emission unchanged. Reuse `src/codegen/data_section.rs` for the data pool.

**Tech Stack:** Rust. No new dependencies.

---

## File Structure

All new files under `src/codegen_ir/`:

- Create: `src/codegen_ir/mod.rs` — entry `pub fn generate_user_asm_from_ir(...)`
- Create: `src/codegen_ir/context.rs` — backend state: ABI cursor, value→register map, label counter, emitter, frame layout
- Create: `src/codegen_ir/frame.rs` — frame size calculation, prologue/epilogue emission (mirrors `src/codegen/functions/locals.rs` + frame emission)
- Create: `src/codegen_ir/value_placement.rs` — value → register-or-slot mapping for the 1:1 backend (no real register allocator yet — values land in fixed slots, then move into ABI registers at uses)
- Create: `src/codegen_ir/lower_inst.rs` — per-opcode emitter dispatcher
- Create: `src/codegen_ir/lower_inst/arithmetic.rs` — IAdd/ISub/IMul/ISDiv/INeg/bitwise/shifts
- Create: `src/codegen_ir/lower_inst/floats.rs` — FAdd/FSub/.../FPow
- Create: `src/codegen_ir/lower_inst/comparison.rs` — ICmp/FCmp/StrCmp/PhpLooseEq/PhpIdentical/Spaceship
- Create: `src/codegen_ir/lower_inst/conversion.rs` — IToF/FToI/IToStr/.../MixedBox/MixedUnbox/Cast
- Create: `src/codegen_ir/lower_inst/strings.rs` — StrConcat/StrLen/StrCharAt/StrPersist/StrInterpolate
- Create: `src/codegen_ir/lower_inst/arrays.rs` — ArrayNew/Get/Set/Push/CowEnsureUnique
- Create: `src/codegen_ir/lower_inst/hashes.rs` — HashGet*/Set*/KeyExists/Iter*
- Create: `src/codegen_ir/lower_inst/objects.rs` — ObjectNew/PropGet/PropSet/VTableLookup/InstanceOf
- Create: `src/codegen_ir/lower_inst/calls.rs` — Call/IndirectCall/MethodCall/BuiltinCall/RuntimeCall/ExternCall, closure ops
- Create: `src/codegen_ir/lower_inst/ownership.rs` — Acquire/Release/Move/Borrow (no-op for Move/Borrow at runtime)
- Create: `src/codegen_ir/lower_term.rs` — Br/CondBr/Switch/Return/Throw/Fatal/Unreachable
- Create: `src/codegen_ir/block_emit.rs` — block label naming, block ordering, jumps between blocks
- Create: `src/codegen_ir/tests/` — integration tests using `compile_and_run`-style helpers with the IR backend
- Modify: `src/lib.rs` — add `pub mod codegen_ir;`
- Modify: `src/main.rs` — add `mod codegen_ir;` for the binary crate module tree
- Modify: `src/pipeline.rs` — when the IR backend feature is requested, call `codegen_ir::generate_user_asm_from_ir(...)` instead of `codegen::generate_user_asm(...)`

---

## Task 1: Wire the module and add `--ir-backend` CLI flag

**Files:**
- Create: `src/codegen_ir/mod.rs` (skeleton)
- Modify: `src/lib.rs`
- Modify: `src/main.rs` (module declaration)
- Modify: `src/cli.rs` (CLI argument parser)
- Modify: `src/pipeline.rs`

- [ ] **Step 1: Module skeleton**

```rust
// src/codegen_ir/mod.rs
//! Purpose:
//! IR-consuming assembly backend. Produces functionally equivalent ASM to
//! `src/codegen/` while reading from an EIR `Module` instead of an AST.
//!
//! Called from:
//! - `crate::pipeline::compile()` when the `--ir-backend` flag is set.
//!
//! Key details:
//! - Phase 04: 1:1 lowering, no optimization, no register allocation.
//! - Phase 06: adds linear-scan register allocator.
//! - Phase 09: replaces `src/codegen/` as the default.

mod block_emit;
mod context;
mod frame;
mod lower_inst;
mod lower_term;
mod value_placement;

#[cfg(test)]
mod tests;

use crate::codegen::platform::Target;
use crate::ir::Module;

pub fn generate_user_asm_from_ir(module: &Module, _gc_stats: bool, _heap_debug: bool) -> String {
    // Implementation in Task 3.
    unimplemented!("phase 04")
}
```

- [ ] **Step 2: CLI flag**

Add `--ir-backend` (boolean) to `src/cli.rs` and thread it through `CliConfig`. When set:
1. Run frontend + optimizer + AST → IR lowering.
2. Call `codegen_ir::generate_user_asm_from_ir(...)`.
3. Hand the assembly to the existing assembler/linker pipeline (unchanged).
4. Produce a binary as usual.

- [ ] **Step 3: Pipeline switch**

In `src/pipeline.rs::compile`, add a branch:

```rust
let user_asm = if config.ir_backend {
    let ir = crate::ir_lower::lower_program(/* args */);
    crate::codegen_ir::generate_user_asm_from_ir(&ir, gc_stats, heap_debug)
} else {
    crate::codegen::generate_user_asm(/* args */)
};
```

The runtime path (`codegen::generate_runtime`) stays identical regardless of backend choice.

- [ ] **Step 4: Stub test passes**

```rust
// tests/ir_backend_smoke_test.rs
#[test]
#[ignore]
fn ir_backend_hello_world() {
    // Will be unignored in Task 4 once smallest emitters work.
    let out = compile_and_run_with(["--ir-backend"], "<?php echo 42;");
    assert_eq!(out, "42");
}
```

- [ ] **Step 5: Commit**

```bash
git add src/codegen_ir/mod.rs src/lib.rs src/main.rs src/cli.rs src/pipeline.rs tests/ir_backend_smoke_test.rs
git commit -m "feat(codegen_ir): scaffold IR-consuming backend and --ir-backend flag"
```

---

## Task 2: Implement frame and value placement

**Files:**
- Create: `src/codegen_ir/frame.rs`
- Create: `src/codegen_ir/value_placement.rs`
- Test: `src/codegen_ir/tests/frame_test.rs`

The Phase 04 backend does *not* allocate registers. Each EIR value gets a stable stack slot. At each use, the slot is loaded into the canonical result register expected by downstream emitters (mirroring the AST emitter's behavior). This is intentionally "the dumb mapping" — Phase 06 replaces it with linear scan.

- [ ] **Step 1: Implement `ValuePlacement`**

```rust
// src/codegen_ir/value_placement.rs
//! Purpose:
//! Phase 04 placement: every SSA value gets a unique stack slot. Loads to
//! canonical registers happen at use sites.
//!
//! Called from:
//! - `crate::codegen_ir::context` and lowering helpers
//!
//! Key details:
//! - Replaced by linear scan in phase 06. Until then, expect heavy stack
//!   traffic; correctness is the priority.

use std::collections::HashMap;

use crate::ir::{Function, IrType, ValueId};

pub struct ValuePlacement {
    pub slot_of: HashMap<u32, i32>, // ValueId raw -> negative offset from x29
    pub total_slot_bytes: usize,
}

pub fn allocate(func: &Function) -> ValuePlacement {
    let mut placement = ValuePlacement {
        slot_of: HashMap::new(),
        total_slot_bytes: 0,
    };
    let mut offset: i32 = 0;
    for v in &func.values {
        let bytes = bytes_for(v.ir_type);
        if bytes == 0 { continue; }
        offset -= bytes as i32;
        let raw = match &v.def {
            crate::ir::ValueDef::BlockParam { .. } | crate::ir::ValueDef::Instruction { .. } => {
                // raw is the index in func.values
                offset_into_func_values(func, v)
            }
        };
        placement.slot_of.insert(raw, offset);
    }
    placement.total_slot_bytes = (-offset as usize).next_multiple_of(16);
    placement
}

fn bytes_for(t: IrType) -> usize {
    match t {
        IrType::I64 | IrType::F64 | IrType::Heap(_) => 8,
        IrType::Str => 16,
        IrType::Void => 0,
    }
}

fn offset_into_func_values(func: &Function, v: &crate::ir::Value) -> u32 {
    func.values.iter()
        .position(|other| std::ptr::eq(other, v))
        .map(|i| i as u32)
        .unwrap()
}
```

- [ ] **Step 2: Implement frame emission**

```rust
// src/codegen_ir/frame.rs
//! Purpose:
//! Emits function prologue and epilogue using ABI helpers from `crate::codegen::abi`.
//!
//! Called from:
//! - `crate::codegen_ir::context::emit_function`
//!
//! Key details:
//! - Frame size = locals + value-placement slots, 16-byte aligned.

use crate::codegen::abi::{self, frame};
use crate::codegen::emit::Emitter;
use crate::codegen::platform::Arch;
use crate::ir::{Function, LocalKind};

pub fn emit_prologue(emitter: &mut Emitter, func: &Function, frame_bytes: usize) {
    // Mirror src/codegen/functions/* prologue emission. Use abi::frame helpers.
    // ARM64:
    //   sub sp, sp, #N
    //   stp x29, x30, [sp, #N-16]
    //   add x29, sp, #N-16
    // X86_64:
    //   push rbp
    //   mov rbp, rsp
    //   sub rsp, #N
    abi::frame::emit_function_prologue(emitter, frame_bytes);
}

pub fn emit_epilogue(emitter: &mut Emitter, frame_bytes: usize) {
    abi::frame::emit_function_epilogue(emitter, frame_bytes);
}
```

(Confirm helper names by reading `src/codegen/abi/frame.rs` and adjusting; the existing AST backend already has the primitives we need.)

- [ ] **Step 3: Test placement**

A minimal test asserts that allocation for a function with one I64 value yields one 8-byte slot.

- [ ] **Step 4: Commit**

```bash
git add src/codegen_ir/frame.rs src/codegen_ir/value_placement.rs src/codegen_ir/tests/frame_test.rs
git commit -m "feat(codegen_ir): implement Phase 04 value placement and frame emission"
```

---

## Task 3: Implement block emission and terminators

**Files:**
- Create: `src/codegen_ir/block_emit.rs`
- Create: `src/codegen_ir/lower_term.rs`
- Test: `src/codegen_ir/tests/block_test.rs`

- [ ] **Step 1: Failing test**

```rust
#[test]
fn emits_label_for_each_block() {
    let asm = compile_simple_ir_to_asm(/* fn returning const 7 */);
    assert!(asm.contains(".LBB_"));
}
```

- [ ] **Step 2 / 3: Implement**

Block labels: `format!(".LBB_{fn_name}_{block_id}")`. Use `ctx.next_label(prefix)` style with a global counter so labels don't collide across functions (mirroring the existing emitter's label-counter discipline in `src/codegen/context.rs`).

Block ordering: topological, with the entry block first. Fall-through optimization where possible: if the terminator of block N is `Br(N+1)`, omit the branch instruction.

Terminator lowering:
- `Br(target, args)` — move args into the target's parameter slots, then `b .LBB_target`.
- `CondBr(cond, then, else_)` — load `cond` into a register, `cbz reg, else_label`, branch to `then_label` (or fall-through if next block is `then`).
- `Switch` — if dense, emit a jump table from `src/codegen/runtime/data.rs` patterns; otherwise chained `cmp`+`b.eq`.
- `Return(v)` — move `v` into the result register (`x0`/`d0`/`x1+x2` per type), branch to epilogue.
- `Throw(v)` — call `__rt_throw` with `v` in `x0`.
- `Fatal { message_id }` — call `__rt_fatal` with message pointer.
- `Unreachable` — `udf #0` on ARM64 / `ud2` on x86_64.

Block-parameter move scheduling: when a `Br(target, [v0, v1])` enters a block with params `[p0, p1]`, copy `v0 -> slot(p0)`, `v1 -> slot(p1)`. Handle parallel-move semantics with a topological sort + cycle break (one extra scratch register, mirroring how `src/codegen/abi/values.rs` handles register shuffles).

- [ ] **Step 4: Commit**

```bash
git add src/codegen_ir/block_emit.rs src/codegen_ir/lower_term.rs src/codegen_ir/tests/block_test.rs
git commit -m "feat(codegen_ir): emit blocks, labels, and terminators"
```

---

## Task 4: Lower scalar arithmetic and comparison

**Files:**
- Create: `src/codegen_ir/lower_inst.rs` (dispatcher)
- Create: `src/codegen_ir/lower_inst/arithmetic.rs`
- Create: `src/codegen_ir/lower_inst/floats.rs`
- Create: `src/codegen_ir/lower_inst/comparison.rs`
- Modify: `tests/ir_backend_smoke_test.rs` to unignore the hello-world test

- [ ] **Step 1: Failing test**

Unignore `ir_backend_hello_world`. It expects `42`. Will fail until scalar lowering is in place.

- [ ] **Step 2: Implement scalar arithmetic**

Each op emits the same instruction sequence the AST backend emits today, with operands loaded from the placement slots:

```
// Op::IAdd, operands [a, b], result r:
ldr x1, [x29, #slot(a)]      // load left operand from slot
ldr x0, [x29, #slot(b)]      // load right operand from slot
add x0, x1, x0               // add operands, result in x0
str x0, [x29, #slot(r)]      // store result to result slot
```

Each `emitter.instruction(...)` MUST have a `// ` comment at column 81 per `CLAUDE.md` policy.

- [ ] **Step 3: Run smoke test**

Run: `cargo test --test ir_backend_smoke_test`
Expected: pass (prints `42`).

- [ ] **Step 4: Commit**

```bash
git add src/codegen_ir/lower_inst.rs src/codegen_ir/lower_inst/arithmetic.rs src/codegen_ir/lower_inst/floats.rs src/codegen_ir/lower_inst/comparison.rs tests/ir_backend_smoke_test.rs
git commit -m "feat(codegen_ir): lower scalar arithmetic and comparison"
```

---

## Task 5: Lower constants, locals, conversions

**Files:**
- Create: `src/codegen_ir/lower_inst/conversion.rs`
- Modify: `src/codegen_ir/lower_inst.rs`
- Test: `src/codegen_ir/tests/conversions_test.rs`

- [ ] **Step 1 / 2 / 3 / 4 / 5**: TDD pattern as before.

`ConstI64` / `ConstF64` / `ConstStr` / `ConstNull` lower to literal-load patterns from the existing AST emitter (use the same data-section interning).

`LoadLocal(slot_id)` / `StoreLocal(slot_id)` lower to `ldr`/`str` against the PHP local slot (separate from value-placement slots — locals are named, values are anonymous). `IToF` / `FToI` / `IToStr` etc. call the existing `__rt_*` routines.

- [ ] **Step 6: Commit**

---

## Task 6: Lower strings, arrays, hashes, objects

Five separate commits, one per file:

- [ ] **Step 1**: `lower_inst/strings.rs` — reuse `__rt_str_concat`, `__rt_str_persist`, `__rt_str_char_at` runtime routines. The emitter just sets up arguments and `bl`s.

- [ ] **Step 2**: `lower_inst/arrays.rs` — reuse `__rt_array_new`, `__rt_array_get_int`, `__rt_array_set_int`, `__rt_array_push`, `__rt_array_cow_ensure`.

- [ ] **Step 3**: `lower_inst/hashes.rs` — reuse `__rt_hash_*`, `__rt_iter_*`.

- [ ] **Step 4**: `lower_inst/objects.rs` — reuse `__rt_object_alloc`, inline vtable lookup pattern from existing class-methods emission.

- [ ] **Step 5**: Tests + commits per cluster.

Read the existing emitter file (e.g., `src/codegen/expr/arrays/`) for each cluster and reproduce its assembly pattern.

---

## Task 7: Lower calls, builtins, externs, closures

**Files:**
- Create: `src/codegen_ir/lower_inst/calls.rs`

This is the most surface-area cluster. Recipe:

- **`Call(func_id, args)`**:
  1. Look up `func_id` in `module.data.function_names` → function symbol.
  2. Use ABI helpers (`abi::values::*`) to place args in registers/stack: int args in `x0..x7`, floats in `d0..d7`, stack overflow per ABI.
  3. Emit `bl <symbol>` (ARM64) / `call <symbol>` (x86_64).
  4. Move result from result register into the result value's slot.

- **`IndirectCall(fn_ptr, sig, args)`**: same but `blr <reg>` / `call <reg>`.

- **`MethodCall(obj, method_id, args)`**:
  1. Load `obj` into the first arg register (`x0` ARM64, `rdi` x86_64).
  2. `VTableLookup` produced a function-pointer SSA value earlier; emit indirect call.
  3. Same arg placement and result handling.

- **`BuiltinCall(builtin, args)`**: dispatch to the existing builtin emitters in `src/codegen/builtins/`. The IR knows which builtin, the lowering knows where to find its codegen. This is the largest opcode — *do not* re-implement each builtin; call into the existing emitter helpers with the operand values placed appropriately.

- **`RuntimeCall(rt_routine, args)`**: similar to `BuiltinCall` but always inline `bl __rt_<name>`.

- **`ExternCall(name, args)`**: same as `Call` but with C-ABI conversions on strings (already handled by `src/codegen/ffi.rs` — reuse).

- **`ClosureNew`**: allocate closure object on heap with captured environment. Reuse `src/codegen/functions/closures.rs` patterns.

- [ ] **Step 1 / 2 / 3 / 4**: TDD pattern.
- [ ] **Step 5: Commit**

```bash
git commit -m "feat(codegen_ir): lower calls, builtins, externs, closures"
```

---

## Task 8: Lower ownership ops

**Files:**
- Create: `src/codegen_ir/lower_inst/ownership.rs`

- `Acquire(v)` → `bl __rt_incref` with `v` in `x0`.
- `Release(v)` → `bl __rt_decref_any` with `v` in `x0`.
- `Move(v)` → no-op (semantic only). Validator already checked balance.
- `Borrow(v)` → no-op (semantic only).

- [ ] **Step 1: Failing test**: a fixture that allocates an array, assigns over it (forcing release of the old), and checks `--gc-stats` reports correct alloc/free counts.

- [ ] **Step 2 / 3 / 4: Implement and verify.**

- [ ] **Step 5: Commit**

```bash
git commit -m "feat(codegen_ir): lower Acquire/Release ownership ops"
```

---

## Task 9: Function-level driver

**Files:**
- Create: `src/codegen_ir/context.rs`
- Modify: `src/codegen_ir/mod.rs`

- [ ] **Step 1: Implement `emit_function`**

```rust
// src/codegen_ir/context.rs
pub struct BackendContext<'e> {
    pub emitter: &'e mut Emitter,
    pub placement: ValuePlacement,
    pub func: &'e Function,
    pub data: &'e mut DataSection,
}

pub fn emit_function(emitter: &mut Emitter, data: &mut DataSection, func: &Function) {
    let placement = value_placement::allocate(func);
    let locals_bytes = func.locals.iter().map(|s| s.php_type.stack_size()).sum::<usize>();
    let frame_bytes = ((locals_bytes + placement.total_slot_bytes).next_multiple_of(16)) + 16;

    emit_function_label(emitter, &func.name);
    frame::emit_prologue(emitter, func, frame_bytes);
    block_emit::emit_blocks(emitter, &mut BackendContext {
        emitter, placement, func, data,
    });
    // Epilogue is emitted by Return terminators.
}
```

- [ ] **Step 2: Wire `generate_user_asm_from_ir`**

```rust
// src/codegen_ir/mod.rs (fix the unimplemented! from Task 1)
pub fn generate_user_asm_from_ir(module: &Module, _gc_stats: bool, _heap_debug: bool) -> String {
    let mut emitter = Emitter::new(module.target);
    if module.target.arch == Arch::X86_64 {
        emitter.emit_text_prelude();
    }
    let mut data = DataSection::new();
    // Translate module.data into data_section.
    seed_data_section(&mut data, &module.data);

    for f in &module.functions {
        context::emit_function(&mut emitter, &mut data, f);
    }
    for f in &module.class_methods {
        context::emit_function(&mut emitter, &mut data, f);
    }
    // Interface return wrappers and main emission stay in src/codegen/ for now;
    // Phase 09 either ports them or keeps them via a shared helper.
    crate::codegen::emit_main_and_finalize_from_ir(/* ... */)
}
```

The `emit_main_and_finalize_from_ir` is a new entry point that mirrors `src/codegen/main_emission.rs::emit_main_and_finalize` but reads the `main` function body from the IR module (the AST → IR pass produces a `main` function). Add this helper to `src/codegen/main_emission.rs` in a separate commit if its surface is too small to deserve its own file.

- [ ] **Step 3: Commit**

```bash
git commit -m "feat(codegen_ir): wire generate_user_asm_from_ir end-to-end"
```

---

## Task 10: Parity testing — run every existing codegen test with `--ir-backend`

**Files:**
- Create: `src/codegen_ir/tests/parity.rs`

- [ ] **Step 1: Replay codegen tests through `--ir-backend`**

For each `tests/codegen/*.rs` test, the standard helper is `compile_and_run`. Add a sibling `compile_and_run_ir` (or a flag on the existing helper) that compiles with `--ir-backend` set. Then **mirror every test** in a new `tests/ir_backend_parity/` directory.

A pragmatic approach: a single integration test file that walks the public test fixtures (where available) and asserts each `--ir-backend` output matches the `compile_and_run` (legacy) output. Where tests use inline source, the parity test re-runs the same inline source with the IR backend.

The Docker Linux scripts must be run as well. Use `./scripts/test-linux-*.sh ir_backend` to filter to the new tests.

- [ ] **Step 2: Iterate until parity**

This is where most of Phase 04's calendar time lives. Each parity failure points at:
- A missing or incorrectly emitted opcode
- An ABI mismatch
- An ownership op imbalance
- A wrong effect annotation
- A wrong block-parameter move sequencing

Fix one failure at a time, with a focused regression test in `tests/ir_backend_parity/`.

- [ ] **Step 3: Commit each fix individually**

Aim for many small commits (`fix(codegen_ir): correct stack alignment for str result`, `fix(codegen_ir): release order at if-merge`, etc.). This keeps the PR review tractable and lets the team revert specific regressions if needed.

---

## Task 11: Final parity gate

- [ ] **Step 1: Run the gates**

```bash
cargo build
cargo test                          # legacy backend (default)
cargo test -- --include-ignored     # legacy backend incl. SDL2 etc.
cargo test --features ir-backend    # if a feature flag is used; else use env or a separate test runner
./scripts/test-linux-x86_64.sh
./scripts/test-linux-arm64.sh
```

The new `tests/ir_backend_parity/` corpus must pass with `--ir-backend` enabled at the binary level.

- [ ] **Step 2: Benchmark parity (no regression expected, possibly small slowdown)**

Run the benchmark harness with `--ir-backend` and verify there is *no major regression* compared to the legacy backend on compute benchmarks. A 0–20% slowdown is acceptable in Phase 04 — Phase 06 recovers this and more.

If a >50% slowdown appears on any benchmark, stop and diagnose before claiming Phase 04 done. Likely cause: redundant slot loads (every value spilled and re-loaded). This is expected, but pathological cases (deep call chains, hot loop bodies) may need a minimal Phase 04 mitigation: keep up to 4 SSA values "pinned" in callee-saved scratch registers across the loop body. Document the mitigation in `docs/internals/the-codegen.md`.

- [ ] **Step 3: Commit final benchmark output as a baseline**

```bash
git add benchmarks/ir_backend_phase4_baseline.json
git commit -m "perf(codegen_ir): record phase 04 baseline (parity, no opt yet)"
```

---

## Exit criteria

- Every `tests/codegen/` fixture passes with `--ir-backend` set.
- Docker Linux gates green for `--ir-backend`.
- Benchmark regression bounded (≤20% slowdown vs legacy on any single benchmark).
- Legacy backend untouched and still default.
- Zero compiler warnings.
- Each `emitter.instruction(...)` call in `src/codegen_ir/` has a column-81 `//` comment per project policy.
