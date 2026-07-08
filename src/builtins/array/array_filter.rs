//! Purpose:
//! Home of the PHP `array_filter` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - The PHP golden signature is `optional(&["array","callback","mode"], 1, &[null, 0])`.
//!   The legacy CHECK arm required 2 or 3 arguments (`args.len() < 2 || args.len() > 3`),
//!   so `min_args: 2` reproduces that enforcement in `check_arity`; the derived max of 3
//!   from the optional signature already matches.
//! - `check` validates the first argument is an indexed array, builds callback dummy args
//!   based on the static mode value, and validates the callback signature. The return type
//!   preserves the input array element type.
//! - `lower` is a thin wrapper over the shared `arrays::lower_array_filter` emitter.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::codegen_ir::context::FunctionContext;
use crate::codegen_ir::CodegenIrError;
use crate::errors::CompileError;
use crate::ir::Instruction;
use crate::types::PhpType;

builtin! {
    name: "array_filter",
    area: Array,
    params: [array: Mixed, callback: Mixed = DefaultSpec::Null, mode: Mixed = DefaultSpec::Int(0)],
    min_args: 2,
    returns: Mixed,
    check: check,
    lazy_check: true,
    lower: lower,
    summary: "Filters elements of an array using a callback function.",
    php_manual: "https://www.php.net/manual/en/function.array-filter.php",
}

/// Returns the filtered array type for an `array_filter` call.
///
/// Validates the first argument is an indexed array, builds the callback dummy args
/// based on the optional mode argument, and validates the callback. Arity (2 or 3 args)
/// is pre-validated by `check_arity`.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    // Infer the array + optional mode, but NOT the callback (arg 1) — checked below
    // with its value parameter typed from the element (EC-28).
    let arr_ty = cx.checker.infer_type(&cx.args[0], cx.env)?;
    if let Some(mode) = cx.args.get(2) {
        cx.checker.infer_type(mode, cx.env)?;
    }
    // Indexed receivers keep the packed runtime path; associative receivers
    // are desugared by the EIR frontend into the prelude
    // `__elephc_array_filter_hash` impl (keys preserved). Both report the
    // receiver's own type as the result. Mixed receivers (json_decode
    // results, adaptive locals) desugar to `__elephc_array_filter_any`,
    // whose adaptive foreach handles either runtime shape — the result
    // shape is input-dependent, so it stays Mixed.
    let result_ty = match &arr_ty {
        PhpType::Array(_) | PhpType::AssocArray { .. } => arr_ty.clone(),
        PhpType::Mixed => PhpType::Mixed,
        _ => {
            return Err(CompileError::new(
                cx.span,
                "array_filter() first argument must be array",
            ))
        }
    };
    let (dummy_args, elem_binding) =
        crate::types::checker::builtins::array_filter_callback_dummy_args(
            &arr_ty,
            cx.args.get(2),
            cx.span,
        );
    let mut env_with_elem;
    let cb_env: &crate::types::TypeEnv = match &elem_binding {
        Some((binding_name, binding_ty)) => {
            env_with_elem = cx.env.clone();
            env_with_elem.insert(binding_name.clone(), binding_ty.clone());
            &env_with_elem
        }
        None => cx.env,
    };
    crate::types::checker::builtins::check_callback_builtin_call(
        cx.checker,
        &cx.args[1],
        &dummy_args,
        cx.span,
        cb_env,
        "array_filter() callback",
    )?;
    Ok(result_ty)
}

/// Lowers an `array_filter` call by dispatching to the shared array emitter.
fn lower(ctx: &mut FunctionContext, inst: &Instruction) -> Result<(), CodegenIrError> {
    crate::codegen_ir::lower_inst::builtins::arrays::lower_array_filter(ctx, inst)
}
