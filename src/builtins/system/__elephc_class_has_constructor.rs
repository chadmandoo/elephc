//! Purpose:
//! Home of the internal `__elephc_class_has_constructor` intrinsic: its
//! declaration and lowering. Compiler-synthesized; not PHP-visible.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - `internal: true` keeps it out of PHP-visible builtin name sets and
//!   `function_exists()`; it is reachable only through the synthetic
//!   `ReflectionClass::getConstructor()` body.
//! - Returns 1 when the named class declares (or inherits) `__construct`,
//!   0 when it does not or the class is unknown, via the
//!   `__rt_class_has_constructor` scan over the `_classes_by_name` table and
//!   its parallel position-indexed `_class_has_ctor` flag table.

use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "__elephc_class_has_constructor",
    area: System,
    params: [class_name: Str],
    returns: Int,
    lower: lower,
    summary: "Returns 1 when the named class has a constructor, else 0.",
    internal: true,
}

/// Lowers the call by dispatching to the shared unary string runtime emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::strings::lower_unary_string_runtime(
        ctx,
        inst,
        "__elephc_class_has_constructor",
        "__rt_class_has_constructor",
    )
}
