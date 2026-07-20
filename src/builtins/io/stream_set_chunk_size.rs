//! Purpose:
//! Home of the PHP `stream_set_chunk_size` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers both arguments and returns `Int`
//!   (the previous chunk size, or the PHP default of 8192 on failure).
//! - `lower` is a thin wrapper over `io::lower_stream_set_chunk_size` in the EIR backend.


builtin! {
    name: "stream_set_chunk_size",
    area: Io,
    params: [stream: Mixed, size: Int],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamSetChunkSize,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Sets the read chunk size on a stream.",
    php_manual: "function.stream-set-chunk-size",
}
