//! Purpose:
//! Home of the PHP `spl_autoload_register` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - The autoload registration is an AOT stub: all three parameters are optional
//!   and any combination of 0–3 arguments is accepted. Returns `true` always.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "spl_autoload_register",
    area: Spl,
    params: [
        callback: Mixed = DefaultSpec::Null,
        throw: Bool = DefaultSpec::Bool(true),
        prepend: Bool = DefaultSpec::Bool(false),
    ],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::SplAutoloadRegister,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Register given function as __autoload() implementation.",
    php_manual: "https://www.php.net/manual/en/function.spl-autoload-register.php",
}
