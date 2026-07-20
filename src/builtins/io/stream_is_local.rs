//! Purpose:
//! Home of the PHP `stream_is_local` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers the stream argument and returns `Bool`.
//! - `lower` is a thin wrapper over `io::lower_stream_is_local` in the EIR backend.


builtin! {
    name: "stream_is_local",
    area: Io,
    params: [stream: Mixed],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamIsLocal,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Checks if a stream is a local stream.",
    php_manual: "function.stream-is-local",
}
