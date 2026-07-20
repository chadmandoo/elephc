//! Purpose:
//! Home of the PHP `is_file` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `is_file` is a pure-data builtin whose return type
//!   (`Bool`) is fully determined by its declaration. The registry common path
//!   infers the argument and enforces arity before falling back to `returns`.
//! - `lower` is a thin wrapper over `io::lower_is_file` in the EIR backend.


builtin! {
    name: "is_file",
    area: Io,
    params: [filename: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::IsFile,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Tells whether the filename is a regular file.",
    php_manual: "function.is-file",
}
