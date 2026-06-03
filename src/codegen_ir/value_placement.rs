//! Purpose:
//! Maps EIR SSA values to stable stack slots for the Phase 04 backend.
//! This deliberately favors simple correctness over register efficiency.
//!
//! Called from:
//! - `crate::codegen_ir` function and instruction lowering helpers.
//!
//! Key details:
//! - Each non-void SSA value gets a slot below the frame pointer.
//! - Phase 06 replaces this with linear-scan register allocation.

use std::collections::HashMap;

use crate::ir::{Function, IrType, ValueId};

/// Stack-slot table for the Phase 04 spill-everything backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValuePlacement {
    pub slot_of: HashMap<ValueId, usize>,
    pub total_slot_bytes: usize,
}

impl ValuePlacement {
    /// Returns the frame-pointer-relative byte offset for a value if it has a slot.
    pub fn slot(&self, value: ValueId) -> Option<usize> {
        self.slot_of.get(&value).copied()
    }
}

/// Allocates a frame slot for every non-void SSA value in a function.
pub fn allocate(func: &Function) -> ValuePlacement {
    let mut slot_of = HashMap::new();
    let mut offset = 0usize;
    for (index, value) in func.values.iter().enumerate() {
        let bytes = bytes_for(value.ir_type);
        if bytes == 0 {
            continue;
        }
        offset += bytes;
        slot_of.insert(ValueId::from_raw(index as u32), offset);
    }
    ValuePlacement {
        slot_of,
        total_slot_bytes: align_to_16(offset),
    }
}

/// Returns the slot size for one EIR storage type.
pub fn bytes_for(ty: IrType) -> usize {
    match ty {
        IrType::I64 | IrType::F64 | IrType::Heap(_) => 8,
        IrType::Str => 16,
        IrType::Void => 0,
    }
}

/// Rounds a byte count up to the next 16-byte stack-alignment boundary.
fn align_to_16(bytes: usize) -> usize {
    (bytes + 15) & !15
}

#[cfg(test)]
mod tests {
    //! Purpose:
    //! Unit tests for Phase 04 EIR value placement.
    //!
    //! Called from:
    //! - `cargo test` through Rust's test harness.
    //!
    //! Key details:
    //! - These tests verify the stack-slot contract before instruction lowering uses it.

    use crate::ir::{Builder, Function, IrType};
    use crate::types::PhpType;

    use super::{allocate, bytes_for};

    /// Verifies one integer value gets one 8-byte slot and a 16-byte aligned frame area.
    #[test]
    fn allocates_one_i64_value_slot() {
        let mut function = Function::new("test".to_string(), IrType::I64, PhpType::Int);
        let mut builder = Builder::new(&mut function);
        let entry = builder.create_named_block("entry", Vec::new());
        builder.set_entry(entry);
        builder.position_at_end(entry);
        let value = builder.emit_const_i64(7);

        let placement = allocate(&function);

        assert_eq!(placement.slot(value), Some(8));
        assert_eq!(placement.total_slot_bytes, 16);
    }

    /// Verifies string values reserve both pointer and length words in placement.
    #[test]
    fn allocates_string_value_as_two_words() {
        let mut function = Function::new("test".to_string(), IrType::Str, PhpType::Str);
        let data_id = crate::ir::DataId::from_raw(0);
        let mut builder = Builder::new(&mut function);
        let entry = builder.create_named_block("entry", Vec::new());
        builder.set_entry(entry);
        builder.position_at_end(entry);
        let value = builder.emit_const_str(data_id);

        let placement = allocate(&function);

        assert_eq!(placement.slot(value), Some(16));
        assert_eq!(placement.total_slot_bytes, 16);
    }

    /// Verifies void storage does not consume a spill slot.
    #[test]
    fn void_values_have_no_slot_size() {
        assert_eq!(bytes_for(IrType::Void), 0);
    }
}
