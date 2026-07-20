//! Purpose:
//! Home of the PHP `floatval` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin with no check hook; arity and arg inference are handled by the registry common path.


builtin! {
    name: "floatval",
    area: Types,
    params: [value: Mixed],
    returns: Float,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::Floatval,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the float value of a variable.",
    php_manual: "function.floatval",
}
