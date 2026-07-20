//! Purpose:
//! Home of the PHP `is_a` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   all via `crate::builtins::registry`.
//!
//! Key details:
//! - No check hook: the registry common path infers all arguments and returns
//!   the declared `Bool` type.
//! - `allow_string` defaults to `false` (PHP's default for `is_a`).
//! - `lower` is a thin wrapper over `types::lower_is_a_relation` parameterized
//!   with this builtin's name.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "is_a",
    area: Callables,
    params: [object_or_class: Mixed, class: Str, allow_string: Bool = DefaultSpec::Bool(false)],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::IsA,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Checks whether an object is of a given type or has it as one of its parents.",
    php_manual: "function.is-a",
}
