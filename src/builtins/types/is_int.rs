//! Purpose:
//! Home of the PHP `is_int` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin with no check hook; arity and arg inference are handled by the registry common path.

builtin! {
    name: "is_int",
    area: Types,
    params: [value: Mixed],
    returns: Bool,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::IsInt,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Checks whether a variable is an integer.",
    php_manual: "function.is-int",
}
