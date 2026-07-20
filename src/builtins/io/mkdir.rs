//! Purpose:
//! Home of the PHP `mkdir` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook: `mkdir` is a pure-data builtin whose `Bool` return type is
//!   fully determined by its declaration. Unlike `unlink`, `mkdir` has no PHAR
//!   side effect, so no library-linking check hook is required. The registry
//!   common path infers the argument and enforces the exactly-1-argument arity
//!   before falling back to `returns`.
//! - `lower` is a thin wrapper over `io::lower_mkdir` in the EIR backend.


builtin! {
    name: "mkdir",
    area: Io,
    params: [directory: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Mkdir,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Makes a directory.",
    php_manual: "function.mkdir",
}
