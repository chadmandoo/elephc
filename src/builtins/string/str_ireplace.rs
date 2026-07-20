//! Purpose:
//! Home of the PHP `str_ireplace` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   all via `crate::builtins::registry`.
//!
//! Key details:
//! - The declared signature includes an optional `count` param, but `max_args: 3`
//!   caps arity so only three arguments are accepted, matching PHP's practical use.
//! - `lower` is a thin wrapper over the shared `lower_string_replace` emitter.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "str_ireplace",
    area: String,
    params: [search: Str, replace: Str, subject: Str, count: Mixed = DefaultSpec::Null],
    max_args: 3,
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StrIreplace,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Case-insensitive version of str_replace().",
    php_manual: "https://www.php.net/manual/en/function.str-ireplace.php",
}
