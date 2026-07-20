//! Purpose:
//! Home of the PHP `stream_bucket_append` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers both arguments and returns `Void`.
//! - `lower` dispatches to `io::lower_stream_bucket_append_or_prepend` in the EIR backend.


builtin! {
    name: "stream_bucket_append",
    area: Io,
    params: [brigade: Mixed, bucket: Mixed],
    returns: Void,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamBucketAppend,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Appends a bucket to the brigade.",
    php_manual: "function.stream-bucket-append",
}
