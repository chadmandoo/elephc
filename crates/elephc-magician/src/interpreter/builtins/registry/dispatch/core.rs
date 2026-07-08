//! Purpose:
//! Dispatches remaining already evaluated core debug-output builtins by dynamic
//! callable name.
//!
//! Called from:
//! - `crate::interpreter::builtins::registry::dispatch`.
//!
//! Key details:
//! - Callable, constant, and process-control builtins migrate through the
//!   declarative registry; this file preserves legacy debug-output behavior.

use super::super::*;

/// Attempts to dispatch evaluated core debug-output builtins.
pub(in crate::interpreter) fn eval_core_builtin_with_values(
    name: &str,
    evaluated_args: &[RuntimeCellHandle],
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<Option<RuntimeCellHandle>, EvalStatus> {
    let result = match name {
        "print_r" => eval_print_r_result(evaluated_args, context, values)?,
        "var_dump" => eval_var_dump_result(evaluated_args, context, values)?,
        _ => return Ok(None),
    };
    Ok(Some(result))
}
