# Phase 03 — AST → IR Lowering

> **For agentic workers:** Build the pass that consumes a typed AST (after frontend/optimizer) and produces an EIR `Module`. No assembly is emitted; the existing AST → ASM backend remains the production path. Output is exercised by a new `--emit-ir` CLI flag and by a parallel test harness.

**Goal:** Lower every `ExprKind` and `StmtKind` variant currently handled by `src/codegen/` into EIR, preserving PHP semantics. Validate the resulting module after each top-level lowering.

**Architecture:** New module `src/ir_lower/` that mirrors the structure of `src/codegen/expr/` and `src/codegen/stmt/`. The lowering walks the AST top-down and feeds the EIR `Builder`. Ownership operations are inserted explicitly. Function dispatchers, class methods, variants, and main emission are mirrored.

**Tech Stack:** Rust. Uses `src/ir/`, `src/parser/ast/`, `src/types/`, `src/codegen/program_usage/`. No new external dependencies.

---

## File Structure

All new files under `src/ir_lower/`:

- Create: `src/ir_lower/mod.rs` — entry `pub fn lower_program(...)`
- Create: `src/ir_lower/context.rs` — lowering context (local slot map, label counter, ownership table)
- Create: `src/ir_lower/program.rs` — orchestration: functions, class methods, main, variants
- Create: `src/ir_lower/function.rs` — function body lowering (entry block, param load, prologue/epilogue)
- Create: `src/ir_lower/expr/mod.rs` — dispatcher
- Create: `src/ir_lower/expr/literals.rs` — literals and constants
- Create: `src/ir_lower/expr/variables.rs` — variable load/store, super-globals, statics
- Create: `src/ir_lower/expr/arithmetic.rs` — binary arithmetic, unary, casts, conversions
- Create: `src/ir_lower/expr/comparison.rs` — comparison, equality, spaceship
- Create: `src/ir_lower/expr/strings.rs` — concat, interpolation, char access
- Create: `src/ir_lower/expr/arrays.rs` — array literals, access, assignment, push
- Create: `src/ir_lower/expr/objects.rs` — `new`, `->`, methods, `::`, `instanceof`
- Create: `src/ir_lower/expr/calls.rs` — function calls, builtins, externs, closures, first-class callables
- Create: `src/ir_lower/expr/control.rs` — ternary, short-circuit `&&`/`||`, null coalesce, throw, error suppress, print
- Create: `src/ir_lower/expr/closures.rs` — closure expressions and arrow functions
- Create: `src/ir_lower/expr/ptr_ffi.rs` — ptr_cast, buffer_new
- Create: `src/ir_lower/expr/match_expr.rs` — `match` expression
- Create: `src/ir_lower/stmt/mod.rs` — dispatcher
- Create: `src/ir_lower/stmt/control_flow.rs` — `if`, `while`, `do_while`, `for`, `foreach`, `break`, `continue`, `switch`
- Create: `src/ir_lower/stmt/exceptions.rs` — `try`, `catch`, `finally`, `throw`
- Create: `src/ir_lower/stmt/assignments.rs` — `Assign`, `ArrayAssign`, `ArrayPush`, `PropertyAssign`, `StaticPropertyAssign`, all variants
- Create: `src/ir_lower/stmt/declarations.rs` — `FunctionDecl`, `ClassDecl`, `EnumDecl`, `InterfaceDecl`, `TraitDecl`, `PackedClassDecl`, `ExternFunctionDecl`, etc. (most are no-ops at lowering since they were already handled by frontend; lowering records IDs)
- Create: `src/ir_lower/stmt/includes.rs` — `Include`, `IncludeOnce*`
- Create: `src/ir_lower/stmt/output.rs` — `Echo`, statement-form `Print`
- Create: `src/ir_lower/ownership.rs` — helpers for inserting `Acquire`/`Release` on heap values
- Create: `src/ir_lower/effects_lookup.rs` — looks up effect summaries for builtins/runtime calls from existing analysis
- Create: `src/ir_lower/tests/` — integration tests (snapshot-based using the printer)
- Modify: `src/lib.rs` — add `pub mod ir_lower;`
- Modify: `src/main.rs` — add `mod ir_lower;` for the binary crate module tree
- Modify: `src/cli.rs` — add `--emit-ir` parsing to `CliConfig`
- Modify: `src/pipeline.rs` — honor `emit_ir` after frontend and optimization

---

## Task 1: Wire the module and add `--emit-ir`

**Files:**
- Create: `src/ir_lower/mod.rs` (skeleton)
- Modify: `src/lib.rs`
- Modify: `src/main.rs` (module declaration)
- Modify: `src/cli.rs` (CLI argument parser)
- Modify: `src/pipeline.rs` (emit path)

- [ ] **Step 1: Confirm CLI parser and module roots**

