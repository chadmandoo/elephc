# Plan: eval, elephc-magician, and Literal Eval AOT

## Task

- [x] Define the target semantics of `eval`: visible caller scope, persistent
  writes, variables created inside eval visible after eval, `unset`, output,
  parse errors, fragment-local `return`, dynamic declarations, and `$this`.
- [x] Add `crates/elephc-magician` as an optional bridge and link it only when a
  program requires the runtime eval fallback.
- [x] Add the ABI, `RuntimeFeatures`, linker bridge, and runtime helpers needed
  to call `__elephc_eval_execute` from the current EIR backend.
- [x] Implement `ElephcEvalContext` and `ElephcEvalScope` shared by native code
  and the interpreter, including flush/reload of observable locals.
- [x] Implement runtime parsing, EvalIR/interpreter, and the value bridge for the
  eval subset supported by magician.
- [x] Support variables, assignments, output, return, control flow, arrays,
  include/require, dynamic calls, declarations, classes/objects, reflection,
  callables, references/by-ref, and error cleanup in the magician fallback for
  the subset covered by tests.
- [x] Model `eval` as an effect barrier for the optimizer/type checker: no DCE,
  no constant propagation through observable locals, and dynamic fallback where
  needed.
- [x] Add repeatable magician benchmarks with Elephc native, Elephc eval, PHP
  native, and PHP eval variants.
- [x] Add parse cache, parse-error cache, and include parse/file cache without
  freezing context, scope, magic constants, or include_once state.
- [x] Add caches for eval symbol lookup, direct builtin dispatch, callable
  resolution, and conservative `RuntimeValueOps` optimizations.
- [x] Add an unboxed scalar fast path, optional linear EvalIR/stack VM, and
  targeted array/reference/COW optimizations in the bridge.
- [x] Implement conservative literal `eval` AOT for scalars, output, return,
  store/scope read-write, and AOT/fallback assembly markers.
- [x] Extend literal eval AOT to internal locals, `while`, `if`, `break`,
  `continue`, comparisons/truthiness, modulo, and the prime-sum benchmark up to
  `100000`.
- [x] Extend literal eval AOT to common static builtins, known static functions,
  typed public static methods, and static callbacks through `call_user_func*()`.
- [x] Avoid linking `elephc_magician` for programs whose literal eval calls are
  fully AOT.
- [x] Update parity tests to distinguish shared builtins, documented eval-only
  builtins, and static-only builtins not yet present in magician.
- [ ] Reduce the remaining manual AOT mini-codegen and converge on internal EIR
  functions for supported literal fragments.
- [ ] Expand AOT only where semantics are covered. Full arrays/iterables,
  object/member access, references/by-ref, `global`, `static`, variable
  variables, `try`/`throw`, include/require, and declarations stay fallback
  until they have a dedicated model and tests.
- [ ] Close or explicitly maintain the static-only builtin gap: implement them
  in magician or keep them in a tested allowlist until eval exposes them.
- [ ] Promote the most useful AOT acceptance benchmarks into the permanent
  benchmark suite without including compile/link time in runtime numbers.
- [ ] Update user/internal docs after every semantic extension of the eval or AOT
  subset.
- [ ] Run focused checks on all three supported targets for every change that
  touches ABI, runtime ownership, eval codegen, or fallback/AOT selection.

## Plan Scope

This plan replaces and merges:

- `.plans/elephc-eval-complete-plan.md`
- `.plans/elephc-eval-aot-complete-plan.md`
- `.plans/elephc-magician-performance-plan.md`

This plan remains in `.plans` to track only the remaining eval/magician work.
All plans in `.plans` must be written in English. Completed sections document
the state already reached and act as guardrails against reintroducing old
approaches or regressions.

## Current State

Eval support has two paths:

1. Runtime fallback through `libelephc-magician`, called by
   `__elephc_eval_execute`.
2. Literal eval AOT, when the fragment is a compile-time-known string and the
   classifier considers it semantically safe.

After the rebase onto `main`, the active backend is the EIR path under
`src/ir_lower/`, `src/ir_passes/`, and `src/codegen/lower_inst/`. Historical
references to `src/codegen_ir/` in older plans are obsolete.

Current central files:

- `crates/elephc-magician/src/`
- `src/eval_aot.rs`
- `src/ir_lower/expr/mod.rs`
- `src/ir_lower/program.rs`
- `src/codegen/lower_inst/builtins/eval.rs`
- `src/codegen_support/runtime/eval_bridge.rs`
- `src/codegen_support/runtime_features.rs`
- `tests/codegen/eval.rs`
- `tests/codegen/eval_callables.rs`
- `tests/codegen/eval_callable_ref_errors.rs`
- `tests/codegen/eval_constructors.rs`
- `tests/codegen/eval_closures.rs`
- `tests/codegen/eval_reflection_invocation.rs`
- `tests/builtin_parity_tests.rs`

