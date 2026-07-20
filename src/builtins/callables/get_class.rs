//! Purpose:
//! Home of the PHP `get_class` builtin: its declaration and semantic metadata.
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
    name: "get_class",
    area: Callables,
    params: [object: Mixed = DefaultSpec::Null],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::GetClass,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the name of the class of an object.",
    php_manual: "function.get-class",
}
