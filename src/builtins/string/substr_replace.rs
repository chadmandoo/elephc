//! Purpose:
//! Home of the PHP `substr_replace` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   all via `crate::builtins::registry`.
//!
//! Key details:
//! - Accepts required `string`, `replace`, and `offset` params, plus an optional
//!   `length` param defaulting to null.
//! - `lower` is a thin wrapper over the shared `lower_substr_replace` emitter.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "substr_replace",
    area: String,
    params: [string: Str, replace: Str, offset: Int, length: Mixed = DefaultSpec::Null],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::SubstrReplace,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Replaces text within a portion of a string.",
    php_manual: "https://www.php.net/manual/en/function.substr-replace.php",
}
