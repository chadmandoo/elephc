//! Purpose:
//! Home of the PHP `link` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook: `link` is a pure-data builtin whose `Bool` return type is
//!   fully determined by its declaration. The registry common path infers the
//!   arguments and enforces the exactly-2-argument arity before falling back to
//!   `returns`.
//! - `lower` is a thin wrapper over `io::lower_link` in the EIR backend.


builtin! {
    name: "link",
    area: Io,
    params: [target: Str, link: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Link,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Creates a hard link.",
    php_manual: "function.link",
}
