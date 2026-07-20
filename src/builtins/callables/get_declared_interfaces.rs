//! Purpose:
//! Home of the PHP `get_declared_interfaces` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook via support),
//!   and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - Check hook returns `Array<Str>` unconditionally (zero-arg builtin).
//! - `lower` is a thin wrapper over `types::lower_get_declared_names` parameterized
//!   with this builtin's name.


builtin! {
    name: "get_declared_interfaces",
    area: Callables,
    params: [],
    returns: Mixed,
    check: crate::builtins::callables::support::check_declared_names,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::GetDeclaredInterfaces,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Returns an array of all declared interfaces.",
    php_manual: "function.get-declared-interfaces",
}
