//! Purpose:
//! Home of the PHP `spl_autoload_unregister` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - The AOT stub accepts exactly one callable argument and returns `true`.


builtin! {
    name: "spl_autoload_unregister",
    area: Spl,
    params: [callback: Mixed],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::SplAutoloadUnregister,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Unregister given function as __autoload() implementation.",
    php_manual: "https://www.php.net/manual/en/function.spl-autoload-unregister.php",
}
