//! Purpose:
//! Home of the PHP `vprintf` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   all via `crate::builtins::registry`.
//!
//! Key details:
//! - Accepts a required `format` string and a `values` array.
//! - `lower` is a thin wrapper over the shared `lower_vprintf` emitter.


builtin! {
    name: "vprintf",
    area: String,
    params: [format: Str, values: Mixed],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Vprintf,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Outputs a formatted string using an array of values.",
    php_manual: "https://www.php.net/manual/en/function.vprintf.php",
}
