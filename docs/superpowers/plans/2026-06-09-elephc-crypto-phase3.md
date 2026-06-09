# elephc-crypto Phase 3 (hash_hmac / hash_file / hash_equals / hash_algos) Plan

> **For agentic workers:** REQUIRED SUB-SKILL: superpowers:subagent-driven-development. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Add the remaining one-shot/utility hash builtins on top of the Phase 2 crypto machinery: `hash_hmac`, `hash_file`, `hash_equals`, `hash_algos`. No incremental HashContext (Phase 4); no phar/fork removal (Phase 5).

**Tech Stack:** Rust, target-aware ARM64 + x86_64 assembly.

**Spec:** `docs/superpowers/specs/2026-06-09-elephc-crypto-design.md` (Component 3; Phase 3). Phases 1-2 done on branch `feat/elephc-crypto`.

---

## Established machinery to reuse (from Phases 1-2)

- Crate ABI: `elephc_crypto_hash(name,name_len,data,data_len,out)->isize` and `elephc_crypto_hmac(name,name_len,key,key_len,data,data_len,out)->isize` (raw digest into a 64-byte caller buffer; `-1` = unknown algo OR non-crypto checksum). Both already exist in `crates/elephc-crypto/src/lib.rs`.
- Slot + publish: `_elephc_crypto_hash_fn` in `src/codegen/runtime/data/fixed.rs`; `publish_elephc_crypto_function_pointers` in `src/codegen/builtins/strings/hash_crypto.rs` (publishes the address into the slot; both arches). The unknown-algorithm `ValueError` thrower also lives in `hash_crypto.rs` (`hash()` message).
- Shared hash dispatch: `__rt_hash` in `src/codegen/runtime/strings/hash.rs` (algo regs + data regs + binary flag ARM64 `x5`/x86_64 `r10`; marshals the C ABI; raises ValueError on -1) and the shared formatter `__rt_digest_to_string` in `src/codegen/runtime/strings/digest_to_string.rs` (length-driven hex, or raw bytes when the flag is set).
- Conditional link: `checker.require_builtin_library("elephc_crypto")` (all targets) for any builtin that references the crypto slots. The `RuntimeFeatures.descriptor_invoker` flag (in `runtime_features.rs`) already forces the crate link for dynamic-dispatch programs; since the dynamic dispatcher includes ALL builtins, the new crypto builtins are covered by it automatically once their emitters call `publish`.
- Builtin "add" checklist: catalog (`src/types/checker/builtins/catalog.rs`), signature (`src/types/signatures.rs` — both `builtin_call_sig` and the first-class-callable sig), checker arm (`src/types/checker/builtins/strings.rs`), codegen emitter + dispatcher (`src/codegen/builtins/strings/mod.rs`), runtime routine + registration (`src/codegen/runtime/strings/mod.rs`, `runtime/emitters.rs`), optimizer effects (`src/optimize/effects/builtins.rs`), codegen return-type table if needed (`src/codegen/functions/types/builtins.rs`).

## Templates to read (existing code, not the plan)
- Array-of-strings return: `src/codegen/builtins/strings/str_split.rs` + its runtime (uses `__rt_array_new(cap, 16)` + `__rt_array_push_str(arr, ptr, len)` → returns the array handle, re-saving after each push).
- Bool-returning two-string compare: `src/codegen/builtins/strings/str_contains.rs` / `strpos.rs` (two-string ABI: ARM64 `x1/x2`,`x3/x4`; x86_64 `rdi/rsi`,`rdx/rcx`; bool result in `x0`/`rax` via `cset`/`setcc`).
- File read: `src/codegen/builtins/io/file_get_contents.rs` + `src/codegen/runtime/io/file_get_contents.rs` (`__rt_file_get_contents` / `__rt_file_get_contents_maybe_url`: filename ptr/len → heap (ptr,len), null on failure; the buffer is a persistent heap allocation, safe to pass onward).
- The Phase-2 `hash()` emitter `src/codegen/builtins/strings/hash.rs` (algo+data+binary marshalling, `emit_binary_flag`, publish call) — the closest template for hash_hmac/hash_file.

## Conventions (MANDATORY)
Column-81 `//` on every emitted instruction; `//!`/`///` docs; `abi::` helpers; both arches per change; zero warnings; NEVER `cargo fmt`; no Co-Authored-By; TDD. Verify alignment with the CLAUDE.md python snippet on touched runtime files.

