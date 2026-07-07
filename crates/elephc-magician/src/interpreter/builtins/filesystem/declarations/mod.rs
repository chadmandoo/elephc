//! Purpose:
//! Declarative eval registry entries and dispatch adapters for filesystem builtins.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem` module loading.
//! - `crate::interpreter::builtins::hooks` for migrated filesystem dispatch.
//!
//! Key details:
//! - This first tranche covers path/string-like helpers only; stream, stat,
//!   mutating, and by-reference filesystem calls stay on the legacy path.

use super::super::super::*;
use super::*;

mod basename;
mod dirname;
mod fnmatch;
mod pathinfo;

/// Dispatches direct expression-level calls for declaratively migrated filesystem builtins.
pub(in crate::interpreter) fn eval_builtin_filesystem_call(
    name: &str,
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    match name {
        "basename" => eval_builtin_basename(args, context, scope, values),
        "dirname" => eval_builtin_dirname(args, context, scope, values),
        "fnmatch" => eval_builtin_fnmatch(args, context, scope, values),
        "pathinfo" => eval_builtin_pathinfo(args, context, scope, values),
        _ => Err(EvalStatus::RuntimeFatal),
    }
}

/// Dispatches evaluated-argument calls for declaratively migrated filesystem builtins.
pub(in crate::interpreter) fn eval_filesystem_values_result(
    name: &str,
    evaluated_args: &[RuntimeCellHandle],
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    match name {
        "basename" => match evaluated_args {
            [path] => eval_basename_result(*path, None, values),
            [path, suffix] => eval_basename_result(*path, Some(*suffix), values),
            _ => Err(EvalStatus::RuntimeFatal),
        },
        "dirname" => match evaluated_args {
            [path] => eval_dirname_result(*path, None, values),
            [path, levels] => eval_dirname_result(*path, Some(*levels), values),
            _ => Err(EvalStatus::RuntimeFatal),
        },
        "fnmatch" => match evaluated_args {
            [pattern, filename] => eval_fnmatch_result(*pattern, *filename, None, values),
            [pattern, filename, flags] => {
                eval_fnmatch_result(*pattern, *filename, Some(*flags), values)
            }
            _ => Err(EvalStatus::RuntimeFatal),
        },
        "pathinfo" => match evaluated_args {
            [path] => eval_pathinfo_result(*path, None, values),
            [path, flags] => eval_pathinfo_result(*path, Some(*flags), values),
            _ => Err(EvalStatus::RuntimeFatal),
        },
        _ => Err(EvalStatus::RuntimeFatal),
    }
}
