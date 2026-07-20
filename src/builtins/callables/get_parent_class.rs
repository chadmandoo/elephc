//! Purpose:
//! Home of the PHP `get_parent_class` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the registry common path infers the optional argument and
//!   returns the declared `Str` type.
//! - `lower` is a thin wrapper over `types::lower_class_name_lookup` parameterized
//!   with this builtin's name.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "get_parent_class",
    area: Callables,
    params: [object_or_class: Mixed = DefaultSpec::Null],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::GetParentClass,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the name of the parent class of an object or class.",
    php_manual: "function.get-parent-class",
}
