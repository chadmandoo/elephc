//! Purpose:
//! Home of the PHP `long2ip` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `long2ip` is a pure-data builtin whose return type
//!   (`Str`) is fully determined by its declaration. The registry derives the
//!   return type from the `returns:` field without calling a check hook.
//! - `lower` is a thin wrapper over the dedicated `lower_long2ip` emitter in the
//!   strings lowering module.


builtin! {
    name: "long2ip",
    area: String,
    params: [ip: Int],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Long2ip,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Converts an IPv4 address from long integer to dotted string notation.",
    php_manual: "https://www.php.net/manual/en/function.long2ip.php",
}
