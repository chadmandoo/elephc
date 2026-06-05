//! Purpose:
//! Lowers SPL object-introspection builtins for the EIR backend.
//! Handles stable object ids and object hashes using the concrete heap pointer.
//!
//! Called from:
//! - `crate::codegen_ir::lower_inst::builtins::lower_builtin_call()`.
//!
//! Key details:
//! - The legacy backend exposes the object pointer as a process-stable identity.
//!   `spl_object_hash()` stringifies that same identity with the shared itoa helper.

use crate::codegen::abi;
use crate::codegen::platform::Arch;
use crate::codegen_ir::{CodegenIrError, Result};
use crate::ir::Instruction;
use crate::types::PhpType;

use super::super::super::context::FunctionContext;
use super::{expect_operand, store_if_result};

const EXTS_PTR_SYMBOL: &str = "_spl_autoload_exts_ptr";
const EXTS_LEN_SYMBOL: &str = "_spl_autoload_exts_len";
const NULL_SENTINEL: i64 = 0x7fff_ffff_ffff_fffe;

const SPL_CLASS_NAMES: &[&str] = &[
    "AppendIterator",
    "ArrayAccess",
    "ArrayIterator",
    "ArrayObject",
    "BadFunctionCallException",
    "BadMethodCallException",
    "CachingIterator",
    "CallbackFilterIterator",
    "Countable",
    "DomainException",
    "DirectoryIterator",
    "EmptyIterator",
    "Error",
    "Exception",
    "FilterIterator",
    "FilesystemIterator",
    "GlobIterator",
    "InfiniteIterator",
    "InvalidArgumentException",
    "Iterator",
    "IteratorAggregate",
    "IteratorIterator",
    "JsonSerializable",
    "LengthException",
    "LimitIterator",
    "LogicException",
    "MultipleIterator",
    "NoRewindIterator",
    "OuterIterator",
    "OutOfBoundsException",
    "OutOfRangeException",
    "OverflowException",
    "ParentIterator",
    "RangeException",
    "RecursiveArrayIterator",
    "RecursiveCachingIterator",
    "RecursiveCallbackFilterIterator",
    "RecursiveDirectoryIterator",
    "RecursiveFilterIterator",
    "RecursiveIterator",
    "RecursiveIteratorIterator",
    "RecursiveRegexIterator",
    "RegexIterator",
    "RuntimeException",
    "SeekableIterator",
    "SplDoublyLinkedList",
    "SplFixedArray",
    "SplFileInfo",
    "SplFileObject",
    "SplObserver",
    "SplQueue",
    "SplStack",
    "SplSubject",
    "SplTempFileObject",
    "Stringable",
    "Throwable",
    "Traversable",
    "TypeError",
    "UnderflowException",
    "UnexpectedValueException",
    "ValueError",
];

/// Lowers autoload registration stubs by preserving arg effects and returning true.
pub(super) fn lower_spl_autoload_bool(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    name: &str,
) -> Result<()> {
    match name {
        "spl_autoload_register" => super::ensure_arg_count_between(inst, name, 0, 3)?,
        "spl_autoload_unregister" => super::ensure_arg_count(inst, name, 1)?,
        _ => return Err(CodegenIrError::unsupported(format!("autoload bool stub {}", name))),
    }
    emit_args_for_side_effects(ctx, inst)?;
    abi::emit_load_int_immediate(ctx.emitter, abi::int_result_reg(ctx.emitter), 1);
    store_if_result(ctx, inst)
}

/// Lowers no-op autoload calls by preserving arg effects and returning PHP null if used.
pub(super) fn lower_spl_autoload_void(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    name: &str,
) -> Result<()> {
    match name {
        "spl_autoload_call" => super::ensure_arg_count(inst, name, 1)?,
        "spl_autoload" => super::ensure_arg_count_between(inst, name, 1, 2)?,
        _ => return Err(CodegenIrError::unsupported(format!("autoload void stub {}", name))),
    }
    emit_args_for_side_effects(ctx, inst)?;
    emit_null_result(ctx);
    store_if_result(ctx, inst)
}

/// Lowers `spl_autoload_functions()` to an indexed array of AOT rule placeholders.
pub(super) fn lower_spl_autoload_functions(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    super::ensure_arg_count(inst, "spl_autoload_functions", 0)?;
    let rule_count = crate::codegen::autoload_rule_count();
    emit_int_array(ctx, rule_count.max(1), |ctx| emit_autoload_function_placeholders(ctx, rule_count))?;
    store_if_result(ctx, inst)
}

/// Lowers `spl_autoload_extensions()` against the legacy mutable extension globals.
pub(super) fn lower_spl_autoload_extensions(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    super::ensure_arg_count_between(inst, "spl_autoload_extensions", 0, 1)?;
    if inst.operands.is_empty() {
        emit_extensions_read(ctx);
        return store_if_result(ctx, inst);
    }

    let value = expect_operand(inst, 0)?;
    match ctx.value_php_type(value)?.codegen_repr() {
        PhpType::Str => emit_extensions_write(ctx, value)?,
        PhpType::Void => {
            ctx.load_value_to_result(value)?;
            emit_extensions_read(ctx);
        }
        other => {
            return Err(CodegenIrError::unsupported(format!(
                "spl_autoload_extensions for PHP type {:?}",
                other
            )));
        }
    }
    store_if_result(ctx, inst)
}