Run: `grep -rln "pub(crate) fn parse_args\\|fn main" src/`
In this branch, `src/cli.rs` owns argument parsing and `src/main.rs` owns the binary crate module declarations.

- [ ] **Step 2: Create skeleton**

```rust
// src/ir_lower/mod.rs
//! Purpose:
//! Lowers a typed `Program` AST into an EIR `Module`. Preserves PHP semantics
//! including evaluation order, ownership, and effect annotations.
//!
//! Called from:
//! - `crate::pipeline::compile()` when `--emit-ir` or `--ir-backend` is set
//!
//! Key details:
//! - Lowering is one pass over the AST; ownership ops are inserted explicitly.
//! - Validation runs after every function lowering to catch builder bugs early.

mod context;
mod effects_lookup;
mod expr;
mod function;
mod ownership;
mod program;
mod stmt;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use crate::codegen::platform::Target;
use crate::ir::Module;
use crate::parser::ast::Program;
use crate::types::{
    ClassInfo, EnumInfo, ExternClassInfo, ExternFunctionSig, FunctionSig, InterfaceInfo,
    PackedClassInfo, PhpType, TypeEnv,
};

#[allow(clippy::too_many_arguments)]
pub fn lower_program(
    program: &Program,
    global_env: &TypeEnv,
    functions: &HashMap<String, FunctionSig>,
    interfaces: &HashMap<String, InterfaceInfo>,
    classes: &HashMap<String, ClassInfo>,
    enums: &HashMap<String, EnumInfo>,
    packed_classes: &HashMap<String, PackedClassInfo>,
    extern_functions: &HashMap<String, ExternFunctionSig>,
    extern_classes: &HashMap<String, ExternClassInfo>,
    extern_globals: &HashMap<String, PhpType>,
    target: Target,
) -> Module {
    program::lower(
        program, global_env, functions, interfaces, classes, enums,
        packed_classes, extern_functions, extern_classes, extern_globals, target,
    )
}
```

- [ ] **Step 3: Add `--emit-ir` to the CLI**

Locate the existing argument parser. Add a boolean flag `--emit-ir`. When set:

1. Run the full frontend (lex, parse, name-resolve, type-check, optimize).
2. Call `ir_lower::lower_program(...)`.
3. Call `ir::print_module(&module)`, print to stdout.
4. Skip codegen, assembler, linker.
5. Exit zero.

Place implementation in `src/cli.rs` alongside `--emit-asm` and `--check`. Add an `emit_ir` field to `CliConfig`, extend the usage string, and reject `--emit-ir` with `--emit-asm` or `--check` because all three are mutually exclusive output modes.

- [ ] **Step 4: Add an end-to-end test**

```rust
// tests/ir_emit_test.rs (or inside an existing tests/ file)
#[test]
fn emit_ir_prints_a_function_for_hello_world() {
    let temp = tests_common::compile_inline(
        "<?php function greet() { return 7; } echo greet();",
        &["--emit-ir"],
    );
    assert!(temp.stdout.contains("function greet"));
    assert!(temp.stdout.contains("const_i64 7"));
}
```

(Adjust `tests_common::compile_inline` to whatever helper already exists; many codegen tests use a `compile_and_run` helper — add a sibling `compile_with_args` for non-binary outputs.)

- [ ] **Step 5: Build and run**

```bash
cargo build
cargo test --test ir_emit_test
```

The test will fail because `lower_program` is not yet implemented. That is expected; subsequent tasks fix it. Mark the test `#[ignore]` for now and remove the ignore in Task 14.

- [ ] **Step 6: Commit**

```bash
git add src/ir_lower/mod.rs src/lib.rs src/main.rs src/cli.rs src/pipeline.rs tests/ir_emit_test.rs
git commit -m "feat(ir_lower): scaffold AST → IR lowering and --emit-ir flag"
```

---

## Task 2: Implement `LoweringContext`

**Files:**
- Create: `src/ir_lower/context.rs`

- [ ] **Step 1: Sketch the context**

The lowering context tracks per-function state: which SSA value currently holds each PHP local, the slot table, the heap ownership of each in-flight value, the active break/continue stack, the exception-handler stack, the closure-capture environment.

