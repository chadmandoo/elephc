# elephc-magician Performance Plan

| Order | Work item | Objective | Status | Notes |
|---:|---|---|---|---|
| 1 | Dedicated magician benchmark suite | Measure real hot spots before larger interpreter or bridge refactors | Not started | Establishes a stable baseline for parse, dispatch, scalar arithmetic, builtin calls, callable calls, arrays, references, and include paths. |
| 2 | Eval fragment parse cache | Avoid repeated tokenization and parsing for identical runtime source bytes | Done | Implemented in commit `4ba0efb5c` through `crates/elephc-magician/src/parse_cache.rs`. |
| 3 | Parse-error cache | Avoid reparsing repeated invalid fragments | Done | Included in the same parse cache as successful `EvalProgram` results. |
| 4 | Dynamic context/scope preservation for cached parses | Ensure caching does not freeze magic constants, variables, declarations, or runtime state | Done | The cache stores only immutable parse results; execution still receives the current `ElephcEvalContext` and `ElephcEvalScope`. |
| 5 | Include parse cache | Avoid repeated parsing of identical PHP code blocks loaded through include/require | Partially done | The parse cache now covers parsed include code blocks, but file reads, stat/canonicalize work, and include-path resolution are not cached. |
| 6 | Eval symbol lookup cache | Speed up repeated function/class/method lookup for symbols declared by eval | Not started | Should come after parse caching and before deeper interpreter changes. |
| 7 | Direct builtin dispatch | Avoid repeated string matching and generic dispatch for common builtins | Not started | Needs benchmark data to prioritize the first builtin groups. |
| 8 | Callable resolution cache | Avoid reconstructing equivalent string/array/first-class/closure callables repeatedly | Not started | Best implemented after symbol and builtin lookup behavior is stable. |
| 9 | Reduce `RuntimeValueOps` calls on simple operations | Cut bridge overhead for arithmetic, comparisons, casts, and simple output paths | Not started | This starts touching the interpreter core and needs before/after benchmark evidence. |
| 10 | Unboxed scalar fast paths | Avoid boxing/unboxing for hot int/float/bool/string paths inside eval execution | Not started | Likely high impact for compute-heavy loops, but more invasive than dispatch caches. |
| 11 | Compact bytecode or linear EvalIR form | Reduce tree-walk overhead and branch-heavy dispatch in the current EvalIR interpreter | Not started | Should be guided by benchmarks after simpler caches and scalar fast paths are measured. |
| 12 | Array/reference/COW bridge optimizations | Reduce cost of array mutation and by-reference parameter handling | Not started | Semantically delicate; should happen after scalar and dispatch fast paths are stable. |
| 13 | AOT for literal `eval` | Compile `eval('...')` fragments ahead of time instead of interpreting them through magician | Partial | Commit `4bc962407` marks literal eval calls as AOT candidates, but still uses the bridge fallback. |

## 1. Dedicated Magician Benchmark Suite

### Goal

Create repeatable benchmarks that isolate where `elephc-magician` spends time before making larger performance changes. This should prevent optimizing only the obvious parse path while missing the real cost of interpreter dispatch, value boxing, builtin lookup, callable resolution, or array/reference handling.

### Scope

Add a small benchmark harness that compares:

- Native elephc code without eval.
- elephc code that enters magician through `eval`.
- PHP standard execution without eval.
- PHP execution through `eval`.

The suite should include both microbenchmarks and a few mixed workloads:

- Repeated identical small `eval` fragments.
- One large `eval` fragment with a compute-heavy loop.
- Arithmetic-only loops.
- String concatenation and output.
- Array reads/writes.
- Function calls declared inside eval.
- Builtin calls inside eval.
- Callable dispatch inside eval.
- Include/require with repeated code blocks.

### Likely Files

- `benches/` or `scripts/bench-eval-*` depending on existing project conventions.
- `tests/codegen/eval*.rs` only for correctness guards, not timing.
- `crates/elephc-magician/src/` only if benchmark-only hooks are required behind `#[cfg(test)]` or a feature gate.

### Validation

Each benchmark should record:

