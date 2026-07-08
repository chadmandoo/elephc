//! Purpose:
//! Accepts stream socket connections and exposes socket metadata/lifecycle calls.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::stream_sockets` re-exports.
//!
//! Key details:
//! - TLS enablement is conservative: disabling succeeds for valid streams while
//!   enabling reports false because eval does not manage TLS state.

use super::*;

/// Evaluates `stream_socket_accept()` over full eval call metadata.
pub(in crate::interpreter) fn eval_builtin_stream_socket_accept_call(
    args: &[EvalCallArg],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let evaluated_args = eval_call_arg_values(args, context, scope, values)?;
    let (bound, _) = bind_evaluated_ref_builtin_args(
        &["socket", "timeout", "peer_name"],
        &evaluated_args,
        false,
    )?;
    let socket = required_evaluated_ref_arg(&bound, 0)?;
    let peer_name_target = optional_evaluated_ref_arg(&bound, 2)
        .map(|arg| arg.ref_target.clone().ok_or(EvalStatus::RuntimeFatal))
        .transpose()?;
    let (result, peer_name) =
        eval_stream_socket_accept_with_peer_result(socket.value, context, values)?;
    eval_write_socket_output_ref_target(peer_name_target.as_ref(), peer_name, context, values)?;
    Ok(result)
}

/// Accepts one pending TCP connection from a listener resource.
pub(in crate::interpreter) fn eval_stream_socket_accept_result(
    socket: RuntimeCellHandle,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let id = eval_socket_resource_id(socket, values)?;
    match context.stream_resources_mut().accept_tcp(id) {
        Some(id) => values.resource(id),
        None => values.bool_value(false),
    }
}

/// Accepts one TCP connection and returns the accepted resource plus peer endpoint name.
pub(in crate::interpreter) fn eval_stream_socket_accept_with_peer_result(
    socket: RuntimeCellHandle,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<(RuntimeCellHandle, Option<String>), EvalStatus> {
    let id = eval_socket_resource_id(socket, values)?;
    let Some(accepted_id) = context.stream_resources_mut().accept_tcp(id) else {
        return values.bool_value(false).map(|result| (result, None));
    };
    let peer_name = context.stream_resources().socket_name(accepted_id, true);
    let result = values.resource(accepted_id)?;
    Ok((result, peer_name))
}

/// Evaluates `stream_socket_get_name(socket, remote)`.
pub(in crate::interpreter) fn eval_builtin_stream_socket_get_name(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let [socket, remote] = args else {
        return Err(EvalStatus::RuntimeFatal);
    };
    let socket = eval_expr(socket, context, scope, values)?;
    let remote = eval_expr(remote, context, scope, values)?;
    eval_stream_socket_get_name_result(socket, remote, context, values)
}

/// Returns a tracked local or remote socket endpoint name.
pub(in crate::interpreter) fn eval_stream_socket_get_name_result(
    socket: RuntimeCellHandle,
    remote: RuntimeCellHandle,
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let id = eval_socket_resource_id(socket, values)?;
    let remote = values.truthy(remote)?;
    match context.stream_resources().socket_name(id, remote) {
        Some(name) => values.string(&name),
        None => values.bool_value(false),
    }
}

/// Evaluates `stream_socket_shutdown(stream, mode)`.
pub(in crate::interpreter) fn eval_builtin_stream_socket_shutdown(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let [stream, mode] = args else {
        return Err(EvalStatus::RuntimeFatal);
    };
    let stream = eval_expr(stream, context, scope, values)?;
    let mode = eval_expr(mode, context, scope, values)?;
    eval_stream_socket_shutdown_result(stream, mode, context, values)
}

/// Applies a socket shutdown mode to a stream resource.
pub(in crate::interpreter) fn eval_stream_socket_shutdown_result(
    stream: RuntimeCellHandle,
    mode: RuntimeCellHandle,
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let id = eval_socket_resource_id(stream, values)?;
    let mode = eval_int_value(mode, values)?;
    values.bool_value(
        context
            .stream_resources()
            .socket_shutdown(id, mode)
            .unwrap_or(false),
    )
}

/// Evaluates `stream_socket_enable_crypto(...)`.
pub(in crate::interpreter) fn eval_builtin_stream_socket_enable_crypto(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    if !(2..=4).contains(&args.len()) {
        return Err(EvalStatus::RuntimeFatal);
    }
    let stream = eval_expr(&args[0], context, scope, values)?;
    let enable = eval_expr(&args[1], context, scope, values)?;
    for arg in &args[2..] {
        let _ = eval_expr(arg, context, scope, values)?;
    }
    eval_stream_socket_enable_crypto_result(stream, enable, context, values)
}

/// Returns TLS enablement status for eval socket streams.
pub(in crate::interpreter) fn eval_stream_socket_enable_crypto_result(
    stream: RuntimeCellHandle,
    enable: RuntimeCellHandle,
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let id = eval_socket_resource_id(stream, values)?;
    if !context.stream_resources().has_stream(id) {
        return values.bool_value(false);
    }
    let disabled = !values.truthy(enable)?;
    values.bool_value(disabled)
}
