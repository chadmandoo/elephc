# AIC native-compilation — functional-core / imperative-shell boundary (EC-6, AIC epic #483)

## Decision

elephc compiles AIC's **functional core** to native; the **imperative shell** stays
interpreted (Swoole / FrankenPHP). A class belongs to the SHELL — and is therefore
**excluded from the "core compilability" denominator** — when it depends on a runtime
capability that is inherently non-AOT-static or binds an external engine:

- **Generators / `yield from`** (coroutine-like lazy state machines) — e.g.
  `AIC\Http\Domain\GeneratorStream`. Streaming is a shell concern.
- **`SimpleXMLElement` / libxml** (external C library) — e.g.
  `AIC\GateScripts\Domain\IntegerXmlAttribute`. XML parsing binds an external engine.

These are NOT counted against core compilability — they are *deliberately* interpreted.
If a native path is ever required for one, it becomes its own **elephc-runtime** ticket
(native generators / a libxml binding), not a core-compilability gap.

## Rationale

The elephc thesis (`HANDOFF-AIC.md`, AIC ADR 0056) is *functional-core-native,
imperative-shell-interpreted*. A Domain class that streams via generators or parses XML
is, by that definition, shell-adjacent — its residence in a `Domain/` namespace is
incidental. Excluding it keeps "core compilability %" a truthful measure of what the AOT
path covers, rather than penalizing it for features that were never meant to compile.

## Encoding

`aic-bundler/survey.sh` filters these shell classes out of the core scan (see the
`str_contains` shell-exclusion terms in its root selection). Extend that list as the
boundary evolves.

## Status (2026-07-05, current AIC)

Core = `Domain` + `Contracts`. Measured: Contracts 91%; Domain 58% raw (60%+ once the two
shell classes above are excluded). Every *other* remaining Domain gap is an **elephc
compiler feature**, not a shell reclassification:

| Gap | Ticket | State |
|-----|--------|-------|
| Typed class constants | EC-1 (#484) | ✅ done, merged |
| `mb_strlen` builtin | EC-2 (#485) | ✅ done, merged |
| `get_debug_type` / `mb_ereg_match` builtins | EC-2 (#485) | remaining (mixed-object class-name runtime / multibyte regex) |
| Type-checker completeness (string-keyed arrays, comparison operands, type resolution) | EC-3 (#486) | remaining (checker + codegen) |
| Static interface methods | EC-4 (#487) | remaining (needs `InterfaceInfo.static_methods` + conformance/dispatch) |
| Native closures (Closure-typed values) | EC-5 (#488) | remaining (env-capture codegen) |
| Generators + SimpleXML | EC-6 (#489) | ✅ reclassified as shell (this doc) |
