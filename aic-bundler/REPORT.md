# P4 — AIC → native bundler + compilability analyzer

**Goal:** given a real AIC entrypoint, auto-resolve its transitive dependency
closure from the Composer classmap, emit a `require_once` bundle, compile it to
a native binary with the all-fixes elephc, and byte-diff the result against host
PHP 8.5. Surface every gap the compiler hits on real AIC code.

Everything here lives in the scratchpad + the `chadmandoo/elephc` fork. **Zero
changes to `/srv/stacks/aic`** (read-only classmap + source).

## The bundler (`bundle.php`)

- Reads AIC's `vendor/composer/autoload_classmap.php` (FQCN → file), read-only.
- Greedy tokenizer closure walk: for every name-token in a file, resolve it two
  ways — as fully-qualified and as same-namespace — keep classmap hits. In
  strict one-class-per-file PSR-4 code this captures every dep (use lines,
  qualified refs, bare same-namespace refs, trait-use, attributes) with no full
  name resolver. Skips member/case/method-name positions so the external list is
  a trustworthy gap surface.
- **Excludes vendor polyfills** (`symfony/polyfill`, and PHP built-in interfaces
  like `Stringable`/`UnitEnum`/`JsonSerializable`): on a native target elephc
  provides these itself; their polyfill files are `PHP_VERSION_ID < N`
  conditionals that reference host-only constants.
- Emits `_bundle.php` (topologically-ordered `require_once`s) + `_manifest.json`.

Because P1–P6 now live *in the compiler*, the bundler needs **no source
rewrites** — it only resolves the closure.

## Compilability scan — ward-components `Domain` (20 pure value objects/enums)

`scan.sh` bundles + `--check`s each root independently.

```
CLASS                      FILES  RES   FIRST ERROR
AssetManifest              1      OK
BuildDiff                  1      OK
BuildResult                1      OK
Component                  2      OK
ComponentDescriptor        2      OK
ComponentTier              1      OK
EmittedAsset               4      OK
HasCustomTemplate          1      OK
HasHeadAssets              1      OK
HeadAsset                  3      OK
HeadAssetMode              1      OK
HeadAssetType              1      OK
Html                       1      OK
IsCacheable                1      OK
PlannedManifest            2      OK
Previewable                1      GAP   Static interface methods are not supported yet: Previewable::previews
PropDescriptor             2      OK
PropParameter              1      OK
RenderResult               2      OK
SourceComponent            1      OK
----
Domain roots: 20   compiles-clean: 19   has-gap: 1
```

**19/20 (85%→95%) of the Domain layer is binary-enabled** through the compiler
front-end with the six fixes applied.

## Capstone — real multi-file AIC slice, compiled + run

Bundler-resolved closure `HeadAsset` + `HeadAssetType` + `HeadAssetMode`,
compiled to native, driver constructs assets and prints `dedupKey()`:

```
css:default:/build/app.css      <- P6: $mode = HeadAssetMode::Default default taken
js:module:/build/app.js         <- P5: reserved-keyword enum case `Default` resolves
js:defer:/build/defer.js
```

**native ≡ PHP 8.5, byte-identical.** Exercises P5 + P6 on genuine AIC source.

## Gap catalog (found by compiling real AIC code)

| # | Gap | Status |
|---|-----|--------|
| P1 | `declare(strict_types=1)` rejected | **FIXED** `feat/declare-strict-types` |
| P2 | enum as property / promoted-param type ("Unknown type") | **FIXED** `fix/enum-as-property-type` |
| — | `self`/`static`/`parent` in variadic param types | **FIXED** `fix/variadic-self-param-type` |
| P3 | `htmlspecialchars` flags / `ENT_HTML5` / encoding args | **FIXED** `fix/htmlspecialchars-flags` |
| P5 | reserved keyword as enum-case name (`case Default`) | **FIXED** `fix/enum-case-keyword-names` |
| P6 | enum-case value as parameter default (`E $x = E::Case`) | **FIXED** `fix/enum-case-default-value` |
| P7 | **static interface methods** (`interface { static fn ...}`) | open — bigger feature |

Each fix is a standalone branch off `main` (clean upstream PR) and merged into
`integration/all-fixes`. Every fix carries byte-parity + regression tests; the
full codegen/error/parser suites regress clean (the 140 pre-existing codegen
failures are environmental — offline HTTP/FTP fetches, timezone DB, external
libs — verified identical on the pre-fix baseline).

### Notes / smaller follow-ups
- `case Match` as an enum-case name stays unsupported (`::Match` canonicalizes to
  `"MATCH"` to back a builtin SPL regex constant) — pre-existing, astronomically
  rare, absent from AIC.
- `PHP_VERSION_ID` is undefined in elephc — only surfaced via the excluded
  polyfill; add to the constant table if ever needed by first-party code.

## Next
- P7 (static interface methods) unlocks `Previewable` → the full Domain layer.
- Extend the analyzer past `Domain` (Application/Infrastructure layers pull the
  Swoole/coroutine shell — the natural functional-core/imperative-shell boundary).
