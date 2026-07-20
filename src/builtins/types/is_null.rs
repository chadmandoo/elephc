//! Purpose:
//! Home of the PHP `is_null` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin with no check hook; arity and arg inference are handled by the registry common path.


builtin! {
    name: "is_null",
    area: Types,
    params: [value: Mixed],
    returns: Bool,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::IsNull,
            crate::builtins::semantics::BuiltinTargetStrategy::EirPrimitive,
    ),
    summary: "Checks whether a variable is null.",
    php_manual: "function.is-null",
}
