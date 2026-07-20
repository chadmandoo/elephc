//! Purpose:
//! Home of the PHP `str_repeat` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `str_repeat` is a pure-data builtin whose return
//!   type (`Str`) is fully determined by its declaration. The registry derives the
//!   return type from the `returns:` field without calling a check hook.
//! - `lower` is a thin wrapper over the dedicated `lower_str_repeat` emitter in the
//!   strings lowering module.


builtin! {
    name: "str_repeat",
    area: String,
    params: [string: Str, times: Int],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StrRepeat,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Repeats a string a given number of times.",
    php_manual: "https://www.php.net/manual/en/function.str-repeat.php",
}
