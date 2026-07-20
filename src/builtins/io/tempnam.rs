//! Purpose:
//! Home of the PHP `tempnam` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook: `tempnam` is a pure-data builtin whose `Str` return type is
//!   fully determined by its declaration. The registry common path infers the
//!   arguments and enforces the exactly-2-argument arity before falling back to
//!   `returns`.
//! - `lower` is a thin wrapper over `io::lower_tempnam` in the EIR backend.


builtin! {
    name: "tempnam",
    area: Io,
    params: [directory: Str, prefix: Str],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Tempnam,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Creates a file with a unique filename.",
    php_manual: "function.tempnam",
}
