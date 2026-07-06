//! Purpose:
//! Home of the PHP `preg_match_all` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - The third param `matches` is by-reference (`ref matches: Mixed = DefaultSpec::EmptyArray`)
//!   and the fourth is `flags` (defaulting to PREG_PATTERN_ORDER = 1), mirroring PHP.
//! - `lazy_check: true` suppresses the registry's default pre-inference loop so the hook
//!   can infer the other args while deliberately skipping args[2] (`$matches`), a
//!   write-only output parameter that is undefined before the call.
//! - Calls that pass `$matches` never reach `lower`: the EIR frontend desugars them into
//!   an assignment from the stdlib-prelude `__elephc_preg_match_all_impl` plus a
//!   `count($matches[0])` result (see `lower_static_preg_match_all_capture`). `lower`
//!   only handles the 2-argument count-only form via `__rt_preg_match_all`.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
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
/// Infers args[0]/args[1] (pattern and subject) and args[3] (flags) to trigger
/// type-environment side effects, but deliberately skips inference of args[2]
/// (`$matches`) because it is a write-only output parameter that is undefined before
/// the call. Passing a non-variable (such as a literal or function call) to `$matches`
/// is a compile-time error.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    cx.checker.infer_type(&cx.args[0], cx.env)?;
    cx.checker.infer_type(&cx.args[1], cx.env)?;
    if cx.args.len() >= 3 && !matches!(cx.args[2].kind, ExprKind::Variable(_)) {
        return Err(CompileError::new(
            cx.args[2].span,
            "preg_match_all() parameter $matches must be passed a variable",
        ));
    }
    if let Some(flags) = cx.args.get(3) {
        cx.checker.infer_type(flags, cx.env)?;
    }
    Ok(PhpType::Int)
}

/// Lowers a `preg_match_all` call by dispatching to the shared regex emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::regex::lower_preg_match_all(ctx, inst)
}
