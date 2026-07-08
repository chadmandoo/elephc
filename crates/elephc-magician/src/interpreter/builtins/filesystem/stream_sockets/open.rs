//! Purpose:
//! Opens TCP stream socket resources for eval stream socket builtins.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::stream_sockets` re-exports.
//!
//! Key details:
//! - Opened sockets enter eval's normal stream table so existing stream IO
//!   helpers own reads, writes, and close behavior.

use super::*;

/// Evaluates `stream_socket_server(address)`.
pub(in crate::interpreter) fn eval_builtin_stream_socket_server(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let [address] = args else {
        return Err(EvalStatus::RuntimeFatal);
    };
    let address = eval_expr(address, context, scope, values)?;
    eval_stream_socket_server_result(address, context, values)
}

/// Opens a TCP listener resource.
pub(in crate::interpreter) fn eval_stream_socket_server_result(
    address: RuntimeCellHandle,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let address = eval_path_string(address, values)?;
    match context.stream_resources_mut().open_tcp_listener(&address) {
        Some(id) => values.resource(id),
        None => values.bool_value(false),
    }
}

/// Evaluates `stream_socket_client(address)`.
pub(in crate::interpreter) fn eval_builtin_stream_socket_client(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let [address] = args else {
        return Err(EvalStatus::RuntimeFatal);
    };
    let address = eval_expr(address, context, scope, values)?;
    eval_stream_socket_client_result(address, context, values)
}

/// Opens a connected TCP stream resource.
pub(in crate::interpreter) fn eval_stream_socket_client_result(
    address: RuntimeCellHandle,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let address = eval_path_string(address, values)?;
    match context.stream_resources_mut().open_tcp_stream(&address) {
        Some(id) => values.resource(id),
        None => values.bool_value(false),
    }
}

/// Evaluates `fsockopen()` or `pfsockopen()` over full eval call metadata.
pub(in crate::interpreter) fn eval_builtin_fsockopen_call(
    args: &[EvalCallArg],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let evaluated_args = eval_call_arg_values(args, context, scope, values)?;
    let (bound, _) = bind_evaluated_ref_builtin_args(
        &["hostname", "port", "error_code", "error_message", "timeout"],
        &evaluated_args,
        false,
    )?;
    let host = required_evaluated_ref_arg(&bound, 0)?;
    let port = required_evaluated_ref_arg(&bound, 1)?;
    let error_code_target = optional_evaluated_ref_arg(&bound, 2)
        .map(|arg| arg.ref_target.clone().ok_or(EvalStatus::RuntimeFatal))
        .transpose()?;
    let error_message_target = optional_evaluated_ref_arg(&bound, 3)
        .map(|arg| arg.ref_target.clone().ok_or(EvalStatus::RuntimeFatal))
        .transpose()?;
    let (result, error_code, error_message) =
        eval_fsockopen_with_error_result(host.value, port.value, context, values)?;
    eval_write_socket_int_output_ref_target(
        error_code_target.as_ref(),
        error_code,
        context,
        values,
    )?;
    eval_write_socket_output_ref_target(
        error_message_target.as_ref(),
        Some(error_message),
        context,
        values,
    )?;
    Ok(result)
}

/// Opens a connected TCP stream from host and port cells.
pub(in crate::interpreter) fn eval_fsockopen_result(
    host: RuntimeCellHandle,
    port: RuntimeCellHandle,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let host = eval_path_string(host, values)?;
    let port = eval_int_value(port, values)?;
    match context
        .stream_resources_mut()
        .open_tcp_stream_host_port(&host, port)
    {
        Some(id) => values.resource(id),
        None => values.bool_value(false),
    }
}

/// Opens a host/port TCP stream and returns PHP `fsockopen()` error outputs.
pub(in crate::interpreter) fn eval_fsockopen_with_error_result(
    host: RuntimeCellHandle,
    port: RuntimeCellHandle,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<(RuntimeCellHandle, i64, String), EvalStatus> {
    let host = eval_path_string(host, values)?;
    let port = eval_int_value(port, values)?;
    match context
        .stream_resources_mut()
        .open_tcp_stream_host_port_result(&host, port)
    {
        Ok(id) => Ok((values.resource(id)?, 0, String::new())),
        Err(error) => {
            let error_code = i64::from(error.raw_os_error().unwrap_or(0));
            Ok((values.bool_value(false)?, error_code, error.to_string()))
        }
    }
}
