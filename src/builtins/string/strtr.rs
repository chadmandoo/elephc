//! Purpose:
//! Home of the PHP `strtr` builtin (3-argument character-map form): its declaration and lowering.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - This declaration covers only the 3-argument form `strtr($str, $from, $to)`, which
//!   translates single bytes through a `from`→`to` map. The 2-argument pair-array form
//!   (`strtr($str, $replacements)`) takes an array operand and longest-match replacement
//!   semantics; it is a separate lowering follow-up, so a 2-arg call is rejected at arity
//!   here rather than miscompiled.
//! - No `check` hook is needed: three `Str` arguments in, one `Str` out.
//! - `lower` reuses the shared `lower_string_replace` emitter (which materializes exactly
//!   three string operands into the runtime ABI) with the `__rt_strtr` helper. EIR-only,
//!   so it joins the callable-dispatch wrapper exclusion.

use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "strtr",
    area: String,
    params: [string: Str, from: Str, to: Str],
    min_args: 2,
    returns: Str,
    check: check,
    lazy_check: true,
    lower: lower,
    summary: "Translates characters of a string using a from/to byte map or a pair array.",
    php_manual: "https://www.php.net/manual/en/function.strtr.php",
}

/// Validates both strtr forms: the 3-argument char-map form (three strings)
/// and the 2-argument pair-array form (`strtr($s, $replacements)`), which the
/// EIR frontend desugars into the prelude `__elephc_strtr_pairs` impl. Lazy
/// because the 2-argument form's second argument is an ARRAY where the
/// declared parameter says Str.
fn check(
    cx: &mut crate::builtins::spec::BuiltinCheckCtx,
) -> Result<crate::types::PhpType, crate::errors::CompileError> {
    use crate::types::PhpType;
    let first = cx.checker.infer_type(&cx.args[0], cx.env)?;
    if !matches!(first.codegen_repr(), PhpType::Str | PhpType::Mixed | PhpType::Union(_)) {
        return Err(crate::errors::CompileError::new(
            cx.span,
            "strtr() first argument must be a string",
        ));
    }
    if cx.args.len() == 2 {
        let pairs = cx.checker.infer_type(&cx.args[1], cx.env)?;
        if !matches!(
            pairs.codegen_repr(),
            PhpType::Array(_) | PhpType::AssocArray { .. } | PhpType::Mixed | PhpType::Union(_)
        ) {
            return Err(crate::errors::CompileError::new(
                cx.span,
                "strtr() second argument must be an array in the 2-argument form",
            ));
        }
        return Ok(PhpType::Str);
    }
    for arg in &cx.args[1..] {
        let ty = cx.checker.infer_type(arg, cx.env)?;
        if !matches!(ty.codegen_repr(), PhpType::Str | PhpType::Mixed | PhpType::Union(_)) {
            return Err(crate::errors::CompileError::new(
                cx.span,
                "strtr() from/to arguments must be strings in the 3-argument form",
            ));
        }
    }
    Ok(PhpType::Str)
}

/// Lowers a 3-argument `strtr` call by dispatching to the shared string-replace emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::strings::lower_string_replace(
        ctx,
        inst,
        "strtr",
        "__rt_strtr",
    )
}
