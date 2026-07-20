//! Purpose:
//! Home of the PHP `copy` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through
//!   `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook: `copy` is a pure-data builtin whose `Bool` return type is
//!   fully determined by its declaration. The registry common path infers the
//!   arguments and enforces the exactly-2-argument arity before falling back to
//!   `returns`.


builtin! {
    name: "copy",
    area: Io,
    params: [from: Str, to: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::Copy,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Copies a file.",
    php_manual: "function.copy",
}
