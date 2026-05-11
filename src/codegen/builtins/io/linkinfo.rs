//! Purpose:
//! Emits PHP `linkinfo` builtin calls.
//! Returns the `st_dev` field of the link (or 0 on failure — PHP quirk: not `false`).
//!
//! Called from:
//! - `crate::codegen::builtins::io::emit()`.
//!
//! Key details:
//! - The runtime helper invokes libc `lstat()` and returns the 32-bit `st_dev`
//!   field on success, or 0 on failure.

use crate::codegen::abi;
use crate::codegen::context::Context;
use crate::codegen::data_section::DataSection;
use crate::codegen::emit::Emitter;
use crate::codegen::expr::emit_expr;
use crate::parser::ast::Expr;
use crate::types::PhpType;

pub fn emit(
    _name: &str,
    args: &[Expr],
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> Option<PhpType> {
    emitter.comment("linkinfo()");
    emit_expr(&args[0], emitter, ctx, data);
    abi::emit_call_label(emitter, "__rt_linkinfo");                             // libc lstat() wrapper that returns the device id
    Some(PhpType::Int)
}
