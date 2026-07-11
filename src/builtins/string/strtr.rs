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

use crate::codegen::context::FunctionContext;
use crate::codegen::CodegenIrError;
use crate::ir::Instruction;

builtin! {
    name: "strtr",
    area: String,
    params: [string: Str, from: Str, to: Str],
    returns: Str,
    lower: lower,
    summary: "Translates characters of a string using a from/to byte map (3-argument form).",
    php_manual: "https://www.php.net/manual/en/function.strtr.php",
}

/// Lowers a 3-argument `strtr` call by dispatching to the shared string-replace emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen::lower_inst::builtins::strings::lower_string_replace(
        ctx,
        inst,
        "strtr",
        "__rt_strtr",
    )
}
