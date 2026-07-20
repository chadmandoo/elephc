//! Purpose:
//! Home of the PHP `stream_resolve_include_path` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers the filename argument and returns `Mixed`.
//! - `returns: Mixed` reflects the `string|false` PHP return type.
//! - `lower` is a thin wrapper over `io::lower_stream_resolve_include_path` in the EIR backend.


builtin! {
    name: "stream_resolve_include_path",
    area: Io,
    params: [filename: Str],
    returns: Mixed,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamResolveIncludePath,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Resolves filename against the include path.",
    php_manual: "function.stream-resolve-include-path",
}