## Consolidated Architecture

### Magician Fallback

`elephc-magician` is an optional bridge staticlib. Programs without runtime eval
must not link it. The fallback remains mandatory for:

- dynamic eval;
- literal eval that cannot be parsed or is not supported by the AOT classifier;
- constructs whose runtime semantics are not yet modeled in AOT;
- dynamic declarations, include/require, references/by-ref, global/static,
  variable variables, dynamic objects/members, and throwables until covered.

The fallback receives:

- global eval context;
- local eval scope;
- global scope when needed;
- code pointer/length;
- result buffer.

The value model must not diverge from native runtime behavior. Boxing, refcount,
COW, references, and cleanup must stay consistent with the elephc runtime.

### Scope Sync

Native code must synchronize with eval scope only for values observable by the
fragment:

- before the call: flush variables read or written when needed;
- during eval: magician operates on the shared scope;
- after eval: reload variables that may have been written, created, or unset.

When analysis is imprecise, semantics wins over performance: use the fallback or
treat the fragment as a stronger barrier.

### Literal Eval AOT

The compiler analyzes literal fragments at compile time:

```text
literal string
  -> parse as PHP fragment
  -> normalize/name-resolve compatibly with the context
  -> classify AOT eligibility
  -> plan reads/writes/calls/fallback
  -> native lowering or magician fallback
```

The AOT plan must preserve:

- `return expr;` returns from eval, not from the caller;
- fallthrough without `return` produces `null`;
- output remains a visible side effect;
- caller variables known at compile time can be read and written;
- variables created by the fragment are visible after eval if that AOT path
  declares creation support;
- every uncovered construct remains an explicit fallback.

AOT paths emit assembly markers such as `eval literal AOT compiled...`.
Fallback paths emit markers with a readable reason where possible.

## Completed Work

### Eval Runtime and Bridge

Completed:

- `elephc-magician` crate;
- C/Rust ABI for `__elephc_eval_execute`;
- `elephc_magician` linker bridge;
- runtime feature detection;
- eval language construct in checking/lowering;
- materialized scope, context, and value bridge;
- observable-local flush/reload;
- error/status mapping and cleanup.

Codegen and interpreter coverage includes eval at top level, in functions, and
in methods, shared scope, nested eval, return/output, created variables, local
mutation, callables, constructors, closures, and reflection.

### Magician Interpreter

Completed for the current subset:

- runtime lexer/parser for eval fragments without `<?php` tags;
- EvalIR/interpreter;
- basic expressions/statements;
- control flow;
- arrays and COW on supported paths;
- include/require;
- dynamic functions/classes and runtime metadata;
- interpreter-side builtin registry/dispatch;
- callable forms and `Closure::fromCallable`;
- classes, interfaces, traits, enums, static members, and reflection in the
  covered subset;
- throw/fatal/status handling where supported.

### Magician Performance

Completed:

- `scripts/benchmark_magician.py` benchmark suite with fixtures under
  `benchmarks/magician/cases/`;
- parse cache and parse-error cache;
- include cache with metadata validation;
- lookup cache for eval/native symbols;
- direct builtin dispatch for hot paths;
- conservative callable resolution cache;
- fewer `RuntimeValueOps` calls for output/simple scalars;
- temporary int/bool evaluator for assignment/return/condition;
- optional linear EvalIR for straight-line fragments;
- narrow fast paths for indexed-array writes.

### Literal Eval AOT

Completed:

- `EvalLiteralCall` preserves the literal payload in EIR;
- `src/eval_aot.rs` classifies eligibility and fallback reasons;
- `src/codegen/lower_inst/builtins/eval.rs` tries AOT before the bridge;
- support for scalars, arithmetic, concat/output, print, return, stores,
  read/write scope, and boxed Mixed scope paths;
- support for internal locals, assignments/compound assignments,
  while/if/break/continue, modulo, comparisons, and truthiness sufficient for
  the prime benchmark;
- support for common static builtins;
- support for known static functions;
- support for typed public static methods;
- support for static callbacks in `call_user_func()` and
  `call_user_func_array()`, including string, array, `Class::class`, and
  immediate first-class static forms;
- tests proving no `__elephc_eval_execute` call and no `elephc_magician` link
  for fully AOT fragments;
- prime-sum benchmark up to `100000` without the bridge, output `454396537`.

