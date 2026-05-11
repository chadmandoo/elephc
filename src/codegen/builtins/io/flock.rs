//! Purpose:
//! Emits PHP `flock` advisory-locking builtin calls over runtime file handles.
//! Validates the stream argument before invoking the libc `flock` wrapper.
//!
//! Called from:
//! - `crate::codegen::builtins::io::emit()`.
//!
//! Key details:
//! - The runtime translates the PHP `LOCK_UN` value (3) to the POSIX value (8)
//!   while preserving the `LOCK_NB` flag bit.

use crate::codegen::context::Context;
use crate::codegen::data_section::DataSection;
use crate::codegen::emit::Emitter;
use crate::codegen::expr::emit_expr;
use crate::codegen::{abi, platform::Arch};
use crate::parser::ast::Expr;
use crate::types::PhpType;

use super::stream_arg::emit_stream_fd_arg;

pub fn emit(
    _name: &str,
    args: &[Expr],
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> Option<PhpType> {
    emitter.comment("flock()");
    emit_stream_fd_arg("flock", &args[0], emitter, ctx, data);
    abi::emit_push_reg(emitter, abi::int_result_reg(emitter));                  // preserve the file descriptor while the operation expression is evaluated
    emit_expr(&args[1], emitter, ctx, data);
    match emitter.target.arch {
        Arch::AArch64 => {
            emitter.instruction("mov x1, x0");                                  // move the lock operation into the second runtime argument register
            abi::emit_pop_reg(emitter, "x0");                                   // restore the file descriptor into the primary integer register
        }
        Arch::X86_64 => {
            emitter.instruction("mov rdx, rax");                                // move the lock operation into the secondary x86_64 integer argument register
            abi::emit_pop_reg(emitter, "rax");                                  // restore the file descriptor into the primary integer register
        }
    }
    abi::emit_call_label(emitter, "__rt_flock");                                // call the runtime libc flock(fd, op) wrapper that translates LOCK_UN
    Some(PhpType::Bool)
}
