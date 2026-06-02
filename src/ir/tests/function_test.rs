//! Purpose:
//! Verifies EIR function/module containers and data-pool interning.
//!
//! Called from:
//! - `crate::ir::tests`.
//!
//! Key details:
//! - Phase 02 containers are pure data structures with deterministic IDs.

use crate::codegen::platform::{Arch, Platform, Target};
use crate::ir::{Function, IrType, LocalKind, Module};
use crate::types::PhpType;

/// New functions start empty and retain their declared return metadata.
#[test]
fn empty_function_has_no_blocks() {
    let function = Function::new("foo".to_string(), IrType::Void, PhpType::Void);
    assert_eq!(function.blocks.len(), 0);
    assert_eq!(function.name, "foo");
    assert_eq!(function.return_type, IrType::Void);
}

/// Module construction records target metadata and starts with empty function sets.
#[test]
fn module_has_target_metadata() {
    let target = Target::new(Platform::MacOS, Arch::AArch64);
    let module = Module::new(target);
    assert_eq!(module.target, target);
    assert!(module.functions.is_empty());
    assert!(module.class_methods.is_empty());
}

/// Data-pool string interning returns the same ID for repeated literals.
#[test]
fn data_pool_deduplicates_strings() {
    let mut module = Module::new(Target::new(Platform::Linux, Arch::X86_64));
    let first = module.data.intern_string("hello");
    let second = module.data.intern_string("hello");
    assert_eq!(first, second);
    assert_eq!(module.data.strings.len(), 1);
}

/// Adding a local slot assigns a zero-based local ID.
#[test]
fn add_local_assigns_slot_id() {
    let mut function = Function::new("locals".to_string(), IrType::Void, PhpType::Void);
    let slot = function.add_local(
        Some("x".to_string()),
        IrType::I64,
        PhpType::Int,
        LocalKind::PhpLocal,
    );
    assert_eq!(slot.as_raw(), 0);
    assert_eq!(function.locals.len(), 1);
}
