//! Purpose:
//! Home of the PHP `memory_get_peak_usage` builtin: its declaration and lowering.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `memory_get_peak_usage` is a pure-data builtin whose return
//!   type (`Int`) is fully determined by its declaration. The optional `real_usage` flag is
//!   accepted and ignored — the lowering reads elephc's peak heap high-watermark (`_gc_peak`).

use crate::builtins::spec::DefaultSpec;
use crate::codegen::context::FunctionContext;
use crate::codegen::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "memory_get_peak_usage",
    area: System,
    params: [real_usage: Bool = DefaultSpec::Bool(false)],
    returns: Int,
    lower: lower,
    summary: "Returns the peak heap memory allocated to the process during its run, in bytes.",
    php_manual: "https://www.php.net/manual/en/function.memory-get-peak-usage.php",
}

/// Lowers a `memory_get_peak_usage` call by dispatching to the shared system emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen::lower_inst::builtins::system::lower_memory_get_peak_usage(ctx, inst)
}
