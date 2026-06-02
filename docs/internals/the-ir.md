---
title: "The EIR Design"
description: "Planned elephc intermediate representation between AST optimization and assembly emission."
sidebar:
  order: 13
---

**Status:** planned for the v0.24.x EIR track. The current production
pipeline still emits assembly directly from the checked and optimized AST.
The implementation phases live in `.plans/eir-*.md`.

**Source once implemented:** `src/ir/`, `src/ir_lower/`, and
`src/codegen_ir/`.

EIR is the planned elephc intermediate representation. It sits between the
AST-level optimizer and the assembly emitter, giving the compiler a
function-wide control-flow and value model without replacing the hand-written
assembly backend.

The design is intentionally PHP-shaped. It represents arrays, hashes, Mixed
boxing, ownership, copy-on-write checks, runtime calls, fatal paths, and exact
evaluation order as first-class compiler concepts. It is not a generic LLVM- or
Cranelift-style IR.

## Pipeline Position

The current backend lowers this shape:

```text
PHP source -> frontend passes -> AST optimizer -> AST codegen -> assembly
```

The EIR track changes only the backend boundary:

```text
PHP source
  -> Lexer
  -> Parser
  -> Magic constants
  -> Conditional compilation
  -> Resolver
  -> NameResolver
  -> AST constant folding
  -> Type checker / warnings
  -> AST optimizer passes
  -> AST -> EIR lowering
  -> EIR validation and passes
  -> EIR -> assembly backend
  -> assembler / linker
  -> binary
```

The AST optimizer remains useful. It handles local PHP-preserving rewrites such
as constant folding, control-flow pruning, and conservative dead-code
elimination. EIR adds the function-wide machinery that AST walking does not
provide: basic blocks, value identity, liveness, dominance, register allocation,
and later CSE / LICM / inlining.

## Design Goals

- Preserve PHP semantics exactly where elephc implements PHP syntax.
- Keep target-specific ABI details in `src/codegen/abi/`; EIR is target-aware
  through metadata, not hardcoded registers.
- Keep runtime ownership explicit so refcount, COW, and cleanup paths are
  visible to validators and passes.
- Keep assembly educational: the final emitter still writes hand-authored,
  commented assembly instructions.
- Make the first backend migration behavior-preserving. Register allocation and
  optimization come after parity.

## Types

EIR uses a small storage type lattice. PHP-level distinctions that share the
same runtime representation are preserved as metadata on values and
instructions.

| EIR type | Storage | Maps from `PhpType` |
|---|---|---|
| `I64` | One integer register or stack slot | `Int`, `Bool`, `Pointer(_)`, `Resource(_)`, `Callable` |
| `F64` | One floating-point register or stack slot | `Float` |
| `Str` | `(ptr, len)` pair | `Str` |
| `Heap(kind)` | One pointer to a runtime header or descriptor | `Iterable`, `Mixed`, `Array(_)`, `AssocArray { .. }`, `Buffer(_)`, `Object(_)`, `Packed(_)`, `Union(_)` |
| `Void` | No materialized value | `Void`, `Never` |

`Bool` and `Int` share `I64` storage. The PHP distinction is carried in value
metadata, because the emitted representation is the same width.

`Str` is always a two-register value, matching the current ABI helpers for PHP
strings.

`Heap(kind)` carries a subkind such as `Array`, `Hash`, `Object`, `Mixed`,
`Iterable`, `Buffer`, or `Packed`. The subkind guides validation and lowering,
while runtime helpers still dispatch through their existing headers and
metadata.

## Module Structure

EIR is SSA-lite with block parameters instead of phi nodes. Values are defined
once and branch arguments feed values into successor block parameters.

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
    NonHeap,
    Owned,
    Borrowed,
    MaybeOwned,
}
```

`Ownership` is attached to every SSA value, not just locals. Scalars are
`NonHeap`. Refcounted values start as `Owned` or `Borrowed`, and CFG joins can
produce `MaybeOwned` when incoming ownership states differ.

```rust
pub struct BasicBlock {
    pub id: BlockId,
    pub params: Vec<ValueId>,
    pub instructions: Vec<InstId>,
    pub terminator: Terminator,
}

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

