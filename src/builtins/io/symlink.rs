//! Purpose:
//! Home of the PHP `symlink` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through
//!   `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook: `symlink` is a pure-data builtin whose `Bool` return type is
//!   fully determined by its declaration. The registry common path infers the
//!   arguments and enforces the exactly-2-argument arity before falling back to
//!   `returns`.


builtin! {
    name: "symlink",
    area: Io,
    params: [target: Str, link: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::Symlink,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Creates a symbolic link.",
    php_manual: "function.symlink",
}
