# elephc IR — Overview and Vision

**Document version:** 2026-05-12
**Author:** Architecture proposal for elephc
**Target series:** v0.24.x (introduction), v0.25.x+ (optimization passes)

---

## Goal

Introduce a **domain-specific intermediate representation** (called **EIR — elephc IR**) between the AST-level optimizer and the assembly emitter, so that:

1. A real **register allocator** can see the entire function and avoid the per-expression spill/reload pattern that currently caps throughput.
2. **Instruction scheduling**, **CSE**, **LICM**, **peephole over wider windows**, and **inlining** become possible without retrofitting them onto an AST walker that has no notion of basic blocks or value identity.
3. The codegen pipeline gains a **clean boundary** between *semantic* lowering (AST → IR, preserves PHP semantics) and *physical* lowering (IR → ASM, preserves performance).
4. The educational character of the project is preserved: ASM is still hand-emitted, every instruction still commented at column 81, every emitter file still readable line-by-line.

## Non-goals

- **Do not** replace the hand-written ASM backend with a third-party crate (Cranelift, LLVM, etc.). EIR is ours.
- **Do not** redesign `PhpType`, the type checker, the parser, or the runtime. EIR consumes the existing semantic model.
- **Do not** rewrite the AST-level optimizer (`src/optimize/`) up front. AST-level folding, propagation, and DCE remain; IR-level optimizations are added on top, not in place.
- **Do not** ship register allocation in the first PR. The first deliverable is a 1:1 lowering with **zero behavior change**.
- **Do not** introduce a generic IR (SSA-CFG with abstract value semantics like LLVM/CLIF). EIR is **PHP-specific**: it has `MixedBox`, `ArrayCowEnsureUnique`, `Fatal`, ownership state — operations LLVM and Cranelift would never have.

## Why a custom IR over Cranelift

Decided already in the design discussion preceding these plans. Recap:

- **Identity**: the project's value proposition is the educational, fully hand-rolled toolchain. Cranelift dissolves that.
- **PHP semantics**: Mixed boxing, COW, ownership lattice, exact eval order, fatal vs throw, `__rt_*` runtime calls — all hostile to a generic optimizing IR. They are first-class in EIR.
- **Migration cost**: Cranelift migration is 6–9 months of refactor work with no visible features. EIR Phase 1 alone is 4–6 weeks and unlocks subsequent optimization phases.
- **Reversibility**: if we ever want Cranelift, EIR → CLIF is a much smaller hop than AST → CLIF.

## Architecture

```
PHP source
  → Lexer
  → Parser
  → Magic constants
  → Conditional compilation
  → Resolver
  → NameResolver
  → Constant folding (AST)
  → Type checker / warnings
  → Optimizer passes (AST)         ◄── unchanged
  → AST → EIR lowering             ◄── NEW
  → EIR passes:
      • validation
      • effect annotation finalization
      • (later) peephole / CSE / LICM / register allocation
  → EIR → ASM emission              ◄── replaces direct AST → ASM
  → assembler / linker
  → binary
```

Two new modules will be introduced under `src/`:

- `src/ir/` — EIR types, builder, validator, printer, passes
- `src/codegen_ir/` — the new IR-consuming backend (renamed once stable; see Phase 5)

The current `src/codegen/` keeps emitting assembly during the migration. A feature flag selects which pipeline runs. When EIR reaches parity, the legacy path is removed.

## Tech stack

- Rust (existing toolchain, no new dependencies)
- No external crates required for Phases 1–6
- Insta or hand-rolled snapshot tests for IR pretty-printer (decide in Phase 02; prefer no new dep)
- Existing `as` + `ld` test infrastructure remains the final correctness gate

## Phases and deliverables

Each phase is **independently shippable**. After every phase the test suite must pass with `cargo test -- --include-ignored` and no regression in the benchmark harness.

| Phase | Plan | Deliverable | Visible to users? |
|-------|------|-------------|-------------------|
| 1 | [01](eir-01-design-spec.md) | EIR design specification document | No |
| 2 | [02](eir-02-ir-module-skeleton.md) | `src/ir/` module: types, instructions, builder, validator, printer | No |
| 3 | [03](eir-03-ast-to-ir-lowering.md) | AST → EIR lowering, no optimizations, full test parity through `--emit-ir` | Yes (`--emit-ir` flag) |
| 4 | [04](eir-04-ir-to-asm-backend.md) | EIR → ASM backend producing equivalent assembly to current codegen | No (parity check only) |
| 5 | [05](eir-05-switchover-behind-flag.md) | `--ir-backend` flag, then default switchover, then legacy codegen removed | Yes (perf neutral) |
| 6 | [06](eir-06-linear-scan-register-allocator.md) | Linear-scan register allocator, first real perf gain | Yes (~15–25% perf on compute) |
| 7 | [07](eir-07-peephole-and-local-opts.md) | IR-level peephole, dead store elimination, identity ops | Yes (~5–10% more) |
| 8 | [08](eir-08-cse-licm-inlining.md) | CSE, LICM, inlining of small functions | Yes (~10–20% more on loops) |
| 9 | [09](eir-09-legacy-cleanup.md) | Remove legacy path, consolidate docs, finalize internals chapter | No (cleanup) |

## Definition of done (per phase)

