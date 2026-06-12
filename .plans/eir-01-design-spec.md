# Phase 01 — EIR Design Specification

> **For agentic workers:** This phase produces *documentation only*. No code is written. Phase 02 implements the spec defined here.

**Goal:** Produce a complete written specification of elephc IR (EIR) — types, instructions, terminators, effects, ownership semantics, and validation rules — covering every existing AST node the codegen currently lowers.

**Architecture:** SSA-form CFG with block parameters. PHP-specific instructions (Mixed boxing, COW, fatal, runtime calls) are first-class. Effects are explicit metadata, not implicit ordering.

**Tech Stack:** Markdown. No code. Output is a single doc that Phase 02 implements verbatim.

---

## File Structure

The output of this phase is one document:

- Create: `docs/internals/the-ir.md` — the canonical EIR specification (replaces nothing; new doc)

This repository now has `docs/internals/the-ir.md` and a `docs/README.md` link. Later phases should update `docs/internals/the-codegen.md` when the backend transition becomes real.

---

## Task 1: Specify EIR types

**Files:**
- Create: `docs/internals/the-ir.md` (section: "Types")

- [ ] **Step 1: Write the types section**

```markdown
## Types

EIR uses a minimal type lattice. Type-level distinctions that the runtime
treats uniformly (e.g., Array vs Hash, Object vs Mixed) are *not* separate
IR types — they are carried as metadata on operations.

| EIR type | Storage | Maps from `PhpType` |
|----------|---------|---------------------|
| `I64`    | 1 integer register | `Int`, `Bool`, `Pointer`, `Resource`, `Callable` |
| `F64`    | 1 float register   | `Float` |
| `Str`    | pair `(ptr, len)`, 2 registers | `Str` |
| `Heap`   | 1 integer register (pointer to heap header) | `Array(_)`, `AssocArray{..}`, `Object(_)`, `Mixed`, `Iterable`, `Union(_)`, `Buffer(_)` |
| `Void`   | zero registers | `Void`, `Never` |

Notes:
- `Bool` and `Int` share `I64` storage. The PHP-level distinction is preserved
  via type metadata on the producing operation (`def_php_type`), not via a
  separate IR type. This avoids per-operation duplication of arithmetic
  opcodes.
- `Str` is two registers everywhere it appears, matching the existing
  `(ptr, len)` ABI from `src/codegen/abi/registers.rs`.
- `Heap` is uniform pointer-to-header. The runtime's `__rt_decref_any` uses
  the heap header to dispatch by kind. Operations that need the kind
  (e.g., `ArrayGet` vs `HashGetStr`) take the kind as an immediate
  attribute.

PHP-level type information is preserved on each `Value`'s metadata
(`Value.php_type: PhpType`) for diagnostics, validator checks, and to
inform passes that care (e.g., `MixedBox` cannot apply to a value whose
PHP type is already `Mixed`).
```

- [ ] **Step 2: Self-review for completeness**

Ensure each variant of `PhpType` (Int, Float, Str, Bool, Void, Never, Iterable, Mixed, Array, AssocArray, Buffer, Callable, Object, Packed, Pointer, Resource, Union) maps to exactly one EIR type. Cross-reference `src/types/model.rs`.

---

## Task 2: Specify `Value`, `BasicBlock`, `Function`, `Module`

**Files:**
- Modify: `docs/internals/the-ir.md` (append: "Module structure")

- [ ] **Step 1: Write the module structure section**

````markdown
## Module structure

### `ValueId` and `Value`

Values are SSA: each `ValueId` is defined exactly once. A `ValueId` is a
`u32` index into the owning function's value table.

```rust
pub struct Value {
    pub ir_type: IrType,
    pub php_type: PhpType,
    pub def: ValueDef,
    pub ownership: Ownership,
}

pub enum ValueDef {
    BlockParam { block: BlockId, index: u16 },
    Instruction { block: BlockId, index: u32 },
}

pub enum Ownership {
    NonHeap,    // I64/F64/Void scalars; never need release
    Owned,      // refcounted; this value owns +1 refcount
    Borrowed,   // refcounted; this value does not own a refcount
    MaybeOwned, // refcounted; ownership joins across CFG merges
}
```

