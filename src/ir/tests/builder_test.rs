//! Purpose:
//! Verifies the checked construction API for blocks, values, instructions, and terminators.
//!
//! Called from:
//! - `crate::ir::tests`.
//!
//! Key details:
//! - The builder must preserve table ID relationships that the validator later checks.

use crate::ir::{Builder, Function, IrType, Terminator};
use crate::types::PhpType;

/// Builds a minimal function that returns a constant.
#[test]
fn build_function_with_return() {
    let mut function = Function::new("ret_42".to_string(), IrType::I64, PhpType::Int);
    {
        let mut builder = Builder::new(&mut function);
        let entry = builder.create_named_block("entry", vec![]);
        builder.set_entry(entry);
        builder.position_at_end(entry);
        let value = builder.emit_const_i64(42);
        builder.terminate(Terminator::Return { value: Some(value) });
    }
    assert_eq!(function.blocks.len(), 1);
    assert_eq!(function.values.len(), 1);
    assert_eq!(function.instructions.len(), 1);
}

/// Builds a branch into a block with a block parameter.
#[test]
fn build_function_with_block_param_and_iadd() {
    let mut function = Function::new("add_one".to_string(), IrType::I64, PhpType::Int);
    {
        let mut builder = Builder::new(&mut function);
        let entry = builder.create_named_block("entry", vec![]);
        let body = builder.create_named_block("body", vec![(IrType::I64, PhpType::Int)]);
        builder.set_entry(entry);
        builder.position_at_end(entry);
        let one = builder.emit_const_i64(1);
        builder.terminate(Terminator::Br {
            target: body,
            args: vec![one],
        });

        let arg = builder.block_param(body, 0);
        builder.position_at_end(body);
        let one_again = builder.emit_const_i64(1);
        let sum = builder.emit_iadd(arg, one_again);
        builder.terminate(Terminator::Return { value: Some(sum) });
    }
    assert_eq!(function.blocks.len(), 2);
    assert_eq!(function.blocks[1].params.len(), 1);
    assert_eq!(function.blocks[1].instructions.len(), 2);
}