- All existing tests pass (`cargo test`, `cargo test -- --include-ignored`)
- New tests cover the phase's added surface
- Benchmark harness shows no regression (or shows the expected gain for Phases 6–8)
- `cargo build` clean, zero warnings
- Linux x86_64 and Linux ARM64 verified via `scripts/test-linux-*.sh`
- `docs/internals/` updated where the change is user-visible internally (e.g., `the-codegen.md`, new `the-ir.md`)
- Commit history follows project conventions (`feat:`, `refactor:`, no `Co-Authored-By`)

## Risks and mitigations

| Risk | Likelihood | Mitigation |
|------|-----------|------------|
| EIR design too abstract / not PHP-shaped | High if rushed | Phase 01 must spec each instruction by walking 10+ real codegen sites |
| Phase 4 fails to produce byte-identical assembly | High | Phases 4 and 5 do *not* require byte-identical assembly, only semantically equivalent (same `compile_and_run` output). Snapshot tests on stdout, not on `.s` |
| Phase 3 PR becomes a 20k-line monster | Very high without discipline | Lower AST → IR per AST family in separate sub-tasks within Phase 3 (literals, locals, arithmetic, calls, control flow, classes, etc.) |
| Register allocator interacts badly with ABI helpers | Medium | Phase 6 starts with caller-saved-only scratch allocation, then expands. ABI helpers in `src/codegen/abi/` are reused, not reinvented |
| Ownership/refcount semantics drift during IR lowering | High | Encode ownership in EIR as explicit ops (`Acquire`/`Release`/`Move`/`Borrow`). Validator rejects unbalanced cleanup paths |
| Performance gain is smaller than predicted | Medium | Benchmark before/after every optimization phase. If Phase 6 alone delivers <10% on compute benchmarks, stop and diagnose before continuing to Phase 7 |
| Educational value lost to abstraction | Low if disciplined | EIR is *added* between AST and ASM; the ASM emitter retains every comment. New `docs/internals/the-ir.md` teaches IR. The pipeline story actually gets better |

## Out of scope

- Generic IR features (linear scan SSA destruction with phi nodes — we use block parameters instead)
- Cross-module optimization (we don't have separate compilation yet)
- Profile-guided optimization
- SIMD vectorization (later, post-1.0 if at all)
- A textual IR parser for round-tripping `.eir` files (printer is for tests/debug only, not bidirectional)
- WebAssembly backend (post-1.0 product track, see ROADMAP v1.2.x)

## Relationship to existing optimizer

`src/optimize/` operates on the AST and stays there. AST-level folding and propagation are good at:

- Constant folding of pure arithmetic on literal subtrees
- Dead branch elimination in `if (false)`
- Reachability/control-flow normalization
- Alias-aware scalar propagation across statement boundaries

These are kept. EIR-level optimizations target what AST-level can't see:

- **Liveness** of values across an entire function
- **Value identity** for CSE (two `array_get %a, 0` calls produce the same SSA value; AST nodes are structural but not value-identified)
- **Basic-block dominance** for LICM
- **Register placement** for elimination of redundant moves and spills
- **Instruction scheduling** for pipelining

## Relationship to ROADMAP

- The external plan series was written against an older roadmap shape where register allocation, peephole optimization, inlining, tail-call optimization, and deeper DCE/propagation were standalone performance bullets.
- This repository's `ROADMAP.md` has already been reconciled: **v0.24.x** covers EIR introduction and register allocation, **v0.25.x** covers EIR optimization passes, and **v0.26.x** covers performance closure, legacy cleanup, and 0.x stabilization.
- Treat `ROADMAP.md` as the source of truth for release placement; treat these `.plans/eir-*` files as the execution detail behind those roadmap bullets.

## Open design questions (decided here)

These are decided in this proposal, not deferred:

- **SSA form**: SSA-lite with block parameters. No phi nodes. Block params are easier to construct and lower than phi-based SSA, at no real cost for our scale.
- **Value naming**: `ValueId` is a u32 index into a per-function value table. Values are SSA: defined exactly once.
- **CFG representation**: Functions own a `Vec<BasicBlock>`, each block owns a `Vec<Instruction>` and one terminator. Blocks reference each other by `BlockId(u32)`.
- **Types in IR**: minimal — `I64`, `F64`, `Str`, `Heap`, `Void`. Heap subkind (`Array`/`Hash`/`Object`/`Mixed`/`Iterable`/`Union`) is carried as metadata on operations, not in the IR type itself, because the runtime handles them uniformly via heap headers.
- **Ownership**: tracked in EIR as explicit ops (`Acquire`, `Release`, `Move`, `Borrow`). The validator checks for balance along all paths.
- **PHP-specific ops**: yes, first-class. `MixedBox`, `ArrayCowEnsureUnique`, `Fatal(msg_idx)`, `RuntimeCall`, `BuiltinCall` are distinct from generic `Call`.
- **Effects**: each instruction carries effect bits (Pure, ReadsHeap, ReadsGlobal, ReadsFs, WritesHeap, WritesGlobal, WritesFs, MayThrow, MayFatal, MayDeoptimize). Set at builder time, refined by validator.

## What success looks like

After Phase 9:

- The compiler still ships PHP-correct programs.
- Benchmarks show **40–70% performance improvement** on compute-heavy workloads (function calls, tight loops, arithmetic-heavy programs) compared to pre-EIR baseline.
- `docs/internals/` has a new chapter (`the-ir.md`) explaining EIR with the same pedagogical care as `the-codegen.md`.
- The codebase passes all gates: tests, ignored tests, Docker Linux tests, benchmark harness, zero compiler warnings.
- A future migration to Cranelift, if ever desired, is a 6–8 week project instead of 6–9 months — because EIR is the hard part.