`Ownership` mirrors `HeapOwnership` from `src/codegen/context.rs` but is
attached to *every* SSA value, not just locals.

### `BasicBlock`

```rust
pub struct BasicBlock {
    pub id: BlockId,
    pub params: Vec<ValueId>,         // block parameters (SSA-lite)
    pub instructions: Vec<InstId>,    // indices into function's instruction pool
    pub terminator: Terminator,
}
```

Blocks have a single terminator at the end and may have parameters at the
top. Branch arguments carry SSA values into the destination block,
replacing phi nodes.

### `Function`

```rust
pub struct Function {
    pub name: String,
    pub params: Vec<FunctionParam>,
    pub return_type: IrType,
    pub return_php_type: PhpType,
    pub blocks: Vec<BasicBlock>,
    pub values: Vec<Value>,
    pub instructions: Vec<Instruction>,
    pub locals: Vec<LocalSlot>,       // stack slots for PHP locals
    pub entry: BlockId,
    pub source_signature_ref: Option<FunctionSigRef>,
    pub flags: FunctionFlags,         // is_main, is_method, is_closure, etc.
}

pub struct LocalSlot {
    pub name: String,                 // PHP variable name; "" for synthetic
    pub php_type: PhpType,
    pub kind: LocalKind,              // PhpVariable, Hidden, Static, Global
}
```

### `Module`

```rust
pub struct Module {
    pub functions: Vec<Function>,
    pub class_methods: Vec<Function>, // flattened class methods
    pub data: DataPool,               // string literals, runtime tables
    pub extern_decls: Vec<ExternDecl>,
    pub target: Target,
}
```

A Module is one compilation unit. The runtime (`__rt_*` routines) is
*not* in the Module — it lives outside, exactly as today.
````

- [ ] **Step 2: Cross-check field names against existing types**

Walk `src/types/`, `src/parser/ast/`, and `src/codegen/context.rs` and verify field names match the spelling already used (e.g., `php_type` matches `PhpType`, `class_id` matches `ClassInfo.class_id`).

---

## Task 3: Specify the Instruction set

**Files:**
- Modify: `docs/internals/the-ir.md` (append: "Instructions")

This is the largest task. Specify every opcode by walking the existing codegen sites that produce it. Each opcode entry MUST include:

- Operands and result type
- Effects (Pure / Reads* / Writes* / MayThrow / MayFatal / MayDeoptimize)
- Lowering target (which `__rt_*` routine or inline ASM pattern)
- AST node(s) that produce it

- [ ] **Step 1: Write the literals/locals/globals section**

```markdown
### Literals and locals

| Op | Operands | Result | Effects | Lowers to |
|----|----------|--------|---------|-----------|
| `ConstI64(i64)` | — | `I64` | Pure | `mov reg, #imm` (or constant pool for large values) |
| `ConstF64(f64)` | — | `F64` | Pure | adr + ldr from data section |
| `ConstStr(string_id)` | — | `Str` | Pure | adr to label + immediate length |
| `ConstNull` | — | `I64` | Pure | `mov reg, #0` |
| `LoadLocal(slot_id)` | — | (slot.ir_type) | Reads(local) | `ldr reg, [x29, #-off]` |
| `StoreLocal(slot_id)` | val | `Void` | Writes(local) | `str reg, [x29, #-off]` |
| `LoadGlobal(name_id)` | — | (decl.ir_type) | Reads(global) | adr + ldr |
| `StoreGlobal(name_id)` | val | `Void` | Writes(global) | adr + str |
```

- [ ] **Step 2: Write the arithmetic/bitwise/comparison section**

```markdown
### Scalar arithmetic and bitwise

All scalar ops operate on `I64` or `F64` operands matching the op's domain.

