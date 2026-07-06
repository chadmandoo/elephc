//! Purpose:
//! Home of the PHP `restore_error_handler` builtin: its declaration and lowering.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - Accepted NO-OP pairing `set_error_handler` (see its module note): with no
//!   handler stack to pop, restoring always "succeeds". PHP returns true.

use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "restore_error_handler",
    area: System,
    params: [],
    returns: Bool,
    lower: lower,
    summary: "Restores the previous error handler (accepted no-op; always true).",
    php_manual: "https://www.php.net/manual/en/function.restore-error-handler.php",
}

/// Lowers `restore_error_handler` to constant true.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::system::lower_restore_error_handler(ctx, inst)
}
