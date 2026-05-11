//! Purpose:
//! Emits PHP `fgetc` stream builtin calls.
//! Reads exactly one byte from a stream resource through the runtime helper.
//!
//! Called from:
//! - `crate::codegen::builtins::io::emit()`.
//!
//! Key details:
//! - The runtime helper tail-calls `__rt_fread` with length = 1; the empty
//!   string is returned at EOF (length 0).

use crate::codegen::context::Context;
use crate::codegen::data_section::DataSection;
use crate::codegen::emit::Emitter;
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
    emitter.comment("fgetc()");
    emit_stream_fd_arg("fgetc", &args[0], emitter, ctx, data);
    if emitter.target.arch == Arch::X86_64 {
        emitter.instruction("mov rdi, rax");                                    // move the file descriptor into the first SysV fread helper argument register
    }
    abi::emit_call_label(emitter, "__rt_fgetc");                                // call the runtime helper that reads exactly one byte (empty string at EOF)
    Some(PhpType::Str)
}
