//! Purpose:
//! Registers PHP's `is_real` alias with the shared float-predicate semantics.
//!
//! Called from:
//! - The builtin registry through `crate::builtins::types`.
//!
//! Key details:
//! - The alias uses the same typed EIR target as `is_float`.

builtin! {
    name: "is_real",
    area: Types,
    params: [value: Mixed],
    returns: Bool,
    semantics: crate::builtins::semantics::runtime_target_semantics(
        crate::ir::BuiltinRuntimeTarget::IsFloat,
        crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Alias of is_float().",
    php_manual: "function.is-real",
}
