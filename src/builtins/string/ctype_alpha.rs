//! Purpose:
//! Home of the PHP `ctype_alpha` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `ctype_alpha` is a pure-data builtin whose return type
//!   (`Bool`) is fully determined by its declaration. The registry derives the
//!   return type from the `returns:` field without calling a check hook.
//! - `lower` is a thin wrapper over the dedicated `lower_ctype_alpha` emitter in the
//!   ctype lowering module.


builtin! {
    name: "ctype_alpha",
    area: String,
    params: [text: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::CtypeAlpha,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Checks if all characters in the string are alphabetic.",
    php_manual: "https://www.php.net/manual/en/function.ctype-alpha.php",
}
