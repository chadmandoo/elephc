//! Purpose:
//! Home of the PHP `ucwords` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - The declared signature carries the full golden param list (`string`, `separators`),
//!   but `max_args: 1` caps `check_arity` so a second argument is rejected, matching the
//!   legacy CHECK arm which enforced exactly one argument.
//! - No `check` hook is needed: the return type (`Str`) is fully determined by the
//!   declaration. The registry dispatch still infers each argument unconditionally, so
//!   undefined-variable diagnostics fire exactly as the legacy arm produced them.
//! - `lower` is a thin wrapper over the shared `lower_unary_string_runtime` emitter,
//!   passing the `__rt_ucwords` runtime helper.


builtin! {
    name: "ucwords",
    area: String,
    params: [string: Str, separators: Str = crate::builtins::spec::DefaultSpec::Str(" \t\r\n\u{0c}\u{0b}")],
    max_args: 1,
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Ucwords,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Uppercases the first character of each word in a string.",
    php_manual: "https://www.php.net/manual/en/function.ucwords.php",
}