| Op | Operands | Result | Effects | Lowers to (ARM64) |
|----|----------|--------|---------|--------------------|
| `IAdd(a, b)` | I64, I64 | I64 | Pure | `add` |
| `ISub(a, b)` | I64, I64 | I64 | Pure | `sub` |
| `IMul(a, b)` | I64, I64 | I64 | Pure | `mul` |
| `ISDiv(a, b)` | I64, I64 | I64 | MayFatal (div by zero in PHP modes that fatal) | `sdiv` |
| `ISMod(a, b)` | I64, I64 | I64 | MayFatal | `sdiv`+`msub` |
| `INeg(a)` | I64 | I64 | Pure | `neg` |
| `IBitAnd/Or/Xor/Not(a,b)/a` | I64 (1 or 2) | I64 | Pure | `and`/`orr`/`eor`/`mvn` |
| `IShl(a, b)` | I64, I64 | I64 | Pure | `lsl` |
| `IShrA(a, b)` | I64, I64 | I64 | Pure | `asr` (PHP `>>` is arithmetic) |
| `FAdd/FSub/FMul/FDiv(a, b)` | F64, F64 | F64 | Pure | `fadd`/`fsub`/`fmul`/`fdiv` |
| `FNeg(a)` | F64 | F64 | Pure | `fneg` |
| `FPow(a, b)` | F64, F64 | F64 | Pure (libc) | `bl pow` |

### Comparison

| Op | Operands | Result | Effects |
|----|----------|--------|---------|
| `ICmp(predicate, a, b)` | I64, I64 | I64 (0/1) | Pure |
| `FCmp(predicate, a, b)` | F64, F64 | I64 (0/1) | Pure |
| `StrCmpEq(a, b)` | Str, Str | I64 (0/1) | Pure (calls `__rt_str_eq`) |
| `PhpLooseEq(a, b)` | any, any | I64 (0/1) | MayDeoptimize (object comparison may invoke __toString) |
| `PhpIdentical(a, b)` | any, any | I64 (0/1) | Pure (type-tag aware) |
| `Spaceship(a, b)` | any, any | I64 | as PhpLooseEq |

`predicate` is `Eq`, `Ne`, `Slt`, `Sle`, `Sgt`, `Sge` for integers and the
float equivalents (`Olt`, ...) for floats, mapping to PHP's signed
comparison semantics.
```

- [ ] **Step 3: Write the conversion / cast section**

```markdown
### Conversions

| Op | From | To | Effects | Notes |
|----|------|----|---------|-------|
| `IToF(a)` | I64 | F64 | Pure | PHP int-to-float widening |
| `FToI(a)` | F64 | I64 | Pure | PHP float-to-int (truncate, PHP rules) |
| `IToStr(a)` | I64 | Str | AllocConcatBuf | calls `__rt_itoa` |
| `FToStr(a)` | F64 | Str | AllocConcatBuf | calls `__rt_ftoa` |
| `BoolToStr(a)` | I64 | Str | Pure | "" or "1" |
| `StrToI(a)` | Str | I64 | Pure | calls `__rt_str_to_int` |
| `StrToF(a)` | Str | F64 | Pure | calls `__rt_str_to_float` |
| `MixedBox(a)` | any (non-Mixed) | Heap | AllocHeap | tags value into Mixed cell |
| `MixedUnbox(a, expected_tag)` | Heap (Mixed) | I64/F64/Str/Heap | Pure + MayFatal | extracts payload |
| `MixedTagOf(a)` | Heap (Mixed) | I64 | Pure | returns tag |
| `Cast(a, to_php_type)` | any | matching IR type | as PHP cast | dispatches to specific helper |
```

- [ ] **Step 4: Write the string ops section**

```markdown
### String operations

| Op | Operands | Result | Effects | Lowers to |
|----|----------|--------|---------|-----------|
| `StrConcat(a, b)` | Str, Str | Str | AllocConcatBuf | `__rt_str_concat` |
| `StrLen(a)` | Str | I64 | Pure | inline `mov reg, len` |
| `StrCharAt(a, i)` | Str, I64 | Str (1-char) | MayFatal (oob), AllocConcatBuf | `__rt_str_char_at` |
| `StrPersist(a)` | Str | Str | AllocHeap (idempotent) | `__rt_str_persist` |
| `StrInterpolate(parts, vals)` | varargs | Str | AllocConcatBuf | builds in concat buf |
```

- [ ] **Step 5: Write the array/hash/object ops section**

```markdown
### Array and hash