```rust
//! Purpose:
//! Per-function lowering state: local slot allocation, current value bindings
//! for PHP locals, control-flow stacks, ownership map.
//!
//! Called from:
//! - `crate::ir_lower::function`, `crate::ir_lower::stmt`, `crate::ir_lower::expr`
//!
//! Key details:
//! - SSA semantics: PHP locals are *not* SSA. The context tracks the current
//!   SSA value bound to each local via a `HashMap<String, ValueId>`. Stores to
//!   a local rebind the local to a fresh SSA value at the current program
//!   point; uses load from the most recent binding (no explicit phi insertion
//!   — block parameters at merge points handle joins).

use std::collections::HashMap;

use crate::ir::{BlockId, Builder, Function, IrType, ValueId};
use crate::types::PhpType;

pub struct LoweringContext<'a, 'f> {
    pub func: &'f mut Function,
    pub builder_state: BuilderState,
    // local name -> most recent SSA value bound
    pub local_bindings: HashMap<String, ValueId>,
    // local name -> slot index in func.locals
    pub local_slots: HashMap<String, u32>,
    pub loop_stack: Vec<LoopFrame>,
    pub handler_stack: Vec<HandlerFrame>,
    pub captures: HashMap<String, ValueId>,
    pub label_counter: &'a std::sync::atomic::AtomicUsize,
}

pub struct BuilderState {
    pub current_block: Option<BlockId>,
}

pub struct LoopFrame {
    pub continue_block: BlockId,
    pub break_block: BlockId,
    pub continue_args_template: Vec<IrType>,
    pub break_args_template: Vec<IrType>,
}

pub struct HandlerFrame {
    pub catch_block: BlockId,
    pub finally_block: Option<BlockId>,
}

impl<'a, 'f> LoweringContext<'a, 'f> {
    pub fn declare_local(&mut self, name: &str, php_ty: PhpType) -> u32 {
        if let Some(idx) = self.local_slots.get(name) { return *idx; }
        let idx = self.func.locals.len() as u32;
        self.func.locals.push(crate::ir::LocalSlot {
            name: name.to_string(),
            php_type: php_ty,
            kind: crate::ir::LocalKind::PhpVariable,
        });
        self.local_slots.insert(name.to_string(), idx);
        idx
    }

    pub fn current_value_of(&self, name: &str) -> Option<ValueId> {
        self.local_bindings.get(name).copied()
    }

    pub fn rebind(&mut self, name: &str, value: ValueId) {
        self.local_bindings.insert(name.to_string(), value);
    }
}
```

- [ ] **Step 2: Commit**

```bash
git add src/ir_lower/context.rs
git commit -m "feat(ir_lower): add LoweringContext skeleton"
```

---

## Task 3: Implement function-level lowering

**Files:**
- Create: `src/ir_lower/function.rs`

- [ ] **Step 1: Write the function lowering driver**

The driver:
1. Creates the entry block with no parameters.
2. Allocates locals for each function parameter, each `static`, each `global`, each PHP local discovered (mirroring `src/codegen/functions/locals.rs` collection logic).
3. Emits parameter `LoadLocal` ops at the top of the entry block.
4. Walks the function body via `stmt::lower_stmt(...)`.
5. If control falls off the end and the return type is `Void`, emits `Return(None)`; otherwise `Fatal` (mirroring PHP behavior where falling off a non-void function with declared return type is undefined / fatal in strict types).
6. Runs `validate_function`.

```rust
//! Purpose:
//! Lowers a single PHP function body into an IR `Function`.
//!
//! Called from:
//! - `crate::ir_lower::program::lower`
//!
//! Key details:
//! - Locals are pre-collected before lowering to ensure stable slot indices.
//! - Validation runs at end-of-function.

use crate::ir::{Builder, Function, FunctionFlags, FunctionParam, IrType, Terminator};
use crate::ir_lower::context::LoweringContext;
use crate::parser::ast::Stmt;
use crate::types::{FunctionSig, PhpType, TypeEnv};

pub fn lower_function(
    name: &str,
    sig: &FunctionSig,
    body: &[Stmt],
    global_env: &TypeEnv,
    // ... + frontend metadata maps; same signature as src/codegen/functions emit_function
) -> Function {
    let return_ir = IrType::from_php(&sig.return_type);
    let mut func = Function::new(name.to_string(), return_ir, sig.return_type.clone());
    // Populate params:
    for p in &sig.params {
        func.params.push(FunctionParam {
            name: p.name.clone(),
            ir_type: IrType::from_php(&p.ty),
            php_type: p.ty.clone(),
            by_ref: p.by_ref,
            variadic: p.variadic,
        });
    }
    func.flags = FunctionFlags { /* derive */ ..Default::default() };

    // Build entry block.
    {
        let mut b = Builder::new(&mut func);
        let entry = b.create_block_with_params(vec![]);
        b.set_entry(entry);
        b.position_at_end(entry);
        // ... allocate locals, load params, lower body via stmt::lower_stmt
    }

    // Validate.
    crate::ir::validate_function(&func).expect("ir validation failed");
    func
}
```

- [ ] **Step 2: Commit (incomplete; expanded in later tasks)**

```bash
git add src/ir_lower/function.rs
git commit -m "feat(ir_lower): function-level lowering skeleton"
```

---

## Task 4: Lower literals and locals

**Files:**
- Create: `src/ir_lower/expr/mod.rs`
- Create: `src/ir_lower/expr/literals.rs`
- Create: `src/ir_lower/expr/variables.rs`
- Test: `src/ir_lower/tests/literals_test.rs`

