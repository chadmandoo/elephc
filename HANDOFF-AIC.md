# AIC → elephc native-compilation — session handoff

**Date:** 2026-07-01 · **elephc:** v0.25.2 · **Fork:** `github.com/chadmandoo/elephc`
(upstream `origin` = `illegalstudio/elephc`). All work below is on this fork; nothing
was pushed upstream and no PRs were opened (deferred by owner).

## Thesis

Compile AIC (a strict-types, coroutine-native pure-PHP CMS-slim framework) down to a
native binary via **elephc** — a from-scratch Rust PHP→native-assembly compiler (its
own EIR IR, **no LLVM**, no Swoole/extensions; compiles a static SUBSET of PHP). The
bet: AIC's discipline (no `mixed`, reflection-free container, build-time-compiled
discovery in `bootstrap/cache`, one-class-per-file PSR-4) is exactly what makes it
uniquely AOT-compilable — the same discipline that makes it AI-legible. elephc is the
"functional core" (AOT-native); Swoole/FrankenPHP stays the "imperative shell"
(interpreted). Parity oracle throughout: compile → run native → byte-diff vs host PHP 8.5.

## Branches on the fork (all off `main`, each a clean standalone upstream-PR candidate)

| Branch | Gap fixed | Head commit |
|--------|-----------|-------------|
| `feat/declare-strict-types` | P1: `declare(strict_types=1)` / declare directives rejected | `88e78ca` |
| `fix/enum-as-property-type` | P2: enum as property / promoted-param type → "Unknown type" | `18e9495` |
| `fix/variadic-self-param-type` | `self`/`static`/`parent` in variadic param types | `7ec6be8` |
| `fix/htmlspecialchars-flags` | P3: `htmlspecialchars`/`htmlentities` flags + `ENT_HTML5` + encoding | `0658c67` |
| `fix/enum-case-keyword-names` | P5: reserved keyword as enum-case name (`case Default`) | `3b6f150` |
| `fix/enum-case-default-value` | P6: enum-case value as param default (`E $x = E::Case`) typed as Str | `cf9805c` |
| `integration/all-fixes` | octopus-merge of all 6 (build the binary from here) | (merge) |
| `aic-handoff` | this doc + `aic-bundler/` (off integration/all-fixes) | — |

Each fix carries byte-parity + regression tests (codegen/error suites). The full
codegen/error/parser suites regress clean — the 140 remaining codegen failures are
**pre-existing environmental** (offline HTTP/FTP fetches, timezone DB, external libs),
proven identical on the pre-fix baseline.

## Fix locations (for resuming / upstreaming)

- **P1** `src/lexer/token.rs` (+`Declare` token), `src/lexer/literals/identifiers.rs`, `src/parser/stmt/mod.rs` (dispatch + recovery), `src/parser/stmt/namespace_use.rs` (`parse_declare` → `StmtKind::Synthetic(body)`).
- **P2** `src/types/checker/driver/mod.rs` — enum-name pre-declaration loop, inserted AFTER the *second* `checker.declared_classes = class_map.keys()...` assignment (the first is clobbered).
- **variadic** `src/types/checker/driver/mod.rs` — in `substitute_relative_class_types_in_methods`, also substitute `method.variadic_type`.
- **P3** `src/codegen/prescan.rs` + `src/types/checker/driver/init.rs` (ENT_* constants), `src/name_resolver/names.rs` (global-const fallback), `src/types/signatures.rs` (1–3 args), `src/types/checker/builtins/strings.rs`, `src/codegen_ir/lower_inst/builtins/strings.rs` + `builtins.rs` (dispatch), `src/codegen/runtime/strings/htmlspecialchars.rs` (flag-aware escaper, ARM64+x86_64; x86_64 verified locally, ARM by CI).
- **P5** `src/parser/stmt/oop/body.rs` — enum-case name parser now uses `keyword_name::bareword_name_from_token` (mirrors class-const-name handling), rejects `class`. NOTE: `case Match` stays unsupported (`::Match` canonicalizes to `"MATCH"` for a builtin SPL regex constant — pre-existing, rare).
- **P6** `src/types/checker/type_compat/declarations.rs` — `validate_declared_default_type`: when the declared param type is `Object(E)` and the default is a `ScopedConstantAccess` naming that same `E`, accept it (ordering-independent name comparison; the later semantic pass validates the case exists). Root cause: syntactic inference types every `::` access as `Str`.

## Build & verify

```bash
git checkout integration/all-fixes && cargo build --release   # ~53s → target/release/elephc
# compile+run any PHP subset file → native binary beside it:
target/release/elephc path/to/main.php && ./path/to/main
target/release/elephc --check path/to/main.php                # front-end only (fast gap probe)
```

## The bundler + compilability analyzer (`aic-bundler/`)

- `bundle.php` — given AIC FQCN root(s), walks the transitive closure via AIC's Composer classmap (READ-ONLY), excludes vendor polyfills (`Stringable` etc. are elephc builtins), emits a topologically-ordered `require_once` `_bundle.php` + `_manifest.json`. No source rewrites needed (P1–P6 live in the compiler).
- `scan.sh` — bundles + `--check`s each root in a namespace; prints a per-class OK/GAP compilability map.
- `REPORT.md` — the P4 writeup.

Usage (paths were `/srv/stacks/aic` + this fork's `target/release/elephc`):
```bash
php aic-bundler/bundle.php --classmap=/srv/stacks/aic/vendor/composer/autoload_classmap.php \
    --out=/tmp/out --root='AIC\Components\Domain\HeadAsset'
./aic-bundler/scan.sh 'AIC\Components\Domain\\'
```

## Results

- **Compilability scan of ward-components `Domain` (20 pure VO/enum files): 19/20 compile clean** to the native front-end with the 6 fixes.
- **Capstone (compile + RUN):** bundler-resolved `HeadAsset` + `HeadAssetType` + `HeadAssetMode` → native binary → **byte-identical to PHP 8.5** (`css:default:/build/app.css` …). Exercises P5 (`HeadAssetMode::Default` keyword case) + P6 (the `$mode = HeadAssetMode::Default` default) on real AIC source.

## Remaining gap → next step

- **P7: static interface methods** — `Previewable::previews()` is `public static function` on an interface; elephc: "Static interface methods are not supported yet." The lone Domain holdout. Bigger feature than P1–P6. Landing it → the whole Domain layer compiles.
- Then extend the analyzer past `Domain` (Application/Infrastructure pull the Swoole/coroutine shell — the functional-core / imperative-shell boundary).
- P3 upstream follow-ups: `ENT_SUBSTITUTE`/`ENT_IGNORE`/encoding handling, an `examples/` entry.

## Relationship to the other track

Owner sequencing: **FrankenPHP runtime swap FIRST** (Forge epic **#475**, children #476–480),
**then** resume this compilation track. The two are complementary: FrankenPHP makes the
*interpreted shell* deployable as a single binary; elephc makes the *functional core*
native. See AIC memory `frankenphp_and_elephc_tracks.md`.

## To resume in a fresh session

```bash
git clone git@github.com:chadmandoo/elephc.git && cd elephc
git checkout aic-handoff        # this doc + aic-bundler/
git checkout integration/all-fixes && cargo build --release
./aic-bundler/scan.sh 'AIC\Components\Domain\\'   # confirm 19/20 still holds
```
Start P7 from `integration/all-fixes`; add the fix as a new `fix/static-interface-methods`
branch off `main`, then merge into `integration/all-fixes` (mind the co-located
`tests/codegen/types/enums.rs` merge — resolve by keeping all tests).
