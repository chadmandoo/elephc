//! Purpose:
//! Home of the PHP `array_merge` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - The PHP golden signature is `variadic(&[], "arrays")` (min=0). The native
//!   lowering handles exactly 2 operands; `min_args: 2` keeps that floor while
//!   3+ argument calls are folded by the EIR frontend into nested 2-argument
//!   merges (`array_merge(a, b, c)` → `array_merge(array_merge(a, b), c)`), so
//!   the lowering still only ever sees pairs.
//! - `check` validates that every argument is an indexed or associative array
//!   (or Mixed) and returns the pairwise-folded result type. The 2-arg logic
//!   mirrors the legacy checker: when the left operand is an empty indexed
//!   array (element type `Void`), the result adopts the right operand's
//!   element type if it is a scalar-merge type.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::errors::CompileError;
use crate::ir::Instruction;
use crate::types::PhpType;

builtin! {
    name: "array_merge",
    area: Array,
    params: [],
    variadic: "arrays",
    min_args: 2,
    returns: Mixed,
    check: check,
    lower: lower,
    summary: "Merges the elements of two or more arrays.",
    php_manual: "https://www.php.net/manual/en/function.array-merge.php",
}

/// Validates every argument is an array (or Mixed) and returns the merged result type.
///
/// Arity (2+ args) is pre-validated by `check_arity`; 3+ argument calls are folded
/// into nested pairs by the EIR frontend, so the result type is derived by the same
/// pairwise fold. For each pair: when the left operand is an empty indexed array
/// (element type `Void`), the result adopts the right operand's element type if it
/// is a scalar-merge-compatible type.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    // The 3+ fold desugar bails on spread/named args, which would reach the
    // pair-only native lowering — reject them here with a real error.
    if cx.args.len() > 2
        && (crate::types::call_args::has_named_args(cx.args)
            || cx
                .args
                .iter()
                .any(|arg| matches!(arg.kind, crate::parser::ast::ExprKind::Spread(_))))
    {
        return Err(CompileError::new(
            cx.span,
            "array_merge() spread/named arguments are not supported with 3+ arguments",
        ));
    }
    let mut merged = cx.checker.infer_type(&cx.args[0], cx.env)?;
    // A Mixed operand (an `array`-hinted property's element, a `?? []` result) is an array at
    // runtime in well-typed code — runtime-enforced PHP, same trust posture as the argument
    // boundary. The merged result is Mixed because the element type is unknown.
    if !matches!(
        merged,
        PhpType::Array(_) | PhpType::AssocArray { .. } | PhpType::Mixed
    ) {
        return Err(CompileError::new(
            cx.span,
            "array_merge() first argument must be array",
        ));
    }
    for arg in &cx.args[1..] {
        let next = cx.checker.infer_type(arg, cx.env)?;
        if matches!(merged, PhpType::Mixed) {
            continue;
        }
        merged = array_merge_return_type(merged, next);
    }
    Ok(merged)
}

/// Lowers an `array_merge` call by delegating to the shared array-merge emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::arrays::lower_array_merge(ctx, inst)
}

/// Infers the return type for `array_merge(first, second)`.
///
/// When `first` is an empty indexed array (element type `Void`), the merged result
/// adopts `second`'s element type if it is a scalar-merge-compatible type; otherwise
/// the result keeps `first`'s type. For non-empty indexed arrays, the left operand
/// type is returned unchanged (matching legacy checker behavior).
fn array_merge_return_type(first: PhpType, second: PhpType) -> PhpType {
    match first {
        PhpType::Array(elem) if is_empty_array_element_type(elem.as_ref()) => match second {
            PhpType::Array(right) if is_scalar_merge_element_type(right.as_ref()) => {
                PhpType::Array(right)
            }
            _ => PhpType::Array(elem),
        },
        other => other,
    }
}

/// Returns true for the element sentinel used by statically empty indexed arrays.
fn is_empty_array_element_type(ty: &PhpType) -> bool {
    matches!(ty.codegen_repr(), PhpType::Void)
}

/// Returns true for element types that the scalar merge runtime helper copies safely.
fn is_scalar_merge_element_type(ty: &PhpType) -> bool {
    matches!(
        ty.codegen_repr(),
        PhpType::Int | PhpType::Bool | PhpType::Float | PhpType::Callable | PhpType::Void
    )
}