The `kind` immediate distinguishes indexed Array, hash AssocArray, and
mixed-key payload.

| Op | Operands | Result | Effects | Notes |
|----|----------|--------|---------|-------|
| `ArrayNew(kind, capacity)` | — | Heap | AllocHeap | calls `__rt_array_new` |
| `ArrayLen(a)` | Heap | I64 | Pure | inline header read |
| `ArrayGet(arr, idx)` | Heap, I64 | (element ir_type) | Reads(heap) + MayFatal? | `__rt_array_get_int` |
| `ArraySet(arr, idx, val)` | Heap, I64, any | Void | Writes(heap), AllocHeap (COW) | `__rt_array_set_int` |
| `ArrayPush(arr, val)` | Heap, any | Void | Writes(heap), AllocHeap | `__rt_array_push` |
| `ArrayCowEnsureUnique(arr)` | Heap | Heap | AllocHeap (maybe) | `__rt_array_cow_ensure` |
| `HashGetStr(h, key)` | Heap, Str | (value ir_type) | Reads(heap) + MayFatal? | `__rt_hash_get_str` |
| `HashGetInt(h, key)` | Heap, I64 | (value ir_type) | Reads(heap) + MayFatal? | `__rt_hash_get_int` |
| `HashSetStr/Int(h, k, v)` | Heap, key, val | Void | Writes(heap), AllocHeap | `__rt_hash_set_*` |
| `HashKeyExists(h, k)` | Heap, key | I64 | Reads(heap) | `__rt_hash_exists_*` |
| `IterNext(iter)` | Heap | (key, value) | Reads(heap), MayDeoptimize (Iterator::next) | `__rt_iter_next` |
| `IterCurrent(iter)` | Heap | (key, value) | Reads(heap) | `__rt_iter_current` |

### Object

| Op | Operands | Result | Effects | Notes |
|----|----------|--------|---------|-------|
| `ObjectNew(class_id)` | — | Heap | AllocHeap | `__rt_object_alloc` then constructor call |
| `PropGet(obj, offset, ir_type)` | Heap, immediate | ir_type | Reads(heap) | inline `ldr` |
| `PropSet(obj, offset, val)` | Heap, immediate, any | Void | Writes(heap) | inline `str` + retain handling |
| `VTableLookup(obj, method_id)` | Heap | I64 (fn ptr) | Reads(heap) | reads class header |
| `InstanceOf(obj, class_id)` | Heap | I64 | Reads(heap) | `__rt_instanceof` |
```

- [ ] **Step 6: Write the calls section**

```markdown
### Calls

| Op | Operands | Result | Effects | Notes |
|----|----------|--------|---------|-------|
| `Call(func_id, args)` | varargs | (sig return) | per callee | user-defined PHP call |
| `IndirectCall(fn_ptr, sig, args)` | varargs | (sig return) | MayDeoptimize | closure/callable |
| `MethodCall(obj, method_id, args)` | Heap + varargs | (sig return) | per callee | virtual dispatch |
| `BuiltinCall(builtin, args)` | varargs | (builtin return) | per builtin (from `src/optimize/effects/builtins.rs`) | inline-emit |
| `RuntimeCall(rt_routine, args)` | varargs | (rt return) | per routine | `__rt_*` |
| `ExternCall(name, args)` | varargs | (extern return) | per FFI sig | direct C call |
```

Each call instruction declares its **effect summary** at lowering time
from the existing `src/optimize/effects/` analysis. The IR keeps the
summary; passes do not re-derive it.

- [ ] **Step 7: Write the ownership/control-flow section**

```markdown
### Ownership operations

These operations are *explicit* in the IR. The AST → IR builder inserts
them. The validator checks balance along every path. The register
allocator must not eliminate them; later passes may move them.

| Op | Operands | Result | Effects | Lowers to |
|----|----------|--------|---------|-----------|
| `Acquire(a)` | refcounted | Void | Writes(refcount) | `bl __rt_incref` |
| `Release(a)` | refcounted | Void | Writes(refcount), MayFatal (debug heap) | `bl __rt_decref_any` |
| `Move(a)` | any | (same type) | Pure (transfers ownership) | no-op at codegen, validator-only |
| `Borrow(a)` | refcounted | (same type) | Pure | no-op at codegen, validator-only |