- [ ] **Step 1: Write the failing test**

```rust
// src/ir_lower/tests/literals_test.rs
use crate::ir_lower::tests::lower_and_print;

#[test]
fn lowers_int_literal_return() {
    let printed = lower_and_print("<?php return 42;");
    assert!(printed.contains("const_i64 42"));
    assert!(printed.contains("return"));
}

#[test]
fn lowers_string_literal_return_via_echo() {
    let printed = lower_and_print(r#"<?php echo "hi";"#);
    assert!(printed.contains("const_str"));
}

#[test]
fn lowers_local_assignment_and_use() {
    let printed = lower_and_print("<?php $x = 5; echo $x;");
    assert!(printed.contains("store_local"));
    assert!(printed.contains("load_local"));
}
```

The `lower_and_print` helper compiles inline source up to and including IR lowering, then returns the printed IR.

- [ ] **Step 2: Run test (fail)**

Run: `cargo test --lib ir_lower::tests::literals_test`
Expected: lowering panics or returns empty IR.

- [ ] **Step 3: Implement `expr::lower_expr` dispatcher + literals + variables**

Dispatcher matches on `ExprKind`. Each case delegates to a focused helper. Initial coverage: `IntLiteral`, `StringLiteral`, `FloatLiteral`, `BoolLiteral`, `Null`, `Variable`.

For `Variable(name)`: if a binding exists in `ctx.local_bindings`, return it (no `LoadLocal` needed in SSA). Otherwise emit `LoadLocal(slot_for(name))` and rebind. (Optimization: PHP semantics require loads to see the latest store; SSA + per-block rebinding handles this naturally.)

- [ ] **Step 4: Run test (pass)**

Run: `cargo test --lib ir_lower::tests::literals_test`
Expected: pass.

- [ ] **Step 5: Commit**

```bash
git add src/ir_lower/expr/mod.rs src/ir_lower/expr/literals.rs src/ir_lower/expr/variables.rs src/ir_lower/tests/literals_test.rs
git commit -m "feat(ir_lower): lower literals and variable access"
```

---

## Task 5: Lower arithmetic, comparison, bitwise

**Files:**
- Create: `src/ir_lower/expr/arithmetic.rs`
- Create: `src/ir_lower/expr/comparison.rs`
- Test: `src/ir_lower/tests/arithmetic_test.rs`

- [ ] **Step 1: Failing tests**

```rust
#[test]
fn lowers_int_add() {
    let printed = lower_and_print("<?php $r = 1 + 2;");
    assert!(printed.contains("iadd"));
}

#[test]
fn lowers_float_add_promotes_int_operand() {
    let printed = lower_and_print("<?php $r = 1 + 2.5;");
    assert!(printed.contains("i_to_f"));
    assert!(printed.contains("fadd"));
}

#[test]
fn lowers_string_concat() {
    let printed = lower_and_print(r#"<?php $r = "a" . "b";"#);
    assert!(printed.contains("str_concat"));
}

#[test]
fn lowers_int_compare_eq() {
    let printed = lower_and_print("<?php $r = (1 == 2);");
    assert!(printed.contains("icmp"));
    assert!(printed.contains("Eq"));
}

#[test]
fn lowers_php_identical_uses_type_aware_op() {
    let printed = lower_and_print("<?php $r = (1 === 1);");
    assert!(printed.contains("php_identical"));
}
```

- [ ] **Step 2: Run failing tests**

Run: `cargo test --lib ir_lower::tests::arithmetic_test`

- [ ] **Step 3: Implement**

Walk `src/codegen/expr/binops/arithmetic.rs` and `src/codegen/expr/binops/comparison.rs` for each operator's PHP semantics. Mirror them: dispatch on operand types from the AST's typed metadata, emit either I-ops or F-ops, insert coercions as needed.

For `PhpLooseEq` and `PhpIdentical`: see `src/codegen/expr/compare/`.

For `BinOp::Concat` (PHP `.`): always `StrConcat`, with `IToStr`/`FToStr`/`BoolToStr` coercions inserted for non-string operands.

- [ ] **Step 4: Run passing tests**

Run: `cargo test --lib ir_lower::tests::arithmetic_test`

- [ ] **Step 5: Commit**

```bash
git add src/ir_lower/expr/arithmetic.rs src/ir_lower/expr/comparison.rs src/ir_lower/tests/arithmetic_test.rs
git commit -m "feat(ir_lower): lower arithmetic, comparison, concat"
```

---

## Task 6: Lower control-flow statements (if, while, for, do-while, break/continue)

**Files:**
- Create: `src/ir_lower/stmt/mod.rs`
- Create: `src/ir_lower/stmt/control_flow.rs`
- Test: `src/ir_lower/tests/control_flow_test.rs`

