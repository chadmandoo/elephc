//! Purpose:
//! Home of the internal `__elephc_class_file` intrinsic: its declaration and lowering.
//! Compiler-synthesized; not PHP-visible.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - `internal: true` keeps it out of PHP-visible builtin name sets and
//!   `function_exists()`; it is reachable only through the synthetic
//!   `ReflectionClass::getFileName()` body.
//! - Returns the declaring file's canonical path for a class name via the
//!   `__rt_class_file_by_name` scan over the `_classes_by_name` table (whose
//!   file columns come from the stamped `__ELEPHC_FILE__` class constants);
//!   an unknown class yields the empty string.

use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "__elephc_class_file",
    area: System,
    params: [class_name: Str],
    returns: Str,
    lower: lower,
    summary: "Returns the declaring file path recorded for a class name.",
    internal: true,
}

/// Lowers the call by dispatching to the shared unary string runtime emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::strings::lower_unary_string_runtime(
        ctx,
        inst,
        "__elephc_class_file",
        "__rt_class_file_by_name",
    )
}