### Terminators

Every basic block ends with exactly one terminator.

| Term | Operands | Notes |
|------|----------|-------|
| `Br(target, args)` | block, list | unconditional branch |
| `CondBr(cond, then, then_args, else_, else_args)` | I64 cond | conditional |
| `Switch(scrutinee, cases, default)` | I64 | jump table or chain |
| `Return(value?)` | optional value | function epilogue |
| `Throw(value)` | Heap (exception object) | longjmp via `__rt_throw` |
| `Fatal(msg_id)` | immediate | unrecoverable error path |
| `Unreachable` | — | provably unreachable (after `never`-typed call) |
```

- [ ] **Step 8: Self-review the instruction set**

Walk every file under `src/codegen/expr/` and `src/codegen/stmt/`. For each emission helper, ask "which EIR ops does this lower to?" — if you cannot answer, the IR is missing something or the helper is doing too much (which may be a refactor finding). Make a checklist of files visited.

Files to walk (minimum):
- `src/codegen/expr/scalars.rs`
- `src/codegen/expr/binops/arithmetic.rs`
- `src/codegen/expr/binops/comparison.rs`
- `src/codegen/expr/binops/array_union.rs`
- `src/codegen/expr/calls.rs` and `src/codegen/expr/calls/args.rs`
- `src/codegen/expr/arrays.rs` (and sub-dir)
- `src/codegen/expr/objects.rs` (and sub-dir)
- `src/codegen/expr/chains.rs`
- `src/codegen/expr/assignment.rs`
- `src/codegen/expr/variables.rs`
- `src/codegen/expr/ternary.rs`
- `src/codegen/expr/compare/`
- `src/codegen/expr/coerce.rs`
- `src/codegen/expr/helpers.rs`
- `src/codegen/expr/ownership.rs`
- `src/codegen/stmt/` (all)
- `src/codegen/builtins/` (sample 5 categories: strings, arrays, math, io, oop)
- `src/codegen/runtime/` (sample 5 categories)

---

## Task 4: Specify effect lattice

**Files:**
- Modify: `docs/internals/the-ir.md` (append: "Effects")

- [ ] **Step 1: Write the effects section**

```markdown
## Effects

Each EIR instruction carries an immutable `Effects` bitset assigned at
construction time. Effects model what an instruction *may* do:

```rust
bitflags! {
    pub struct Effects: u16 {
        const READS_LOCAL    = 0b0000_0000_0000_0001;
        const READS_HEAP     = 0b0000_0000_0000_0010;
        const READS_GLOBAL   = 0b0000_0000_0000_0100;
        const READS_FS       = 0b0000_0000_0000_1000;
        const WRITES_LOCAL   = 0b0000_0000_0001_0000;
        const WRITES_HEAP    = 0b0000_0000_0010_0000;
        const WRITES_GLOBAL  = 0b0000_0000_0100_0000;
        const WRITES_FS      = 0b0000_0000_1000_0000;
        const ALLOC_HEAP     = 0b0000_0001_0000_0000;
        const ALLOC_CONCAT   = 0b0000_0010_0000_0000;
        const MAY_THROW      = 0b0000_0100_0000_0000;
        const MAY_FATAL      = 0b0000_1000_0000_0000;
        const MAY_DEOPT      = 0b0001_0000_0000_0000;
        const REFCOUNT_OP    = 0b0010_0000_0000_0000;
    }
}

impl Effects {
    pub const PURE: Effects = Effects::empty();
    pub fn is_pure(&self) -> bool { self.is_empty() }
    pub fn may_observe(&self) -> bool {
        self.intersects(
            Effects::READS_LOCAL | Effects::READS_HEAP | Effects::READS_GLOBAL
                | Effects::READS_FS | Effects::ALLOC_HEAP
                | Effects::MAY_THROW | Effects::MAY_FATAL | Effects::MAY_DEOPT
        )
    }
    pub fn may_mutate(&self) -> bool {
        self.intersects(
            Effects::WRITES_LOCAL | Effects::WRITES_HEAP | Effects::WRITES_GLOBAL
                | Effects::WRITES_FS | Effects::REFCOUNT_OP
        )
    }
}
```

