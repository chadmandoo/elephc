//! Purpose:
//! Home of the PHP `stream_context_set_params` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers both arguments and returns `Bool`.
//! - `lower` is a thin wrapper over `io::lower_stream_context_set_params` in the EIR backend.


builtin! {
    name: "stream_context_set_params",
    area: Io,
    params: [context: Mixed, params: Mixed],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamContextSetParams,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Sets parameters on the specified context.",
    php_manual: "function.stream-context-set-params",
}
