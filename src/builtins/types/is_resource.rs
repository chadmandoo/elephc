//! Purpose:
//! Home of the PHP `is_resource` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin with no check hook; arity and arg inference are handled by the registry common path.


builtin! {
    name: "is_resource",
    area: Types,
    params: [value: Mixed],
    returns: Bool,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::IsResource,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Checks whether a variable is a resource.",
    php_manual: "function.is-resource",
}