- [ ] **Step 1: Failing tests**

```rust
#[test]
fn lowers_if_else_creates_three_blocks() {
    let printed = lower_and_print("<?php if ($x) { echo 1; } else { echo 2; }");
    // Expect bb0 (entry), bb1 (then), bb2 (else), bb3 (merge)
    assert!(printed.contains("bb0"));
    assert!(printed.contains("cond_br"));
    assert!(printed.contains("bb3"));
}

#[test]
fn lowers_while_loop_has_header_body_exit() {
    let printed = lower_and_print("<?php $i = 0; while ($i < 10) { $i = $i + 1; }");
    assert!(printed.contains("cond_br"));
    let br_count = printed.matches("br bb").count();
    assert!(br_count >= 2, "expected at least 2 branches, got {br_count}");
}

#[test]
fn lowers_break_branches_to_loop_break_block() {
    let printed = lower_and_print("<?php while (1) { break; }");
    assert!(printed.contains("br bb"));
}
```

- [ ] **Step 2 / 3 / 4**: Same TDD shape.

**Lowering recipes**:

- `If(cond, then, else)`:
  1. Lower `cond` to ValueId `c` of type I64 (PHP truthiness via `PhpBool` coercion if `c.ir_type` isn't already I64-bool).
  2. Create `then_bb`, `else_bb`, `merge_bb`.
  3. Compute the set of *live* PHP local bindings that may differ across branches. Each becomes a `merge_bb` block parameter.
  4. Terminate current with `CondBr(c, then_bb, [], else_bb, [])`.
  5. Lower each branch into its block. Each branch ends with `Br(merge_bb, [snapshots of live locals after this branch])`.
  6. Rebind PHP locals at `merge_bb` entry to its parameters.

- `While(cond, body)`:
  1. Snapshot live locals into a list `merge_locals`.
  2. Create `header_bb` with parameters mirroring `merge_locals`. Terminate current with `Br(header_bb, [current snapshot])`.
  3. Inside `header_bb`, rebind locals to its params, lower `cond`, emit `CondBr(c, body_bb, [params], exit_bb, [params])`.
  4. Lower body into `body_bb`. At body end, `Br(header_bb, [updated snapshot])`.
  5. Continue lowering after `exit_bb`.
  6. `break` → `Br(exit_bb, [snapshot])`; `continue` → `Br(header_bb, [snapshot])`. Multi-level `break N` walks `ctx.loop_stack` N entries deep.

- `For(init, cond, update, body)` — desugar to `init; while (cond) { body; update; }` at lowering time. PHP-faithful enough.

- `DoWhile(body, cond)` — body block runs first; `header_bb` is implicit at end.

- [ ] **Step 5: Commit**

```bash
git add src/ir_lower/stmt/mod.rs src/ir_lower/stmt/control_flow.rs src/ir_lower/tests/control_flow_test.rs
git commit -m "feat(ir_lower): lower if/while/for/do-while and break/continue"
```

---

## Task 7: Lower arrays (literals, access, assignment, push, foreach)

**Files:**
- Create: `src/ir_lower/expr/arrays.rs`
- Create or modify: `src/ir_lower/stmt/control_flow.rs` for `Foreach`
- Test: `src/ir_lower/tests/arrays_test.rs`

- [ ] **Step 1: Failing tests**

```rust
#[test]
fn lowers_array_literal_indexed() {
    let printed = lower_and_print("<?php $a = [1, 2, 3];");
    assert!(printed.contains("array_new"));
    assert!(printed.contains("array_push"));
}

#[test]
fn lowers_array_literal_assoc_uses_hash_new() {
    let printed = lower_and_print(r#"<?php $a = ["k" => 1];"#);
    assert!(printed.contains("Hash"));
    assert!(printed.contains("hash_set_str"));
}

#[test]
fn lowers_array_get_indexed() {
    let printed = lower_and_print("<?php $a = [1,2]; $x = $a[0];");
    assert!(printed.contains("array_get"));
}

#[test]
fn lowers_foreach_emits_iter_ops() {
    let printed = lower_and_print("<?php foreach ([1,2] as $v) { echo $v; }");
    assert!(printed.contains("iter_start"));
    assert!(printed.contains("iter_next"));
}
```

- [ ] **Step 2 / 3 / 4**: Implement and verify.

Notes for the lowering:

- `ArrayLiteral([e0, e1, e2])`: emit `ArrayNew` (capacity hint = literal length), then `ArrayPush` for each element. Result is `Owned`.
- `ArrayLiteralAssoc`: `HashNew` (or `ArrayNew` with hash kind), then `HashSetStr`/`HashSetInt` per pair, with PHP integer-key normalization for numeric strings (mirror `src/types/array_keys.rs`).
- `ArrayAccess`: indexed → `ArrayGet`; assoc → `HashGetStr`/`HashGetInt` depending on key type.
- `ArrayAssign(target, key, val)`: emit `ArrayCowEnsureUnique` first, then `ArraySet`/`HashSet*`. The COW op is conditional in lowering — only insert when ownership state indicates possibly-shared.
- `Foreach`:
  - `IterStart` on the iterable, producing an iterator value.
  - Header block calls `IterCurrent` for `(key?, value)`. PHP semantics: value is by-value (copies) unless `&$v` (by-ref) is used. For by-value, emit `Acquire` if refcounted; else nothing.
  - Body lowered, terminated by `IterNext` and back-edge.
  - Exit calls `IterEnd` and any final cleanup.

- [ ] **Step 5: Commit**

```bash
git add src/ir_lower/expr/arrays.rs src/ir_lower/stmt/control_flow.rs src/ir_lower/tests/arrays_test.rs
git commit -m "feat(ir_lower): lower arrays, hashes, foreach"
```

---

## Task 8: Lower function calls, builtins, externs, closures, first-class callables

**Files:**
- Create: `src/ir_lower/expr/calls.rs`
- Create: `src/ir_lower/expr/closures.rs`
- Create: `src/ir_lower/effects_lookup.rs`
- Test: `src/ir_lower/tests/calls_test.rs`

- [ ] **Step 1: Failing tests**

```rust
#[test]
fn lowers_user_function_call() {
    let src = "<?php function f($x) { return $x; } echo f(7);";
    let printed = lower_and_print(src);
    assert!(printed.contains("call fn(f)"));
}

#[test]
fn lowers_builtin_strlen_with_pure_effects() {
    let printed = lower_and_print(r#"<?php $n = strlen("abc");"#);
    assert!(printed.contains("builtin_call"));
    // strlen is pure
    assert!(!printed.contains("alloc_heap"));
}

#[test]
fn lowers_extern_call_with_conservative_effects() {
    let src = r#"<?php extern "c" function getpid(): int; echo getpid();"#;
    let printed = lower_and_print(src);
    assert!(printed.contains("extern_call"));
    assert!(printed.contains("MAY_THROW") || printed.contains("WRITES_HEAP"));
}
```

- [ ] **Step 2 / 3 / 4**: Implement.

Key implementation notes:

- **Argument evaluation order**: PHP source order. EIR records arguments in source order; the ABI placement (which register/stack slot) is decided by the backend at lowering to ASM (Phase 04), not by IR lowering. This is one of the structural wins of the IR.

- **Named arguments / spread**: reuse `src/types/call_args.rs::plan_call_args` (already exists per `CLAUDE.md`). The planner produces a regular-argument layout; lowering iterates the planned layout in source order and emits an `IndirectArg` shape for variadic tail entries.

- **Effects**: `BuiltinCall` looks up effects from `src/optimize/effects/builtins.rs` via `effects_lookup::builtin_effects(name)`. `Call` uses `FunctionSig` purity flags. `ExternCall` is always conservative.

- **First-class callables**: `FirstClassCallable(target)` lowers to either a function-pointer constant (if target is a static function) or a `ClosureNew` with an explicit env capture (if target is a method on an object).

- **Closures (`Closure { params, captures, body }`)**: emit a *fresh anonymous function* in the IR module, taking the captures as additional parameters at the front. The `Closure` expression itself lowers to `ClosureNew(anonymous_fn, [captured locals])`.

- [ ] **Step 5: Commit**

```bash
git add src/ir_lower/expr/calls.rs src/ir_lower/expr/closures.rs src/ir_lower/effects_lookup.rs src/ir_lower/tests/calls_test.rs
git commit -m "feat(ir_lower): lower calls, builtins, externs, closures"
```

---

## Task 9: Lower objects (new, ->, ::, instanceof, method calls)

**Files:**
- Create: `src/ir_lower/expr/objects.rs`
- Test: `src/ir_lower/tests/objects_test.rs`

- [ ] **Step 1: Failing tests** (one per primitive shape)
- [ ] **Step 2 / 3 / 4**: Implement.

Notes:
- `new ClassName(args)` → `ObjectNew(class_id)` + `Call(ctor)` with args.
- `$obj->prop` → `PropGet(obj, offset)`. Offset is computed from `ClassInfo.property_layout` at lowering time (already known after type checking).
- `$obj->method(args)` → `VTableLookup(obj, method_id)` returning a function pointer, then `IndirectCall(ptr, sig, args)`. For statically known final methods, skip the lookup and emit `Call(direct_fn_id)`.
- `instanceof` → `InstanceOf(obj, class_id)`.
- Static method/property accesses → direct `Call`/`LoadGlobal` on the class's static storage.
- Nullsafe (`?->`) → conditional branch: if null, the chain result is null; else proceed.

- [ ] **Step 5: Commit**

```bash
git add src/ir_lower/expr/objects.rs src/ir_lower/tests/objects_test.rs
git commit -m "feat(ir_lower): lower objects and method calls"
```

---

## Task 10: Lower exception flow (`try`/`catch`/`finally`, `throw`)

**Files:**
- Create: `src/ir_lower/stmt/exceptions.rs`
- Test: `src/ir_lower/tests/exceptions_test.rs`

- [ ] **Step 1: Failing tests**
- [ ] **Step 2 / 3 / 4**: Implement.

Notes:
- Push a `HandlerFrame` on `ctx.handler_stack` when entering `try`.
- Operations with `MAY_THROW` effect lowered while a handler is on the stack get an *implicit edge* to the handler block. The structural validator does not check this yet (Phase 09 hardens it); for now, ensure cleanup paths run.
- `throw expr` → `Throw(exc_value)` terminator. Builder records the value as owned at the throw point; the runtime takes ownership via `__rt_throw`.
- `finally` runs on all exit paths (normal, throw, return, break, continue). Lower as a *cleanup block* that the handler-walk inserts before each exit terminator. Implementation strategy: lower the `finally` body once into a synthetic block, then duplicate-or-call from each exit point. PHP semantics require the finally body to run; calling is simpler than inlining.

- [ ] **Step 5: Commit**

```bash
git add src/ir_lower/stmt/exceptions.rs src/ir_lower/tests/exceptions_test.rs
git commit -m "feat(ir_lower): lower try/catch/finally and throw"
```

---

## Task 11: Lower assignments, ownership transfer ops

**Files:**
- Create: `src/ir_lower/stmt/assignments.rs`
- Create: `src/ir_lower/ownership.rs`
- Test: `src/ir_lower/tests/ownership_test.rs`

- [ ] **Step 1: Failing tests**

```rust
#[test]
fn array_local_assignment_releases_previous_owned_array() {
    let src = "<?php $a = [1,2]; $a = [3,4];";
    let printed = lower_and_print(src);
    assert!(printed.contains("release"), "expected release of old array");
    assert_eq!(printed.matches("array_new").count(), 2);
}

#[test]
fn passing_array_to_call_emits_borrow_for_borrowing_callees() {
    let src = "<?php function f(array $a): int { return count($a); } $x = [1]; echo f($x);";
    let printed = lower_and_print(src);
    assert!(printed.contains("borrow"));
}
```

- [ ] **Step 2 / 3 / 4**: Implement.

Notes on ownership lowering:
- When a local of refcounted type is overwritten, emit `Release(old_value)` *before* the store.
- Arguments to callees: pass-by-value semantics; if the callee's signature is `borrowing` (a heuristic the type checker already computes for builtins), pass `Borrow(val)`. Otherwise, the callee receives an extra refcount (caller emits `Acquire(val)` before the call).
- Return values: a function returning a refcounted type returns ownership to the caller (caller takes `Owned`).
- At CFG merges, ownership states are joined per the lattice. The validator (Phase 02 structural; extended in Phase 09) verifies balance.

- [ ] **Step 5: Commit**

```bash
git add src/ir_lower/stmt/assignments.rs src/ir_lower/ownership.rs src/ir_lower/tests/ownership_test.rs
git commit -m "feat(ir_lower): lower assignments with explicit ownership ops"
```

---

## Task 12: Lower switch, match, ternary, short-circuit operators

**Files:**
- Create: `src/ir_lower/expr/control.rs`
- Create: `src/ir_lower/expr/match_expr.rs`
- Modify: `src/ir_lower/stmt/control_flow.rs` for `Switch`
- Test: `src/ir_lower/tests/control_expr_test.rs`

- [ ] **Step 1 / 2 / 3 / 4**: Implement and verify.

Notes:
- `Ternary { cond, then, else }` → `CondBr` to two blocks, each producing a value, joined at a merge block with one parameter.
- `&&` and `||` (short-circuit): same shape as ternary with constant operand on one side.
- `NullCoalesce`: similar; check IsNull(lhs), branch to use rhs or lhs.
- `Switch(scrutinee, cases)` → `Switch` terminator if cases are integer literals; otherwise lower as chained `if`s.
- `Match` expression → similar to switch but expression-valued.

- [ ] **Step 5: Commit**

```bash
git add src/ir_lower/expr/control.rs src/ir_lower/expr/match_expr.rs src/ir_lower/stmt/control_flow.rs src/ir_lower/tests/control_expr_test.rs
git commit -m "feat(ir_lower): lower ternary, short-circuit ops, switch, match"
```

---

## Task 13: Lower remaining AST variants

**Files:**
- Create or modify the remaining `src/ir_lower/expr/*.rs` and `src/ir_lower/stmt/*.rs` files
- Test: `src/ir_lower/tests/exhaustive_test.rs`

- [ ] **Step 1: Build an exhaustive test fixture**

For every `ExprKind` and `StmtKind` variant not yet covered, write at least one inline PHP snippet that produces it. Use the catalogue from `docs/internals/the-ir.md` Phase 01 Task 7 as the master checklist.

Variants still pending after Tasks 4-12:

ExprKind: `PreIncrement`, `PostIncrement`, `PreDecrement`, `PostDecrement`, `Negate`, `Not`, `BitNot`, `Cast`, `ErrorSuppress`, `Print`, `ConstRef`, `ClassConstant`, `ScopedConstantAccess`, `NewScopedObject`, `MagicConstant`, `Yield`, `YieldFrom`, `PtrCast`, `BufferNew`, `Spread`, `NamedArg`, `ClosureCall`, `ExprCall`, `FirstClassCallable` (variants beyond Task 8), `This`, `NewObject` (cover construction edge cases).

StmtKind: `IfDef`, `TypedAssign`, `Throw` (statement form), `Synthetic`, `NamespaceDecl`, `NamespaceBlock`, `UseDecl`, `FunctionDecl`, `FunctionVariantGroup`, `FunctionVariantMark`, `Return`, `ConstDecl`, `ListUnpack`, `Global`, `StaticVar`, `ClassDecl`, `EnumDecl`, `PackedClassDecl`, `InterfaceDecl`, `TraitDecl`, `PropertyAssign`, `StaticPropertyAssign`, `StaticPropertyArrayPush`, `StaticPropertyArrayAssign`, `PropertyArrayPush`, `PropertyArrayAssign`, `ExternFunctionDecl`, `ExternClassDecl`, `ExternGlobalDecl`, `Include`, `IncludeOnceMark`, `IncludeOnceGuard`.

Most declarations (`FunctionDecl`, `ClassDecl`, etc.) are no-ops at lowering — the frontend already produced `FunctionSig`/`ClassInfo`, and the IR module references those. The lowering walks the program once to discover *bodies* for function-level lowering (Task 3); declaration statements themselves only need to be ignored.

- [ ] **Step 2: Implement remaining variants**

One commit per variant cluster (e.g., "lower magic constants", "lower static vars and globals", "lower include statements"). The implementation either:
- Reuses an existing emitter (e.g., MagicConstant should already be lowered to a string/int/file by the magic-constants pass; if a residual node reaches lowering, panic with a clear bug message).
- Adds a focused helper.

- [ ] **Step 3: Run the exhaustive test**

The exhaustive test compiles each fixture and asserts that no `unimplemented!()` panic fires and that the printed IR is non-empty and validates.

- [ ] **Step 4: Commit (one commit per cluster)**

---

## Task 14: Validate every test from `tests/codegen/` through the lowering

**Files:**
- Create: `src/ir_lower/tests/codegen_corpus_test.rs`

- [ ] **Step 1: Auto-discover codegen test PHP fixtures**

Walk the `tests/codegen/` tree. Many tests use inline source via `compile_and_run`. Build a smaller corpus from the fixtures that *do* live in files. For inline source tests, parameterize a helper that takes a string.

- [ ] **Step 2: Run lowering over each fixture**

For each fixture:
1. Lex, parse, name-resolve, type-check, optimize (full frontend).
2. Call `ir_lower::lower_program`.
3. Call `ir::validate_module`.
4. Assert success.

This is a soft correctness gate: it doesn't verify *behavior*, only that lowering doesn't crash and produces structurally valid IR.

- [ ] **Step 3: Commit**

```bash
git add src/ir_lower/tests/codegen_corpus_test.rs
git commit -m "test(ir_lower): exercise lowering across codegen corpus"
```

---

## Task 15: Unignore the end-to-end `--emit-ir` test

- [ ] **Step 1: Remove `#[ignore]` from `tests/ir_emit_test.rs`**

- [ ] **Step 2: Run full suite**

Run:
```bash
cargo build
cargo test
cargo test -- --include-ignored
./scripts/test-linux-x86_64.sh
./scripts/test-linux-arm64.sh
```

Expected: all green. The legacy codegen path is still the default; `--emit-ir` is a parallel diagnostic.

- [ ] **Step 3: Document `--emit-ir`**

Edit `docs/internals/the-ir.md` (created in Phase 01): add a "Using `--emit-ir`" section with an example.

- [ ] **Step 4: Commit**

```bash
git add tests/ir_emit_test.rs docs/internals/the-ir.md
git commit -m "feat: enable --emit-ir end-to-end"
```

---

## Exit criteria

- Every `ExprKind` and `StmtKind` variant has at least one lowering test.
- `--emit-ir` works on the entire codegen corpus without panicking.
- `ir::validate_module` succeeds on every lowered program.
- All previously passing tests still pass (legacy backend still in use).
- Docker Linux gates green.
- Zero compiler warnings.
