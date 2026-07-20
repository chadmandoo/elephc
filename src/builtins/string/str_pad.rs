//! Purpose:
//! Home of the PHP `str_pad` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   all via `crate::builtins::registry`.
//!
//! Key details:
//! - Accepts required `string` and `length` params, plus optional `pad_string`
//!   and `pad_type` params with PHP-compatible defaults.
//! - `lower` is a thin wrapper over the shared `lower_str_pad` emitter.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "str_pad",
    area: String,
    params: [
        string: Str,
        length: Int,
        pad_string: Str = DefaultSpec::Str(" "),
        pad_type: Int = DefaultSpec::Int(1)
    ],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StrPad,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Pads a string to a certain length with another string.",
    php_manual: "https://www.php.net/manual/en/function.str-pad.php",
}
