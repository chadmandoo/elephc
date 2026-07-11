//! Purpose:
//! Home of the PHP `getmypid` builtin: its declaration and lowering.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `getmypid` is a pure-data builtin whose return type
//!   (`Int`) is fully determined by its declaration. The registry common path
//!   enforces the zero-argument arity before falling back to `returns`.

use crate::codegen::context::FunctionContext;
use crate::codegen::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "getmypid",
    area: System,
    params: [],
    returns: Int,
    lower: lower,
    summary: "Returns the process ID of the current PHP process.",
    php_manual: "https://www.php.net/manual/en/function.getmypid.php",
}

/// Lowers a `getmypid` call by dispatching to the shared system emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen::lower_inst::builtins::system::lower_getmypid(ctx, inst)
}
