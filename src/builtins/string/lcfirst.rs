//! Purpose:
//! Home of the PHP `lcfirst` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `lcfirst` is a pure-data builtin whose return
//!   type (`Str`) is fully determined by its declaration. The registry derives the
//!   return type from the `returns:` field without calling a check hook.
//! - `lower` is a thin wrapper over the dedicated `lower_lcfirst` emitter in the
//!   strings lowering module.


builtin! {
    name: "lcfirst",
    area: String,
    params: [string: Str],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Lcfirst,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Lowercases the first character of a string.",
    php_manual: "https://www.php.net/manual/en/function.lcfirst.php",
}
