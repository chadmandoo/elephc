//! Purpose:
//! Canonical home for the in-band runtime sentinel constants shared across codegen.
//! Owns the null sentinel and the uninitialized-typed-property sentinel.
//!
//! Called from:
//! - `crate::codegen` emitters that produce or detect sentinel-encoded values.
//!
//! Key details:
//! - The null sentinel is an in-band i64 (`PHP_INT_MAX - 1`): every i64 bit pattern is a valid
//!   PHP int, so the real integer `9223372036854775806` collides with it. The structural fix
//!   (`NullRepr::Tagged`) replaces sentinel checks with a tagged scalar representation.
//! - The uninitialized-property sentinel lives in a separate metadata word (`offset + 8`),
//!   never in the value word, so it does not collide with property values.

/// In-band null marker for unboxed scalar slots: `0x7fff_ffff_ffff_fffe` (= `PHP_INT_MAX - 1`).
pub(crate) const NULL_SENTINEL: i64 = 0x7fff_ffff_ffff_fffe;

/// Marker stored in a typed property's metadata word while the property is uninitialized:
/// `0x7fff_ffff_ffff_fffd` (= `PHP_INT_MAX - 2`).
pub(crate) const UNINITIALIZED_TYPED_PROPERTY_SENTINEL: i64 = 0x7fff_ffff_ffff_fffd;

#[cfg(test)]
mod tests {
    use super::*;

    /// Locks the canonical null sentinel bit pattern shared by every producer and consumer.
    #[test]
    fn null_sentinel_constant_value() {
        assert_eq!(NULL_SENTINEL, 0x7fff_ffff_ffff_fffe_u64 as i64);
        assert_eq!(NULL_SENTINEL, i64::MAX - 1);
    }

    /// Locks the uninitialized-typed-property sentinel bit pattern used in property metadata.
    #[test]
    fn uninitialized_property_sentinel_constant_value() {
        assert_eq!(
            UNINITIALIZED_TYPED_PROPERTY_SENTINEL,
            0x7fff_ffff_ffff_fffd_u64 as i64
        );
        assert_eq!(UNINITIALIZED_TYPED_PROPERTY_SENTINEL, i64::MAX - 2);
    }
}
