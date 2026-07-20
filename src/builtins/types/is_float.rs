//! Purpose:
//! Home of the PHP `is_float` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin with no check hook; arity and arg inference are handled by the registry common path.

builtin! {
    name: "is_float",
    area: Types,
    params: [value: Mixed],
    returns: Bool,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::IsFloat,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Checks whether a variable is a floating-point number.",
    php_manual: "function.is-float",
}
