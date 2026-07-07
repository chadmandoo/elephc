//! Purpose:
//! Home of the PHP `base64_decode` builtin: its declaration and lowering.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: the legacy CHECK arm declared a `Str` return type
//!   (matching the migration golden), fully determined by this declaration. The
//!   registry derives the return type from the `returns:` field without a check hook.
//! - `lower` is a thin wrapper over the shared `lower_unary_string_runtime` emitter,
//!   passing the `__rt_base64_decode` runtime helper.

use crate::builtins::spec::DefaultSpec;
use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "base64_decode",
    area: String,
    params: [string: Str, strict: Bool = DefaultSpec::Bool(false)],
    returns: Str,
    check: check,
    lower: lower,
    summary: "Decodes a Base64-encoded string back into its original data.",
    php_manual: "https://www.php.net/manual/en/function.base64-decode.php",
}

/// Types the two base64_decode forms: the 1-argument decode stays `Str`; the
/// 2-argument strict form is `string|false` (the EIR frontend desugars it into
/// the prelude `__elephc_base64_decode_ex` impl, which returns false on a
/// strict-mode violation).
fn check(
    cx: &mut crate::builtins::spec::BuiltinCheckCtx,
) -> Result<crate::types::PhpType, crate::errors::CompileError> {
    use crate::types::PhpType;
    if cx.args.len() == 2 {
        return Ok(PhpType::Union(vec![PhpType::Str, PhpType::Bool]));
    }
    Ok(PhpType::Str)
}

/// Lowers a `base64_decode` call by dispatching to the shared per-arch unary string runtime.
fn lower(
    ctx: &mut FunctionContext,
    inst: &Instruction,
) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::strings::lower_unary_string_runtime(
        ctx,
        inst,
        "base64_decode",
        "__rt_base64_decode",
    )
}
