//! Purpose:
//! Home of the internal `__elephc_base64_decode_raw` intrinsic: the raw
//! chunked decoder behind the prelude `__elephc_base64_decode_ex` impl.
//! Compiler-synthesized; not PHP-visible.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - `internal: true` keeps it out of PHP-visible builtin name sets. It exists
//!   because ALL user `base64_decode()` calls are desugared into the prelude
//!   impl (whitespace cleaning, strict validation, tail padding), and the impl
//!   needs an alias for the raw `__rt_base64_decode` decoder that the desugar
//!   never rewrites — calling `base64_decode` from inside the impl would
//!   re-desugar into the impl itself and recurse forever.

use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "__elephc_base64_decode_raw",
    area: String,
    params: [string: Str],
    returns: Str,
    lower: lower,
    summary: "Raw base64 chunk decoder (prelude-internal alias).",
    internal: true,
}

/// Lowers the call by dispatching to the shared per-arch unary string runtime.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::strings::lower_unary_string_runtime(
        ctx,
        inst,
        "__elephc_base64_decode_raw",
        "__rt_base64_decode",
    )
}
