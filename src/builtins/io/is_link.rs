//! Purpose:
//! Home of the PHP `is_link` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `is_link` is a pure-data builtin whose return type
//!   (`Bool`) is fully determined by its declaration. The registry common path
//!   infers the argument and enforces arity before falling back to `returns`.
//! - `lower` is a thin wrapper over `io::lower_is_link` in the EIR backend.


builtin! {
    name: "is_link",
    area: Io,
    params: [filename: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::IsLink,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Tells whether the filename is a symbolic link.",
    php_manual: "function.is-link",
}
