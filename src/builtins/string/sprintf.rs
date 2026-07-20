//! Purpose:
//! Home of the PHP `sprintf` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   all via `crate::builtins::registry`.
//!
//! Key details:
//! - Accepts a required `format` string plus a variadic `values` list.
//! - `lower` is a thin wrapper over the shared `lower_sprintf` emitter.


builtin! {
    name: "sprintf",
    area: String,
    params: [format: Str],
    variadic: "values",
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Sprintf,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Returns a formatted string.",
    php_manual: "https://www.php.net/manual/en/function.sprintf.php",
}
