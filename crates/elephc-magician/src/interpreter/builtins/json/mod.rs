//! Purpose:
//! Declarative eval registry entries and dispatch adapters for JSON builtins.
//!
//! Called from:
//! - `crate::interpreter::builtins` module loading.
//! - `crate::interpreter::builtins::hooks` for migrated JSON dispatch.
//!
//! Key details:
//! - The JSON parser/encoder engine remains in `crate::interpreter::json`.
//! - This module only owns registry metadata and small hook adapters.

use super::super::{
    eval_builtin_json_decode, eval_builtin_json_encode, eval_builtin_json_last_error,
    eval_builtin_json_last_error_msg, eval_builtin_json_validate, eval_json_decode_result,
    eval_json_encode_result, eval_json_validate_result, ElephcEvalContext, ElephcEvalScope,
    EvalExpr, EvalStatus, RuntimeCellHandle, RuntimeValueOps,
};

mod json_decode;
mod json_encode;
mod json_last_error;
mod json_last_error_msg;
mod json_validate;

/// Dispatches direct expression-level calls for declaratively migrated JSON builtins.
pub(in crate::interpreter) fn eval_builtin_json_call(
    name: &str,
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    match name {
        "json_decode" => eval_builtin_json_decode(args, context, scope, values),
        "json_encode" => eval_builtin_json_encode(args, context, scope, values),
        "json_last_error" => eval_builtin_json_last_error(args, context, values),
        "json_last_error_msg" => eval_builtin_json_last_error_msg(args, context, values),
        "json_validate" => eval_builtin_json_validate(args, context, scope, values),
        _ => Err(EvalStatus::RuntimeFatal),
    }
}

/// Dispatches evaluated-argument calls for declaratively migrated JSON builtins.
pub(in crate::interpreter) fn eval_json_values_result(
    name: &str,
    evaluated_args: &[RuntimeCellHandle],
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    match name {
        "json_decode" => match evaluated_args {
            [json] => eval_json_decode_result(*json, None, None, None, context, values),
            [json, associative] => {
                eval_json_decode_result(*json, Some(*associative), None, None, context, values)
            }
            [json, associative, depth] => eval_json_decode_result(
                *json,
                Some(*associative),
                Some(*depth),
                None,
                context,
                values,
            ),
            [json, associative, depth, flags] => eval_json_decode_result(
                *json,
                Some(*associative),
                Some(*depth),
                Some(*flags),
                context,
                values,
            ),
            _ => Err(EvalStatus::RuntimeFatal),
        },
        "json_encode" => match evaluated_args {
            [value] => eval_json_encode_result(*value, None, None, context, values),
            [value, flags] => eval_json_encode_result(*value, Some(*flags), None, context, values),
            [value, flags, depth] => {
                eval_json_encode_result(*value, Some(*flags), Some(*depth), context, values)
            }
            _ => Err(EvalStatus::RuntimeFatal),
        },
        "json_last_error" => {
            if !evaluated_args.is_empty() {
                return Err(EvalStatus::RuntimeFatal);
            }
            values.int(context.json_last_error())
        }
        "json_last_error_msg" => {
            if !evaluated_args.is_empty() {
                return Err(EvalStatus::RuntimeFatal);
            }
            values.string(context.json_last_error_msg())
        }
        "json_validate" => match evaluated_args {
            [json] => eval_json_validate_result(*json, None, None, context, values),
            [json, depth] => eval_json_validate_result(*json, Some(*depth), None, context, values),
            [json, depth, flags] => {
                eval_json_validate_result(*json, Some(*depth), Some(*flags), context, values)
            }
            _ => Err(EvalStatus::RuntimeFatal),
        },
        _ => Err(EvalStatus::RuntimeFatal),
    }
}
