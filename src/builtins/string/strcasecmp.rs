//! Purpose:
//! Home of the PHP `strcasecmp` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `strcasecmp` is a pure-data builtin whose return
//!   type (`Int`) is fully determined by its declaration. The registry derives the
//!   return type from the `returns:` field without calling a check hook.
//! - `lower` is a thin wrapper over `lower_binary_string_runtime` which dispatches
//!   to the shared `__rt_strcasecmp` runtime helper.


builtin! {
    name: "strcasecmp",
    area: String,
    params: [string1: Str, string2: Str],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Strcasecmp,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Binary safe case-insensitive string comparison. Returns negative, zero, or positive.",
    php_manual: "https://www.php.net/manual/en/function.strcasecmp.php",
}
