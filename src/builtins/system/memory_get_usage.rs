//! Purpose:
//! Home of the PHP `memory_get_usage` builtin: its declaration and lowering.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `memory_get_usage` is a pure-data builtin whose return type
//!   (`Int`) is fully determined by its declaration. The optional `real_usage` flag is accepted
//!   and ignored — elephc tracks a single heap footprint (`_gc_live`), which the lowering reads.

use crate::builtins::spec::DefaultSpec;
use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "memory_get_usage",
    area: System,
    params: [real_usage: Bool = DefaultSpec::Bool(false)],
    returns: Int,
    lower: lower,
    summary: "Returns the amount of heap memory currently allocated to the process, in bytes.",
    php_manual: "https://www.php.net/manual/en/function.memory-get-usage.php",
}

/// Lowers a `memory_get_usage` call by dispatching to the shared system emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::system::lower_memory_get_usage(ctx, inst)
}
