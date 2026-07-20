//! Purpose:
//! Home of the PHP `rename` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook: `rename` is a pure-data builtin whose `Bool` return type is
//!   fully determined by its declaration. The registry common path infers the
//!   arguments and enforces the exactly-2-argument arity before falling back to
//!   `returns`.
//! - `lower` is a thin wrapper over `io::lower_rename` in the EIR backend.


builtin! {
    name: "rename",
    area: Io,
    params: [from: Str, to: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Rename,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Renames a file or directory.",
    php_manual: "function.rename",
}
