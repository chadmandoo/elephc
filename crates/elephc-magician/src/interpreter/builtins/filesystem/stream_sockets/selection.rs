//! Purpose:
//! Implements stream socket pairs and `stream_select()` over eval resources.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::stream_sockets` re-exports.
//!
//! Key details:
//! - `stream_select()` rewrites read/write/except arrays through by-reference
//!   targets after validating resource handles.

use super::*;

/// Evaluates `stream_socket_pair(domain, type, protocol)`.
pub(in crate::interpreter) fn eval_builtin_stream_socket_pair(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let [domain, socket_type, protocol] = args else {
        return Err(EvalStatus::RuntimeFatal);
    };
    let _ = eval_expr(domain, context, scope, values)?;
    let _ = eval_expr(socket_type, context, scope, values)?;
    let _ = eval_expr(protocol, context, scope, values)?;
    eval_stream_socket_pair_result(context, values)
}

/// Creates a pair of connected local stream resources.
pub(in crate::interpreter) fn eval_stream_socket_pair_result(
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let Some((left, right)) = context.stream_resources_mut().open_socket_pair() else {
        return values.bool_value(false);
    };
    let mut result = values.array_new(2)?;
    let key = values.int(0)?;
    let value = values.resource(left)?;
    result = values.array_set(result, key, value)?;
    let key = values.int(1)?;
    let value = values.resource(right)?;
    values.array_set(result, key, value)
}

/// Evaluates `stream_select()` over full eval call metadata.
pub(in crate::interpreter) fn eval_builtin_stream_select_call(
    args: &[EvalCallArg],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let evaluated_args = eval_call_arg_values(args, context, scope, values)?;
    let (bound, _) = bind_evaluated_ref_builtin_args(
        &["read", "write", "except", "seconds", "microseconds"],
        &evaluated_args,
        false,
    )?;
    let read = required_evaluated_ref_arg(&bound, 0)?;
    let write = required_evaluated_ref_arg(&bound, 1)?;
    let except = required_evaluated_ref_arg(&bound, 2)?;
    let seconds = required_evaluated_ref_arg(&bound, 3)?;
    let targets = vec![
        read.ref_target.clone().ok_or(EvalStatus::RuntimeFatal)?,
        write.ref_target.clone().ok_or(EvalStatus::RuntimeFatal)?,
        except.ref_target.clone().ok_or(EvalStatus::RuntimeFatal)?,
    ];
    let mut selected_args = vec![read.value, write.value, except.value, seconds.value];
    if let Some(microseconds) = optional_evaluated_ref_arg(&bound, 4) {
        selected_args.push(microseconds.value);
    }
    let result = eval_stream_select_result(&selected_args, context, values)?;
    eval_write_stream_select_empty_arrays(&targets, context, values)?;
    Ok(result)
}

/// Evaluates materialized `stream_select(...)` arguments.
pub(in crate::interpreter) fn eval_stream_select_result(
    evaluated_args: &[RuntimeCellHandle],
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    if !(4..=5).contains(&evaluated_args.len()) {
        return Err(EvalStatus::RuntimeFatal);
    }
    for array in evaluated_args.iter().take(3) {
        eval_stream_select_cast_array(*array, context, values)?;
    }
    values.int(0)
}

/// Writes conservative empty readiness arrays back to `stream_select()` lvalues.
fn eval_write_stream_select_empty_arrays(
    targets: &[EvalReferenceTarget],
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<(), EvalStatus> {
    for target in targets {
        let value = values.array_new(0)?;
        eval_write_direct_ref_target(
            target,
            value,
            context,
            values,
            Some(ScopeCellOwnership::Owned),
        )?;
    }
    Ok(())
}

/// Invokes `stream_cast(STREAM_CAST_FOR_SELECT)` for wrapper resources in an array.
fn eval_stream_select_cast_array(
    array: RuntimeCellHandle,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<(), EvalStatus> {
    if !values.is_array_like(array)? {
        return Ok(());
    }
    let len = values.array_len(array)?;
    for position in 0..len {
        let key = values.array_iter_key(array, position)?;
        let value = values.array_get(array, key)?;
        eval_stream_select_cast_value(value, context, values)?;
    }
    Ok(())
}

/// Invokes `stream_cast()` for one userspace-wrapper stream resource value.
fn eval_stream_select_cast_value(
    value: RuntimeCellHandle,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<(), EvalStatus> {
    if values.type_tag(value)? != EVAL_TAG_RESOURCE {
        return Ok(());
    }
    let display_id = eval_int_value(value, values)?;
    let Some(id) = display_id.checked_sub(1) else {
        return Ok(());
    };
    let Some(result) =
        eval_user_wrapper_stream_cast_result(id, EVAL_STREAM_CAST_FOR_SELECT, context, values)?
    else {
        return Ok(());
    };
    values.release(result)
}

/// Converts a runtime resource cell into eval's zero-based socket id.
pub(super) fn eval_socket_resource_id(
    resource: RuntimeCellHandle,
    values: &mut impl RuntimeValueOps,
) -> Result<i64, EvalStatus> {
    if values.type_tag(resource)? != EVAL_TAG_RESOURCE {
        return Err(EvalStatus::RuntimeFatal);
    }
    let display_id = eval_int_value(resource, values)?;
    display_id.checked_sub(1).ok_or(EvalStatus::RuntimeFatal)
}
