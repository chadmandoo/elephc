//! Purpose:
//! Home of the PHP `set_error_handler` builtin: its declaration and lowering.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - Accepted NO-OP: elephc natives have no PHP error/warning machinery, so there
//!   is nothing to route to an installed handler. The dominant functional-core
//!   idiom installs a swallow-everything handler purely to SILENCE an interpreter
//!   warning around a failable call (UploadedFile::silentFopen) — natively the
//!   call just returns its failure value with no warning to suppress, so
//!   accepting and ignoring the handler is behaviour-preserving.
//! - The callback argument is still evaluated and type-checked (a closure literal
//!   must compile); the return is PHP's "previous handler", which under no-op
//!   semantics is always null ("none was ever set").
//! - DOCUMENTED LIMIT: a program whose handler expects to OBSERVE calls will not
//!   see any — that is a Tier-C shell-boundary concern, not a core one.

use crate::builtins::spec::DefaultSpec;
use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "set_error_handler",
    area: System,
    params: [callback: Mixed, error_levels: Int = DefaultSpec::Int(32767)],
    returns: Null,
    lower: lower,
    summary: "Registers an error handler (accepted no-op; natives emit no PHP warnings).",
    php_manual: "https://www.php.net/manual/en/function.set-error-handler.php",
}

/// Lowers `set_error_handler` to the null sentinel ("no previous handler").
/// The operands were already evaluated for side effects by the argument lowering.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::system::lower_error_handler_noop(ctx, inst)
}
