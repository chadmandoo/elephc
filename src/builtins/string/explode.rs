//! Purpose:
//! Home of the PHP `explode` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - The declared signature carries the full golden param list (`separator`, `string`,
//!   `limit`). Three-argument calls never reach `lower` as builtin calls: the EIR
//!   frontend desugars them into the stdlib prelude's `__elephc_explode_limit`
//!   (see `lower_static_explode_limit`), which applies PHP's positive/negative limit
//!   semantics over the plain two-argument split.
//! - `check` returns `PhpType::Array(Box::new(PhpType::Str))`. A check hook is required
//!   because the `builtin!` macro `returns:` field cannot express an array type inline.
//!   Argument types are inferred by the common registry dispatch path before the hook
//!   fires.
//! - `lower` is a thin wrapper over the shared `lower_explode` emitter.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::errors::CompileError;
use crate::ir::Instruction;
use crate::types::PhpType;

builtin! {
    name: "explode",
    area: String,
    params: [separator: Str, string: Str, limit: Int = DefaultSpec::IntMax],
    returns: Mixed,
    check: check,
    lower: lower,
    summary: "Splits a string by a separator into an array of substrings.",
    php_manual: "https://www.php.net/manual/en/function.explode.php",
}

/// Returns `PhpType::Array(Box::new(PhpType::Str))` for an `explode` call.
///
/// A check hook is required because the `builtin!` macro cannot express array return
/// types inline. Argument types are inferred by the common registry dispatch path before
/// this hook fires; arity (2 or 3 arguments) is validated by the registry.
fn check(_cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    Ok(PhpType::Array(Box::new(PhpType::Str)))
}

/// Lowers an `explode` call by dispatching to the shared `lower_explode` emitter.
///
/// A limit operand here means the frontend desugar declined the call shape (named or
/// spread arguments), which the two-argument emitter cannot honor.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    if inst.operands.len() > 2 {
        return Err(CodegenIrError::unsupported(
            "explode limit argument that is not a direct positional argument",
        ));
    }
    crate::codegen_ir::lower_inst::builtins::strings::lower_explode(ctx, inst)
}
