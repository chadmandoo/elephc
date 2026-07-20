//! Purpose:
//! Home of the PHP `var_dump` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `var_dump` is a pure-data builtin whose return type
//!   (`Void`) is fully determined by its declaration. The registry common path
//!   infers every argument and enforces arity before falling back to `returns`.
//! - `var_dump` is variadic (`var_dump($value, ...$values)`): each argument is
//!   dumped independently in source order, matching PHP.
//! - `lower` is a thin wrapper over `debug::lower_var_dump` in the EIR backend.


builtin! {
    name: "var_dump",
    area: Io,
    params: [value: Mixed],
    variadic: "values",
    returns: Void,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::VarDump,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Dumps information about a variable.",
    php_manual: "function.var-dump",
}
