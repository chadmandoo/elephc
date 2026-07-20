//! Purpose:
//! Home of the PHP `str_starts_with` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `str_starts_with` is a pure-data builtin whose
//!   return type (`Bool`) is fully determined by its declaration. The registry
//!   derives the return type from the `returns:` field without calling a check hook.
//! - `lower` is a thin wrapper over `lower_binary_string_runtime` which dispatches
//!   to the shared `__rt_str_starts_with` runtime helper.


builtin! {
    name: "str_starts_with",
    area: String,
    params: [haystack: Str, needle: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StrStartsWith,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Checks if a string starts with a given substring.",
    php_manual: "https://www.php.net/manual/en/function.str-starts-with.php",
}
