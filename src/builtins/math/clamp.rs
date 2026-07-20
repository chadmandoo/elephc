//! Purpose:
//! Home of the PHP `clamp` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - A `check` hook is required because the return type depends on all three argument
//!   types: all-Str returns Str, all-Int returns Int, Int/Float mix returns Float,
//!   anything else returns Mixed.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "clamp",
    area: Math,
    params: [value: Mixed, min: Mixed, max: Mixed],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Clamp,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Clamps a value to be within a specified range.",
    php_manual: "https://www.php.net/manual/en/function.clamp.php",
}

/// Returns the most precise result type for `clamp($value, $min, $max)`.
///
/// All-string operands return `Str`; all-int return `Int`; int/float mix returns
/// `Float`; any other combination returns `Mixed`.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    let mut arg_types = Vec::with_capacity(cx.args.len());
    for arg in cx.args {
        arg_types.push(cx.checker.infer_type(arg, cx.env)?);
    }
    if arg_types.iter().all(|ty| *ty == PhpType::Str) {
        Ok(PhpType::Str)
    } else if arg_types.iter().all(|ty| *ty == PhpType::Int) {
        Ok(PhpType::Int)
    } else if arg_types
        .iter()
        .all(|ty| matches!(ty, PhpType::Int | PhpType::Float))
    {
        Ok(PhpType::Float)
    } else {
        Ok(PhpType::Mixed)
    }
}
