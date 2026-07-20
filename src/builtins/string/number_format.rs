//! Purpose:
//! Home of the PHP `number_format` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through
//!   `crate::builtins::registry`.
//!
//! Key details:
//! - Accepts a required `num` float and optional `decimals`, `decimal_separator`,
//!   and `thousands_separator` params with PHP-compatible defaults.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "number_format",
    area: String,
    params: [
        num: Float,
        decimals: Int = DefaultSpec::Int(0),
        decimal_separator: Str = DefaultSpec::Str("."),
        thousands_separator: Str = DefaultSpec::Str(",")
    ],
    returns: Str,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::NumberFormat,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Formats a number with grouped thousands.",
    php_manual: "https://www.php.net/manual/en/function.number-format.php",
}
