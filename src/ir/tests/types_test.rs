//! Purpose:
//! Verifies EIR storage type mapping from the checked PHP type model.
//!
//! Called from:
//! - `crate::ir::tests`.
//!
//! Key details:
//! - The tests pin storage contracts that future lowering and ABI codegen use.

use crate::ir::{IrHeapKind, IrType};
use crate::types::PhpType;

/// Maps PHP integer values to one integer register.
#[test]
fn maps_int_to_i64() {
    assert_eq!(IrType::from_php(&PhpType::Int), IrType::I64);
}

/// Maps PHP booleans to integer storage.
#[test]
fn maps_bool_to_i64() {
    assert_eq!(IrType::from_php(&PhpType::Bool), IrType::I64);
}

/// Maps PHP strings to the two-register string ABI storage type.
#[test]
fn maps_str_to_str() {
    assert_eq!(IrType::from_php(&PhpType::Str), IrType::Str);
}

/// Maps indexed arrays to the array heap subkind.
#[test]
fn maps_array_to_heap_array() {
    let php_ty = PhpType::Array(Box::new(PhpType::Int));
    assert_eq!(IrType::from_php(&php_ty), IrType::Heap(IrHeapKind::Array));
}

/// Maps associative arrays to the hash heap subkind.
#[test]
fn maps_assoc_array_to_heap_hash() {
    let php_ty = PhpType::AssocArray {
        key: Box::new(PhpType::Str),
        value: Box::new(PhpType::Int),
    };
    assert_eq!(IrType::from_php(&php_ty), IrType::Heap(IrHeapKind::Hash));
}

/// Maps unions to the union heap subkind rather than erasing them in EIR.
#[test]
fn maps_union_to_heap_union() {
    let php_ty = PhpType::Union(vec![PhpType::Int, PhpType::Str]);
    assert_eq!(IrType::from_php(&php_ty), IrType::Heap(IrHeapKind::Union));
}

/// Reports target-register arity for each storage class.
#[test]
fn register_count_matches_storage_type() {
    assert_eq!(IrType::I64.register_count(), 1);
    assert_eq!(IrType::F64.register_count(), 1);
    assert_eq!(IrType::Str.register_count(), 2);
    assert_eq!(IrType::Heap(IrHeapKind::Mixed).register_count(), 1);
    assert_eq!(IrType::Void.register_count(), 0);
}
