//! Purpose:
//! Home of the internal `__elephc_class_name_of` intrinsic: the runtime resolver
//! behind PHP 8.0's dynamic `$expr::class`. Compiler-synthesized; not PHP-visible.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - `internal: true` — reached only through the parser desugar of `$expr::class`
//!   (a `Name::class` / `static::class` receiver folds statically and never gets
//!   here).
//! - PHP `::class` on an expression: an OBJECT receiver yields its class name, a
//!   STRING receiver yields itself. Lowering dispatches on the receiver's static
//!   type (Object → class-name table; Str → identity; Mixed → runtime dispatch).

use crate::builtins::spec::BuiltinCheckCtx;
use crate::codegen::context::FunctionContext;
use crate::codegen::CodegenIrError;
use crate::errors::CompileError;
use crate::ir::Instruction;
use crate::types::PhpType;

builtin! {
    name: "__elephc_class_name_of",
    area: Callables,
    params: [value: Mixed],
    returns: Str,
    check: check,
    lower: lower,
    summary: "Resolves the class name for a dynamic `$expr::class` receiver.",
    internal: true,
}

/// Rejects `$expr::class` on a statically-string receiver, matching PHP 8's
/// `TypeError: Cannot use "::class" on string` (caught at compile time here — a
/// string is never a valid dynamic `::class` receiver, only objects are).
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    let arg_ty = cx.checker.infer_type(&cx.args[0], cx.env)?;
    if arg_ty == PhpType::Str {
        return Err(CompileError::new(
            cx.args[0].span,
            "Cannot use \"::class\" on a string (only objects have a dynamic ::class)",
        ));
    }
    Ok(PhpType::Str)
}

/// Lowers `__elephc_class_name_of(value)` through the dynamic class-name dispatcher.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen::lower_inst::builtins::types::lower_dynamic_class_name_of(ctx, inst)
}