- Runtime wall clock.
- Number of eval invocations.
- Fragment size.
- Whether the fragment is literal or dynamic.
- Whether parse cache should hit.
- Output correctness against PHP where practical.

### Risks

Benchmark results can be misleading if compile+assemble+link time is included for runtime comparisons. Runtime-only binaries should be generated once and executed repeatedly.

## 2. Eval Fragment Parse Cache

### Goal

Avoid repeated tokenization and parsing for identical eval source bytes.

### Current State

Done in commit `4ba0efb5c`.

The implementation adds `crates/elephc-magician/src/parse_cache.rs` and routes these call sites through it:

- `crates/elephc-magician/src/ffi/execute.rs`
- `crates/elephc-magician/src/interpreter/include_exec.rs`

### Design

The cache is process-local, bounded, and keyed by exact fragment bytes. It stores immutable `EvalProgram` instances behind `Arc`, plus parse errors.

Current policy:

- FIFO capacity: 256 entries.
- Maximum cacheable source: 64 KiB.
- Larger fragments bypass the cache.
- Mutex poisoning is recovered by taking the inner cache.

### Validation Already Run

- `cargo test -p elephc-magician parse_cache`
- `cargo test -p elephc-magician execute_program_nested_eval_uses_same_scope`
- `cargo test -p elephc-magician execute_program_include_uses_call_site_and_returns_file_result`
- `cargo test --test codegen_tests test_eval_return_value`
- `git diff --check`

### Follow-Up

After benchmarks exist, revisit capacity and maximum source size. If workloads show many repeated fragments above 64 KiB, consider size-based memory budgeting instead of a hard source-length cutoff.

## 3. Parse-Error Cache

### Goal

Avoid reparsing invalid fragments that are repeatedly passed to `eval`.

### Current State

Done as part of the parse cache.

### Design

The cached result type is:

```rust
Result<Arc<EvalProgram>, EvalParseError>
```

This means both successful parses and parse errors are reusable.

### Validation Already Run

The unit test `parse_cache::tests::cache_reuses_parse_errors` verifies that parse errors are cached and returned without reparsing.

### Follow-Up

If user code frequently emits many distinct invalid fragments, error caching could retain noise. Benchmark and memory telemetry should decide whether invalid-fragment caching needs a lower capacity or should be disabled for very large invalid inputs.

## 4. Dynamic Context And Scope Preservation

### Goal

Guarantee the parse cache does not alter PHP-observable runtime behavior.

The cache must not freeze:

- Variables from the caller scope.
- Variables created by eval.
- Function/class/interface/trait/enum declarations.
- Magic constants that depend on the current call site.
- Include file metadata.
- Pending throw state.
- Return values.

### Current State

Done for the parse cache.

### Design

The cache stores only the parsed `EvalProgram`. Execution still happens through the existing interpreter entry points and receives the current context and scope every time.

Magic constants remain safe because EvalIR stores magic-constant nodes and runtime evaluation resolves context-dependent values through `ElephcEvalContext`.

### Validation Already Run

- Nested eval scope sharing: `execute_program_nested_eval_uses_same_scope`.
- Include file magic and scope sharing: `execute_program_include_uses_call_site_and_returns_file_result`.
- Native bridge return value: `test_eval_return_value`.

### Follow-Up

Add a focused regression test for the same cached fragment executed under two different call sites, verifying `__FILE__` and `__DIR__` remain context-sensitive.

## 5. Include Parse Cache

### Goal

Avoid reparsing identical PHP code blocks loaded through include/require.

### Current State

Partially done.

The current parse cache is used by `eval_execute_include_code()`, so the parsed PHP code block can be reused if the exact source bytes match.

### Remaining Work

The following costs are not cached yet:

- File read bytes.
- `canonicalize()` for include-once keys.
- Include path resolution.
- Open/stat checks.
- Split scanning for multiple `<?php ... ?>` blocks in one file.

### Likely Files

- `crates/elephc-magician/src/interpreter/include_exec.rs`
- Possibly `crates/elephc-magician/src/context.rs` for include-once metadata if shared caching needs context-level state.

### Implementation Plan

