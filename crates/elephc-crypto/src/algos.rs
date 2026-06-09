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

use digest::DynDigest;

/// Object-safe streaming hash state covering both RustCrypto `DynDigest`
/// algorithms and the non-`DynDigest` checksums (crc32, adler32, fnv, joaat).
pub trait HashState {
    /// Feeds more input into the running digest.
    fn update(&mut self, data: &[u8]);
    /// Consumes the state and returns the raw digest bytes.
    fn finalize_box(self: Box<Self>) -> Vec<u8>;
    /// Raw digest size in bytes (e.g. 32 for sha256).
    fn output_size(&self) -> usize;
    /// Algorithm block size in bytes, used by the HMAC key schedule.
    fn block_size(&self) -> usize;
    /// Clones the running state (backs hash_copy / `clone $ctx`).
    fn box_clone(&self) -> Box<dyn HashState>;
}

/// Wraps any RustCrypto `DynDigest` hasher as a `HashState`, carrying the
/// algorithm block size (DynDigest does not expose it).
struct DigestState {
    inner: Box<dyn DynDigest>,
    block: usize,
}

impl HashState for DigestState {
    /// Feeds input into the boxed DynDigest.
    fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }
    /// Finalizes the boxed DynDigest into its raw digest bytes.
    fn finalize_box(self: Box<Self>) -> Vec<u8> {
        self.inner.finalize().to_vec()
    }
    /// Returns the DynDigest's raw output size.
    fn output_size(&self) -> usize {
        self.inner.output_size()
    }
    /// Returns the stored algorithm block size.
    fn block_size(&self) -> usize {
        self.block
    }
    /// Clones the underlying DynDigest state.
    fn box_clone(&self) -> Box<dyn HashState> {
        Box::new(DigestState { inner: self.inner.box_clone(), block: self.block })
    }
}

/// Builds a boxed `DigestState` for a default-constructed RustCrypto hasher.
fn digest_state<D>(block: usize) -> Box<dyn HashState>
where
    D: digest::Digest + digest::FixedOutputReset + Clone + 'static,
{
    Box::new(DigestState { inner: Box::new(D::new()), block })
}

/// Resolves a PHP hash() algorithm name to a freshly initialized `HashState`,
/// or `None` if the algorithm is unsupported (caller maps to PHP ValueError).
pub fn make(name: &str) -> Option<Box<dyn HashState>> {
    use blake2::{Blake2b512, Blake2s256};
    use ripemd::{Ripemd128, Ripemd160, Ripemd256, Ripemd320};
    use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
    use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
    Some(match name {
        "md2" => digest_state::<md2::Md2>(16),
        "md4" => digest_state::<md4::Md4>(64),
        "md5" => digest_state::<md5::Md5>(64),
        "sha1" => digest_state::<sha1::Sha1>(64),
        "sha224" => digest_state::<Sha224>(64),
        "sha256" => digest_state::<Sha256>(64),
        "sha384" => digest_state::<Sha384>(128),
        "sha512" => digest_state::<Sha512>(128),
        "sha512/224" => digest_state::<Sha512_224>(128),
        "sha512/256" => digest_state::<Sha512_256>(128),
        "sha3-224" => digest_state::<Sha3_224>(144),
        "sha3-256" => digest_state::<Sha3_256>(136),
        "sha3-384" => digest_state::<Sha3_384>(104),
        "sha3-512" => digest_state::<Sha3_512>(72),
        "ripemd128" => digest_state::<Ripemd128>(64),
        "ripemd160" => digest_state::<Ripemd160>(64),
        "ripemd256" => digest_state::<Ripemd256>(64),
        "ripemd320" => digest_state::<Ripemd320>(64),
        "whirlpool" => digest_state::<whirlpool::Whirlpool>(64),
        "blake2b512" => digest_state::<Blake2b512>(128),
        "blake2s256" => digest_state::<Blake2s256>(64),
        // Non-crypto checksums are added in Task 3.
        _ => return None,
    })
}
