//! Purpose:
//! Dispatches already evaluated numeric formatting and printf-family builtins by dynamic callable name.
//!
//! Called from:
//! - `crate::interpreter::builtins::registry::dispatch`.
//!
//! Key details:
//! - Returns `Ok(None)` for names outside this domain so the parent dispatcher can
//!   continue probing other builtin families.

use super::super::super::super::*;
use super::super::super::*;

/// Attempts to dispatch evaluated numeric formatting and printf-family builtins.
pub(in crate::interpreter) fn eval_formatting_builtin_with_values(
    name: &str,
    evaluated_args: &[RuntimeCellHandle],
    _context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<Option<RuntimeCellHandle>, EvalStatus> {
    let result = match name {
        "sscanf" => {
            let [input, format, ..] = evaluated_args else {
                return Err(EvalStatus::RuntimeFatal);
            };
            eval_sscanf_result(*input, *format, values)?
        }
        "sprintf" | "printf" => eval_sprintf_like_result(name, evaluated_args, values)?,
        "vsprintf" | "vprintf" => eval_vsprintf_like_result(name, evaluated_args, values)?,
        _ => return Ok(None),
    };
    Ok(Some(result))
}
