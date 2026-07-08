//! Purpose:
//! Home of the PHP `sort` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - The golden signature is `first_param_ref(optional(["array","flags"], 1, [0]))`: the
//!   `array` param is by-reference (the `ref` marker is what makes by-reference mutation
//!   lower correctly — ir_lower reads `ref_params` from the registry sig).
//! - PHP's optional `int $flags` is accepted and validated as an int constant. The flag
//!   VALUE is not dispatched on: elephc's sort already compares by the element type
//!   (string arrays compare as strings, int arrays numerically), which matches
//!   SORT_STRING/SORT_NUMERIC on homogeneous typed arrays — the shapes strict-typed
//!   callers pass (WalCommitWriter/FileLockManager sort string lists with SORT_STRING).
//! - `check` requires the first argument be an Array or AssocArray, returning Void.
//! - `lower` is a thin wrapper over the shared `arrays::lower_sort` emitter.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::errors::CompileError;
use crate::ir::Instruction;
use crate::types::PhpType;

builtin! {
    name: "sort",
    area: Array,
    params: [ref array: Mixed, flags: Mixed = DefaultSpec::Int(0)],
    min_args: 1,
    returns: Void,
    check: check,
    lower: lower,
    summary: "Sorts an array in ascending order.",
    php_manual: "https://www.php.net/manual/en/function.sort.php",
}

/// Validates the argument types for a `sort` call.
///
/// Requires the first argument be an indexed or associative array; the optional
/// second argument (PHP's `int $flags`) must be an int. Arity (1 or 2) is
/// pre-validated by the registry. Returns `Ok(PhpType::Void)` on success.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    let ty = cx.checker.infer_type(&cx.args[0], cx.env)?;
    // Mixed receivers (adaptive array_keys results, Mixed-returned lists)
    // are arrays at runtime in well-typed code; the EIR desugar routes them
    // through the prelude copy-sort.
    if !matches!(
        ty,
        PhpType::Array(_) | PhpType::AssocArray { .. } | PhpType::Mixed
    ) {
        return Err(CompileError::new(cx.span, &format!("{}() argument must be array", cx.name)));
    }
    if let Some(flags) = cx.args.get(1) {
        let flags_ty = cx.checker.infer_type(flags, cx.env)?;
        if !matches!(flags_ty, PhpType::Int | PhpType::Mixed) {
            return Err(CompileError::new(
                cx.span,
                &format!("{}() flags argument must be an int", cx.name),
            ));
        }
    }
    Ok(PhpType::Void)
}

/// Lowers a `sort` call by dispatching to the shared array emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::arrays::lower_sort(ctx, inst)
}
