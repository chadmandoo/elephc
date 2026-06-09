//! Purpose:
//! Pure-Rust hashing/HMAC bridge staticlib for elephc's PHP hash() family.
//! Exposes a C ABI of raw-digest functions keyed by algorithm name, consumed by
//! compiled PHP binaries via function-pointer slots (see src/codegen runtime).
//!
//! Called from:
//! - Compiled PHP program assembly through the `_elephc_crypto_*_fn` slots.
//! - `cargo test -p elephc-crypto` (the rlib) for in-isolation validation.
//!
//! Key details:
//! - All ABI functions are `#[no_mangle] pub extern "C"`; raw digests are written
//!   into a caller-provided 64-byte buffer (max digest size across supported algos).
//! - `ctx` handles are thin pointers to a boxed `HashCtx`; `final`/`free` own them.

mod algos;
