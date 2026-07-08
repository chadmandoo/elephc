//! Purpose:
//! Builds PHP metadata arrays for eval-local stream resources.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::streams::eval_unary_stream_result`.
//!
//! Key details:
//! - Metadata mirrors eval's local stream table, not host PHP stream wrappers.

use super::*;

/// Builds PHP's stream metadata array for one eval-local stream resource.
pub(in crate::interpreter) fn eval_stream_get_meta_data_result(
    id: i64,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let Some(meta) = context.stream_resources().meta_data(id) else {
        return values.bool_value(false);
    };
    let mut result = values.assoc_new(9)?;
    result = eval_stream_meta_set_bool(result, "timed_out", false, values)?;
    result = eval_stream_meta_set_bool(result, "blocked", true, values)?;
    result = eval_stream_meta_set_bool(result, "eof", meta.eof, values)?;
    result = eval_stream_meta_set_string(result, "wrapper_type", "plainfile", values)?;
    result = eval_stream_meta_set_string(result, "stream_type", "STDIO", values)?;
    result = eval_stream_meta_set_string(result, "mode", &meta.mode, values)?;
    result = eval_stream_meta_set_int(result, "unread_bytes", 0, values)?;
    result = eval_stream_meta_set_bool(result, "seekable", true, values)?;
    eval_stream_meta_set_string(result, "uri", &meta.uri, values)
}

/// Inserts a boolean field into the stream metadata array.
fn eval_stream_meta_set_bool(
    array: RuntimeCellHandle,
    key: &str,
    value: bool,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let key = values.string(key)?;
    let value = values.bool_value(value)?;
    values.array_set(array, key, value)
}

/// Inserts an integer field into the stream metadata array.
fn eval_stream_meta_set_int(
    array: RuntimeCellHandle,
    key: &str,
    value: i64,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let key = values.string(key)?;
    let value = values.int(value)?;
    values.array_set(array, key, value)
}

/// Inserts a string field into the stream metadata array.
fn eval_stream_meta_set_string(
    array: RuntimeCellHandle,
    key: &str,
    value: &str,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let key = values.string(key)?;
    let value = values.string(value)?;
    values.array_set(array, key, value)
}
