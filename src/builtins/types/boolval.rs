//! Purpose:
//! Home of the PHP `boolval` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin with no check hook; arity and arg inference are handled by the registry common path.


builtin! {
    name: "boolval",
    area: Types,
    params: [value: Mixed],
    returns: Bool,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::Boolval,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the boolean value of a variable.",
    php_manual: "function.boolval",
}
