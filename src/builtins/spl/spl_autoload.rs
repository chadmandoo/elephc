//! Purpose:
//! Home of the PHP `spl_autoload` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - Accepts 1 required argument (`class`) and 1 optional argument (`file_extensions`).
//! - The AOT stub evaluates arguments for side effects and returns void.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "spl_autoload",
    area: Spl,
    params: [class: Mixed, file_extensions: Mixed = DefaultSpec::Null],
    returns: Void,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::SplAutoload,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Default implementation for __autoload().",
    php_manual: "https://www.php.net/manual/en/function.spl-autoload.php",
}
