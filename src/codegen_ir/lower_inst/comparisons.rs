//! Purpose:
//! Lowers scalar equality EIR opcodes for the Phase 04 backend.
//!
//! Called from:
//! - `crate::codegen_ir::lower_inst::lower_instruction()`.
//!
//! Key details:
//! - Strict equality respects static PHP type identity before comparing payloads.
//! - Loose equality is intentionally limited to scalar int/bool/null and
//!   string-vs-string cases until mixed numeric/string coercions are lowered.

use crate::codegen::abi;
use crate::codegen::platform::Arch;
use crate::ir::{Instruction, ValueId};
use crate::types::PhpType;

use super::super::context::FunctionContext;
use super::{expect_operand, store_if_result};
use crate::codegen_ir::{CodegenIrError, Result};

/// Lowers strict equality or inequality for scalar values.
pub(super) fn lower_strict_eq(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    is_equal: bool,
) -> Result<()> {
    let lhs = expect_operand(inst, 0)?;
    let rhs = expect_operand(inst, 1)?;
    let lhs_ty = ctx.value_php_type(lhs)?;
    let rhs_ty = ctx.value_php_type(rhs)?;
    if lhs_ty != rhs_ty {
        emit_bool_literal(ctx, !is_equal);
        return store_if_result(ctx, inst);
    }
    match lhs_ty {
        PhpType::Int | PhpType::Bool | PhpType::Void | PhpType::Never => {
            emit_intish_compare(ctx, lhs, rhs, is_equal, false)?;
        }
        PhpType::Float => {
            emit_float_compare(ctx, lhs, rhs, is_equal)?;
        }
        PhpType::Str => {
            emit_string_eq_call(ctx, lhs, rhs, is_equal, "__rt_str_eq")?;
        }
        other => {
            return Err(CodegenIrError::unsupported(format!(
                "{} for PHP type {:?}",
                inst.op.name(),
                other
            )))
        }
    }
    store_if_result(ctx, inst)
}

/// Lowers loose equality or inequality for scalar int/bool/null and string pairs.
pub(super) fn lower_loose_eq(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    is_equal: bool,
) -> Result<()> {
    let lhs = expect_operand(inst, 0)?;
    let rhs = expect_operand(inst, 1)?;
    let lhs_ty = ctx.value_php_type(lhs)?;
    let rhs_ty = ctx.value_php_type(rhs)?;
    if lhs_ty == PhpType::Str && rhs_ty == PhpType::Str {
        emit_string_eq_call(ctx, lhs, rhs, is_equal, "__rt_str_loose_eq")?;
    } else if intish_or_null(&lhs_ty) && intish_or_null(&rhs_ty) {
        let compare_truthiness = lhs_ty == PhpType::Bool || rhs_ty == PhpType::Bool;
        emit_intish_compare(ctx, lhs, rhs, is_equal, compare_truthiness)?;
    } else {
        return Err(CodegenIrError::unsupported(format!(
            "{} for PHP types {:?} and {:?}",
            inst.op.name(),
            lhs_ty,
            rhs_ty
        )));
    }
    store_if_result(ctx, inst)
}

/// Returns true for scalar values that can participate in the current loose integer path.
fn intish_or_null(ty: &PhpType) -> bool {
    matches!(ty, PhpType::Int | PhpType::Bool | PhpType::Void | PhpType::Never)
}

/// Emits an integer-like equality comparison into the integer result register.
fn emit_intish_compare(
    ctx: &mut FunctionContext<'_>,
    lhs: ValueId,
    rhs: ValueId,
    is_equal: bool,
    compare_truthiness: bool,
) -> Result<()> {
    let lhs_reg = abi::secondary_scratch_reg(ctx.emitter);
    let rhs_reg = abi::tertiary_scratch_reg(ctx.emitter);
    load_intish_value(ctx, lhs, lhs_reg, compare_truthiness)?;
    load_intish_value(ctx, rhs, rhs_reg, compare_truthiness)?;
    match ctx.emitter.target.arch {
        Arch::AArch64 => {
            ctx.emitter.instruction(&format!("cmp {}, {}", lhs_reg, rhs_reg));  // compare scalar equality operands
            ctx.emitter.instruction(&format!("cset x0, {}", equality_cond(is_equal, ctx.emitter.target.arch))); // materialize scalar equality as boolean
        }
        Arch::X86_64 => {
            ctx.emitter.instruction(&format!("cmp {}, {}", lhs_reg, rhs_reg));  // compare scalar equality operands
            ctx.emitter.instruction(&format!("set{} al", equality_cond(is_equal, ctx.emitter.target.arch))); // materialize scalar equality in the low byte
            ctx.emitter.instruction("movzx rax, al");                           // widen the equality byte into the integer result register
        }
    }
    Ok(())
}

