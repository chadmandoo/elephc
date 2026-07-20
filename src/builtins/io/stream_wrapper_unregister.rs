//! Purpose:
//! Home of the PHP `stream_wrapper_unregister` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers the protocol argument and returns `Bool`.
//! - `lower` is a thin wrapper over `io::lower_stream_wrapper_unregister` in the EIR backend.


builtin! {
    name: "stream_wrapper_unregister",
    area: Io,
    params: [protocol: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamWrapperUnregister,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Unregisters a previously registered URL wrapper.",
    php_manual: "function.stream-wrapper-unregister",
}
