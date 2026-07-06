//! Purpose:
//! Home of the PHP `get_debug_type` builtin: its declaration and lowering.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - Statically-typed arguments never reach this lowering: the EIR frontend folds
//!   them to constant type-name strings (`lower_static_get_debug_type`). Only
//!   Mixed/Union arguments arrive here — the declared `Mixed` parameter boxes them,
//!   so `__rt_get_debug_type` always receives a heap value it can dispatch on
//!   (kind-tag adaptive; objects resolve their FQCN via `_classes_by_name`).

use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "get_debug_type",
    area: System,
    params: [value: Mixed],
    returns: Str,
    lower: lower,
    summary: "Returns the resolved PHP type name of a value.",
    php_manual: "https://www.php.net/manual/en/function.get-debug-type.php",
}

/// Lowers a `get_debug_type` call through the boxed-value runtime dispatcher.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::system::lower_get_debug_type(ctx, inst)
}
