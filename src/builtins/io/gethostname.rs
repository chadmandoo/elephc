//! Purpose:
//! Home of the PHP `gethostname` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers no arguments and returns `Str`.
//! - `lower` dispatches to `io::lower_gethostname` in the EIR backend.


builtin! {
    name: "gethostname",
    area: Io,
    params: [],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Gethostname,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Gets the standard host name for the local machine.",
    php_manual: "function.gethostname",
}
