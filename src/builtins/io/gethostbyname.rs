//! Purpose:
//! Home of the PHP `gethostbyname` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers the hostname argument and returns `Str`.
//! - `lower` dispatches to `io::lower_gethostbyname` in the EIR backend.


builtin! {
    name: "gethostbyname",
    area: Io,
    params: [hostname: Str],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Gethostbyname,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Gets the IPv4 address corresponding to the given Internet host name.",
    php_manual: "function.gethostbyname",
}
