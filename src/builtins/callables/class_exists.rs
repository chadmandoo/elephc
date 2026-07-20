//! Purpose:
//! Home of the PHP `class_exists` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook via support),
//!   and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - The check hook validates that the first argument is a string literal and the
//!   optional autoload argument is a literal bool or int (AOT constraint).
//! - Arguments are pre-inferred by the registry common path before the hook runs.
//! - `lower` is a thin wrapper over `lower_class_like_exists` parameterized with
//!   this builtin's name.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "class_exists",
    area: Callables,
    params: [class: Str, autoload: Bool = DefaultSpec::Bool(true)],
    returns: Bool,
    check: crate::builtins::callables::support::check_class_like_exists,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ClassExists,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Checks whether the class has been defined.",
    php_manual: "function.class-exists",
}