pub struct Module {
    pub functions: Vec<Function>,
    pub data: DataPool,
    pub extern_decls: Vec<ExternDecl>,
    pub target: Target,
}
```

The runtime is not part of the `Module`. Shared `__rt_*` routines stay in the
runtime object path described in [The Code Generator](the-codegen.md).

## Instructions

Each instruction records operands, result type, PHP type metadata, source span
where useful, and an immutable effect summary.

### Literals and Storage

| Op | Operands | Result | Effects | Lowering |
|---|---|---|---|---|
| `ConstI64(i64)` | none | `I64` | pure | immediate or constant-pool load |
| `ConstF64(f64)` | none | `F64` | pure | data-section load |
| `ConstStr(string_id)` | none | `Str` | pure | data label plus length |
| `ConstNull` | none | `I64` | pure | zero/null sentinel |
| `LoadLocal(slot)` | none | slot type | reads local | frame slot load |
| `StoreLocal(slot, value)` | value | `Void` | writes local | frame slot store |
| `LoadGlobal(name)` | none | declared type | reads global | global data load |
| `StoreGlobal(name, value)` | value | `Void` | writes global | global data store |

### Scalars and Comparisons

| Op | Operands | Result | Effects |
|---|---|---|---|
| `IAdd`, `ISub`, `IMul` | `I64`, `I64` | `I64` | pure |
| `ISDiv`, `ISMod` | `I64`, `I64` | `I64` | may fatal on invalid divisor |
| `INeg`, `IBitNot` | `I64` | `I64` | pure |
| `IBitAnd`, `IBitOr`, `IBitXor`, `IShl`, `IShrA` | `I64`, `I64` | `I64` | pure |
| `FAdd`, `FSub`, `FMul`, `FDiv`, `FPow` | `F64`, `F64` | `F64` | pure, except target libcall details |
| `ICmp(predicate)` | `I64`, `I64` | `I64` bool | pure |
| `FCmp(predicate)` | `F64`, `F64` | `I64` bool | pure |
| `StrCmpEq` | `Str`, `Str` | `I64` bool | reads bytes |
| `PhpLooseEq`, `PhpIdentical`, `Spaceship` | typed values | `I64` | PHP comparison effects |

Comparison predicates preserve PHP signed integer and floating-point comparison
semantics. Loose comparison remains PHP-specific because it may involve runtime
coercions or object/string behavior.

### Conversions

| Op | From | To | Effects |
|---|---|---|---|
| `IToF` | `I64` | `F64` | pure |
| `FToI` | `F64` | `I64` | pure PHP truncation rules |
| `IToStr`, `FToStr` | scalar | `Str` | allocates concat/runtime buffer |
| `BoolToStr` | `I64` bool | `Str` | pure or static string |
| `StrToI`, `StrToF` | `Str` | scalar | reads bytes |
| `MixedBox` | non-Mixed value | `Heap(Mixed)` | allocates heap |
| `MixedUnbox` | `Heap(Mixed)` | requested type | may fatal on invalid tag |
| `MixedTagOf` | `Heap(Mixed)` | `I64` | reads heap tag |
| `Cast(to_php_type)` | typed value | matching EIR type | PHP cast effects |

### Strings

| Op | Operands | Result | Effects |
|---|---|---|---|
| `StrConcat` | `Str`, `Str` | `Str` | allocates concat buffer |
| `StrLen` | `Str` | `I64` | pure |
| `StrCharAt` | `Str`, `I64` | `Str` | may fatal, allocates one-byte string |
| `StrPersist` | `Str` | `Str` | may allocate heap |
| `StrInterpolate` | parts and values | `Str` | concat-buffer effects |

### Arrays, Hashes, and Iterables

| Op | Operands | Result | Effects |
|---|---|---|---|
| `ArrayNew(kind, capacity)` | none | `Heap(Array)` or `Heap(Hash)` | allocates heap |
| `ArrayLen` | array heap | `I64` | reads heap |
| `ArrayGet` | array heap, index | element type | reads heap, may fatal |
| `ArraySet` | array heap, index, value | `Void` | writes heap, may allocate |
| `ArrayPush` | array heap, value | `Void` | writes heap, may allocate |
| `ArrayCowEnsureUnique` | array heap | array heap | may allocate |
| `HashGetStr`, `HashGetInt` | hash heap, key | value type | reads heap, may fatal |
| `HashSetStr`, `HashSetInt` | hash heap, key, value | `Void` | writes heap, may allocate |
| `HashKeyExists` | hash heap, key | `I64` bool | reads heap |
| `IterStart`, `IterCurrent`, `IterNext`, `IterEnd` | iterable heap | iterator values | reads or writes runtime iterator state |

Indexed arrays and associative arrays stay separate operations even when both
lower to heap pointers, because COW, key handling, and runtime helper selection
differ.

### Objects and Classes

| Op | Operands | Result | Effects |
|---|---|---|---|
| `ObjectNew(class_id)` | constructor args | `Heap(Object)` | allocates heap, then calls constructor |
| `NewDynamicObject` | class-string value, fallback metadata, args | `Heap(Object)` | may fatal, allocates heap |
| `PropGet` | object heap, field metadata | field type | reads heap |
| `PropSet` | object heap, field metadata, value | `Void` | writes heap, ownership-sensitive |
| `DynamicPropGet`, `DynamicPropSet` | object heap, property value, optional value | typed value or `Void` | runtime property effects |
| `StaticPropGet`, `StaticPropSet` | static receiver metadata | typed value or `Void` | reads or writes global/static storage |
| `VTableLookup` | object heap, method id | function pointer | reads object metadata |
| `InstanceOf` | object or mixed value, target | `I64` bool | reads class metadata |
| `ClassConstant`, `ScopedConstantGet` | receiver metadata | typed value | reads metadata or constant storage |

### Calls

| Op | Operands | Result | Effects |
|---|---|---|---|
| `Call(func_id, args)` | PHP args | signature return | callee summary |
| `IndirectCall(callable, args)` | callable descriptor and args | signature return | conservative callable effects |
| `MethodCall(receiver, method_id, args)` | object and args | signature return | method summary |
| `StaticMethodCall(receiver, method_id, args)` | args | signature return | method summary |
| `BuiltinCall(name, args)` | args | builtin return | builtin effect table |
| `RuntimeCall(name, args)` | args | runtime return | runtime effect table |
| `ExternCall(name, args)` | C ABI args | extern return | conservative FFI effects |

Call argument planning remains shared with the existing type-checker and codegen
rules. EIR lowering consumes the semantic call-argument plan; it does not
reimplement named/spread matching locally.

### Ownership

Ownership operations are explicit so optimization passes can reason about
refcount lifetimes.

| Op | Operands | Result | Effects |
|---|---|---|---|
| `Acquire` | refcounted value | `Void` | refcount write |
| `Release` | refcounted value | `Void` | refcount write, debug heap may fatal |
| `Move` | any value | same type | validator-only transfer |
| `Borrow` | refcounted value | same type | validator-only borrowed alias |

`Move` and `Borrow` usually lower to no machine instruction. They are still
part of the IR because removing or moving them incorrectly can break cleanup
balance.

### Terminators

Every block ends with exactly one terminator.

| Terminator | Operands | Meaning |
|---|---|---|
| `Br(target, args)` | destination and block args | unconditional jump |
| `CondBr(cond, then, then_args, else, else_args)` | `I64` bool and destinations | conditional jump |
| `Switch(scrutinee, cases, default)` | integer/string-compatible subject | case dispatch |
| `Return(value?)` | optional value | function exit and ownership transfer |
| `Throw(value)` | exception object | exception runtime path |
| `Fatal(msg_id)` | diagnostic message id | unrecoverable PHP fatal |
| `Unreachable` | none | impossible path after prior proof |

## Effects

Each instruction carries an immutable effect bitset assigned by the builder.
Passes consume this summary; they do not rediscover effects from scratch.

| Effect | Meaning |
|---|---|
| `READS_LOCAL`, `WRITES_LOCAL` | touches PHP local or synthetic local slots |
| `READS_HEAP`, `WRITES_HEAP` | observes or mutates heap/runtime object state |
| `READS_GLOBAL`, `WRITES_GLOBAL` | observes or mutates global/static state |
| `READS_FS`, `WRITES_FS` | may touch filesystem, process, or external state |
| `ALLOC_HEAP` | may allocate refcounted runtime storage |
| `ALLOC_CONCAT` | may use the concat scratch buffer |
| `MAY_THROW` | may raise a catchable exception |
| `MAY_FATAL` | may terminate with a PHP fatal |
| `MAY_DEOPT` | may invoke dynamic behavior not reducible to a pure local op |
| `REFCOUNT_OP` | changes runtime refcount state |

Effect data comes from hardcoded scalar rules, the existing builtin effect
model in `src/optimize/effects/`, function signatures, extern signatures, and
a runtime-routine effect table added with EIR.

## Validator

The validator runs after lowering and after every IR pass. Failures are compiler
bugs, not user-facing PHP diagnostics.

Structural rules:

- Every block has exactly one terminator.
- Every `ValueId` is defined exactly once.
- Uses are dominated by definitions, or use a value passed as a block
  parameter.
- Branch argument counts and types match destination block parameters.
- Instruction operand and result `IrType`s match their opcode contract.
- The entry block has no block parameters; PHP function parameters are modeled
  through function metadata and local slots.

Ownership rules:

- Each `Owned` refcounted value is consumed exactly once on every reachable CFG
  path by `Release`, `Move`, or `Return`.
- Returning an `Owned` value transfers ownership to the caller.
- Returning a borrowed value requires a prior `Acquire`.
- `Borrow` never changes refcount and must not outlive its owner.
- CFG joins merge compatible ownership states; incompatible joins become
  `MaybeOwned` and must be resolved before codegen.

Effect rules:

- Pure operations cannot hide heap/global/filesystem dependencies.
- `MAY_FATAL` and `MAY_THROW` operations cannot be reordered past visible side
  effects unless a pass proves the transformation preserves PHP behavior.
- `ALLOC_CONCAT` operations keep statement-boundary ordering so concat-buffer
  reuse remains valid.

## Textual Format

EIR has a printer for `--emit-ir`, tests, and debugging. The format is
printer-only; there is no planned parser in the v0.24 track.

```eir
function add_pair(p0: I64, p1: I64) -> I64 {
  entry:
    v0 = const_i64 0
    store_local slot[0] "result", v0
    v1 = load_local slot[0]
    v2 = iadd v1, p0
    store_local slot[0], v2
    v3 = load_local slot[0]
    v4 = iadd v3, p1
    store_local slot[0], v4
    v5 = load_local slot[0]
    return v5
}
```

Function headers show IR types. Heap values print their subkind, such as
`Heap[Array]` or `Heap[Object:Point]`. Effects can be printed as comments when
useful for snapshots.

## AST Lowering Catalogue

Lowering is implemented under `src/ir_lower/` and must cover every current
variant in `src/parser/ast/expr.rs` and `src/parser/ast/stmt.rs`.

Expression lowering:

| Variant | EIR lowering |
|---|---|
| `StringLiteral`, `IntLiteral`, `FloatLiteral`, `BoolLiteral`, `Null` | Emit constant operations with PHP type metadata. |
| `Variable`, `This`, `ConstRef` | Load from local, implicit receiver, or resolved constant storage. |
| `BinaryOp` | Lower operands in PHP source order, then emit scalar, concat, comparison, logical, null-coalesce, array-union, or power operation based on type and operator. |
| `InstanceOf` | Lower receiver and target metadata, then emit `InstanceOf` or runtime dynamic target handling. |
| `Negate`, `Not`, `BitNot` | Lower operand and emit unary scalar op or PHP coercion before the op. |
| `Throw`, `ErrorSuppress`, `Print` | Preserve observable control/output behavior with terminator/runtime or output ops. |
| `NullCoalesce`, `Ternary`, `ShortTernary`, `Match`, `Pipe` | Build blocks that preserve PHP short-circuiting and source evaluation order. |
| `Assignment` | Lower prelude, target address/storage plan, RHS, ownership transfer, and result value. |
| `PreIncrement`, `PostIncrement`, `PreDecrement`, `PostDecrement` | Load local, compute new value, store it, and return old or new value as PHP requires. |
| `FunctionCall`, `ClosureCall`, `ExprCall`, `MethodCall`, `StaticMethodCall`, `NullsafeMethodCall` | Consume the shared call-argument plan, preserve source-order side effects, then emit direct, indirect, virtual, static, or nullsafe call operations. |
| `NamedArg`, `Spread` | Remain call-argument forms consumed by call lowering, not standalone runtime ops. |
| `ArrayLiteral`, `ArrayLiteralAssoc`, `ArrayAccess` | Emit array/hash allocation, element insertion, COW checks, reads, and key handling. |
| `Cast`, `PtrCast`, `BufferNew` | Emit conversion, pointer typing, or buffer allocation operations with checker-provided metadata. |
| `Closure`, `FirstClassCallable` | Emit callable descriptors, captures, and deferred callable bodies. |
| `NewObject`, `NewDynamicObject`, `NewScopedObject` | Emit object allocation plus constructor dispatch, including late-static or dynamic class resolution. |
| `PropertyAccess`, `DynamicPropertyAccess`, `NullsafePropertyAccess`, `NullsafeDynamicPropertyAccess`, `StaticPropertyAccess` | Emit property metadata lookup, runtime dynamic access, nullsafe branching, or static storage access. |
| `ClassConstant`, `ScopedConstantAccess`, `MagicConstant` | Use already-resolved metadata or lowered literals; raw magic constants should not reach optimizer/codegen. |
| `Yield`, `YieldFrom` | Emit generator state-machine values and suspension/forwarding operations once generator lowering reaches EIR. |

Statement lowering:

| Variant | EIR lowering |
|---|---|
| `Echo`, `ExprStmt`, `Throw`, `Return` | Lower contained expression and emit output, discarded-value cleanup, throw terminator, or return terminator. |
| `Assign`, `TypedAssign`, `RefAssign`, `ListUnpack` | Lower RHS, validate/store into locals or ref cells, and model ownership transfer. |
| `ArrayAssign`, `NestedArrayAssign`, `ArrayPush` | Emit target loads, COW uniqueness, write/push runtime ops, and balanced cleanup. |
| `PropertyAssign`, `PropertyArrayPush`, `PropertyArrayAssign` | Emit object/property storage operations with value retention and array write semantics. |
| `StaticPropertyAssign`, `StaticPropertyArrayPush`, `StaticPropertyArrayAssign` | Emit static-property storage operations and runtime array mutation where needed. |
| `If`, `IfDef`, `Switch` | Build conditional CFG blocks; `IfDef` should normally be resolved before IR, but lowering handles any residual synthetic form conservatively. |
| `While`, `DoWhile`, `For`, `Foreach` | Build loop header/body/exit blocks, break/continue targets, iterator state, and loop-carried ownership. |
| `Break`, `Continue` | Branch to the resolved loop target with required cleanup. |
| `Try` | Build exception-region blocks, catch dispatch, finally paths, and cleanup edges. |
| `Include`, `IncludeOnceMark`, `IncludeOnceGuard` | Lower residual include guards produced by resolver; most executable include bodies are already inlined before EIR. |
| `Synthetic` | Lower contained statements in order. |
| `NamespaceDecl`, `NamespaceBlock`, `UseDecl` | Usually removed or flattened by name resolution; any residual form lowers by processing contained statements or no-op metadata. |
| `FunctionDecl`, `FunctionVariantGroup`, `FunctionVariantMark` | Register or emit function bodies/variant dispatch metadata at module level. |
| `ConstDecl`, `Global`, `StaticVar` | Emit constant metadata, global binding storage, or static slot initialization. |
| `ClassDecl`, `EnumDecl`, `PackedClassDecl`, `InterfaceDecl`, `TraitDecl` | Emit or record class-like metadata; method bodies lower as functions. |
| `ExternFunctionDecl`, `ExternClassDecl`, `ExternGlobalDecl` | Record FFI metadata for `ExternCall`, pointer layout, and global access lowering. |

## CLI Surface

`--emit-ir` is planned for Phase 03. It prints the EIR for the compiled program
and exits before assembly and linking.

`--ir-backend` is planned for Phase 04/05. It selects the IR-consuming backend
while the legacy AST backend remains available. After the default switch and
validation period, the legacy backend is removed by the Phase 09 cleanup plan.
