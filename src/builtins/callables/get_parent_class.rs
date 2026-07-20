//! Purpose:
//! Home of the PHP `get_parent_class` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through
//!   `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the registry common path infers the optional argument and
//!   returns the declared `Str` type.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "get_parent_class",
    area: Callables,
    params: [object_or_class: Mixed = DefaultSpec::Null],
    returns: Str,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::GetParentClass,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the name of the parent class of an object or class.",
    php_manual: "function.get-parent-class",
}
