//! Purpose:
//! Home of the PHP `vsprintf` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   all via `crate::builtins::registry`.
//!
//! Key details:
//! - Accepts a required `format` string and a `values` array.
//! - `lower` is a thin wrapper over the shared `lower_vsprintf` emitter.


builtin! {
    name: "vsprintf",
    area: String,
    params: [format: Str, values: Mixed],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Vsprintf,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Returns a formatted string using an array of values.",
    php_manual: "https://www.php.net/manual/en/function.vsprintf.php",
}
