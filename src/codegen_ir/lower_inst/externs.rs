//! Purpose:
//! Lowers scalar extern function calls from EIR into the target C ABI.
//! Covers the Phase 04 parity path for non-string, non-callable FFI calls.
//!
//! Called from:
//! - `crate::codegen_ir::lower_inst::lower_instruction()`.
//!
//! Key details:
//! - Source-order evaluation already happened during AST-to-EIR lowering; this
//!   module only materializes precomputed SSA values into C ABI locations.
//! - String parameters are converted to call-scoped C strings and released
//!   immediately after the foreign call returns.
//! - Callable extern parameters require trampoline handling and remain explicit
//!   unsupported cases until their dedicated lowering lands.

use crate::codegen::abi;
use crate::codegen::platform::Arch;
use crate::ir::{ExternDecl, ExternParamDecl, Instruction, ValueId};
use crate::types::PhpType;

use super::super::context::FunctionContext;
use super::{expect_data, expect_operand, store_if_result};
use crate::codegen_ir::{CodegenIrError, Result};

/// Lowers an EIR extern call to a platform-mangled C symbol call.
pub(super) fn lower_extern_call(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    let decl = extern_decl(ctx, inst)?.clone();
    validate_extern_shape(&decl)?;
    if inst.operands.len() != decl.params.len() {
        return Err(CodegenIrError::unsupported(format!(
            "extern call to {} with {} args for {} params",
            decl.name,
            inst.operands.len(),
            decl.params.len()
        )));
    }

    let c_param_types = decl
        .params
        .iter()
        .map(c_abi_param_type)
        .collect::<Vec<_>>();
    let string_arg_count = decl
        .params
        .iter()
        .filter(|param| param.php_type.codegen_repr() == PhpType::Str)
        .count();
    let cleanup_bytes = string_arg_count * 16;
    if cleanup_bytes > 0 {
        abi::emit_reserve_temporary_stack(ctx.emitter, cleanup_bytes);
    }
    let cleanup_base_reg = abi::temp_int_reg(ctx.emitter.target);
    let mut cleanup_idx = 0usize;
    let mut pushed_arg_bytes = 0usize;
    for (idx, param) in decl.params.iter().enumerate() {
        let value = expect_operand(inst, idx)?;
        let pushed_ty = materialize_extern_arg(ctx, value, param)?;
        if param.php_type.codegen_repr() == PhpType::Str {
            abi::emit_temporary_stack_address(ctx.emitter, cleanup_base_reg, pushed_arg_bytes);
            abi::emit_store_to_address(
                ctx.emitter,
                abi::int_result_reg(ctx.emitter),
                cleanup_base_reg,
                cleanup_idx * 16,
            );
            cleanup_idx += 1;
        }
        abi::emit_push_result_value(ctx.emitter, &pushed_ty);
        pushed_arg_bytes += temp_slot_size(&pushed_ty);
    }

    let assignments =
        abi::build_outgoing_arg_assignments_for_target(ctx.emitter.target, &c_param_types, 0);
    let overflow_bytes = abi::materialize_outgoing_args(ctx.emitter, &assignments);
    let symbol = ctx.emitter.target.extern_symbol(&decl.name);
    abi::emit_call_label(ctx.emitter, &symbol);
    abi::emit_release_temporary_stack(ctx.emitter, overflow_bytes);
    normalize_extern_return(ctx, &decl.return_php_type)?;
    release_borrowed_cstr_temps(ctx, string_arg_count, cleanup_bytes, &decl.return_php_type);
    store_if_result(ctx, inst)
}

/// Returns the extern declaration addressed by the instruction's function-name immediate.
fn extern_decl<'a>(
    ctx: &'a FunctionContext<'_>,
    inst: &Instruction,
) -> Result<&'a ExternDecl> {
    let data = expect_data(inst)?;
    let name = ctx.function_name_data(data)?;
    let key = crate::names::php_symbol_key(name.trim_start_matches('\\'));
    ctx.module
        .extern_decls
        .iter()
        .find(|decl| crate::names::php_symbol_key(decl.name.trim_start_matches('\\')) == key)
        .ok_or_else(|| CodegenIrError::unsupported(format!("unknown extern function {}", name)))
}

/// Rejects extern features whose ABI cleanup or trampoline lowering is not implemented yet.
fn validate_extern_shape(decl: &ExternDecl) -> Result<()> {
    for param in &decl.params {
        validate_supported_extern_type(&decl.name, &param.php_type, "parameter")?;
    }
    validate_supported_extern_type(&decl.name, &decl.return_php_type, "return")
}

/// Validates one extern-facing PHP type against the scalar subset this module supports.
fn validate_supported_extern_type(name: &str, ty: &PhpType, position: &str) -> Result<()> {
    match ty.codegen_repr() {
        PhpType::Int | PhpType::Bool | PhpType::Float | PhpType::Str | PhpType::Void
        | PhpType::Pointer(_) => Ok(()),
        other => Err(CodegenIrError::unsupported(format!(
            "extern {} {} type {:?}",
            name,
            position,
            other
        ))),
    }
}

/// Returns the C ABI type for an extern parameter after PHP-specific conversion.
fn c_abi_param_type(param: &ExternParamDecl) -> PhpType {
    match param.php_type.codegen_repr() {
        PhpType::Str => PhpType::Pointer(None),
        other => other,
    }
}

/// Returns the temporary stack bytes used by one pre-materialized extern argument.
fn temp_slot_size(ty: &PhpType) -> usize {
    if matches!(ty.codegen_repr(), PhpType::Void | PhpType::Never) {
        0
    } else {
        16
    }
}