1. Measure include-heavy workloads first.
2. Add a small include-file cache only if file I/O dominates.
3. Key file cache entries by canonical path plus file metadata where available.
4. Keep include_once semantics in `ElephcEvalContext`; do not move "already included" behavior into a global cache.
5. Ensure `__FILE__` and `__DIR__` still come from the current include path, not from cached source metadata.

### Validation

Run focused include tests:

- `execute_program_include_uses_call_site_and_returns_file_result`
- `execute_program_include_once_skips_regularly_included_file`
- `execute_program_missing_include_warns_and_returns_false`
- `execute_program_missing_require_is_runtime_fatal`

### Risks

File caches can easily become stale. A conservative first version should cache parsed source bytes only within one process and avoid hiding file changes unless PHP-compatible behavior is explicitly defined.

## 6. Eval Symbol Lookup Cache

### Goal

Speed up repeated lookup of functions, classes, methods, interfaces, traits, and enums declared dynamically through eval.

### Motivation

Once parsing is cached, repeated dynamic calls may still pay for name normalization, case-insensitive matching, namespace fallback, and symbol-table scans.

### Likely Files

- `crates/elephc-magician/src/context.rs`
- `crates/elephc-magician/src/interpreter/dynamic_functions.rs`
- `crates/elephc-magician/src/interpreter/reflection.rs`
- `crates/elephc-magician/src/interpreter/statements.rs`

### Implementation Plan

1. Inventory current lookup paths for function, class, method, and constant resolution.
2. Identify whether each lookup is already stored in a normalized map.
3. Add cache layers only where repeated lookup still performs normalization or scanning.
4. Invalidate or update caches when eval declares a new symbol.
5. Keep case-insensitive PHP behavior canonical.
6. Preserve namespace fallback for builtins and user symbols.

### Validation

Add tests for:

- Repeated calls to an eval-declared function.
- Case-insensitive lookup.
- Namespaced calls with builtin fallback.
- `function_exists`, `class_exists`, and reflection seeing updated declarations after eval.

### Risks

Incorrect caching here can make declarations invisible or make duplicate declarations appear valid. This must be treated as a semantic change, not a pure optimization.

## 7. Direct Builtin Dispatch

### Goal

Avoid repeated generic builtin lookup and string matching for common builtin calls inside eval.

### Motivation

Eval currently supports many builtins through interpreter dispatch. If a hot loop repeatedly calls the same builtin, the dispatch path should not repeatedly resolve the same function name from scratch.

### Likely Files

- `crates/elephc-magician/src/interpreter/builtins/`
- `crates/elephc-magician/src/interpreter/core_builtins.rs`
- `crates/elephc-magician/src/interpreter/builtin_metadata.rs`
- `crates/elephc-magician/src/interpreter/dynamic_functions.rs`

### Implementation Plan

1. Use benchmarks to rank builtin categories by runtime cost.
2. Add a compact builtin id or function pointer to parsed call expressions when safe.
3. Keep unknown/dynamic call paths generic.
4. Preserve PHP case-insensitivity and namespace fallback.
5. Ensure direct dispatch and generic dispatch share argument validation.

### Validation

For each optimized builtin group, add parity tests for:

- Direct call.
- Case-insensitive call.
- Namespaced fallback.
- Named arguments when supported.
- First-class callable and callable aliases when relevant.

### Risks

A second builtin registry can drift from the canonical catalog. Any direct dispatch table must be generated from or tightly tied to the existing metadata source.

## 8. Callable Resolution Cache

### Goal

Avoid rebuilding equivalent callable targets repeatedly.

### Motivation

Callable semantics now cover string callables, array callables, first-class callable syntax, and closures. Repeatedly resolving the same target can become expensive in callback-heavy eval programs.

### Likely Files

- `crates/elephc-magician/src/interpreter/dynamic_functions.rs`
- `crates/elephc-magician/src/interpreter/reflection.rs`
- `crates/elephc-magician/src/context.rs`
- `crates/elephc-magician/src/ffi/callables.rs`

### Implementation Plan

1. Define a canonical callable key for stable cases.
2. Cache only callables whose target is stable under current context rules.
3. Invalidate or bypass on symbol-table changes when needed.
4. Keep bound object callables separate from static function/class callables.
5. Do not cache by raw runtime-cell pointer unless ownership and lifetime are explicit.

