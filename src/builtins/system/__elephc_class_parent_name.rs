//! Purpose:
//! Home of the internal `__elephc_class_parent_name` intrinsic: its declaration
//! and lowering. Compiler-synthesized; not PHP-visible.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - `internal: true` keeps it out of PHP-visible builtin name sets and
//!   `function_exists()`; it is reachable only through the synthetic
//!   `ReflectionClass::getParentClass()` body.
//! - Returns the parent class's name for a class name via the
//!   `__rt_class_parent_name` scan (`_classes_by_name` → class_id →
//!   `_class_parent_ids` → `_class_name_entries`); an unknown or parentless
//!   class yields the empty string.

use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "__elephc_class_parent_name",
    area: System,
    params: [class_name: Str],
    returns: Str,
    lower: lower,
    summary: "Returns the parent-class name recorded for a class name.",
    internal: true,
}

/// Lowers the call by dispatching to the shared unary string runtime emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::strings::lower_unary_string_runtime(
        ctx,
        inst,
        "__elephc_class_parent_name",
        "__rt_class_parent_name",
    )
}
