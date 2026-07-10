//! Purpose:
//! Home of the PHP `preg_match_all` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - The third param `matches` is by-reference (`ref matches: Mixed = DefaultSpec::EmptyArray`),
//!   the write-only output array populated with every match group; the optional fourth `flags`
//!   selects the ordering (defaults to PREG_PATTERN_ORDER = 1). Mirrors `preg_match`.
//! - `lazy_check: true` suppresses the registry's default pre-inference so the hook can infer
//!   args[0]/args[1] (pattern, subject) while skipping inference of args[2] (`$matches`), a
//!   write-only output parameter undefined before the call.
//! - `check` validates that args[2] (when present) is a `Variable` expression.
//! - `lower` is a thin wrapper over `regex::lower_preg_match_all` in the EIR backend.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::codegen::context::FunctionContext;
use crate::codegen::CodegenIrError;
use crate::errors::CompileError;
use crate::ir::Instruction;
use crate::parser::ast::ExprKind;
use crate::types::PhpType;

builtin! {
    name: "preg_match_all",
    area: System,
    params: [pattern: Str, subject: Str, ref matches: Mixed = DefaultSpec::EmptyArray, flags: Int = DefaultSpec::Int(1)],
    returns: Int,
    check: check,
    lazy_check: true,
    lower: lower,
    summary: "Performs a global regular expression match and returns the number of matches.",
}

/// Validates that `$matches`, when supplied, is a variable expression.
///
/// Infers args[0] (pattern) and args[1] (subject) to trigger type-environment side effects,
/// but deliberately skips inference of args[2] (`$matches`) because it is a write-only output
/// parameter that is undefined before the call. Passing a non-variable to `$matches` is an error.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    cx.checker.infer_type(&cx.args[0], cx.env)?;
    cx.checker.infer_type(&cx.args[1], cx.env)?;
    if cx.args.len() >= 3 && !matches!(cx.args[2].kind, ExprKind::Variable(_)) {
        return Err(CompileError::new(
            cx.args[2].span,
            "preg_match_all() parameter $matches must be passed a variable",
        ));
    }
    Ok(PhpType::Int)
}

/// Lowers a `preg_match_all` call by dispatching to the shared regex emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen::lower_inst::builtins::regex::lower_preg_match_all(ctx, inst)
}
