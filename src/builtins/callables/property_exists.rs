//! Purpose:
//! Registers PHP's `property_exists` metadata lookup as a typed builtin operation.
//!
//! Called from:
//! - The builtin registry through `crate::builtins::callables`.
//!
//! Key details:
//! - Static class metadata and eval-aware lookup remain backend implementation details.

builtin! {
    name: "property_exists",
    area: Callables,
    params: [object_or_class: Mixed, property: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::runtime_target_semantics(
        crate::ir::BuiltinRuntimeTarget::PropertyExists,
        crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Checks whether an object or class has a property.",
    php_manual: "function.property-exists",
}
