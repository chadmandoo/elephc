//! Purpose:
//! Dispatches already evaluated scalar math, casts, predicates, and random builtins by dynamic callable name.
//!
//! Called from:
//! - `crate::interpreter::builtins::registry::dispatch`.
//!
//! Key details:
//! - Returns `Ok(None)` for names outside this domain so the parent dispatcher can
//!   continue probing other builtin families.

use super::super::super::super::*;
use super::super::super::*;

/// Attempts to dispatch evaluated scalar math, casts, predicates, and random builtins.
pub(in crate::interpreter) fn eval_scalars_builtin_with_values(
    name: &str,
    evaluated_args: &[RuntimeCellHandle],
    _context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<Option<RuntimeCellHandle>, EvalStatus> {
    let result = match name {
        "rand" | "mt_rand" => match evaluated_args {
            [] => eval_rand_result(None, None, values)?,
            [min, max] => eval_rand_result(Some(*min), Some(*max), values)?,
            _ => return Err(EvalStatus::RuntimeFatal),
        },
        "random_int" => {
            let [min, max] = evaluated_args else {
                return Err(EvalStatus::RuntimeFatal);
            };
            eval_random_int_result(*min, *max, values)?
        }
        "settype" => {
            let [value, type_name] = evaluated_args else {
                return Err(EvalStatus::RuntimeFatal);
            };
            eval_settype_value_result(*value, *type_name, values)?
        }
        _ => return Ok(None),
    };
    Ok(Some(result))
}
