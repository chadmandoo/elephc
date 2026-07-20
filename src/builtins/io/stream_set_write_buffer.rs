//! Purpose:
//! Home of the PHP `stream_set_write_buffer` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers both arguments and returns `Int`
//!   (0 on success, matching PHP's successful no-op behaviour).
//! - `lower` dispatches to `io::lower_stream_set_buffer`, shared with `stream_set_read_buffer`.


builtin! {
    name: "stream_set_write_buffer",
    area: Io,
    params: [stream: Mixed, size: Int],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamSetWriteBuffer,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Sets the write file buffering on a stream.",
    php_manual: "function.stream-set-write-buffer",
}
