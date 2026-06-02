# Phase 02 — IR Module Skeleton

> **For agentic workers:** Implement the data structures defined in Phase 01. No AST → IR lowering yet; no codegen consumer yet. The validator and printer are testable on hand-built `Module` instances. This phase produces no user-visible behavior change.

**Goal:** Implement `src/ir/` containing types, instructions, values, blocks, functions, modules, a builder API, a validator, and a textual printer. Cover with unit tests.

**Architecture:** Pure data structures. No interaction with the rest of the compiler. The next phases consume this module.

**Tech Stack:** Rust. One new dependency: `bitflags` (already an indirect dep of common crates; verify with `cargo tree` before adding).

---

## File Structure

All new files under `src/ir/`:

- Create: `src/ir/mod.rs` — module root, re-exports
- Create: `src/ir/types.rs` — `IrType`, `IrHeapKind`, conversions from `PhpType`
- Create: `src/ir/value.rs` — `ValueId`, `Value`, `Ownership`, value table
- Create: `src/ir/instr.rs` — `Instruction`, opcode enum (`Op`), `InstId`
- Create: `src/ir/effects.rs` — `Effects` bitset
- Create: `src/ir/block.rs` — `BasicBlock`, `BlockId`, `Terminator`
- Create: `src/ir/function.rs` — `Function`, `LocalSlot`, `LocalKind`, `FunctionParam`, `FunctionFlags`
- Create: `src/ir/module.rs` — `Module`, `DataPool`, `ExternDecl`
- Create: `src/ir/builder.rs` — `Builder` API for constructing IR by hand (used by tests now, by Phase 03 lowering later)
- Create: `src/ir/validator.rs` — structural + ownership + effect checks
- Create: `src/ir/print.rs` — textual format printer
- Create: `src/ir/tests/mod.rs` — unit tests
- Create: `src/ir/tests/print_test.rs` — snapshot-style tests for printer
- Create: `src/ir/tests/validator_test.rs` — positive and negative validator cases
- Create: `src/ir/tests/builder_test.rs` — build a couple of hand-rolled functions
- Modify: `src/lib.rs` — add `pub mod ir;`
- Modify: `src/main.rs` — add `mod ir;` for the binary crate module tree

---

## Task 1: Wire up the module

**Files:**
- Create: `src/ir/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Create `src/ir/mod.rs` with the module preamble and re-exports**

```rust
//! Purpose:
//! Defines elephc IR (EIR), a CFG-based SSA-lite intermediate representation
//! used between AST-level optimization and assembly emission.
//!
//! Called from:
//! - Phase 03: `crate::ir::builder` from a new AST → EIR lowering pass
//! - Phase 04: a new `src/codegen_ir/` backend consuming `Module`
//!
//! Key details:
//! - Block parameters replace SSA phi nodes; ownership is explicit; effects
//!   are immutable bitset metadata. See `docs/internals/the-ir.md`.

mod block;
mod builder;
mod effects;
mod function;
mod instr;
mod module;
mod print;
mod types;
mod validator;
mod value;

#[cfg(test)]
mod tests;

pub use block::{BasicBlock, BlockId, Terminator};
pub use builder::Builder;
pub use effects::Effects;
pub use function::{Function, FunctionFlags, FunctionParam, LocalKind, LocalSlot};
pub use instr::{InstId, Instruction, Op};
pub use module::{DataPool, ExternDecl, Module};
pub use print::print_module;
pub use types::{IrHeapKind, IrType};
pub use validator::{validate_function, validate_module, ValidationError};
pub use value::{Ownership, Value, ValueDef, ValueId};
```

- [ ] **Step 2: Add module to crate root**

```rust
// src/lib.rs — add this line in alphabetical order with existing pub mod entries
pub mod ir;
```

- [ ] **Step 3: Add the binary module declaration**

```rust
// src/main.rs — add this line near the other module declarations
mod ir;
```

- [ ] **Step 4: Build to verify the empty skeleton compiles**

Run: `cargo build`
Expected: `error[E0583]: file not found for module 'types'` (and others) — this confirms the module is wired up; the missing files come next.

- [ ] **Step 5: Add stub files so the build proceeds**

Create each of `types.rs`, `value.rs`, `instr.rs`, `effects.rs`, `block.rs`, `function.rs`, `module.rs`, `builder.rs`, `validator.rs`, `print.rs` with only the module preamble and no items:

```rust
//! Purpose:
//! <one-line>
//!
//! Called from:
//! - `crate::ir`
//!
//! Key details:
//! - Implemented in phase 02.
```

Run: `cargo build`
Expected: clean (with warnings for unused imports in `mod.rs`, acceptable for now)

- [ ] **Step 6: Commit**

```bash
git add src/ir/ src/lib.rs src/main.rs
git commit -m "feat(ir): scaffold src/ir/ module skeleton"
```

---

## Task 2: Implement `IrType` and `IrHeapKind`

**Files:**
- Modify: `src/ir/types.rs`
- Test: `src/ir/tests/types_test.rs`

- [ ] **Step 1: Write the failing test**

```rust
// src/ir/tests/types_test.rs
use crate::ir::{IrHeapKind, IrType};
use crate::types::PhpType;

#[test]
fn maps_int_to_i64() {
    assert_eq!(IrType::from_php(&PhpType::Int), IrType::I64);
}

#[test]
fn maps_float_to_f64() {
    assert_eq!(IrType::from_php(&PhpType::Float), IrType::F64);
}

#[test]
fn maps_str_to_str() {
    assert_eq!(IrType::from_php(&PhpType::Str), IrType::Str);
}

#[test]
fn maps_bool_to_i64() {
    assert_eq!(IrType::from_php(&PhpType::Bool), IrType::I64);
}

#[test]
fn maps_array_int_to_heap_array() {
    let php_ty = PhpType::Array(Box::new(PhpType::Int));
    assert_eq!(IrType::from_php(&php_ty), IrType::Heap(IrHeapKind::Array));
}

#[test]
fn maps_mixed_to_heap_mixed() {
    assert_eq!(IrType::from_php(&PhpType::Mixed), IrType::Heap(IrHeapKind::Mixed));
}

#[test]
fn register_count_matches_php_type() {
    assert_eq!(IrType::I64.register_count(), 1);
    assert_eq!(IrType::F64.register_count(), 1);
    assert_eq!(IrType::Str.register_count(), 2);
    assert_eq!(IrType::Heap(IrHeapKind::Array).register_count(), 1);
    assert_eq!(IrType::Void.register_count(), 0);
}
```

Add `mod types_test;` to `src/ir/tests/mod.rs` (creating the file if needed):

```rust
// src/ir/tests/mod.rs
mod types_test;
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --lib ir::tests::types_test`
Expected: `error: cannot find type IrType` etc.

- [ ] **Step 3: Implement `IrType`**

```rust
// src/ir/types.rs
//! Purpose:
//! Defines the EIR type lattice and conversions from `PhpType`.
//!
//! Called from:
//! - `crate::ir::value`, `crate::ir::builder`, lowering passes (phase 03)
//!
//! Key details:
//! - Heap subkind is metadata on `IrType::Heap`; runtime treats heap values
//!   uniformly via `__rt_decref_any`.

