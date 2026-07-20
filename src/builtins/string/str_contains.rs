//! Purpose:
//! Home of the PHP `str_contains` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `str_contains` is a pure-data builtin whose return
//!   type (`Bool`) is fully determined by its declaration. The registry derives the
//!   return type from the `returns:` field without calling a check hook.
//! - `lower` is a thin wrapper over `lower_str_contains` which uses `__rt_strpos`
//!   and normalizes its signed result to a PHP boolean.


builtin! {
    name: "str_contains",
    area: String,
    params: [haystack: Str, needle: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StrContains,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Determines if a string contains a given substring.",
    php_manual: "https://www.php.net/manual/en/function.str-contains.php",
}
