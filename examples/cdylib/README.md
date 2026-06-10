# `--emit cdylib` end-to-end demo

Compiles a small PHP file containing two `#[Export]`-marked functions into a
loadable shared library, then loads it from a C host that exercises the
lifecycle entry points and the exported functions.

## Build and run

Linux:

```bash
cargo run -- --emit cdylib examples/cdylib/auth.php
cc -o examples/cdylib/host examples/cdylib/host.c -ldl
./examples/cdylib/host examples/cdylib/libauth.so
```

macOS:

```bash
cargo run -- --emit cdylib examples/cdylib/auth.php
cc -o examples/cdylib/host examples/cdylib/host.c
./examples/cdylib/host examples/cdylib/libauth.dylib
```

Expected output:

```
elephc cdylib demo OK: add_i64(40,2)=42, validate_token long=0 short=1
```

## What the demo covers (v1)

- `--emit cdylib` artifact naming: `lib<stem>.{so,dylib}`.
- `dlopen` + `dlsym` resolution of the four lifecycle entry points
  (`elephc_init`, `elephc_shutdown`, `elephc_last_error`, `elephc_free`).
- Scalar parameter marshaling for `int` (round-trip through `add_i64`).
- String-in parameter marshaling for `string` (`validate_token` receives a
  `(const char* ptr, size_t len)` pair in two consecutive integer-arg regs).
- Scalar return values (`int32_t` from `validate_token`, `int64_t` from
  `add_i64`).

## What v1 deliberately does not cover

- **x86_64 targets.** The runtime emits ~700 inline `[rip + sym]` references
  to its own globals which fail as `R_X86_64_PC32` relocations in a `.so`.
  v1 errors out cleanly on `--target linux-x86_64` / `--target macos-x86_64`
  with `--emit cdylib`. Unblocking it requires routing the runtime through
  the GOT helpers in `src/codegen/abi/symbols.rs` (the user-code path was
  ported as part of this change and works on x86_64 — only the shared
  runtime object is still direct-RIP).
- String return values (no `elephc_free`-able host-owned strings yet).
- Array, object, callable, or `null` parameter / return types.
- Exception propagation from PHP back to C.
- Thread-safety guarantees beyond a single-threaded host.

These ship in follow-up iterations once v1 has bedded in.
