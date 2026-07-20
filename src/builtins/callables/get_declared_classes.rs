//! Purpose:
//! Home of the PHP `get_declared_classes` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - Check hook returns `Array<Str>` unconditionally (zero-arg builtin).


builtin! {
    name: "get_declared_classes",
    area: Callables,
    params: [],
    returns: Mixed,
    check: crate::builtins::callables::support::check_declared_names,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::GetDeclaredClasses,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Returns an array of the names of the defined classes.",
    php_manual: "function.get-declared-classes",
}