/// Lowers `spl_classes()` to the static compiler-shipped SPL/core type snapshot.
pub(super) fn lower_spl_classes(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    super::ensure_arg_count(inst, "spl_classes", 0)?;
    emit_string_array(ctx, SPL_CLASS_NAMES)?;
    store_if_result(ctx, inst)
}

/// Lowers `spl_object_id(object)` by returning the loaded object pointer as an integer.
pub(super) fn lower_spl_object_id(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    super::ensure_arg_count(inst, "spl_object_id", 1)?;
    load_object_operand(ctx, inst, "spl_object_id")?;
    store_if_result(ctx, inst)
}

/// Lowers `spl_object_hash(object)` by formatting the loaded object pointer as a string.
pub(super) fn lower_spl_object_hash(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    super::ensure_arg_count(inst, "spl_object_hash", 1)?;
    load_object_operand(ctx, inst, "spl_object_hash")?;
    abi::emit_call_label(ctx.emitter, "__rt_itoa");
    store_if_result(ctx, inst)
}

/// Loads the single object operand into the canonical integer result register.
fn load_object_operand(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    name: &str,
) -> Result<()> {
    let value = expect_operand(inst, 0)?;
    let ty = ctx.load_value_to_result(value)?;
    match ty {
        PhpType::Object(_) => Ok(()),
        other => Err(CodegenIrError::unsupported(format!(
            "{} for PHP type {:?}",
            name,
            other
        ))),
    }
}

/// Evaluates lowered operands in source order and discards their results.
fn emit_args_for_side_effects(ctx: &mut FunctionContext<'_>, inst: &Instruction) -> Result<()> {
    for operand in &inst.operands {
        ctx.load_value_to_result(*operand)?;
    }
    Ok(())
}

/// Emits the shared runtime null sentinel into the integer result register.
fn emit_null_result(ctx: &mut FunctionContext<'_>) {
    abi::emit_load_int_immediate(
        ctx.emitter,
        abi::int_result_reg(ctx.emitter),
        NULL_SENTINEL,
    );
}