/// Loads and coerces an SSA value into the ABI result register expected by an extern parameter.
fn materialize_extern_arg(
    ctx: &mut FunctionContext<'_>,
    value: ValueId,
    param: &ExternParamDecl,
) -> Result<PhpType> {
    let target_ty = param.php_type.codegen_repr();
    let actual_ty = ctx.value_php_type(value)?;
    match (&target_ty, actual_ty.codegen_repr()) {
        (PhpType::Pointer(_), PhpType::Void) => {
            abi::emit_load_int_immediate(ctx.emitter, abi::int_result_reg(ctx.emitter), 0);
        }
        (PhpType::Str, PhpType::Str) => {
            ctx.load_value_to_result(value)?;
            abi::emit_call_label(ctx.emitter, "__rt_str_to_cstr");
            return Ok(PhpType::Pointer(None));
        }
        (PhpType::Float, PhpType::Int | PhpType::Bool) => {
            ctx.load_value_to_result(value)?;
            abi::emit_int_result_to_float_result(ctx.emitter);
        }
        (PhpType::Int, PhpType::Bool) | (PhpType::Bool, PhpType::Int) => {
            ctx.load_value_to_result(value)?;
        }
        (expected, actual) if extern_codegen_types_match(expected, &actual) => {
            ctx.load_value_to_result(value)?;
        }
        (expected, actual) => {
            return Err(CodegenIrError::unsupported(format!(
                "extern parameter ${} expects {:?}, got {:?}",
                param.name,
                expected,
                actual
            )))
        }
    }
    Ok(target_ty)
}

/// Returns true when two scalar extern ABI types can be passed without extra conversion.
fn extern_codegen_types_match(expected: &PhpType, actual: &PhpType) -> bool {
    match (expected, actual) {
        (PhpType::Pointer(_), PhpType::Pointer(_)) => true,
        _ => expected == actual,
    }
}

/// Normalizes extern return registers before storing the EIR result.
fn normalize_extern_return(ctx: &mut FunctionContext<'_>, return_ty: &PhpType) -> Result<()> {
    match return_ty.codegen_repr() {
        PhpType::Void => Ok(()),
        PhpType::Int => {
            emit_sign_extend_i32_result(ctx);
            Ok(())
        }
        PhpType::Bool | PhpType::Float | PhpType::Pointer(_) => Ok(()),
        PhpType::Str => {
            abi::emit_call_label(ctx.emitter, "__rt_cstr_to_str");
            Ok(())
        }
        other => Err(CodegenIrError::unsupported(format!(
            "extern return type {:?}",
            other
        ))),
    }
}

/// Releases call-scoped C-string argument copies after preserving the extern return value.
fn release_borrowed_cstr_temps(
    ctx: &mut FunctionContext<'_>,
    string_arg_count: usize,
    cleanup_bytes: usize,
    return_ty: &PhpType,
) {
    if string_arg_count == 0 {
        return;
    }
    let saved_return_bytes = push_ffi_return_value(ctx, return_ty);
    for idx in 0..string_arg_count {
        abi::emit_load_temporary_stack_slot(
            ctx.emitter,
            abi::int_result_reg(ctx.emitter),
            saved_return_bytes + idx * 16,
        );
        abi::emit_call_label(ctx.emitter, "__rt_heap_free");
    }
    pop_ffi_return_value(ctx, return_ty);
    abi::emit_release_temporary_stack(ctx.emitter, cleanup_bytes);
}

/// Pushes the current extern return value while borrowed C-string args are freed.
fn push_ffi_return_value(ctx: &mut FunctionContext<'_>, return_ty: &PhpType) -> usize {
    match return_ty.codegen_repr() {
        PhpType::Void | PhpType::Never => 0,
        PhpType::Float => {
            abi::emit_push_float_reg(ctx.emitter, abi::float_result_reg(ctx.emitter));
            16
        }
        PhpType::Str => {
            let (ptr_reg, len_reg) = abi::string_result_regs(ctx.emitter);
            abi::emit_push_reg_pair(ctx.emitter, ptr_reg, len_reg);
            16
        }
        _ => {
            abi::emit_push_reg(ctx.emitter, abi::int_result_reg(ctx.emitter));
            16
        }
    }
}

/// Restores a return value preserved by `push_ffi_return_value`.
fn pop_ffi_return_value(ctx: &mut FunctionContext<'_>, return_ty: &PhpType) {
    match return_ty.codegen_repr() {
        PhpType::Void | PhpType::Never => {}
        PhpType::Float => {
            abi::emit_pop_float_reg(ctx.emitter, abi::float_result_reg(ctx.emitter));
        }
        PhpType::Str => {
            let (ptr_reg, len_reg) = abi::string_result_regs(ctx.emitter);
            abi::emit_pop_reg_pair(ctx.emitter, ptr_reg, len_reg);
        }
        _ => {
            abi::emit_pop_reg(ctx.emitter, abi::int_result_reg(ctx.emitter));
        }
    }
}

/// Sign-extends a C `int` return into the target integer result register.
fn emit_sign_extend_i32_result(ctx: &mut FunctionContext<'_>) {
    match ctx.emitter.target.arch {
        Arch::AArch64 => {
            ctx.emitter.instruction("sxtw x0, w0");                             // sign-extend the C int return into PHP's 64-bit integer result
        }
        Arch::X86_64 => {
            ctx.emitter.instruction("movsxd rax, eax");                         // sign-extend the C int return into PHP's 64-bit integer result
        }
    }
}
