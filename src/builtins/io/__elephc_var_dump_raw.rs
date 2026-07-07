//! Purpose:
//! Home of the internal `__elephc_var_dump_raw` intrinsic: the raw single-value
//! var_dump renderer behind the prelude `__elephc_var_dump_value` impl.
//! Compiler-synthesized; not PHP-visible.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - `internal: true` keeps it out of PHP-visible builtin name sets. It exists
//!   because container/Mixed-typed `var_dump()` calls are desugared into the
//!   prelude impl (recursive nested-container rendering), and the impl needs
//!   an alias for the raw builtin walker for NON-array leaves (exact scalar,
//!   float-precision, and object formatting) that the desugar never rewrites —
//!   calling `var_dump` from inside the impl would re-desugar into the impl
//!   itself and recurse forever.

use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "__elephc_var_dump_raw",
    area: Io,
    params: [value: Mixed],
    returns: Void,
    lower: lower,
    summary: "Raw single-value var_dump renderer (prelude-internal alias).",
    internal: true,
}

/// Lowers the call by dispatching to the shared debug emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::debug::lower_var_dump(ctx, inst)
}
