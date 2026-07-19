//! Purpose:
//! Lowers the PHP output-buffering (`ob_*`) builtins for the EIR backend by
//! marshalling into the `__rt_ob_*` runtime helpers and boxing their results.
//!
//! Called from:
//! - The per-builtin `lower` hooks in `crate::builtins::io::ob_*`, via
//!   `crate::codegen::lower_inst::builtins::lower_builtin_call()`.
//!
//! Key details:
//! - `ob_start` ignores its already-evaluated operands: the checker rejects a
//!   non-null handler callback at compile time, and `chunk_size`/`flags` have no
//!   effect because elephc buffers are unchunked with the standard flags.
//! - String-or-false results (`ob_get_contents`/`ob_get_clean`/`ob_get_flush`)
//!   reuse `io::box_owned_string_or_false_result` on the null-pointer failure
//!   convention; `ob_get_length` boxes its -1 sentinel to PHP `false`.
//! - `ob_get_status` boxes the raw hash pointer as a Mixed associative array
//!   (runtime tag 5), mirroring `getdate`/`localtime`; `ob_list_handlers`
//!   returns the raw string-array handle like `hash_algos`.

use crate::codegen::abi;
use crate::codegen::platform::Arch;
use crate::codegen::{CodegenIrError, Result};
use crate::ir::{Instruction, ValueId};
use crate::types::PhpType;

use super::super::super::context::FunctionContext;
use super::{load_value_to_first_int_arg, store_if_result};

/// Lowers `ob_start([$callback[, $chunk_size[, $flags]]])` to `__rt_ob_start`.
///
/// The operands were already evaluated as separate EIR instructions (preserving
/// side effects) and are intentionally unused: the checker only admits a `null`
/// callback, and chunk size/flags are inert in elephc's buffer model.
pub(crate) fn lower_ob_start(ctx: &mut FunctionContext<'_>, inst: &Instruction) -> Result<()> {
    ensure_arg_count_between(inst, "ob_start", 0, 3)?;
    abi::emit_call_label(ctx.emitter, "__rt_ob_start");
    store_if_result(ctx, inst)
}

/// Lowers `ob_get_contents()` and boxes the runtime string-or-false result.
pub(crate) fn lower_ob_get_contents(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    ensure_arg_count_between(inst, "ob_get_contents", 0, 0)?;
    abi::emit_call_label(ctx.emitter, "__rt_ob_contents");
    super::io::box_owned_string_or_false_result(ctx, "ob_contents");
    store_if_result(ctx, inst)
}

/// Lowers `ob_get_clean()`: capture the top buffer's contents, then discard the
/// buffer, returning the captured string (or `false` when no buffer is active).
pub(crate) fn lower_ob_get_clean(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    lower_ob_get_then_end(ctx, inst, "ob_get_clean", "__rt_ob_end_clean")
}

/// Lowers `ob_get_flush()`: capture the top buffer's contents, then flush the
/// buffer to the parent sink and pop it, returning the captured string.
pub(crate) fn lower_ob_get_flush(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    lower_ob_get_then_end(ctx, inst, "ob_get_flush", "__rt_ob_end_flush")
}

/// Shared lowering for `ob_get_clean`/`ob_get_flush`: persist the contents first,
/// run the pop helper (a no-op returning 0 when no buffer is active), and box the
/// persisted string-or-false pair. `name` is used for diagnostics only.
fn lower_ob_get_then_end(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    name: &str,
    end_symbol: &str,
) -> Result<()> {
    ensure_arg_count_between(inst, name, 0, 0)?;
    abi::emit_call_label(ctx.emitter, "__rt_ob_contents");
    match ctx.emitter.target.arch {
        Arch::AArch64 => {
            abi::emit_push_reg_pair(ctx.emitter, "x1", "x2");
            abi::emit_call_label(ctx.emitter, end_symbol);
            abi::emit_pop_reg_pair(ctx.emitter, "x1", "x2");
        }
        Arch::X86_64 => {
            abi::emit_push_reg_pair(ctx.emitter, "rax", "rdx");
            abi::emit_call_label(ctx.emitter, end_symbol);
            abi::emit_pop_reg_pair(ctx.emitter, "rax", "rdx");
        }
    }
    super::io::box_owned_string_or_false_result(ctx, name);
    store_if_result(ctx, inst)
}

/// Lowers `ob_get_length()` and boxes the length-or-false result (the runtime
/// returns -1 when no buffer is active).
pub(crate) fn lower_ob_get_length(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    ensure_arg_count_between(inst, "ob_get_length", 0, 0)?;
    abi::emit_call_label(ctx.emitter, "__rt_ob_length");
    box_int_or_false_result(ctx, "ob_length");
    store_if_result(ctx, inst)
}

