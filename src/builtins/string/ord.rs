//! Purpose:
//! Home of the PHP `ord` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `ord` is a pure-data builtin whose return type
//!   (`Int`) is fully determined by its declaration. The registry derives the
//!   return type from the `returns:` field without calling a check hook.
//! - `lower` is a thin wrapper over the dedicated `lower_ord` emitter in the
//!   strings lowering module.


builtin! {
    name: "ord",
    area: String,
    params: [character: Str],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Ord,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the ASCII value of the first character of a string.",
    php_manual: "https://www.php.net/manual/en/function.ord.php",
}
