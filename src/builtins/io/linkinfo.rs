//! Purpose:
//! Home of the PHP `linkinfo` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `linkinfo` is a pure-data builtin whose return type
//!   (`Int`) is fully determined by its declaration. The registry common path
//!   infers the argument and enforces arity before falling back to `returns`.
//! - `lower` is a thin wrapper over `io::lower_linkinfo` in the EIR backend.


builtin! {
    name: "linkinfo",
    area: Io,
    params: [path: Str],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Linkinfo,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Gets information about a link.",
    php_manual: "function.linkinfo",
}