/// Lowers `ob_get_level()` to the plain integer nesting-depth query.
pub(crate) fn lower_ob_get_level(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    ensure_arg_count_between(inst, "ob_get_level", 0, 0)?;
    abi::emit_call_label(ctx.emitter, "__rt_ob_level");
    store_if_result(ctx, inst)
}

/// Lowers `ob_clean()` to the truncate-top-buffer helper (bool result).
pub(crate) fn lower_ob_clean(ctx: &mut FunctionContext<'_>, inst: &Instruction) -> Result<()> {
    lower_ob_bool_query(ctx, inst, "ob_clean", "__rt_ob_clean")
}

/// Lowers `ob_end_clean()` to the discard-and-pop helper (bool result).
pub(crate) fn lower_ob_end_clean(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    lower_ob_bool_query(ctx, inst, "ob_end_clean", "__rt_ob_end_clean")
}

/// Lowers `ob_end_flush()` to the flush-and-pop helper (bool result).
pub(crate) fn lower_ob_end_flush(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    lower_ob_bool_query(ctx, inst, "ob_end_flush", "__rt_ob_end_flush")
}

/// Lowers `ob_flush()` to the flush-keep-buffer helper (bool result).
pub(crate) fn lower_ob_flush(ctx: &mut FunctionContext<'_>, inst: &Instruction) -> Result<()> {
    lower_ob_bool_query(ctx, inst, "ob_flush", "__rt_ob_flush")
}

/// Shared lowering for the zero-argument bool-returning ob_* helpers.
fn lower_ob_bool_query(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    name: &str,
    runtime_symbol: &str,
) -> Result<()> {
    ensure_arg_count_between(inst, name, 0, 0)?;
    abi::emit_call_label(ctx.emitter, runtime_symbol);
    store_if_result(ctx, inst)
}

/// Lowers `ob_implicit_flush([$enable])`: store the flag (semantically inert in
/// elephc — terminal writes are unbuffered syscalls) and return `true` like PHP 8.
pub(crate) fn lower_ob_implicit_flush(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    ensure_arg_count_between(inst, "ob_implicit_flush", 0, 1)?;
    match inst.operands.first().copied() {
        Some(enable) => {
            resolve_integer_arg_to_result(ctx, enable, "ob_implicit_flush enable flag")?
        }
        None => abi::emit_load_int_immediate(ctx.emitter, abi::int_result_reg(ctx.emitter), 1),
    }
    abi::emit_store_reg_to_symbol(
        ctx.emitter,
        abi::int_result_reg(ctx.emitter),
        "_ob_implicit_flush",
        0,
    );
    abi::emit_load_int_immediate(ctx.emitter, abi::int_result_reg(ctx.emitter), 1);
    store_if_result(ctx, inst)
}

/// Lowers `ob_get_status([$full_status])` through the status-hash runtime helper
/// and boxes the hash pointer as a Mixed associative array.
pub(crate) fn lower_ob_get_status(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    ensure_arg_count_between(inst, "ob_get_status", 0, 1)?;
    match inst.operands.first().copied() {
        Some(flag) => resolve_integer_arg_to_result(ctx, flag, "ob_get_status full_status flag")?,
        None => abi::emit_load_int_immediate(ctx.emitter, abi::int_result_reg(ctx.emitter), 0),
    }
    abi::emit_call_label(ctx.emitter, "__rt_ob_get_status");
    emit_box_hash_pointer_as_assoc_mixed(ctx);
    store_if_result(ctx, inst)
}

/// Lowers `ob_list_handlers()` to the handler-name string-array helper.
pub(crate) fn lower_ob_list_handlers(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
) -> Result<()> {
    ensure_arg_count_between(inst, "ob_list_handlers", 0, 0)?;
    abi::emit_call_label(ctx.emitter, "__rt_ob_list_handlers");
    store_if_result(ctx, inst)
}

