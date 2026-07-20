//! Purpose:
//! Home of the PHP `is_nan` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook), via `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin with no check hook; arity and arg inference are handled by the registry common path.
//! - The parameter is named `num` (matching the PHP golden signature), not `value`.
//! - `lower` is a thin wrapper over the EIR math-module NaN-predicate emitter.


builtin! {
    name: "is_nan",
    area: Types,
    params: [num: Float],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::IsNan,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Checks whether a float is NAN.",
    php_manual: "function.is-nan",
}
