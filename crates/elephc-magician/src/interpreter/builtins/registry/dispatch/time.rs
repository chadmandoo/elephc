//! Purpose:
//! Dispatches remaining already evaluated response-header builtins by dynamic callable name.
//!
//! Called from:
//! - `crate::interpreter::builtins::registry::dispatch`.
//!
//! Key details:
//! - Date, time, and sleep builtins have migrated to declarative specs.
//! - Returns `Ok(None)` for names outside this small legacy surface so the
//!   parent dispatcher can continue probing other builtin families.

use super::super::super::super::*;
use super::super::super::*;

/// Attempts to dispatch evaluated header and response-code builtins.
pub(in crate::interpreter) fn eval_time_builtin_with_values(
    name: &str,
    evaluated_args: &[RuntimeCellHandle],
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<Option<RuntimeCellHandle>, EvalStatus> {
    let result = match name {
        "header" => match evaluated_args {
            [line] => eval_header_result(*line, None, None, context, values)?,
            [line, replace] => eval_header_result(*line, Some(*replace), None, context, values)?,
            [line, replace, response_code] => {
                eval_header_result(*line, Some(*replace), Some(*response_code), context, values)?
            }
            _ => return Err(EvalStatus::RuntimeFatal),
        },
        "http_response_code" => match evaluated_args {
            [] => eval_http_response_code_result(None, context, values)?,
            [response_code] => eval_http_response_code_result(Some(*response_code), context, values)?,
            _ => return Err(EvalStatus::RuntimeFatal),
        },
        _ => return Ok(None),
    };
    Ok(Some(result))
}