Sources of effect data for the builder:
- Arithmetic / comparison / scalar ops — hardcoded
- BuiltinCall — looked up from `src/optimize/effects/builtins.rs`
- Call / MethodCall — derived from `FunctionSig` purity flags, falling
  back to "all effects" when unknown
- ExternCall — always conservative (`READS_HEAP | WRITES_HEAP | MAY_THROW`)
- RuntimeCall — declared in a per-routine table maintained in the IR
  module

The validator checks that operations with `MAY_DEOPT` are not silently
reordered across `ALLOC_HEAP` instructions before passes that don't
preserve allocation order.
```

---

## Task 5: Specify validator rules

**Files:**
- Modify: `docs/internals/the-ir.md` (append: "Validator")

- [ ] **Step 1: Write the validator rules section**

```markdown
## Validator

The validator runs after every IR pass (cheap mode: structural;
expensive mode: ownership + dominance). Failures are *bugs*, not
diagnostics — they abort compilation.

### Structural rules

1. Every basic block ends with exactly one terminator. The terminator
   appears nowhere else in the block.
2. Every `ValueId` is defined exactly once.
3. Every use of a `ValueId` either (a) follows its definition in the
   same block, or (b) the defining block dominates the using block.
4. Block parameter counts at the destination match the argument counts
   in every incoming branch.
5. `IrType` matches between definition and every use.
6. `entry` block has no parameters (function parameters are loaded via
   `LoadLocal` in the entry block).

### Ownership rules

1. Every `Owned` value reaches exactly one consuming op (`Release`,
   `Move`, or `Return`) along every CFG path from its definition.
2. `Borrow` does not increase or decrease refcount; the source must
   outlive the borrowed value.
3. At every CFG merge, ownership states from incoming edges must be
   compatible: identical states merge to themselves; `Owned` + `Owned`
   = `Owned`; `Borrowed` + `Borrowed` = `Borrowed`; mixed = `MaybeOwned`
   (the validator emits a runtime branch in lowering).
4. `Return` of an `Owned` value transfers ownership to the caller;
   `Return` of `Borrowed` requires an explicit prior `Acquire`.

### Effect rules

1. `Pure` operations may not have side-effect dependencies via memory.
2. Operations with `MAY_FATAL` define a control-flow effect: passes that
   reorder them must not move them past observable operations.
3. `ALLOC_CONCAT` operations are sensitive to PHP's concat-buffer reuse
   policy: per-statement reset is preserved by the lowering, and passes
   must not reorder concat ops across statement boundaries.
```

- [ ] **Step 2: Self-review**

Verify ownership rules against `src/codegen/expr/ownership.rs` and `src/codegen/context.rs::HeapOwnership`. Adjust rules if the actual codegen has cases not covered (e.g., container propagation, foreach values).

---

## Task 6: Specify the textual format

**Files:**
- Modify: `docs/internals/the-ir.md` (append: "Textual format")

- [ ] **Step 1: Write the textual format section**

````markdown
## Textual format

EIR has a printable textual format used for snapshot tests, `--emit-ir`,
and debugging. It is **printer-only** — no parser. Format does not need
to be machine-readable, only human-readable.

Example:

```eir
function add_pair(p0: I64, p1: I64) -> I64 {
  entry:
    v0 = const_i64 0
    store_local slot[0] "result", v0
    v1 = load_local slot[0]
    v2 = iadd v1, p0   ; effects: pure
    store_local slot[0], v2
    v3 = load_local slot[0]
    v4 = iadd v3, p1
    store_local slot[0], v4
    v5 = load_local slot[0]
    return v5
}