### Validation

Add tests for repeated:

- String function callables.
- Static string callables like `Class::method`.
- Array callables.
- First-class callables.
- `Closure::fromCallable`.
- By-reference callable arguments.

### Risks

Bound method callables can carry `$this`, visibility scope, or closure binding state. Caching must not reuse a callable across an incompatible object or visibility context.

## 9. Reduce `RuntimeValueOps` Calls On Simple Operations

### Goal

Reduce the number of runtime bridge calls needed for simple operations inside eval.

### Motivation

Even after parse and dispatch are cached, simple arithmetic can still be slow if each operator repeatedly boxes, unboxes, allocates, or crosses through generic runtime hooks.

### Likely Files

- `crates/elephc-magician/src/interpreter/expressions.rs`
- `crates/elephc-magician/src/interpreter/constant_eval.rs`
- `crates/elephc-magician/src/interpreter/runtime_ops.rs`
- `crates/elephc-magician/src/runtime_hooks/ops.rs`

### Implementation Plan

1. Count `RuntimeValueOps` calls in arithmetic-heavy eval benchmarks.
2. Identify pure scalar operations where both operands are already scalar cells.
3. Add internal helper paths that perform combined operation + allocation where safe.
4. Avoid changing behavior for arrays, objects, strings with PHP coercion edge cases, refs, or `mixed` values until covered.
5. Keep fatal/error behavior identical to the current generic path.

### Validation

Add focused interpreter tests for:

- Integer arithmetic.
- Float arithmetic.
- String numeric coercion.
- Division/modulo edge cases.
- Boolean comparisons.
- Error/fatal paths.

### Risks

PHP scalar coercion is subtle. Every fast path needs either exact compatibility or an explicit fallback to the current generic path.

## 10. Unboxed Scalar Fast Paths

### Goal

Avoid boxing and unboxing hot scalar values inside eval loops.

### Motivation

Compute-heavy eval programs are likely dominated by repeated scalar loads, arithmetic, comparisons, and stores. Keeping scalars in a compact internal representation can reduce allocation and bridge overhead.

### Likely Files

- `crates/elephc-magician/src/value.rs`
- `crates/elephc-magician/src/interpreter/expressions.rs`
- `crates/elephc-magician/src/interpreter/statements.rs`
- `crates/elephc-magician/src/interpreter/scope_cells.rs`
- `crates/elephc-magician/src/runtime_hooks/`

### Implementation Plan

1. Introduce an interpreter-local value enum for hot temporaries, not persistent scope cells.
2. Keep scope-visible values as runtime cells unless ownership semantics are fully modeled.
3. Add unboxed paths for integer and boolean first.
4. Add float after edge cases are verified against PHP.
5. Add string only for immutable literals or clearly owned strings.
6. Box only when a value escapes to scope, output, by-ref parameters, arrays, objects, or runtime hooks.

### Validation

Create benchmark and correctness coverage for:

- Integer loops.
- Float loops.
- Mixed scalar expressions.
- Assignments back into scope.
- Function calls that force boxing.
- Early return and throwable cleanup.

### Risks

The danger is splitting magician into two incompatible value systems. The unboxed layer should be a temporary execution optimization with explicit boxing boundaries.

## 11. Compact Bytecode Or Linear EvalIR Form

### Goal

Reduce tree-walk and branch-heavy interpreter dispatch overhead.

### Motivation

The current EvalIR is a structured tree. A compact linear representation can improve cache locality and simplify dispatch, especially for loops.

### Likely Files

- `crates/elephc-magician/src/eval_ir.rs`
- New module such as `crates/elephc-magician/src/eval_bytecode.rs`
- `crates/elephc-magician/src/interpreter/`
- `crates/elephc-magician/src/parser/`

### Implementation Plan

1. Do not replace EvalIR immediately.
2. Add an optional lowering step from `EvalProgram` to a compact executable form.
3. Cache the lowered executable form alongside or inside the parse cache.
4. Start with expression-heavy straight-line code.
5. Add loops and control flow after linear basic blocks are proven.
6. Keep declarations and complex OOP constructs on the existing EvalIR path until needed.

