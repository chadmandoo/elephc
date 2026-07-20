//! Purpose:
//! Home of the PHP `getcwd` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook: `getcwd` is a pure-data builtin whose `Str` return type is
//!   fully determined by its declaration. The registry common path enforces its
//!   0-argument arity before falling back to `returns`.
//! - `lower` is a thin wrapper over `io::lower_getcwd` in the EIR backend.


builtin! {
    name: "getcwd",
    area: Io,
    params: [],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Getcwd,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Gets the current working directory.",
    php_manual: "function.getcwd",
}
