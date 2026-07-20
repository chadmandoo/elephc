//! Purpose:
//! Home of the PHP `defined` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` validates that the argument is a string literal (AOT requirement: the
//!   constant name must be statically known at compile time).
//! - `lower` delegates to the module-level `lower_defined` in `src/codegen/lower_inst/builtins.rs`.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::parser::ast::ExprKind;
use crate::types::PhpType;

builtin! {
    name: "defined",
    area: System,
    params: [constant_name: Str],
    returns: Bool,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Defined,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Checks whether the given named constant exists.",
}

/// Validates that the argument is a string literal.
///
/// AOT compilation requires a statically known constant name; dynamic names cannot
/// be resolved at compile time. Returns `PhpType::Bool` on success.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    cx.checker.infer_type(&cx.args[0], cx.env)?;
    if !matches!(cx.args[0].kind, ExprKind::StringLiteral(_)) {
        return Err(CompileError::new(
            cx.span,
            "defined() first argument must be a string literal in AOT mode",
        ));
    }
    Ok(PhpType::Bool)
}
