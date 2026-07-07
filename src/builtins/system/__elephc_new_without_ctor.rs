//! Purpose:
//! Home of the internal `__elephc_new_without_ctor` intrinsic: its declaration
//! and lowering. Compiler-synthesized; not PHP-visible.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - `internal: true` keeps it out of PHP-visible builtin name sets and
//!   `function_exists()`; it is reachable only through the synthetic
//!   `ReflectionClass::newInstanceWithoutConstructor()` body.
//! - `__rt_new_by_name` allocates, zero-fills, stamps the class id, and runs
//!   the property-default thunk WITHOUT invoking a constructor — matching
//!   PHP's newInstanceWithoutConstructor semantics. The result is a boxed
//!   Mixed object cell, or boxed null for an unknown class name.

use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "__elephc_new_without_ctor",
    area: System,
    params: [class_name: Str],
    returns: Mixed,
    lower: lower,
    summary: "Allocates a class instance without running its constructor.",
    internal: true,
}

/// Lowers the call by dispatching to the shared system emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::system::lower_new_without_ctor(ctx, inst)
}