/// Loads an int/bool/null value into `reg`, optionally coercing to PHP truthiness.
fn load_intish_value(
    ctx: &mut FunctionContext<'_>,
    value: ValueId,
    reg: &str,
    truthy: bool,
) -> Result<()> {
    match ctx.value_php_type(value)? {
        PhpType::Void | PhpType::Never => {
            abi::emit_load_int_immediate(ctx.emitter, reg, 0);
        }
        PhpType::Int | PhpType::Bool => {
            ctx.load_value_to_reg(value, reg)?;
            if truthy {
                emit_reg_nonzero_bool(ctx, reg);
            }
        }
        other => {
            return Err(CodegenIrError::unsupported(format!(
                "integer equality for PHP type {:?}",
                other
            )))
        }
    }
    Ok(())
}

/// Rewrites `reg` to 1 when nonzero and 0 otherwise.
fn emit_reg_nonzero_bool(ctx: &mut FunctionContext<'_>, reg: &str) {
    match ctx.emitter.target.arch {
        Arch::AArch64 => {
            ctx.emitter.instruction(&format!("cmp {}, #0", reg));               // compare scalar value against zero for truthiness
            ctx.emitter.instruction(&format!("cset {}, ne", reg));              // materialize nonzero truthiness in the same register
        }
        Arch::X86_64 => {
            ctx.emitter.instruction(&format!("test {}, {}", reg, reg));         // compare scalar value against zero for truthiness
            ctx.emitter.instruction("setne al");                                // materialize nonzero truthiness in the low byte
            ctx.emitter.instruction(&format!("movzx {}, al", reg));             // widen truthiness into the requested register
        }
    }
}

/// Emits a floating-point equality comparison into the integer result register.
fn emit_float_compare(
    ctx: &mut FunctionContext<'_>,
    lhs: ValueId,
    rhs: ValueId,
    is_equal: bool,
) -> Result<()> {
    let lhs_reg = match ctx.emitter.target.arch {
        Arch::AArch64 => "d1",
        Arch::X86_64 => "xmm1",
    };
    let rhs_reg = abi::float_result_reg(ctx.emitter);
    ctx.load_value_to_reg(lhs, lhs_reg)?;
    ctx.load_value_to_reg(rhs, rhs_reg)?;
    match ctx.emitter.target.arch {
        Arch::AArch64 => {
            ctx.emitter.instruction("fcmp d1, d0");                             // compare strict float equality operands
            ctx.emitter.instruction(&format!("cset x0, {}", equality_cond(is_equal, ctx.emitter.target.arch))); // materialize float equality as boolean
        }
        Arch::X86_64 => {
            ctx.emitter.instruction("ucomisd xmm1, xmm0");                      // compare strict float equality operands
            ctx.emitter.instruction(&format!("set{} al", equality_cond(is_equal, ctx.emitter.target.arch))); // materialize float equality in the low byte
            ctx.emitter.instruction("movzx rax, al");                           // widen the equality byte into the integer result register
        }
    }
    Ok(())
}

/// Calls the selected runtime string equality helper and optionally inverts its boolean result.
fn emit_string_eq_call(
    ctx: &mut FunctionContext<'_>,
    lhs: ValueId,
    rhs: ValueId,
    is_equal: bool,
    helper: &str,
) -> Result<()> {
    match ctx.emitter.target.arch {
        Arch::AArch64 => {
            ctx.load_string_value_to_regs(lhs, "x1", "x2")?;
            ctx.load_string_value_to_regs(rhs, "x3", "x4")?;
            abi::emit_call_label(ctx.emitter, helper);
            if !is_equal {
                ctx.emitter.instruction("eor x0, x0, #1");                      // invert string equality for inequality
            }
        }
        Arch::X86_64 => {
            ctx.load_string_value_to_regs(lhs, "rdi", "rsi")?;
            ctx.load_string_value_to_regs(rhs, "rdx", "rcx")?;
            abi::emit_call_label(ctx.emitter, helper);
            if !is_equal {
                ctx.emitter.instruction("xor rax, 1");                          // invert string equality for inequality
            }
        }
    }
    Ok(())
}

/// Emits a concrete boolean value into the integer result register.
fn emit_bool_literal(ctx: &mut FunctionContext<'_>, value: bool) {
    abi::emit_load_int_immediate(ctx.emitter, abi::int_result_reg(ctx.emitter), i64::from(value));
}

/// Returns the target condition-code suffix for equality or inequality.
fn equality_cond(is_equal: bool, arch: Arch) -> &'static str {
    match (is_equal, arch) {
        (true, Arch::AArch64) => "eq",
        (false, Arch::AArch64) => "ne",
        (true, Arch::X86_64) => "e",
        (false, Arch::X86_64) => "ne",
    }
}