## Open Work

### 1. Converge AOT on Internal EIR Functions

The main debt is reducing the manual mini-codegen in
`src/codegen/lower_inst/builtins/eval.rs`.

Direction:

- represent each AOT fragment as an internal EIR function with a special ABI;
- declare fragment locals separately from caller locals;
- introduce EIR primitives or helper builtins for:
  - `eval_scope_get`;
  - `eval_scope_set`;
  - return/fallthrough `null`;
  - status/fatal propagation;
- send the AOT function through validation, optimization, register allocation,
  and the target-aware backend;
- keep magician fallback as the compatibility path.

Done criteria:

- no further growth of the manual mini-backend for new constructs;
- existing AOT tests continue to pass;
- the assembly marker remains explicit;
- no regression on macOS ARM64, Linux ARM64, or Linux x86_64.

### 2. Extend AOT Beyond the Current Static Subset

Every new construct must be introduced only with a semantic model and tests.
Reasonable priority:

1. arrays/iterables in AOT once COW and ownership are clear;
2. statically resolvable object/member access;
3. references/by-ref only if the ref-cell model is identical to runtime;
4. `global`, `static`, and variable variables;
5. `try`/`throw`;
6. include/require;
7. declarations inside eval.

Everything not modeled stays fallback.

### 3. Compiler/Eval Builtin Parity

`tests/builtin_parity_tests.rs` distinguishes:

- shared compiler/eval builtins;
- documented eval-only builtins;
- static-only builtins registered in the compiler but not yet exposed by
  magician.

When a static-only builtin is implemented in magician:

- remove it from the static-only allowlist;
- add eval signature metadata;
- add interpreter dispatch;
- add named/positional tests when relevant;
- update benchmarks only if the builtin enters an eval hot path.

### 4. Benchmarks and Measurement

The benchmark suite exists. Remaining work:

- decide which AOT benchmarks should become permanent;
- always exclude compile/assemble/link time from runtime numbers;
- keep at least one prime-loop case and one algebra-heavy case as a manual
  regression or CI artifact;
- preserve output correctness against PHP where practical.

### 5. Documentation

Update docs when the subset changes:

- eval enables an optional dynamic runtime;
- literal eval AOT does not embed the parser/compiler in the binary;
- magician fallback remains compatibility semantics;
- fully AOT programs do not link `elephc_magician`;
- constructs that still fall back should be documented when user-visible.

## Tests and Checks

For narrow AOT planner/lowering changes:

```bash
cargo check
cargo test --test codegen_tests literal_eval_static
cargo test --test codegen_tests test_literal_eval_prime_loop_uses_aot_without_execute_bridge
git diff --check
```

For runtime bridge or interpreter changes:

```bash
cargo check
cargo test -p elephc-magician <filter>
cargo test --test codegen_tests eval_<filter>
git diff --check
```

For ABI/codegen/runtime ownership changes:

```bash
cargo check
cargo test --test codegen_tests <focused_eval_filter>
./scripts/test-linux-x86_64.sh <focused_eval_filter>
./scripts/test-linux-arm64.sh <focused_eval_filter>
git diff --check
```

For manual benchmarks:

```bash
python3 scripts/benchmark_magician.py --case algebra_heavy --iterations 5 --warmup 1
python3 scripts/benchmark_magician.py --case literal_scalar_aot --iterations 5 --warmup 1
```

## Risks

- Incomplete scope sync can create stale variables or miss creations/unsets.
- Duplicating manual AOT codegen creates a second backend that is hard to
  maintain.
- Treating eval as ordinary static code can break PHP eval semantics.
- References, COW, arrays, and object properties can introduce double-free,
  leaks, or missed mutations if they bypass runtime helpers.
- `eval('$x + 1;')` returns `null`, not the last expression.
- Over-aggressive fallback selection can miscompile dynamic code.
- Magician optimizations must not freeze context/scope/magic constants.
- Every new path must stay target-aware on macOS ARM64, Linux ARM64, and Linux
  x86_64.

## Final Completion Criteria

The eval/magician work can be considered closed when:

1. magician fallback covers the declared PHP subset with tests;
2. every supported literal eval uses AOT or an explicit fallback reason;
3. the AOT subset does not depend on an unmaintainable manual mini-backend;
4. fully AOT programs do not link `elephc_magician`;
5. static/eval builtin parity has no stale allowlist entries;
6. prime-loop and algebra-heavy benchmarks remain correct and measurable;
7. all three supported targets have focused coverage for every ABI/codegen
   change;
8. docs and tests exactly reflect the supported subset and fallbacks.
