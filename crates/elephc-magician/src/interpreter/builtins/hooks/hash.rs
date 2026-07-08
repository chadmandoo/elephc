//! Purpose:
//! Focused dispatch helpers for declarative hash builtin hooks.
//!
//! Called from:
//! - `crate::interpreter::builtins::hooks::values`.
//!
//! Key details:
//! - These helpers keep the generic evaluated-argument hook table below the
//!   ordinary file-size limit while preserving the existing hash behavior.

use super::super::super::{ElephcEvalContext, EvalStatus, RuntimeCellHandle, RuntimeValueOps};
use super::super::{
    eval_hash_algos_result, eval_hash_copy_result, eval_hash_final_result, eval_hash_init_result,
    eval_hash_update_result,
};

/// Dispatches evaluated `hash_algos()` calls.
pub(super) fn eval_hash_algos_values(
    evaluated_args: &[RuntimeCellHandle],
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    if !evaluated_args.is_empty() {
        return Err(EvalStatus::RuntimeFatal);
    }
    eval_hash_algos_result(values)
}

/// Dispatches evaluated incremental hash-context builtin calls.
pub(super) fn eval_hash_context_values(
    name: &str,
    evaluated_args: &[RuntimeCellHandle],
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    match name {
        "hash_copy" => {
            let [hash_context] = evaluated_args else {
                return Err(EvalStatus::RuntimeFatal);
            };
            eval_hash_copy_result(*hash_context, context, values)
        }
        "hash_final" => match evaluated_args {
            [hash_context] => eval_hash_final_result(*hash_context, false, context, values),
            [hash_context, binary] => {
                let binary = values.truthy(*binary)?;
                eval_hash_final_result(*hash_context, binary, context, values)
            }
            _ => Err(EvalStatus::RuntimeFatal),
        },
        "hash_init" => {
            let [algo] = evaluated_args else {
                return Err(EvalStatus::RuntimeFatal);
            };
            eval_hash_init_result(*algo, context, values)
        }
        "hash_update" => {
            let [hash_context, data] = evaluated_args else {
                return Err(EvalStatus::RuntimeFatal);
            };
            eval_hash_update_result(*hash_context, *data, context, values)
        }
        _ => Err(EvalStatus::RuntimeFatal),
    }
}