### Validation

Run parity tests with both execution engines if a temporary dual path exists:

- Existing magician interpreter unit tests.
- Focused codegen eval tests.
- PHP cross-checks for edge cases.

### Risks

This is a larger architectural change. It should not happen before benchmark data proves tree dispatch is a real bottleneck after parse caching and scalar fast paths.

## 12. Array, Reference, And COW Bridge Optimizations

### Goal

Reduce overhead for array mutation, by-reference parameters, and copy-on-write behavior.

### Motivation

Array and reference-heavy eval code can be expensive because correctness requires preserving PHP aliasing, reference cells, and COW rules.

### Likely Files

- `crates/elephc-magician/src/interpreter/array_literals.rs`
- `crates/elephc-magician/src/interpreter/scope_cells.rs`
- `crates/elephc-magician/src/interpreter/statements.rs`
- `crates/elephc-magician/src/runtime_hooks/`
- `crates/elephc-magician/src/ffi/`

### Implementation Plan

1. Benchmark array mutation and by-ref workloads separately.
2. Identify repeated helper patterns that can be fused safely.
3. Optimize append/set/get paths before broad COW changes.
4. Keep reference binding and mutation behavior covered by regression tests.
5. Add cleanup tests for normal return, fatal, and throwable paths.

### Validation

Add tests for:

- Array append and indexed set.
- Associative key set.
- By-ref function and method parameters.
- Ref-like builtin parameters.
- Aliasing across eval and native code.
- Cleanup after fatal and uncaught throwable.

### Risks

This area has high semantic risk. Incorrect optimization can create stale aliases, missed mutations, double frees, or leaks.

## 13. AOT For Literal `eval`

### Goal

Compile literal eval fragments ahead of time so `eval('...')` can bypass the magician interpreter where possible.

### Current State

Partial.

Commit `4bc962407` marks literal eval calls as AOT candidates through the EIR opcode `EvalLiteralCall`, but the backend still emits the bridge fallback to `__elephc_eval_execute`.

### Motivation

This is the largest potential speedup for literal eval because the code can become normal native elephc code instead of runtime-parsed and interpreted code.

### Likely Files

- `src/ir/`
- `src/ir_lower/expr/`
- `src/codegen_ir/lower_inst/`
- `src/types/`
- `src/name_resolver/`
- `src/resolver/`
- `src/optimize/`
- `crates/elephc-magician/` only for fallback and compatibility.

### Implementation Plan

1. Keep the existing fallback bridge as the compatibility path.
2. Parse literal fragments at compile time.
3. Run the same frontend passes needed for normal PHP code where possible.
4. Reject or fall back for constructs that require runtime-only context.
5. Lower eligible literal eval fragments into EIR.
6. Materialize eval-visible scope reads/writes through the same dynamic-scope bridge used by runtime eval.
7. Preserve eval return semantics: `return` exits eval, not the caller function.
8. Preserve declaration side effects for functions/classes declared by eval.
9. Add diagnostics or assembly markers showing whether a literal eval used AOT or fallback.
10. Expand eligibility gradually.

### Validation

Add parity tests for:

- Literal eval assigning existing variables.
- Literal eval creating variables.
- Literal eval returning values.
- Literal eval with output.
- Literal eval declarations.
- Literal eval using builtins.
- Literal eval inside functions and methods.
- Fallback when unsupported syntax is present.
- No magician link for programs without eval.

### Risks

AOT eval crosses the static/dynamic boundary. The main risk is accidentally treating eval code as ordinary static code and losing PHP eval semantics for scope, declarations, magic constants, or returns.

## Recommended Milestone Order

1. Keep the completed parse cache as the first performance improvement.
2. Add benchmark coverage before changing interpreter internals.
3. Implement symbol lookup, builtin dispatch, and callable caches while they can still be verified as mostly semantic-preserving optimizations.
4. Use benchmark data to decide between reducing `RuntimeValueOps`, unboxed scalars, or compact bytecode first.
5. Treat array/reference/COW improvements as a separate correctness-heavy milestone.
6. Continue AOT literal eval as the strategic long-term path, using the bridge fallback for all unsupported cases.
