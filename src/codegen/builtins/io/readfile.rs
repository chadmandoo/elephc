//! Purpose:
//! Emits PHP `readfile` builtin calls.
//! Streams a path to stdout through the runtime helper and returns bytes copied.
//!
//! Called from:
//! - `crate::codegen::builtins::io::emit()`.
//!
//! Key details:
//! - Returns 0 on failure (file missing, permission denied) — PHP returns false
//!   in that case, but elephc keeps the simpler int-only contract.

use crate::codegen::context::Context;
use crate::codegen::data_section::DataSection;
use crate::codegen::emit::Emitter;
use crate::codegen::expr::emit_expr;
use crate::codegen::abi;
use crate::parser::ast::Expr;
use crate::types::PhpType;

pub fn emit(
    _name: &str,
    args: &[Expr],
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> Option<PhpType> {
    emitter.comment("readfile()");
    emit_expr(&args[0], emitter, ctx, data);
    abi::emit_call_label(emitter, "__rt_readfile");                             // call the runtime helper that opens path + streams contents to stdout
    Some(PhpType::Int)
}
