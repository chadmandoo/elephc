use crate::codegen::context::Context;
use crate::codegen::data_section::DataSection;
use crate::codegen::emit::Emitter;
use crate::codegen::abi;
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
    emitter.comment("fdatasync()");
    emit_stream_fd_arg("fdatasync", &args[0], emitter, ctx, data);
    abi::emit_call_label(emitter, "__rt_fdatasync");                            // libc fdatasync(fd) — falls back to fsync on Darwin
    Some(PhpType::Bool)
}
