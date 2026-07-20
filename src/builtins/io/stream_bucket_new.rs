//! Purpose:
//! Home of the PHP `stream_bucket_new` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers both arguments and returns `Mixed`.
//! - `lower` is a thin wrapper over `io::lower_stream_bucket_new` in the EIR backend.


builtin! {
    name: "stream_bucket_new",
    area: Io,
    params: [stream: Mixed, buffer: Str],
    returns: Mixed,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamBucketNew,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Creates a new bucket for use in a stream filter.",
    php_manual: "function.stream-bucket-new",
}
