//! Purpose:
//! Home of the PHP `gethostname` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the common registry path infers no arguments and returns `Str`.


builtin! {
    name: "gethostname",
    area: Io,
    params: [],
    returns: Str,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::Gethostname,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Gets the standard host name for the local machine.",
    php_manual: "function.gethostname",
}
