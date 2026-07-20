//! Purpose:
//! Home of the PHP `printf` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   all via `crate::builtins::registry`.
//!
//! Key details:
//! - Accepts a required `format` string plus a variadic `values` list.
//! - `lower` is a thin wrapper over the shared `lower_printf` emitter.


builtin! {
    name: "printf",
    area: String,
    params: [format: Str],
    variadic: "values",
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Printf,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Outputs a formatted string.",
    php_manual: "https://www.php.net/manual/en/function.printf.php",
}