/// Allocates an indexed integer array and lets `fill` append values.
fn emit_int_array<F>(
    ctx: &mut FunctionContext<'_>,
    capacity: usize,
    fill: F,
) -> Result<()>
where
    F: FnOnce(&mut FunctionContext<'_>) -> Result<()>,
{
    match ctx.emitter.target.arch {
        Arch::AArch64 => {
            abi::emit_load_int_immediate(ctx.emitter, "x0", capacity as i64);
            abi::emit_load_int_immediate(ctx.emitter, "x1", 8);
        }
        Arch::X86_64 => {
            abi::emit_load_int_immediate(ctx.emitter, "rdi", capacity as i64);
            abi::emit_load_int_immediate(ctx.emitter, "rsi", 8);
        }
    }
    abi::emit_call_label(ctx.emitter, "__rt_array_new");
    fill(ctx)
}

/// Appends placeholder rule indexes to the current autoload-functions array.
fn emit_autoload_function_placeholders(
    ctx: &mut FunctionContext<'_>,
    rule_count: usize,
) -> Result<()> {
    if rule_count == 0 {
        return Ok(());
    }
    match ctx.emitter.target.arch {
        Arch::AArch64 => emit_autoload_function_placeholders_aarch64(ctx, rule_count),
        Arch::X86_64 => emit_autoload_function_placeholders_x86_64(ctx, rule_count),
    }
    Ok(())
}

/// Appends placeholder autoload indexes on AArch64.
fn emit_autoload_function_placeholders_aarch64(
    ctx: &mut FunctionContext<'_>,
    rule_count: usize,
) {
    ctx.emitter.instruction("str x0, [sp, #-16]!");                             // park the autoload-functions array while appending rule indexes
    for index in 0..rule_count {
        ctx.emitter.instruction("ldr x0, [sp]");                                // reload the autoload-functions array for this append
        abi::emit_load_int_immediate(ctx.emitter, "x1", index as i64);
        abi::emit_call_label(ctx.emitter, "__rt_array_push_int");
        ctx.emitter.instruction("str x0, [sp]");                                // preserve the possibly-grown array pointer for the next append
    }
    ctx.emitter.instruction("ldr x0, [sp], #16");                               // restore the final autoload-functions array as the result
}

/// Appends placeholder autoload indexes on x86_64.
fn emit_autoload_function_placeholders_x86_64(
    ctx: &mut FunctionContext<'_>,
    rule_count: usize,
) {
    ctx.emitter.instruction("push rax");                                        // park the autoload-functions array while appending rule indexes
    ctx.emitter.instruction("sub rsp, 8");                                      // keep stack alignment stable across append helper calls
    for index in 0..rule_count {
        ctx.emitter.instruction("mov rdi, QWORD PTR [rsp + 8]");                // reload the autoload-functions array for this append
        abi::emit_load_int_immediate(ctx.emitter, "rsi", index as i64);
        abi::emit_call_label(ctx.emitter, "__rt_array_push_int");
        ctx.emitter.instruction("mov QWORD PTR [rsp + 8], rax");                // preserve the possibly-grown array pointer for the next append
    }
    ctx.emitter.instruction("add rsp, 8");                                      // drop the temporary alignment slot
    ctx.emitter.instruction("pop rax");                                         // restore the final autoload-functions array as the result
}

/// Loads the current autoload extension string from runtime globals.
fn emit_extensions_read(ctx: &mut FunctionContext<'_>) {
    let (ptr_reg, len_reg) = abi::string_result_regs(ctx.emitter);
    abi::emit_load_symbol_to_reg(ctx.emitter, ptr_reg, EXTS_PTR_SYMBOL, 0);
    abi::emit_load_symbol_to_reg(ctx.emitter, len_reg, EXTS_LEN_SYMBOL, 0);
}

/// Writes a new autoload extension string and returns the previous value.
fn emit_extensions_write(ctx: &mut FunctionContext<'_>, value: crate::ir::ValueId) -> Result<()> {
    let (ptr_reg, len_reg) = abi::string_result_regs(ctx.emitter);
    ctx.load_string_value_to_regs(value, ptr_reg, len_reg)?;
    abi::emit_push_reg_pair(ctx.emitter, ptr_reg, len_reg);
    emit_extensions_read(ctx);
    let new_ptr = abi::secondary_scratch_reg(ctx.emitter);
    let new_len = abi::tertiary_scratch_reg(ctx.emitter);
    abi::emit_pop_reg_pair(ctx.emitter, new_ptr, new_len);
    abi::emit_store_reg_to_symbol(ctx.emitter, new_ptr, EXTS_PTR_SYMBOL, 0);
    abi::emit_store_reg_to_symbol(ctx.emitter, new_len, EXTS_LEN_SYMBOL, 0);
    Ok(())
}

/// Allocates an indexed string array and appends all static names.
fn emit_string_array(ctx: &mut FunctionContext<'_>, names: &[&str]) -> Result<()> {
    let capacity = names.len().max(1);
    match ctx.emitter.target.arch {
        Arch::AArch64 => {
            abi::emit_load_int_immediate(ctx.emitter, "x0", capacity as i64);
            abi::emit_load_int_immediate(ctx.emitter, "x1", 16);
        }
        Arch::X86_64 => {
            abi::emit_load_int_immediate(ctx.emitter, "rdi", capacity as i64);
            abi::emit_load_int_immediate(ctx.emitter, "rsi", 16);
        }
    }
    abi::emit_call_label(ctx.emitter, "__rt_array_new");
    match ctx.emitter.target.arch {
        Arch::AArch64 => emit_string_array_fill_aarch64(ctx, names),
        Arch::X86_64 => emit_string_array_fill_x86_64(ctx, names),
    }
    Ok(())
}

/// Appends static string names to the current result array on AArch64.
fn emit_string_array_fill_aarch64(ctx: &mut FunctionContext<'_>, names: &[&str]) {
    ctx.emitter.instruction("str x0, [sp, #-16]!");                             // park the string array while appending names
    for name in names {
        let (label, len) = ctx.data.add_string(name.as_bytes());
        ctx.emitter.instruction("ldr x0, [sp]");                                // reload the string array for this append
        abi::emit_symbol_address(ctx.emitter, "x1", &label);
        abi::emit_load_int_immediate(ctx.emitter, "x2", len as i64);
        abi::emit_call_label(ctx.emitter, "__rt_array_push_str");
        ctx.emitter.instruction("str x0, [sp]");                                // preserve the possibly-grown string array for the next append
    }
    ctx.emitter.instruction("ldr x0, [sp], #16");                               // restore the final string array as the result
}

/// Appends static string names to the current result array on x86_64.
fn emit_string_array_fill_x86_64(ctx: &mut FunctionContext<'_>, names: &[&str]) {
    ctx.emitter.instruction("push rax");                                        // park the string array while appending names
    ctx.emitter.instruction("sub rsp, 8");                                      // keep stack alignment stable across append helper calls
    for name in names {
        let (label, len) = ctx.data.add_string(name.as_bytes());
        ctx.emitter.instruction("mov rdi, QWORD PTR [rsp + 8]");                // reload the string array for this append
        abi::emit_symbol_address(ctx.emitter, "rsi", &label);
        abi::emit_load_int_immediate(ctx.emitter, "rdx", len as i64);
        abi::emit_call_label(ctx.emitter, "__rt_array_push_str");
        ctx.emitter.instruction("mov QWORD PTR [rsp + 8], rax");                // preserve the possibly-grown string array for the next append
    }
    ctx.emitter.instruction("add rsp, 8");                                      // drop the temporary alignment slot
    ctx.emitter.instruction("pop rax");                                         // restore the final string array as the result
}
