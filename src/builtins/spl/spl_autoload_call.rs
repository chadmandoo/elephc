//! Purpose:
//! Home of the PHP `spl_autoload_call` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through
//!   `crate::builtins::registry`.
//!
//! Key details:
//! - The AOT stub accepts exactly one class-name argument and returns void.


builtin! {
    name: "spl_autoload_call",
    area: Spl,
    params: [class: Mixed],
    returns: Void,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::SplAutoloadCall,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Try all registered __autoload() functions to load the requested class.",
    php_manual: "https://www.php.net/manual/en/function.spl-autoload-call.php",
}
