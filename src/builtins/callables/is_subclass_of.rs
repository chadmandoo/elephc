//! Purpose:
//! Home of the PHP `is_subclass_of` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the registry common path infers all arguments and returns
//!   the declared `Bool` type.
//! - `allow_string` defaults to `true` (PHP's default for `is_subclass_of`).
//! - `lower` is a thin wrapper over `types::lower_is_a_relation` parameterized
//!   with this builtin's name.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "is_subclass_of",
    area: Callables,
    params: [object_or_class: Mixed, class: Str, allow_string: Bool = DefaultSpec::Bool(true)],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::IsSubclassOf,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Checks if the object has a given class as one of its parents or implements it.",
    php_manual: "function.is-subclass-of",
}
