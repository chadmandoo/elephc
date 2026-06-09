//! Purpose:
//! Internal hashing state abstraction and the algorithm-name table for
//! elephc-crypto. Unifies RustCrypto DynDigest hashers with non-crypto checksums.
//!
//! Called from:
//! - `crate` (lib.rs) C ABI functions via `make()`.
//!
//! Key details:
//! - `HashState` is object-safe so a heterogeneous hasher lives behind one
//!   `Box<dyn HashState>`. `block_size` feeds the generic HMAC construction.

// Implementation added in Task 2.
