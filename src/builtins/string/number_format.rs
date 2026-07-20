//! Purpose:
//! Home of the PHP `number_format` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   all via `crate::builtins::registry`.
//!
//! Key details:
//! - Accepts a required `num` float and optional `decimals`, `decimal_separator`,
//!   and `thousands_separator` params with PHP-compatible defaults.
//! - `lower` is a thin wrapper over the shared `lower_number_format` emitter.

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
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::NumberFormat,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Formats a number with grouped thousands.",
    php_manual: "https://www.php.net/manual/en/function.number-format.php",
}