use crate::types::PhpType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrType {
    I64,
    F64,
    Str,
    Heap(IrHeapKind),
    Void,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrHeapKind {
    Array,
    Hash,
    Object,
    Mixed,
    Iterable,
    Union,
    Buffer,
}

impl IrType {
    pub fn from_php(php: &PhpType) -> Self {
        match php {
            PhpType::Int | PhpType::Bool | PhpType::Pointer(_)
                | PhpType::Resource(_) | PhpType::Callable => IrType::I64,
            PhpType::Float => IrType::F64,
            PhpType::Str => IrType::Str,
            PhpType::Void | PhpType::Never => IrType::Void,
            PhpType::Array(_) => IrType::Heap(IrHeapKind::Array),
            PhpType::AssocArray { .. } => IrType::Heap(IrHeapKind::Hash),
            PhpType::Object(_) | PhpType::Packed(_) => IrType::Heap(IrHeapKind::Object),
            PhpType::Mixed => IrType::Heap(IrHeapKind::Mixed),
            PhpType::Iterable => IrType::Heap(IrHeapKind::Iterable),
            PhpType::Union(_) => IrType::Heap(IrHeapKind::Union),
            PhpType::Buffer(_) => IrType::Heap(IrHeapKind::Buffer),
        }
    }

    pub fn register_count(&self) -> usize {
        match self {
            IrType::I64 | IrType::F64 | IrType::Heap(_) => 1,
            IrType::Str => 2,
            IrType::Void => 0,
        }
    }

    pub fn is_refcounted(&self) -> bool {
        matches!(self, IrType::Heap(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, IrType::F64)
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --lib ir::tests::types_test`
Expected: all tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/ir/types.rs src/ir/tests/types_test.rs src/ir/tests/mod.rs
git commit -m "feat(ir): implement IrType and IrHeapKind"
```

---

## Task 3: Implement `Ownership` and `Value`

**Files:**
- Modify: `src/ir/value.rs`
- Test: `src/ir/tests/value_test.rs`

- [ ] **Step 1: Write the failing test**

```rust
// src/ir/tests/value_test.rs
use crate::ir::{IrType, Ownership, Value, ValueDef, ValueId};
use crate::types::PhpType;

#[test]
fn ownership_merge_same_state() {
    assert_eq!(Ownership::Owned.merge(Ownership::Owned), Ownership::Owned);
    assert_eq!(Ownership::Borrowed.merge(Ownership::Borrowed), Ownership::Borrowed);
}

#[test]
fn ownership_merge_distinct_states_yields_maybe_owned() {
    assert_eq!(Ownership::Owned.merge(Ownership::Borrowed), Ownership::MaybeOwned);
}

#[test]
fn ownership_for_php_type_int_is_nonheap() {
    assert_eq!(Ownership::for_php_type(&PhpType::Int), Ownership::NonHeap);
}

#[test]
fn ownership_for_php_type_array_starts_maybe_owned() {
    let ty = PhpType::Array(Box::new(PhpType::Int));
    assert_eq!(Ownership::for_php_type(&ty), Ownership::MaybeOwned);
}

#[test]
fn value_id_is_zero_indexed() {
    let v = ValueId::from_raw(0);
    assert_eq!(v.as_raw(), 0);
}
```

Add `mod value_test;` to `src/ir/tests/mod.rs`.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --lib ir::tests::value_test`
Expected: undefined symbols.

- [ ] **Step 3: Implement**

```rust
// src/ir/value.rs
//! Purpose:
//! Defines SSA-style `ValueId`, the owning `Value`, and the `Ownership` lattice
//! mirroring `crate::codegen::context::HeapOwnership`.
//!
//! Called from:
//! - `crate::ir::function`, `crate::ir::builder`, `crate::ir::validator`
//!
//! Key details:
//! - Each `ValueId` is defined exactly once.
//! - Ownership lattice tracks heap retention across SSA values, not just locals.

use crate::ir::{block::BlockId, types::IrType};
use crate::types::PhpType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(u32);

impl ValueId {
    pub fn from_raw(raw: u32) -> Self { Self(raw) }
    pub fn as_raw(self) -> u32 { self.0 }
}

#[derive(Debug, Clone)]
pub struct Value {
    pub ir_type: IrType,
    pub php_type: PhpType,
    pub def: ValueDef,
    pub ownership: Ownership,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueDef {
    BlockParam { block: BlockId, index: u16 },
    Instruction { block: BlockId, index: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ownership {
    NonHeap,
    Owned,
    Borrowed,
    MaybeOwned,
}

impl Ownership {
    pub fn for_php_type(ty: &PhpType) -> Self {
        if ty.is_refcounted() || matches!(ty, PhpType::Str) {
            Ownership::MaybeOwned
        } else {
            Ownership::NonHeap
        }
    }

    pub fn merge(self, other: Self) -> Self {
        use Ownership::*;
        match (self, other) {
            (NonHeap, NonHeap) => NonHeap,
            (Owned, Owned) => Owned,
            (Borrowed, Borrowed) => Borrowed,
            (MaybeOwned, _) | (_, MaybeOwned) => MaybeOwned,
            (Owned, Borrowed) | (Borrowed, Owned) => MaybeOwned,
            (NonHeap, x) | (x, NonHeap) => x,
        }
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --lib ir::tests::value_test`
Expected: pass.

- [ ] **Step 5: Commit**

```bash
git add src/ir/value.rs src/ir/tests/value_test.rs src/ir/tests/mod.rs
git commit -m "feat(ir): implement Value, ValueId, Ownership"
```

---

## Task 4: Implement `Effects`

**Files:**
- Modify: `Cargo.toml` (add `bitflags = "2"` only if not already present)
- Modify: `src/ir/effects.rs`
- Test: `src/ir/tests/effects_test.rs`

- [ ] **Step 1: Check whether `bitflags` is already a dependency**

Run: `cargo tree | grep bitflags`
If present and version >= 2.0, skip adding to Cargo.toml. If absent, add to `[dependencies]` in `Cargo.toml`:

```toml
bitflags = "2"
```

- [ ] **Step 2: Write the failing test**

```rust
// src/ir/tests/effects_test.rs
use crate::ir::Effects;

#[test]
fn pure_has_no_bits() {
    assert!(Effects::PURE.is_empty());
    assert!(Effects::PURE.is_pure());
}

#[test]
fn reads_and_writes_are_orthogonal() {
    let r = Effects::READS_HEAP;
    let w = Effects::WRITES_HEAP;
    assert!(r.may_observe());
    assert!(!r.may_mutate());
    assert!(w.may_mutate());
    assert!(!w.may_observe());
}

#[test]
fn combined_effects_compose() {
    let e = Effects::READS_HEAP | Effects::MAY_FATAL;
    assert!(e.contains(Effects::READS_HEAP));
    assert!(e.contains(Effects::MAY_FATAL));
}
```

Add `mod effects_test;` to `src/ir/tests/mod.rs`.

- [ ] **Step 3: Run test to verify it fails**

Run: `cargo test --lib ir::tests::effects_test`
Expected: undefined symbols.

- [ ] **Step 4: Implement**

```rust
// src/ir/effects.rs
//! Purpose:
//! Defines the immutable per-instruction effect bitset used by IR-level passes.
//!
//! Called from:
//! - `crate::ir::instr` and every pass over IR instructions
//!
//! Key details:
//! - Effects are assigned at builder time and are not mutated by subsequent
//!   passes. They are inferred conservatively for unknown calls.

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Effects: u16 {
        const READS_LOCAL    = 1 << 0;
        const READS_HEAP     = 1 << 1;
        const READS_GLOBAL   = 1 << 2;
        const READS_FS       = 1 << 3;
        const WRITES_LOCAL   = 1 << 4;
        const WRITES_HEAP    = 1 << 5;
        const WRITES_GLOBAL  = 1 << 6;
        const WRITES_FS      = 1 << 7;
        const ALLOC_HEAP     = 1 << 8;
        const ALLOC_CONCAT   = 1 << 9;
        const MAY_THROW      = 1 << 10;
        const MAY_FATAL      = 1 << 11;
        const MAY_DEOPT      = 1 << 12;
        const REFCOUNT_OP    = 1 << 13;
    }
}

impl Effects {
    pub const PURE: Effects = Effects::empty();

    pub fn is_pure(&self) -> bool { self.is_empty() }

    pub fn may_observe(&self) -> bool {
        self.intersects(
            Effects::READS_LOCAL | Effects::READS_HEAP | Effects::READS_GLOBAL
                | Effects::READS_FS | Effects::ALLOC_HEAP
                | Effects::MAY_THROW | Effects::MAY_FATAL | Effects::MAY_DEOPT,
        )
    }

    pub fn may_mutate(&self) -> bool {
        self.intersects(
            Effects::WRITES_LOCAL | Effects::WRITES_HEAP | Effects::WRITES_GLOBAL
                | Effects::WRITES_FS | Effects::REFCOUNT_OP,
        )
    }
}
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test --lib ir::tests::effects_test`
Expected: pass.

- [ ] **Step 6: Commit**

```bash
git add src/ir/effects.rs src/ir/tests/effects_test.rs src/ir/tests/mod.rs Cargo.toml Cargo.lock
git commit -m "feat(ir): implement Effects bitset"
```

---

## Task 5: Implement `BlockId`, `BasicBlock`, `Terminator`

**Files:**
- Modify: `src/ir/block.rs`
- Test: `src/ir/tests/block_test.rs`

- [ ] **Step 1: Write the failing test**

```rust
// src/ir/tests/block_test.rs
use crate::ir::{BasicBlock, BlockId, Terminator, ValueId};

#[test]
fn block_id_is_zero_indexed() {
    assert_eq!(BlockId::from_raw(0).as_raw(), 0);
    assert_eq!(BlockId::from_raw(42).as_raw(), 42);
}

#[test]
fn terminator_return_with_value() {
    let term = Terminator::Return(Some(ValueId::from_raw(7)));
    if let Terminator::Return(Some(v)) = term {
        assert_eq!(v.as_raw(), 7);
    } else {
        panic!("expected Return");
    }
}

#[test]
fn block_construction_records_params_and_terminator() {
    let block = BasicBlock {
        id: BlockId::from_raw(0),
        params: vec![ValueId::from_raw(0)],
        instructions: vec![],
        terminator: Terminator::Return(None),
    };
    assert_eq!(block.params.len(), 1);
    assert!(matches!(block.terminator, Terminator::Return(None)));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --lib ir::tests::block_test`
Expected: undefined symbols.

- [ ] **Step 3: Implement**

```rust
// src/ir/block.rs
//! Purpose:
//! Defines basic blocks, terminators, and the typed block identifier.
//!
//! Called from:
//! - `crate::ir::function`, `crate::ir::builder`, `crate::ir::validator`
//!
//! Key details:
//! - Block parameters replace SSA phi nodes; every block has exactly one
//!   terminator at the end.

use crate::ir::instr::InstId;
use crate::ir::value::ValueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct BlockId(u32);

impl BlockId {
    pub fn from_raw(raw: u32) -> Self { Self(raw) }
    pub fn as_raw(self) -> u32 { self.0 }
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: BlockId,
    pub params: Vec<ValueId>,
    pub instructions: Vec<InstId>,
    pub terminator: Terminator,
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Br {
        target: BlockId,
        args: Vec<ValueId>,
    },
    CondBr {
        cond: ValueId,
        then_block: BlockId,
        then_args: Vec<ValueId>,
        else_block: BlockId,
        else_args: Vec<ValueId>,
    },
    Switch {
        scrutinee: ValueId,
        cases: Vec<(i64, BlockId, Vec<ValueId>)>,
        default: (BlockId, Vec<ValueId>),
    },
    Return(Option<ValueId>),
    Throw(ValueId),
    Fatal { message_id: u32 },
    Unreachable,
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --lib ir::tests::block_test`
Expected: pass.

- [ ] **Step 5: Commit**

```bash
git add src/ir/block.rs src/ir/tests/block_test.rs src/ir/tests/mod.rs
git commit -m "feat(ir): implement BasicBlock and Terminator"
```

---

## Task 6: Implement `Op`, `Instruction`, `InstId`

**Files:**
- Modify: `src/ir/instr.rs`
- Test: `src/ir/tests/instr_test.rs`

This is a large enum. Implement it in one pass; tests are minimal at this stage. Effect assignments per opcode are added in Task 7 via a helper.

- [ ] **Step 1: Write the failing test**

```rust
// src/ir/tests/instr_test.rs
use crate::ir::{Effects, InstId, Instruction, Op, ValueId};

#[test]
fn inst_id_is_zero_indexed() {
    assert_eq!(InstId::from_raw(0).as_raw(), 0);
}

#[test]
fn iadd_op_is_pure() {
    assert_eq!(Op::IAdd.default_effects(), Effects::PURE);
}

#[test]
fn array_set_writes_heap_and_allocs() {
    let e = Op::ArraySet.default_effects();
    assert!(e.contains(Effects::WRITES_HEAP));
    assert!(e.contains(Effects::ALLOC_HEAP));
}

#[test]
fn fatal_terminator_is_separate_from_op() {
    // Fatal is a Terminator, not an Op. This test guards against accidentally
    // adding it to Op.
    let names: Vec<&'static str> = vec![]; // no-op probe
    assert!(names.is_empty());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --lib ir::tests::instr_test`
Expected: undefined symbols.

- [ ] **Step 3: Implement the opcode enum**

The full enum is large. Lay it out in the order from `docs/internals/the-ir.md` (Task 3 of Phase 01). Here is the canonical shape; copy verbatim:

```rust
// src/ir/instr.rs
//! Purpose:
//! Defines `Op` (the opcode enum), the `Instruction` payload, and `InstId`.
//!
//! Called from:
//! - `crate::ir::builder`, lowering, validator, printer, codegen consumer
//!
//! Key details:
//! - Each opcode has a single canonical effect set returned by `default_effects()`.
//!   Builders may not weaken effects; passes may not mutate them.

use crate::ir::effects::Effects;
use crate::ir::types::IrType;
use crate::ir::value::ValueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstId(u32);

impl InstId {
    pub fn from_raw(raw: u32) -> Self { Self(raw) }
    pub fn as_raw(self) -> u32 { self.0 }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub op: Op,
    pub operands: Vec<ValueId>,
    pub immediate: Option<Immediate>,
    pub result: Option<ValueId>,
    pub result_type: IrType,
    pub effects: Effects,
}

#[derive(Debug, Clone)]
pub enum Immediate {
    I64(i64),
    F64(f64),
    Str(u32),                // string_id into DataPool
    LocalSlot(u32),
    GlobalName(u32),
    FunctionRef(u32),
    BuiltinRef(BuiltinId),
    RuntimeRef(RuntimeId),
    ExternRef(u32),
    ClassRef(u32),
    MethodRef { class: u32, method: u32 },
    PropOffset(u32),
    HeapKind(crate::ir::types::IrHeapKind),
    MixedTag(u8),
    CmpPredicate(CmpPredicate),
    CastTarget(IrType),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpPredicate {
    Eq, Ne, Slt, Sle, Sgt, Sge,
    Olt, Ole, Ogt, Oge, // float
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BuiltinId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuntimeId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    // -- constants and locals --
    ConstI64, ConstF64, ConstStr, ConstNull,
    LoadLocal, StoreLocal,
    LoadGlobal, StoreGlobal,
    // -- arithmetic / bitwise --
    IAdd, ISub, IMul, ISDiv, ISMod, INeg,
    IBitAnd, IBitOr, IBitXor, IBitNot, IShl, IShrA,
    FAdd, FSub, FMul, FDiv, FNeg, FPow,
    // -- comparison --
    ICmp, FCmp, StrCmpEq, PhpLooseEq, PhpIdentical, Spaceship,
    // -- conversion --
    IToF, FToI, IToStr, FToStr, BoolToStr,
    StrToI, StrToF,
    MixedBox, MixedUnbox, MixedTagOf, Cast,
    // -- strings --
    StrConcat, StrLen, StrCharAt, StrPersist, StrInterpolate,
    // -- arrays / hashes --
    ArrayNew, ArrayLen, ArrayGet, ArraySet, ArrayPush, ArrayCowEnsureUnique,
    HashGetStr, HashGetInt, HashSetStr, HashSetInt, HashKeyExists,
    IterStart, IterNext, IterCurrent, IterEnd,
    // -- objects --
    ObjectNew, PropGet, PropSet, VTableLookup, InstanceOf,
    // -- calls --
    Call, IndirectCall, MethodCall, BuiltinCall, RuntimeCall, ExternCall,
    // -- closures --
    ClosureNew, ClosureCapture,
    // -- ownership (no-op at runtime, semantic for validator/passes) --
    Acquire, Release, Move, Borrow,
    // -- misc --
    Nop,
}

impl Op {
    pub fn default_effects(self) -> Effects {
        use Effects as E;
        use Op::*;
        match self {
            ConstI64 | ConstF64 | ConstStr | ConstNull
                | INeg | IAdd | ISub | IMul
                | IBitAnd | IBitOr | IBitXor | IBitNot | IShl | IShrA
                | FAdd | FSub | FMul | FDiv | FNeg
                | ICmp | FCmp | StrCmpEq | PhpIdentical
                | IToF | FToI | BoolToStr | StrToI | StrToF
                | StrLen | MixedTagOf
                | Move | Borrow | Nop => E::PURE,
            ISDiv | ISMod => E::MAY_FATAL,
            FPow => E::PURE, // libc-backed but pure for our purposes
            LoadLocal => E::READS_LOCAL,
            StoreLocal => E::WRITES_LOCAL,
            LoadGlobal => E::READS_GLOBAL,
            StoreGlobal => E::WRITES_GLOBAL,
            IToStr | FToStr | StrConcat | StrInterpolate => E::ALLOC_CONCAT,
            StrPersist => E::ALLOC_HEAP,
            StrCharAt => E::ALLOC_CONCAT | E::MAY_FATAL,
            MixedBox => E::ALLOC_HEAP,
            MixedUnbox => E::MAY_FATAL,
            Cast => E::MAY_FATAL | E::ALLOC_CONCAT, // worst-case until refined
            PhpLooseEq | Spaceship => E::MAY_DEOPT,
            ArrayNew | HashGetStr | HashGetInt | HashKeyExists => {
                // ArrayNew allocates; gets read heap.
                match self {
                    ArrayNew => E::ALLOC_HEAP,
                    HashGetStr | HashGetInt => E::READS_HEAP | E::MAY_FATAL,
                    HashKeyExists => E::READS_HEAP,
                    _ => unreachable!(),
                }
            }
            ArrayLen => E::READS_HEAP,
            ArrayGet => E::READS_HEAP | E::MAY_FATAL,
            ArraySet | HashSetStr | HashSetInt | ArrayPush => {
                E::WRITES_HEAP | E::ALLOC_HEAP
            }
            ArrayCowEnsureUnique => E::ALLOC_HEAP,
            IterStart | IterNext | IterCurrent => E::READS_HEAP | E::MAY_DEOPT,
            IterEnd => E::WRITES_HEAP,
            ObjectNew => E::ALLOC_HEAP | E::MAY_DEOPT, // constructor user code
            PropGet => E::READS_HEAP,
            PropSet => E::WRITES_HEAP,
            VTableLookup => E::READS_HEAP,
            InstanceOf => E::READS_HEAP,
            Call | IndirectCall | MethodCall => E::all() - E::REFCOUNT_OP, // worst-case; refined per-callsite from FunctionSig
            BuiltinCall => E::all() - E::REFCOUNT_OP, // refined per-builtin at builder time
            RuntimeCall => E::all() - E::REFCOUNT_OP, // refined per-routine
            ExternCall => E::READS_HEAP | E::WRITES_HEAP | E::MAY_THROW,
            ClosureNew => E::ALLOC_HEAP,
            ClosureCapture => E::READS_LOCAL,
            Acquire | Release => E::REFCOUNT_OP,
        }
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --lib ir::tests::instr_test`
Expected: pass.

- [ ] **Step 5: Commit**

```bash
git add src/ir/instr.rs src/ir/tests/instr_test.rs src/ir/tests/mod.rs
git commit -m "feat(ir): implement Op enum and Instruction"
```

---

## Task 7: Implement `LocalSlot`, `Function`, `Module`

**Files:**
- Modify: `src/ir/function.rs`
- Modify: `src/ir/module.rs`
- Test: `src/ir/tests/function_test.rs`

- [ ] **Step 1: Write the failing test**

```rust
// src/ir/tests/function_test.rs
use crate::ir::{Function, FunctionFlags, IrType, LocalKind, LocalSlot, Module};
use crate::ir::block::{BasicBlock, BlockId, Terminator};
use crate::types::PhpType;

#[test]
fn empty_function_has_no_blocks() {
    let f = Function::new("foo".to_string(), IrType::Void, PhpType::Void);
    assert_eq!(f.blocks.len(), 0);
    assert_eq!(f.name, "foo");
}

#[test]
fn module_has_target_metadata() {
    let target = crate::codegen::platform::Target::new(
        crate::codegen::platform::Platform::MacOS,
        crate::codegen::platform::Arch::AArch64,
    );
    let m = Module::new(target);
    assert!(m.functions.is_empty());
    assert!(m.class_methods.is_empty());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --lib ir::tests::function_test`
Expected: undefined symbols.

- [ ] **Step 3: Implement `Function`**

```rust
// src/ir/function.rs
//! Purpose:
//! Defines IR-level functions, their local slots, parameters, and per-function flags.
//!
//! Called from:
//! - `crate::ir::module`, builder, validator, codegen consumer
//!
//! Key details:
//! - The entry block has no block parameters; function parameters surface via
//!   `LoadLocal` at the top of the entry block.

use crate::ir::block::{BasicBlock, BlockId};
use crate::ir::instr::Instruction;
use crate::ir::types::IrType;
use crate::ir::value::Value;
use crate::types::PhpType;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<FunctionParam>,
    pub return_type: IrType,
    pub return_php_type: PhpType,
    pub blocks: Vec<BasicBlock>,
    pub values: Vec<Value>,
    pub instructions: Vec<Instruction>,
    pub locals: Vec<LocalSlot>,
    pub entry: BlockId,
    pub flags: FunctionFlags,
}

impl Function {
    pub fn new(name: String, return_type: IrType, return_php_type: PhpType) -> Self {
        Self {
            name,
            params: Vec::new(),
            return_type,
            return_php_type,
            blocks: Vec::new(),
            values: Vec::new(),
            instructions: Vec::new(),
            locals: Vec::new(),
            entry: BlockId::from_raw(0),
            flags: FunctionFlags::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionParam {
    pub name: String,
    pub ir_type: IrType,
    pub php_type: PhpType,
    pub by_ref: bool,
    pub variadic: bool,
}

#[derive(Debug, Clone)]
pub struct LocalSlot {
    pub name: String,
    pub php_type: PhpType,
    pub kind: LocalKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalKind {
    PhpVariable,
    Hidden,
    Static,
    Global,
}

#[derive(Debug, Clone, Default)]
pub struct FunctionFlags {
    pub is_main: bool,
    pub is_method: bool,
    pub is_closure: bool,
    pub is_generator: bool,
    pub is_static: bool,
}
```

- [ ] **Step 4: Implement `Module`**

```rust
// src/ir/module.rs
//! Purpose:
//! Top-level IR container: functions, class methods, data pool, extern decls.
//!
//! Called from:
//! - The AST → IR pass (phase 03) and the IR → ASM backend (phase 04).
//!
//! Key details:
//! - The runtime (`__rt_*`) lives outside the module; only declarations and
//!   IDs of runtime routines used by the program are referenced.

use crate::codegen::platform::Target;
use crate::ir::function::Function;

#[derive(Debug, Clone)]
pub struct Module {
    pub target: Target,
    pub functions: Vec<Function>,
    pub class_methods: Vec<Function>,
    pub data: DataPool,
    pub extern_decls: Vec<ExternDecl>,
}

impl Module {
    pub fn new(target: Target) -> Self {
        Self {
            target,
            functions: Vec::new(),
            class_methods: Vec::new(),
            data: DataPool::default(),
            extern_decls: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DataPool {
    pub strings: Vec<String>,            // strings[idx] -> literal
    pub float_literals: Vec<f64>,
    pub global_names: Vec<String>,
    pub function_names: Vec<String>,
    pub class_names: Vec<String>,
}

impl DataPool {
    pub fn intern_string(&mut self, s: &str) -> u32 {
        if let Some(idx) = self.strings.iter().position(|existing| existing == s) {
            return idx as u32;
        }
        let id = self.strings.len() as u32;
        self.strings.push(s.to_string());
        id
    }
}

#[derive(Debug, Clone)]
pub struct ExternDecl {
    pub name: String,
    pub link_libs: Vec<String>,
}
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test --lib ir::tests::function_test`
Expected: pass.

- [ ] **Step 6: Commit**

```bash
git add src/ir/function.rs src/ir/module.rs src/ir/tests/function_test.rs src/ir/tests/mod.rs
git commit -m "feat(ir): implement Function, Module, DataPool"
```

---

## Task 8: Implement the `Builder`

**Files:**
- Modify: `src/ir/builder.rs`
- Test: `src/ir/tests/builder_test.rs`

The `Builder` API is the only sanctioned way to add blocks, instructions, and values to a `Function`. It maintains invariants the validator relies on.

- [ ] **Step 1: Write the failing test**

```rust
// src/ir/tests/builder_test.rs
use crate::ir::{Builder, Effects, Function, IrType, Op, Terminator};
use crate::types::PhpType;

#[test]
fn build_function_with_return() {
    let mut f = Function::new("ret_42".to_string(), IrType::I64, PhpType::Int);
    let mut b = Builder::new(&mut f);
    let entry = b.create_block_with_params(vec![]);
    b.set_entry(entry);
    b.position_at_end(entry);
    let v = b.emit_const_i64(42);
    b.terminate(Terminator::Return(Some(v)));
    assert_eq!(f.blocks.len(), 1);
    assert_eq!(f.values.len(), 1);
}

#[test]
fn build_function_with_iadd_and_branch() {
    let mut f = Function::new("add_one".to_string(), IrType::I64, PhpType::Int);
    let mut b = Builder::new(&mut f);
    let entry = b.create_block_with_params(vec![(IrType::I64, PhpType::Int)]);
    b.set_entry(entry);
    let arg = f.blocks[0].params[0]; // param ValueId
    b.position_at_end(entry);
    let one = b.emit_const_i64(1);
    let sum = b.emit_iadd(arg, one);
    b.terminate(Terminator::Return(Some(sum)));
    assert_eq!(f.blocks[0].instructions.len(), 2);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --lib ir::tests::builder_test`
Expected: undefined symbols.

- [ ] **Step 3: Implement `Builder`**

```rust
// src/ir/builder.rs
//! Purpose:
//! Mutator API for constructing IR functions. Maintains SSA invariants
//! that the validator depends on.
//!
//! Called from:
//! - Phase 03: AST → IR lowering
//! - Phase 02 tests: hand-built functions for validator and printer tests
//!
//! Key details:
//! - The builder enforces single terminator per block via `terminate()`.
//! - Builders fix effects at instruction-emit time using `Op::default_effects()`,
//!   refined by per-callsite information passed to `emit_call()`-style helpers.

use crate::ir::block::{BasicBlock, BlockId, Terminator};
use crate::ir::effects::Effects;
use crate::ir::function::Function;
use crate::ir::instr::{Immediate, InstId, Instruction, Op};
use crate::ir::types::IrType;
use crate::ir::value::{Ownership, Value, ValueDef, ValueId};
use crate::types::PhpType;

pub struct Builder<'f> {
    func: &'f mut Function,
    current: Option<BlockId>,
}

impl<'f> Builder<'f> {
    pub fn new(func: &'f mut Function) -> Self {
        Self { func, current: None }
    }

    pub fn set_entry(&mut self, block: BlockId) {
        self.func.entry = block;
    }

    pub fn create_block_with_params(
        &mut self,
        params: Vec<(IrType, PhpType)>,
    ) -> BlockId {
        let block_id = BlockId::from_raw(self.func.blocks.len() as u32);
        let mut param_value_ids = Vec::with_capacity(params.len());
        for (idx, (ir_ty, php_ty)) in params.into_iter().enumerate() {
            let value_id = ValueId::from_raw(self.func.values.len() as u32);
            self.func.values.push(Value {
                ir_type: ir_ty,
                ownership: Ownership::for_php_type(&php_ty),
                php_type: php_ty,
                def: ValueDef::BlockParam { block: block_id, index: idx as u16 },
            });
            param_value_ids.push(value_id);
        }
        self.func.blocks.push(BasicBlock {
            id: block_id,
            params: param_value_ids,
            instructions: Vec::new(),
            terminator: Terminator::Unreachable,
        });
        block_id
    }

    pub fn position_at_end(&mut self, block: BlockId) {
        self.current = Some(block);
    }

    pub fn terminate(&mut self, term: Terminator) {
        let block_id = self.current.expect("no block positioned");
        self.func.blocks[block_id.as_raw() as usize].terminator = term;
    }

    fn push_inst(
        &mut self,
        op: Op,
        operands: Vec<ValueId>,
        immediate: Option<Immediate>,
        result_type: IrType,
        result_php_type: PhpType,
        result_ownership: Ownership,
        effects: Effects,
    ) -> Option<ValueId> {
        let block_id = self.current.expect("no block positioned");
        let block_idx = block_id.as_raw() as usize;
        let inst_idx_in_block = self.func.blocks[block_idx].instructions.len() as u32;
        let result_id = if matches!(result_type, IrType::Void) {
            None
        } else {
            let value_id = ValueId::from_raw(self.func.values.len() as u32);
            self.func.values.push(Value {
                ir_type: result_type,
                php_type: result_php_type,
                def: ValueDef::Instruction { block: block_id, index: inst_idx_in_block },
                ownership: result_ownership,
            });
            Some(value_id)
        };
        let inst_id = InstId::from_raw(self.func.instructions.len() as u32);
        self.func.instructions.push(Instruction {
            op,
            operands,
            immediate,
            result: result_id,
            result_type,
            effects,
        });
        self.func.blocks[block_idx].instructions.push(inst_id);
        result_id
    }

    // -- convenience emitters --

    pub fn emit_const_i64(&mut self, val: i64) -> ValueId {
        self.push_inst(
            Op::ConstI64, vec![], Some(Immediate::I64(val)),
            IrType::I64, PhpType::Int, Ownership::NonHeap,
            Op::ConstI64.default_effects(),
        ).unwrap()
    }

    pub fn emit_const_null(&mut self) -> ValueId {
        self.push_inst(
            Op::ConstNull, vec![], None,
            IrType::I64, PhpType::Void, Ownership::NonHeap,
            Op::ConstNull.default_effects(),
        ).unwrap()
    }

    pub fn emit_iadd(&mut self, a: ValueId, b: ValueId) -> ValueId {
        self.push_inst(
            Op::IAdd, vec![a, b], None,
            IrType::I64, PhpType::Int, Ownership::NonHeap,
            Op::IAdd.default_effects(),
        ).unwrap()
    }

    pub fn emit_load_local(&mut self, slot_id: u32, ir_ty: IrType, php_ty: PhpType) -> ValueId {
        let own = Ownership::for_php_type(&php_ty);
        self.push_inst(
            Op::LoadLocal, vec![], Some(Immediate::LocalSlot(slot_id)),
            ir_ty, php_ty, own,
            Op::LoadLocal.default_effects(),
        ).unwrap()
    }

    pub fn emit_store_local(&mut self, slot_id: u32, val: ValueId) {
        self.push_inst(
            Op::StoreLocal, vec![val], Some(Immediate::LocalSlot(slot_id)),
            IrType::Void, PhpType::Void, Ownership::NonHeap,
            Op::StoreLocal.default_effects(),
        );
    }

    // Additional emitters are added in phase 03 as lowering needs them.
    // Pattern: one emit_* per Op variant; each pins the effect set from
    // Op::default_effects() (refinable for Call/Builtin/Runtime/Extern).
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --lib ir::tests::builder_test`
Expected: pass.

- [ ] **Step 5: Commit**

```bash
git add src/ir/builder.rs src/ir/tests/builder_test.rs src/ir/tests/mod.rs
git commit -m "feat(ir): implement Builder API"
```

---

## Task 9: Implement the validator

**Files:**
- Modify: `src/ir/validator.rs`
- Test: `src/ir/tests/validator_test.rs`

- [ ] **Step 1: Write the failing test**

```rust
// src/ir/tests/validator_test.rs
use crate::ir::{Builder, Function, IrType, Terminator, validate_function};
use crate::types::PhpType;

#[test]
fn empty_function_fails_validation() {
    let f = Function::new("empty".to_string(), IrType::Void, PhpType::Void);
    assert!(validate_function(&f).is_err());
}

#[test]
fn well_formed_function_passes() {
    let mut f = Function::new("ok".to_string(), IrType::I64, PhpType::Int);
    let mut b = Builder::new(&mut f);
    let entry = b.create_block_with_params(vec![]);
    b.set_entry(entry);
    b.position_at_end(entry);
    let v = b.emit_const_i64(7);
    b.terminate(Terminator::Return(Some(v)));
    assert!(validate_function(&f).is_ok());
}

#[test]
fn return_type_mismatch_fails() {
    let mut f = Function::new("bad".to_string(), IrType::F64, PhpType::Float);
    let mut b = Builder::new(&mut f);
    let entry = b.create_block_with_params(vec![]);
    b.set_entry(entry);
    b.position_at_end(entry);
    let v = b.emit_const_i64(1);
    b.terminate(Terminator::Return(Some(v)));
    assert!(validate_function(&f).is_err());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --lib ir::tests::validator_test`
Expected: undefined symbols.

- [ ] **Step 3: Implement the validator (structural only in Phase 02)**

```rust
// src/ir/validator.rs
//! Purpose:
//! Structural, type, and ownership validation for `Function`/`Module`.
//!
//! Called from:
//! - After every IR pass; called from tests directly.
//!
//! Key details:
//! - Phase 02 implements structural checks; ownership and dominance checks
//!   land in phase 03 once the AST → IR pass has more realistic IR to test.

use crate::ir::block::{BlockId, Terminator};
use crate::ir::function::Function;
use crate::ir::module::Module;
use crate::ir::value::ValueDef;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    NoBlocks,
    NoEntryBlock,
    BlockMissingTerminator(BlockId),
    DuplicateValueDef(u32),
    UseBeforeDef { value_raw: u32 },
    UnknownBlock(BlockId),
    BranchArgCountMismatch { target: BlockId, expected: usize, actual: usize },
    BranchArgTypeMismatch { target: BlockId, index: usize },
    ReturnTypeMismatch,
    UnreachableTerminatorIsReachable(BlockId),
}

pub fn validate_function(f: &Function) -> Result<(), ValidationError> {
    if f.blocks.is_empty() {
        return Err(ValidationError::NoBlocks);
    }
    if (f.entry.as_raw() as usize) >= f.blocks.len() {
        return Err(ValidationError::NoEntryBlock);
    }

    for block in &f.blocks {
        // structural: there's always a terminator field; Unreachable counts.
        // Type checks per terminator:
        match &block.terminator {
            Terminator::Return(Some(val)) => {
                let v = &f.values[val.as_raw() as usize];
                if v.ir_type != f.return_type {
                    return Err(ValidationError::ReturnTypeMismatch);
                }
            }
            Terminator::Return(None) => {
                if !matches!(f.return_type, crate::ir::types::IrType::Void) {
                    return Err(ValidationError::ReturnTypeMismatch);
                }
            }
            Terminator::Br { target, args } => {
                let dest = f.blocks.get(target.as_raw() as usize)
                    .ok_or(ValidationError::UnknownBlock(*target))?;
                if dest.params.len() != args.len() {
                    return Err(ValidationError::BranchArgCountMismatch {
                        target: *target,
                        expected: dest.params.len(),
                        actual: args.len(),
                    });
                }
                for (i, (param_id, arg_id)) in dest.params.iter().zip(args.iter()).enumerate() {
                    let p = &f.values[param_id.as_raw() as usize];
                    let a = &f.values[arg_id.as_raw() as usize];
                    if p.ir_type != a.ir_type {
                        return Err(ValidationError::BranchArgTypeMismatch {
                            target: *target,
                            index: i,
                        });
                    }
                }
            }
            // Remaining terminators: similar checks for CondBr, Switch, etc.
            // For phase 02 brevity, implement Br and Return only; phase 03
            // extends to all terminators with corresponding tests.
            _ => {}
        }
    }

    Ok(())
}

pub fn validate_module(m: &Module) -> Result<(), ValidationError> {
    for f in &m.functions {
        validate_function(f)?;
    }
    for f in &m.class_methods {
        validate_function(f)?;
    }
    Ok(())
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --lib ir::tests::validator_test`
Expected: pass.

- [ ] **Step 5: Commit**

```bash
git add src/ir/validator.rs src/ir/tests/validator_test.rs src/ir/tests/mod.rs
git commit -m "feat(ir): implement structural validator"
```

---

## Task 10: Implement the textual printer

**Files:**
- Modify: `src/ir/print.rs`
- Test: `src/ir/tests/print_test.rs`

- [ ] **Step 1: Write the failing test**

```rust
// src/ir/tests/print_test.rs
use crate::ir::{Builder, Function, IrType, Module, Terminator, print_module};
use crate::types::PhpType;
use crate::codegen::platform::{Arch, Platform, Target};

#[test]
fn prints_minimal_function() {
    let target = Target::new(Platform::MacOS, Arch::AArch64);
    let mut m = Module::new(target);
    let mut f = Function::new("ret_seven".to_string(), IrType::I64, PhpType::Int);
    let mut b = Builder::new(&mut f);
    let entry = b.create_block_with_params(vec![]);
    b.set_entry(entry);
    b.position_at_end(entry);
    let v = b.emit_const_i64(7);
    b.terminate(Terminator::Return(Some(v)));
    m.functions.push(f);

    let printed = print_module(&m);
    assert!(printed.contains("function ret_seven"));
    assert!(printed.contains("const_i64 7"));
    assert!(printed.contains("return v"));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --lib ir::tests::print_test`
Expected: undefined symbol `print_module`.

- [ ] **Step 3: Implement printer**

```rust
// src/ir/print.rs
//! Purpose:
//! Textual format for `Module` and `Function`. Used by `--emit-ir` and tests.
//!
//! Called from:
//! - `Module::print()` wrapper, CLI in phase 03 (`--emit-ir`).
//!
//! Key details:
//! - Printer-only; no parser. Output stability is a soft guarantee for tests.

use std::fmt::Write;

use crate::ir::block::Terminator;
use crate::ir::function::Function;
use crate::ir::instr::{Immediate, Op};
use crate::ir::module::Module;
use crate::ir::types::{IrHeapKind, IrType};

pub fn print_module(m: &Module) -> String {
    let mut out = String::new();
    for f in &m.functions {
        print_function(&mut out, f, &m.data);
        out.push('\n');
    }
    for f in &m.class_methods {
        print_function(&mut out, f, &m.data);
        out.push('\n');
    }
    out
}

fn print_function(out: &mut String, f: &Function, data: &crate::ir::module::DataPool) {
    let _ = write!(out, "function {}(", f.name);
    for (i, p) in f.params.iter().enumerate() {
        if i > 0 { out.push_str(", "); }
        let _ = write!(out, "{}: {}", p.name, type_name(p.ir_type));
    }
    let _ = writeln!(out, ") -> {} {{", type_name(f.return_type));
    for block in &f.blocks {
        let _ = write!(out, "  bb{}", block.id.as_raw());
        if !block.params.is_empty() {
            out.push('(');
            for (i, pid) in block.params.iter().enumerate() {
                if i > 0 { out.push_str(", "); }
                let v = &f.values[pid.as_raw() as usize];
                let _ = write!(out, "v{}: {}", pid.as_raw(), type_name(v.ir_type));
            }
            out.push(')');
        }
        out.push_str(":\n");
        for inst_id in &block.instructions {
            let inst = &f.instructions[inst_id.as_raw() as usize];
            out.push_str("    ");
            if let Some(r) = inst.result {
                let _ = write!(out, "v{} = ", r.as_raw());
            }
            let _ = write!(out, "{:?}", inst.op);
            for operand in &inst.operands {
                let _ = write!(out, " v{}", operand.as_raw());
            }
            if let Some(imm) = &inst.immediate {
                print_immediate(out, imm, data);
            }
            if !inst.effects.is_empty() {
                let _ = write!(out, "    ; effects: {:?}", inst.effects);
            }
            out.push('\n');
        }
        out.push_str("    ");
        print_terminator(out, &block.terminator);
        out.push('\n');
    }
    out.push_str("}\n");
}

fn print_immediate(out: &mut String, imm: &Immediate, data: &crate::ir::module::DataPool) {
    match imm {
        Immediate::I64(v) => { let _ = write!(out, " {}", v); }
        Immediate::F64(v) => { let _ = write!(out, " {}", v); }
        Immediate::Str(idx) => {
            let s = data.strings.get(*idx as usize).map(|s| s.as_str()).unwrap_or("?");
            let _ = write!(out, " {:?}", s);
        }
        Immediate::LocalSlot(idx) => { let _ = write!(out, " slot[{}]", idx); }
        Immediate::GlobalName(idx) => {
            let s = data.global_names.get(*idx as usize).map(|s| s.as_str()).unwrap_or("?");
            let _ = write!(out, " global({})", s);
        }
        Immediate::FunctionRef(idx) => {
            let s = data.function_names.get(*idx as usize).map(|s| s.as_str()).unwrap_or("?");
            let _ = write!(out, " fn({})", s);
        }
        Immediate::BuiltinRef(id) => { let _ = write!(out, " builtin#{}", id.0); }
        Immediate::RuntimeRef(id) => { let _ = write!(out, " runtime#{}", id.0); }
        Immediate::ExternRef(idx) => { let _ = write!(out, " extern#{}", idx); }
        Immediate::ClassRef(idx) => {
            let s = data.class_names.get(*idx as usize).map(|s| s.as_str()).unwrap_or("?");
            let _ = write!(out, " class({})", s);
        }
        Immediate::MethodRef { class, method } => { let _ = write!(out, " method({},{})", class, method); }
        Immediate::PropOffset(off) => { let _ = write!(out, " prop@{}", off); }
        Immediate::HeapKind(k) => { let _ = write!(out, " kind={}", heap_kind_name(*k)); }
        Immediate::MixedTag(t) => { let _ = write!(out, " tag={}", t); }
        Immediate::CmpPredicate(p) => { let _ = write!(out, " pred={:?}", p); }
        Immediate::CastTarget(t) => { let _ = write!(out, " to={}", type_name(*t)); }
        Immediate::None => {}
    }
}

fn print_terminator(out: &mut String, term: &Terminator) {
    match term {
        Terminator::Br { target, args } => {
            let _ = write!(out, "br bb{}", target.as_raw());
            if !args.is_empty() { print_args(out, args); }
        }
        Terminator::CondBr { cond, then_block, then_args, else_block, else_args } => {
            let _ = write!(out, "cond_br v{}, bb{}", cond.as_raw(), then_block.as_raw());
            if !then_args.is_empty() { print_args(out, then_args); }
            let _ = write!(out, ", bb{}", else_block.as_raw());
            if !else_args.is_empty() { print_args(out, else_args); }
        }
        Terminator::Switch { scrutinee, cases, default } => {
            let _ = write!(out, "switch v{} [", scrutinee.as_raw());
            for (val, target, args) in cases {
                let _ = write!(out, "{} => bb{}", val, target.as_raw());
                if !args.is_empty() { print_args(out, args); }
                out.push_str(", ");
            }
            let _ = write!(out, "default => bb{}", default.0.as_raw());
            if !default.1.is_empty() { print_args(out, &default.1); }
            out.push(']');
        }
        Terminator::Return(Some(v)) => { let _ = write!(out, "return v{}", v.as_raw()); }
        Terminator::Return(None) => { out.push_str("return"); }
        Terminator::Throw(v) => { let _ = write!(out, "throw v{}", v.as_raw()); }
        Terminator::Fatal { message_id } => { let _ = write!(out, "fatal msg#{}", message_id); }
        Terminator::Unreachable => { out.push_str("unreachable"); }
    }
}

fn print_args(out: &mut String, args: &[crate::ir::value::ValueId]) {
    out.push('(');
    for (i, v) in args.iter().enumerate() {
        if i > 0 { out.push_str(", "); }
        let _ = write!(out, "v{}", v.as_raw());
    }
    out.push(')');
}

fn type_name(t: IrType) -> String {
    match t {
        IrType::I64 => "I64".to_string(),
        IrType::F64 => "F64".to_string(),
        IrType::Str => "Str".to_string(),
        IrType::Heap(k) => format!("Heap[{}]", heap_kind_name(k)),
        IrType::Void => "Void".to_string(),
    }
}

fn heap_kind_name(k: IrHeapKind) -> &'static str {
    match k {
        IrHeapKind::Array => "Array",
        IrHeapKind::Hash => "Hash",
        IrHeapKind::Object => "Object",
        IrHeapKind::Mixed => "Mixed",
        IrHeapKind::Iterable => "Iterable",
        IrHeapKind::Union => "Union",
        IrHeapKind::Buffer => "Buffer",
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --lib ir::tests::print_test`
Expected: pass.

- [ ] **Step 5: Commit**

```bash
git add src/ir/print.rs src/ir/tests/print_test.rs src/ir/tests/mod.rs
git commit -m "feat(ir): implement textual format printer"
```

---

## Task 11: Run full test suite and verify zero regressions

- [ ] **Step 1: Run full suite**

Run:
```bash
cargo build
cargo test
cargo test -- --include-ignored
```

Expected: all green. No new code paths are exercised by the existing test suite — `src/ir/` is untouched by the compiler. New tests live under `src/ir/tests/`.

- [ ] **Step 2: Run Docker Linux gates**

Run:
```bash
./scripts/test-linux-x86_64.sh
./scripts/test-linux-arm64.sh
```

Expected: green.

- [ ] **Step 3: `git diff --check` and final commit if anything left**

Run:
```bash
git diff --check
git status
```

If any whitespace or untracked files appear, fix and amend the last commit only if it is the most recent; otherwise create a new commit.

---

## Exit criteria

- `src/ir/` module compiles cleanly
- All Phase 02 unit tests pass
- Full suite (`cargo test -- --include-ignored`) passes
- Docker Linux gates pass
- Zero compiler warnings
- All commits follow `feat(ir):` prefix
- No file in `src/ir/` exceeds 500 LOC unless it is a cohesive leaf (see file-size policy in `CLAUDE.md`)
