//! Purpose:
//! Stream-opening builtins for eval-local file resources.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::streams` re-exports.
//! - Filesystem stream declaration dispatchers.
//!
//! Key details:
//! - User wrapper `stream_open` gets the first chance before local file handles
//!   are inserted into eval's stream table.

use super::*;

/// Evaluates PHP `fopen($filename, $mode, ...)` over eval expressions.
pub(in crate::interpreter) fn eval_builtin_fopen(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    if !(2..=4).contains(&args.len()) {
        return Err(EvalStatus::RuntimeFatal);
    }
    let filename = eval_expr(&args[0], context, scope, values)?;
    let mode = eval_expr(&args[1], context, scope, values)?;
    for arg in &args[2..] {
        eval_expr(arg, context, scope, values)?;
    }
    let filename = eval_path_string(filename, values)?;
    let mode = eval_stream_string(mode, values)?;
    eval_fopen_path_result(&filename, &mode, context, scope, values)
}

/// Opens a local file stream and returns a resource cell or PHP false.
pub(in crate::interpreter) fn eval_fopen_result(
    filename: RuntimeCellHandle,
    mode: RuntimeCellHandle,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let filename = eval_path_string(filename, values)?;
    let mode = eval_stream_string(mode, values)?;
    let mut scope = ElephcEvalScope::new();
    eval_fopen_path_result(&filename, &mode, context, &mut scope, values)
}

/// Opens a stream by already-coerced path and mode strings.
fn eval_fopen_path_result(
    filename: &str,
    mode: &str,
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    if let Some(result) = eval_user_wrapper_fopen_result(filename, mode, context, scope, values)? {
        return Ok(result);
    }
    match context.stream_resources_mut().open_path(filename, mode) {
        Some(id) => values.resource(id),
        None => {
            values.warning("Warning: fopen(): Failed to open stream\n")?;
            values.bool_value(false)
        }
    }
}

/// Evaluates PHP `tmpfile()` with no arguments.
pub(in crate::interpreter) fn eval_builtin_tmpfile(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    if !args.is_empty() {
        return Err(EvalStatus::RuntimeFatal);
    }
    eval_tmpfile_result(context, values)
}

/// Creates an anonymous temporary file stream resource or returns PHP false.
pub(in crate::interpreter) fn eval_tmpfile_result(
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    match context.stream_resources_mut().open_tmpfile() {
        Some(id) => values.resource(id),
        None => values.bool_value(false),
    }
}
