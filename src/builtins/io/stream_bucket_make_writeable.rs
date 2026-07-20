//! Purpose:
//! Home of the PHP `stream_bucket_make_writeable` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers the single argument and returns `Mixed`.
//! - `lower` is a thin wrapper over `io::lower_stream_bucket_make_writeable` in the EIR backend.


builtin! {
    name: "stream_bucket_make_writeable",
    area: Io,
    params: [brigade: Mixed],
    returns: Mixed,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamBucketMakeWriteable,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns a bucket object from the brigade for use in a stream filter.",
    php_manual: "function.stream-bucket-make-writeable",
}
