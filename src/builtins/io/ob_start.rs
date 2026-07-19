//! Purpose:
//! Home of the PHP `ob_start` builtin: its declaration and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook when present),
//!   and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` rejects a non-null `$callback`: elephc does not support user output
//! -   handlers, so only the default (null) handler is accepted.
//! - `chunk_size`/`flags` are accepted for signature parity but are inert: buffers
//! -   are unchunked and always cleanable/flushable/removable (the standard flags).
//! - `lower` is a thin wrapper over `output_buffering::lower_ob_start`.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::codegen::context::FunctionContext;
use crate::codegen::CodegenIrError;
use crate::errors::CompileError;
use crate::ir::Instruction;
use crate::parser::ast::ExprKind;
use crate::types::PhpType;

builtin! {
    name: "ob_start",
    area: Io,
    params: [
        callback: Mixed = DefaultSpec::Null,
        chunk_size: Int = DefaultSpec::Int(0),
        flags: Int = DefaultSpec::Int(112)
    ],
    returns: Bool,
    check: check,
    lower: lower,
    summary: "Turns on output buffering.",
    php_manual: "function.ob-start",
}

/// Returns `Bool`, rejecting any non-null `$callback`: user output handlers are
/// not supported by elephc, so only the default (null) handler is accepted.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    if let Some(callback) = cx.args.first() {
        if !matches!(callback.kind, ExprKind::Null) {
            return Err(CompileError::new(
                cx.span,
                "ob_start() output handler callbacks are not supported; pass null",
            ));
        }
    }
    Ok(PhpType::Bool)
}

/// Lowers an `ob_start` call by dispatching to the shared output-buffering emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen::lower_inst::builtins::output_buffering::lower_ob_start(ctx, inst)
}
