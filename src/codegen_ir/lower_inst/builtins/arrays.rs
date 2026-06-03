//! Purpose:
//! Lowers small indexed-array builtins for the EIR backend.
//! Delegates aggregate iteration and key-existence checks to existing runtime helpers.
//!
//! Called from:
//! - `crate::codegen_ir::lower_inst::builtins::lower_builtin_call()`.
//!
//! Key details:
//! - Aggregate helpers only accept indexed arrays with non-float scalar slots
//!   because they read 8-byte integer payloads directly.
//! - Indexed key existence reads only the array header, so element payload type is irrelevant.

use crate::codegen::abi;
use crate::codegen::platform::Arch;
use crate::codegen_ir::{CodegenIrError, Result};
use crate::ir::Instruction;
use crate::types::PhpType;

use super::super::super::context::FunctionContext;
use super::super::{expect_operand, store_if_result};

/// Lowers `array_sum()` over supported indexed-array payloads.
pub(super) fn lower_array_sum(ctx: &mut FunctionContext<'_>, inst: &Instruction) -> Result<()> {
    lower_indexed_array_aggregate(ctx, inst, "array_sum", "__rt_array_sum")
}

/// Lowers `array_product()` over supported indexed-array payloads.
pub(super) fn lower_array_product(ctx: &mut FunctionContext<'_>, inst: &Instruction) -> Result<()> {
    lower_indexed_array_aggregate(ctx, inst, "array_product", "__rt_array_product")
}

/// Lowers `array_key_exists()` for indexed arrays with integer-like keys.
pub(super) fn lower_array_key_exists(ctx: &mut FunctionContext<'_>, inst: &Instruction) -> Result<()> {
    super::ensure_arg_count(inst, "array_key_exists", 2)?;
    let key = expect_operand(inst, 0)?;
    let array = expect_operand(inst, 1)?;
    require_indexed_array_key_exists_types(
        ctx.value_php_type(key)?,
        ctx.value_php_type(array)?,
        "array_key_exists",
    )?;
    match ctx.emitter.target.arch {
        Arch::AArch64 => {
            ctx.load_value_to_reg(array, "x0")?;
            ctx.load_value_to_reg(key, "x1")?;
        }
        Arch::X86_64 => {
            ctx.load_value_to_reg(array, "rdi")?;
            ctx.load_value_to_reg(key, "rsi")?;
        }
    }
    abi::emit_call_label(ctx.emitter, "__rt_array_key_exists");
    store_if_result(ctx, inst)
}

/// Loads an indexed array argument and calls the selected runtime aggregate helper.
fn lower_indexed_array_aggregate(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    name: &str,
    helper: &str,
) -> Result<()> {
    super::ensure_arg_count(inst, name, 1)?;
    let array = expect_operand(inst, 0)?;
    require_supported_indexed_array(ctx.value_php_type(array)?, name)?;
    ctx.load_value_to_result(array)?;
    if ctx.emitter.target.arch == Arch::X86_64 {
        ctx.emitter.instruction("mov rdi, rax");                                // pass the indexed-array pointer as the runtime helper argument
    }
    abi::emit_call_label(ctx.emitter, helper);
    store_if_result(ctx, inst)
}

/// Verifies the aggregate can use the current raw integer-slot runtime helper.
fn require_supported_indexed_array(ty: PhpType, name: &str) -> Result<()> {
    match ty.codegen_repr() {
        PhpType::Array(elem) if matches!(*elem, PhpType::Int | PhpType::Bool | PhpType::Never) => Ok(()),
        other => Err(CodegenIrError::unsupported(format!(
            "{} for PHP type {:?}",
            name,
            other
        ))),
    }
}

/// Verifies indexed-array key existence can use the integer-key runtime helper.
fn require_indexed_array_key_exists_types(
    key_ty: PhpType,
    array_ty: PhpType,
    name: &str,
) -> Result<()> {
    match array_ty.codegen_repr() {
        PhpType::Array(_) => {}
        other => {
            return Err(CodegenIrError::unsupported(format!(
                "{} for PHP array type {:?}",
                name,
                other
            )));
        }
    }
    match key_ty.codegen_repr() {
        PhpType::Int | PhpType::Bool => Ok(()),
        other => Err(CodegenIrError::unsupported(format!(
            "{} key PHP type {:?}",
            name,
            other
        ))),
    }
}
