//! Purpose:
//! Verifies the EIR effect bitset and deterministic effect names.
//!
//! Called from:
//! - `crate::ir::tests`.
//!
//! Key details:
//! - Pure means no bits; observable operations must survive dead-code passes.

use crate::ir::Effects;

/// The pure effect set is empty and reports itself as pure.
#[test]
fn pure_has_no_bits() {
    assert!(Effects::PURE.is_empty());
    assert!(Effects::PURE.is_pure());
}

/// Heap reads and writes are independent effect categories.
#[test]
fn reads_and_writes_are_orthogonal() {
    let read = Effects::READS_HEAP;
    let write = Effects::WRITES_HEAP;
    assert!(read.may_observe());
    assert!(!read.may_mutate());
    assert!(write.may_mutate());
    assert!(!write.may_observe());
}

/// Combined effects retain all component bits.
#[test]
fn combined_effects_compose() {
    let effects = Effects::READS_HEAP | Effects::MAY_FATAL;
    assert!(effects.contains(Effects::READS_HEAP));
    assert!(effects.contains(Effects::MAY_FATAL));
    assert_eq!(effects.names(), vec!["reads_heap", "may_fatal"]);
}