/// Boxes a raw integer-or-sentinel result into PHP `int|false` Mixed form, where
/// -1 in the integer result register marks the failure branch.
fn box_int_or_false_result(ctx: &mut FunctionContext<'_>, label_prefix: &str) {
    let false_label = ctx.next_label(&format!("{}_false", label_prefix));
    let done_label = ctx.next_label(&format!("{}_done", label_prefix));
    match ctx.emitter.target.arch {
        Arch::AArch64 => {
            ctx.emitter.instruction("cmn x0, #1");                              // compare the raw result against the -1 failure sentinel
            ctx.emitter.instruction(&format!("b.eq {}", false_label));          // box PHP false when no buffer was active
            ctx.emitter.instruction("mov x1, x0");                              // pass the length as the Mixed integer payload
            ctx.emitter.instruction("mov x2, #0");                              // integer Mixed payloads do not use a high word
            ctx.emitter.instruction("mov x0, #0");                              // select runtime tag 0 for an integer Mixed value
            abi::emit_call_label(ctx.emitter, "__rt_mixed_from_value");
            ctx.emitter.instruction(&format!("b {}", done_label));              // skip false boxing after building the integer result
            ctx.emitter.label(&false_label);
            ctx.emitter.instruction("mov x1, #0");                              // use zero as the false payload for the Mixed bool box
            ctx.emitter.instruction("mov x2, #0");                              // clear the unused high payload word for bool Mixed boxes
            ctx.emitter.instruction("mov x0, #3");                              // select runtime tag 3 for a boolean false Mixed value
            abi::emit_call_label(ctx.emitter, "__rt_mixed_from_value");
            ctx.emitter.label(&done_label);
        }
        Arch::X86_64 => {
            ctx.emitter.instruction("cmp rax, -1");                             // compare the raw result against the -1 failure sentinel
            ctx.emitter.instruction(&format!("je {}", false_label));            // box PHP false when no buffer was active
            ctx.emitter.instruction("mov rdi, rax");                            // pass the length as the Mixed integer payload
            ctx.emitter.instruction("xor esi, esi");                            // integer Mixed payloads do not use a high word
            ctx.emitter.instruction("xor eax, eax");                            // select runtime tag 0 for an integer Mixed value
            abi::emit_call_label(ctx.emitter, "__rt_mixed_from_value");
            ctx.emitter.instruction(&format!("jmp {}", done_label));            // skip false boxing after building the integer result
            ctx.emitter.label(&false_label);
            ctx.emitter.instruction("xor edi, edi");                            // use zero as the false payload for the Mixed bool box
            ctx.emitter.instruction("xor esi, esi");                            // clear the unused high payload word for bool Mixed boxes
            ctx.emitter.instruction("mov eax, 3");                              // select runtime tag 3 for a boolean false Mixed value
            abi::emit_call_label(ctx.emitter, "__rt_mixed_from_value");
            ctx.emitter.label(&done_label);
        }
    }
}

/// Boxes the raw associative-array hash pointer in the integer result register
/// into a `Mixed` cell (runtime tag 5), mirroring `getdate`/`localtime`/`stat`.
fn emit_box_hash_pointer_as_assoc_mixed(ctx: &mut FunctionContext<'_>) {
    match ctx.emitter.target.arch {
        Arch::AArch64 => {
            ctx.emitter.instruction("mov x1, x0");                              // Mixed payload low word = hash pointer
            ctx.emitter.instruction("mov x2, #0");                              // associative-array payloads do not use the high word
            ctx.emitter.instruction("mov x0, #5");                              // runtime tag 5 = associative array
            abi::emit_call_label(ctx.emitter, "__rt_mixed_from_value");
        }
        Arch::X86_64 => {
            ctx.emitter.instruction("mov rdi, rax");                            // Mixed payload low word = hash pointer
            ctx.emitter.instruction("xor esi, esi");                            // associative-array payloads do not use the high word
            ctx.emitter.instruction("mov rax, 5");                              // runtime tag 5 = associative array
            abi::emit_call_label(ctx.emitter, "__rt_mixed_from_value");
        }
    }
}

/// Resolves one boolean/integer argument into the canonical integer result
/// register, unboxing a boxed `Mixed`/`Union` value through `__rt_mixed_cast_int`.
fn resolve_integer_arg_to_result(
    ctx: &mut FunctionContext<'_>,
    value: ValueId,
    context: &str,
) -> Result<()> {
    match ctx.value_php_type(value)?.codegen_repr() {
        PhpType::Int | PhpType::Bool => {
            ctx.load_value_to_result(value)?;
        }
        PhpType::Mixed | PhpType::Union(_) => {
            load_value_to_first_int_arg(ctx, value)?;
            abi::emit_call_label(ctx.emitter, "__rt_mixed_cast_int");
        }
        ty => {
            return Err(CodegenIrError::unsupported(format!(
                "{} for PHP type {:?}",
                context, ty
            )));
        }
    }
    Ok(())
}

/// Verifies that the builtin call has between the expected lowered operand counts.
fn ensure_arg_count_between(inst: &Instruction, name: &str, min: usize, max: usize) -> Result<()> {
    if (min..=max).contains(&inst.operands.len()) {
        return Ok(());
    }
    Err(CodegenIrError::invalid_module(format!(
        "{} expected {} to {} args, got {}",
        name,
        min,
        max,
        inst.operands.len()
    )))
}
