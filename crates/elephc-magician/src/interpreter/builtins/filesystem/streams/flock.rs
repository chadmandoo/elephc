//! Purpose:
//! Source-sensitive `flock` handling for eval stream resources.
//!
//! Called from:
//! - `crate::interpreter::eval_call` before generic builtin dispatch.
//! - Filesystem stream declaration dispatchers for by-value callable calls.
//!
//! Key details:
//! - Direct calls keep the optional `$would_block` output parameter writable.

use super::*;

/// Evaluates PHP `flock($stream, $operation, &$would_block = null)` over eval call args.
pub(in crate::interpreter) fn eval_builtin_flock(
    args: &[EvalCallArg],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let (stream, operation, would_block_target) =
        eval_flock_direct_args(args, context, scope, values)?;
    let (success, would_block) = eval_flock_result(stream, operation, context, values)?;
    if let Some(target) = would_block_target {
        let value = values.bool_value(would_block)?;
        eval_write_direct_ref_target(
            &target,
            value,
            context,
            values,
            Some(ScopeCellOwnership::Owned),
        )?;
    }
    values.bool_value(success)
}

/// Applies a materialized PHP `flock()` operation to a local eval stream resource.
pub(in crate::interpreter) fn eval_flock_result(
    stream: RuntimeCellHandle,
    operation: RuntimeCellHandle,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<(bool, bool), EvalStatus> {
    let id = eval_stream_resource_id(stream, values)?;
    let operation = eval_int_value(operation, values)?;
    if let Some(success) = eval_user_wrapper_flock_result(id, operation, context, values)? {
        return Ok((success, false));
    }
    Ok(context
        .stream_resources()
        .flock(id, operation)
        .unwrap_or((false, false)))
}

/// Evaluates and binds direct `flock()` arguments while keeping by-ref output writable.
fn eval_flock_direct_args(
    args: &[EvalCallArg],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<
    (
        RuntimeCellHandle,
        RuntimeCellHandle,
        Option<EvalReferenceTarget>,
    ),
    EvalStatus,
> {
    let mut stream = None;
    let mut operation = None;
    let mut would_block = None;
    let mut positional_index = 0;
    let mut saw_named = false;

    for arg in args {
        if arg.is_spread() {
            return Err(EvalStatus::RuntimeFatal);
        }
        let parameter = if let Some(name) = arg.name() {
            saw_named = true;
            name
        } else {
            if saw_named {
                return Err(EvalStatus::RuntimeFatal);
            }
            let parameter = match positional_index {
                0 => "stream",
                1 => "operation",
                2 => "would_block",
                _ => return Err(EvalStatus::RuntimeFatal),
            };
            positional_index += 1;
            parameter
        };

        match parameter {
            "stream" => {
                if stream.is_some() {
                    return Err(EvalStatus::RuntimeFatal);
                }
                stream = Some(eval_expr(arg.value(), context, scope, values)?);
            }
            "operation" => {
                if operation.is_some() {
                    return Err(EvalStatus::RuntimeFatal);
                }
                operation = Some(eval_expr(arg.value(), context, scope, values)?);
            }
            "would_block" => {
                if would_block.is_some() {
                    return Err(EvalStatus::RuntimeFatal);
                }
                let (_, target) = eval_call_arg_value(arg.value(), context, scope, values)?;
                would_block = Some(target.ok_or(EvalStatus::RuntimeFatal)?);
            }
            _ => return Err(EvalStatus::RuntimeFatal),
        }
    }

    let stream = stream.ok_or(EvalStatus::RuntimeFatal)?;
    let operation = operation.ok_or(EvalStatus::RuntimeFatal)?;
    Ok((stream, operation, would_block))
}