function map_word_count(p0: Str) -> Heap[Hash] {
  entry:
    v0 = builtin_call explode " ", p0    ; effects: alloc_heap, alloc_concat
    own v0
    v1 = hash_new                        ; effects: alloc_heap
    own v1
    br loop(v0, 0_i64, v1)

  loop(arr: Heap[Array], i: I64, acc: Heap[Hash]):
    v2 = array_len arr
    v3 = icmp slt i, v2
    cond_br v3, body(arr, i, acc), exit(acc)

  body(arr: Heap[Array], i: I64, acc: Heap[Hash]):
    v4 = array_get arr, i                ; effects: reads_heap, may_fatal
    borrow v4
    ; ... omitted ...
    v5 = iadd i, 1_i64
    br loop(arr, v5, acc)

  exit(acc: Heap[Hash]):
    release v0   ; arr no longer needed
    return acc   ; transfer ownership
}
```

Notes:
- Function header shows IR type. `Heap[Hash]` displays subkind metadata.
- Block params use `name: Type` after `block_name(`.
- Branch arguments use the destination's param order.
- `own`, `borrow`, `release` are the ownership ops.
- Effects, when displayed, appear as `; effects: ...` comments.
````

---

## Task 7: Cross-reference Lowering catalogue

**Files:**
- Modify: `docs/internals/the-ir.md` (append: "AST → EIR lowering catalogue")

- [ ] **Step 1: Write the lowering catalogue**

For every `ExprKind` and `StmtKind` variant in `src/parser/ast/expr.rs` and `src/parser/ast/stmt.rs`, give a one-paragraph lowering recipe. Example entries:

```markdown
### `ExprKind::BinaryOp { op: Add, lhs, rhs }`

Lower `lhs` and `rhs` recursively. Their result types determine the op:

- Both `I64` → `IAdd`
- Both `F64` → `FAdd`
- Mixed → coerce via `IToF` on the int side, then `FAdd`
- `Str` + `Str` → `StrConcat` (PHP `.` is a separate op; `+` on strings
  is an array union or coerced numeric add depending on operand types,
  per PHP rules)

### `StmtKind::Foreach { iter, key, value, body }`

Lower `iter` → `Heap` value. Insert `IterStart`. Loop block reads
`IterCurrent`, binds key/value to locals, runs the body, then
`IterNext` and branches back. Exit block runs `IterEnd`. Owned-vs-
borrowed semantics follow PHP foreach rules (by-value vs by-ref).
```

The full catalogue is mandatory and lists every variant. Step 1 produces the catalogue document; Step 2 is the review.

- [ ] **Step 2: Validate completeness against AST**

Run:
```bash
grep -E "^[[:space:]]*[A-Z][A-Za-z0-9_]*[ {,(]" src/parser/ast/expr.rs src/parser/ast/stmt.rs
```
Cross-check every variant has an entry. Missing entries are plan failures.

---

## Task 8: Specify how `--emit-ir` works

**Files:**
- Modify: `docs/internals/the-ir.md` (append: "CLI surface")

- [ ] **Step 1: Write the CLI section**

```markdown
## CLI

`--emit-ir` (added in Phase 03) prints the EIR for the compiled program
to stdout and exits without invoking the assembler. Useful for debugging
and for snapshot tests.

`--ir-backend` (added in Phase 05) selects the IR pipeline; default off
during Phases 03–04, default on at the end of Phase 05, removed in
Phase 09 when the legacy backend is deleted.
```

---

## Task 9: Self-review and commit

- [ ] **Step 1: Read the document end-to-end**

Check for:
- Placeholders, "TBD", "fill in later"
- Field-name inconsistencies (e.g., `ir_type` vs `irType`)
- Op names that overlap or duplicate
- Missing AST variants in the lowering catalogue
- Missing effects on instructions

Fix inline.

- [ ] **Step 2: Commit**

```bash
git add docs/internals/the-ir.md
git commit -m "docs: introduce EIR design specification (phase 01)"
```

---

## Exit criteria

- `docs/internals/the-ir.md` exists and is complete
- Every `ExprKind` and `StmtKind` has a lowering entry
- Every EIR op has effects, operands, result type, and a lowering target
- Validator rules cover SSA, ownership, and effects
- Textual format example renders a non-trivial function correctly
- No "TBD" / "TODO" / placeholder text anywhere
