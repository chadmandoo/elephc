//! Purpose:
//! Home of the PHP `is_null` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), via `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin with no check hook; arity and arg inference are handled by the registry common path.
//! - `lower` is a thin wrapper over the shared null-predicate emitter.


builtin! {
    name: "is_null",
    area: Types,
    params: [value: Mixed],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::IsNull,
            crate::builtins::semantics::BuiltinTargetStrategy::EirPrimitive,
    ),
    summary: "Checks whether a variable is null.",
    php_manual: "function.is-null",
}
