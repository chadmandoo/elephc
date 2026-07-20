//! Purpose:
//! Home of the PHP `chop` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - `chop` is a PHP alias for `rtrim`. Both share the same signature, runtime
//!   helpers, and parameter defaults.
//! - No `check` hook is needed: `chop` is a pure-data builtin. The registry's arity
//!   check (1 required, 1 optional → 1 or 2 args) exactly matches the legacy check-arm
//!   constraint, so no additional validation is needed.
//! - `lower` is a thin wrapper over `lower_trim_like` routing to the `__rt_rtrim`
//!   family of runtime helpers.


builtin! {
    name: "chop",
    area: String,
    params: [
        string: Str,
        characters: Str = crate::builtins::spec::DefaultSpec::Str(" \n\r\t\u{000b}\u{000c}\0"),
    ],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Chop,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Alias of rtrim: strips whitespace (or other characters) from the end of a string.",
    php_manual: "https://www.php.net/manual/en/function.chop.php",
}