## PHP golden values (oracle = PHP 8.4)
- `hash_hmac("sha256","what do ya want for nothing?","Jefe")` = `5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843`
- `hash_hmac("sha1","abc","key")` = `4fd0b215276ef12f2b3e4c8ecac2811498b656fc`
- `hash_hmac("sha512","abc","key")` = `3926a207c8c42b0c41792cbd3e1a1aaaf5f7a25704f62dfc939c4987dd7ce060009c5bb1c2447355b3216f10b537e9afa7b64a4e5391b0d631172d07939e087a`
- `hash_hmac` unknown/checksum algo → `ValueError`, message EXACTLY: `hash_hmac(): Argument #1 ($algo) must be a valid cryptographic hashing algorithm` (DISTINCT from hash()'s message).
- `hash("sha256","hello")` = `2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824` (for hash_file of a file containing "hello").

---

### Task 1: `hash_hmac($algo, $data, $key, $binary = false)`

**Files:** `runtime/data/fixed.rs`, `builtins/strings/hash_crypto.rs`, `builtins/strings/hash_hmac.rs` (new), `builtins/strings/mod.rs`, `runtime/strings/hash_hmac.rs` (new), `runtime/strings/mod.rs`, `runtime/emitters.rs`, checker `strings.rs`, `signatures.rs`, `catalog.rs`, `optimize/effects/builtins.rs`, codegen tests.

- [ ] **Step 1: Failing codegen tests** (add to `tests/codegen/strings/interpolation_and_hashes.rs`):
```rust
#[test]
fn hash_hmac_matches_php() {
    assert_eq!(compile_and_run(r#"<?php echo hash_hmac("sha256","what do ya want for nothing?","Jefe");"#),
        "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843");
    assert_eq!(compile_and_run(r#"<?php echo hash_hmac("sha1","abc","key");"#),
        "4fd0b215276ef12f2b3e4c8ecac2811498b656fc");
    assert_eq!(compile_and_run(r#"<?php echo bin2hex(hash_hmac("sha256","abc","key",true)) === hash_hmac("sha256","abc","key") ? "1" : "0";"#), "1");
    assert_eq!(compile_and_run(r#"<?php echo strlen(hash_hmac("sha256","abc","key",true));"#), "32");
}
#[test]
fn hash_hmac_rejects_non_crypto_with_value_error() {
    assert_eq!(
        compile_and_run(r#"<?php try { hash_hmac("crc32b","d","k"); } catch (\ValueError $e) { echo $e->getMessage(); }"#),
        "hash_hmac(): Argument #1 ($algo) must be a valid cryptographic hashing algorithm"
    );
}
```
- [ ] **Step 2:** Run → fail (`hash_hmac` unknown). `cargo test --test codegen_tests hash_hmac 2>&1 | grep -E "test result|FAILED"`.
- [ ] **Step 3: Slot + publish.** In `fixed.rs` add `.comm _elephc_crypto_hmac_fn, 8, 3`. In `hash_crypto.rs`, extend the `ENTRIES` array to also publish `("elephc_crypto_hmac","_elephc_crypto_hmac_fn")`. Add a SECOND ValueError thrower (or parameterize the existing one) emitting the hash_hmac message above (add the message string in `fixed.rs`/`runtime/data` like `_hash_unknown_algo_msg`).
- [ ] **Step 4: `__rt_hash_hmac` runtime** (`runtime/strings/hash_hmac.rs`, both arches, registered in `runtime/strings/mod.rs` + `runtime/emitters.rs`). Contract: receives algo (ptr/len), data (ptr/len), key (ptr/len), binary flag — choose non-colliding registers (the emitter sets them; pick regs that don't clash with the 7-arg C ABI). Marshal to `elephc_crypto_hmac(name,name_len,key,key_len,data,data_len,out)`: ARM64 `x0/x1=algo, x2/x3=key, x4/x5=data, x6=out_buf`; x86_64 `rdi/rsi=algo, rdx/rcx=key, r8/r9=data, [stack]=out_buf` (7th arg on stack per SysV — verify; or restructure). **CRITICAL: the marshal must be clobber-free (read each source reg before overwriting it) — derive the move order carefully, like `__rt_hash` did.** Save the binary flag across the call. Load `_elephc_crypto_hmac_fn`, `cbz`/`test`→ the hash_hmac ValueError thrower, indirect-call. On `-1` → throw the hash_hmac ValueError. Else format via `__rt_digest_to_string`.
- [ ] **Step 5: Emitter** (`builtins/strings/hash_hmac.rs`): evaluate args in PHP source order ($algo, $data, $key, optional $binary), then place them in the runtime regs `__rt_hash_hmac` expects (note PHP order data-before-key vs the ABI key-before-data — the emitter just delivers them to the agreed runtime regs; the runtime does the ABI marshal). Reuse `super::hash::emit_binary_flag(args, 3, ...)`. Call `publish_elephc_crypto_function_pointers` before `__rt_hash_hmac`. Dispatcher arm in `mod.rs`.
- [ ] **Step 6: Wiring.** `catalog.rs`: add `"hash_hmac"`. `signatures.rs`: `"hash_hmac" => Some(optional(&["algo","data","key","binary"], 3, vec![bool_lit(false)]))` + a first-class-callable sig returning `PhpType::Str`. Checker arm in `strings.rs`: validate 3-4 args, `require_builtin_library("elephc_crypto")`, return `PhpType::Str`. Effects (`effects/builtins.rs`): hash_hmac is pure-but-can-throw — model like `hash` (NOT in the pure-non-throwing list, since it throws ValueError).
- [ ] **Step 7:** Build clean; `cargo test --test codegen_tests hash_hmac` → pass. Alignment check on the two new/edited runtime files.
- [ ] **Step 8: Commit** `feat(crypto): add hash_hmac() via elephc-crypto`.

### Task 2: `hash_file($algo, $filename, $binary = false)`

**Files:** `builtins/io/hash_file.rs` (new) or `builtins/strings/hash_file.rs`, the right `mod.rs`, checker, signatures, catalog, effects, codegen tests. Reuses `__rt_file_get_contents` + `__rt_hash` (no new runtime routine needed if composed in the emitter).

- [ ] **Step 1: Failing tests.** A file-based test (write a file in the compiled program, then hash it):
```rust
#[test]
fn hash_file_hashes_file_contents() {
    // hash_file of a file containing "hello" equals hash() of "hello"
    assert_eq!(
        compile_and_run(r#"<?php file_put_contents("hf.txt","hello"); echo hash_file("sha256","hf.txt");"#),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
    assert_eq!(
        compile_and_run(r#"<?php file_put_contents("hf2.txt","hello"); echo bin2hex(hash_file("sha256","hf2.txt",true));"#),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}
#[test]
fn hash_file_missing_file_returns_false() {
    // PHP hash_file() returns false (prints "") on a missing file
    assert_eq!(compile_and_run(r#"<?php var_dump(hash_file("sha256","/no/such/file"));"#), "bool(false)");
}
```
(Confirm how existing file_get_contents codegen tests create/locate files relative to the test's temp cwd; mirror that.)
- [ ] **Step 2:** Run → fail. **Step 3: Emitter** — evaluate $algo + optional $binary (preserve), evaluate $filename and call `__rt_file_get_contents` (or `_maybe_url`), check null → return PHP `false`; otherwise move the (ptr,len) into the data regs, restore algo into algo regs + binary flag, call `publish_*`, then `__rt_hash`. The result type is `string|false` — return `PhpType::Str` (matching how other maybe-false string builtins are typed; check how `file_get_contents` types its `string|false`). **Step 4:** wiring (catalog/signatures/checker — `optional(["algo","filename","binary"],2,[false])`, first-class sig; effects: hash_file READS THE FILESYSTEM → NOT pure, model with file-read effects like file_get_contents). **Step 5:** build + test + alignment. **Step 6: Commit** `feat(crypto): add hash_file()`.

### Task 3: `hash_equals($known, $user)` — timing-safe, no crate

**Files:** `builtins/strings/hash_equals.rs` (new), `runtime/strings/hash_equals.rs` (new), the two `mod.rs`, `runtime/emitters.rs`, checker, signatures, catalog, effects, tests.

- [ ] **Step 1: Failing tests:**
```rust
#[test]
fn hash_equals_timing_safe_compare() {
    assert_eq!(compile_and_run(r#"<?php var_dump(hash_equals("abc","abc"));"#), "bool(true)");
    assert_eq!(compile_and_run(r#"<?php var_dump(hash_equals("abc","abd"));"#), "bool(false)");
    assert_eq!(compile_and_run(r#"<?php var_dump(hash_equals("abc","abcd"));"#), "bool(false)");
    assert_eq!(compile_and_run(r#"<?php var_dump(hash_equals("",""));"#), "bool(true)");
}
```
- [ ] **Step 2-4:** Implement `__rt_hash_equals` (both arches) per the strpos two-string ABI: return false immediately on length mismatch; otherwise XOR-accumulate over ALL bytes (constant-time for equal lengths) and return bool. Emitter mirrors `str_contains` (two strings → bool). Wiring: catalog `"hash_equals"`, signature `fixed(["known_string","user_string"])` + first-class sig returning `PhpType::Bool`, checker returns `PhpType::Bool`, effects: PURE (reads args, no throw, no fs). NO `require_builtin_library` (pure asm, no crate). **Step 5:** build + test + alignment. **Step 6: Commit** `feat(crypto): add hash_equals() timing-safe comparison`.

### Task 4: `hash_algos()` — returns the supported algorithm set

**Files:** `builtins/strings/hash_algos.rs` (new), `runtime/strings/hash_algos.rs` (new), the two `mod.rs`, `runtime/emitters.rs`, checker, signatures, catalog, effects, tests.

- [ ] **Step 1: Failing tests** (assert it returns OUR supported set and every entry is hashable; we support a subset of PHP's `hash_algos()` — gost/haval/snefru/tiger/murmur/xxh are documented gaps):
```rust
#[test]
fn hash_algos_lists_supported_and_each_is_hashable() {
    // Spot-check representative members are present
    assert_eq!(compile_and_run(r#"<?php echo in_array("sha256", hash_algos()) ? "1":"0";"#), "1");
    assert_eq!(compile_and_run(r#"<?php echo in_array("crc32c", hash_algos()) ? "1":"0";"#), "1");
    assert_eq!(compile_and_run(r#"<?php echo in_array("whirlpool", hash_algos()) ? "1":"0";"#), "1");
    // We do NOT claim to support these PHP algos
    assert_eq!(compile_and_run(r#"<?php echo in_array("tiger128,3", hash_algos()) ? "1":"0";"#), "0");
    // Every advertised algo must actually hash without throwing
    assert_eq!(compile_and_run(r#"<?php $ok=1; foreach (hash_algos() as $a){ if (hash($a,"x")==="") $ok=0; } echo $ok;"#), "1");
}
```
- [ ] **Step 2-4:** Implement `__rt_hash_algos_list` (both arches) mirroring `str_split`'s array build: `__rt_array_new(N,16)` then `__rt_array_push_str` for each name. **The name list MUST be exactly the crate's `make()` supported set** (md2, md4, md5, sha1, sha224, sha256, sha384, sha512, sha512/224, sha512/256, sha3-224, sha3-256, sha3-384, sha3-512, ripemd128, ripemd160, ripemd256, ripemd320, whirlpool, crc32, crc32b, crc32c, adler32, fnv132, fnv1a32, fnv164, fnv1a64, joaat) — do NOT include blake2 (removed) or unsupported algos. Emit each name as a read-only `.asciz`/byte literal. Emitter calls `__rt_hash_algos_list`. Wiring: catalog `"hash_algos"`, signature `fixed(&[])` + first-class sig returning `PhpType::Array(Box::new(PhpType::Str))`, checker returns that array type, effects: PURE (allocates an array, no throw/fs). NO crate dependency (pure array build). **Step 5:** build + test + alignment. **Step 6: Commit** `feat(crypto): add hash_algos() returning the supported algorithm set`.

### Task 5: Phase-3 gate
- [ ] crc32/phar/hash/md5/sha1 regression (`cargo test --test codegen_tests` filters). Full `cargo test` + `cargo test -- --include-ignored` (only live-DB PDO failures acceptable). Docker `./scripts/test-linux-x86_64.sh` and `-arm64.sh` for `hash_hmac`, `hash_file`, `hash_equals`, `hash_algos` (target-sensitive: new asm + the hmac C-ABI marshal + the `_elephc_crypto_hmac_fn` link). `git diff --check`; `cargo build`/`--release` warning-free. Commit any gate fixes.

---

## Self-Review
**Spec coverage (Phase 3):** hash_hmac (Task 1), hash_file (Task 2), hash_equals (Task 3), hash_algos (Task 4), multi-target gate (Task 5). ✓
**Deferred:** incremental HashContext (Phase 4); phar migration + CommonCrypto/libcrypto removal + docs/examples/ROADMAP (Phase 5).
**Placeholders:** assembly specified by contract + templates + the C-ABI marshal; PHP-golden codegen tests are concrete. ✓
**Risk notes for implementers:**
1. **hash_hmac arg marshal** — the data↔key positional swap (PHP `(algo,data,key)` → ABI `(name,key,data)`) plus a clobber-free register shuffle is the #1 bug risk; derive the move order so no source reg is overwritten before use, exactly as `__rt_hash` did. On x86_64 the 7th C arg (out_buf) is stack-passed — handle SysV stack-arg + 16-byte alignment carefully.
2. **hash_hmac ValueError message differs from hash()** — use the `cryptographic` wording; add a distinct message constant.
3. **hash_algos list = crate's make() set** — keep it in lockstep with `crates/elephc-crypto/src/algos.rs`; the "every advertised algo is hashable" test guards drift (any name not in make() would make hash() throw → test fails).
4. **hash_file returns string|false** — null file read → PHP false; don't feed a null buffer to the hash path.
5. **Linking:** hash_hmac/hash_file reference the crypto slots → `require_builtin_library("elephc_crypto")`; hash_equals/hash_algos are pure (no crate). The descriptor_invoker flag already covers dynamic dispatch to all four since `publish` now publishes both hash+hmac symbols.
