//! Purpose:
//! Home of the PHP `stream_socket_pair` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers the three Int arguments and returns `Mixed`.
//! - PHP returns `array|false`; the builtin emitter widens the success array's slots through
//!   `__rt_array_to_mixed` so the value flows through Mixed pipelines without per-call
//!   special-casing. `Mixed` for the static type keeps every consumer happy.
//! - `lower` dispatches to `io::lower_stream_socket_pair` in the EIR backend.


builtin! {
    name: "stream_socket_pair",
    area: Io,
    params: [domain: Int, type: Int, protocol: Int],
    returns: Mixed,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamSocketPair,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Creates a pair of connected, indistinguishable socket streams.",
    php_manual: "function.stream-socket-pair",
}
