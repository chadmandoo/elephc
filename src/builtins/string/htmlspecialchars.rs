//! Purpose:
//! Home of the PHP `htmlspecialchars` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `htmlspecialchars` is a pure-data builtin whose
//!   return type (`Str`) is fully determined by its declaration. The registry derives
//!   the return type from the `returns:` field without calling a check hook.
//! - `lower` is a thin wrapper over the shared `lower_html_escape` emitter,
//!   passing the builtin name for diagnostics; the runtime helper is
//!   `__rt_htmlspecialchars`.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "htmlspecialchars",
    area: String,
    params: [string: Str, flags: Int = DefaultSpec::Int(11), encoding: Str = DefaultSpec::Str("UTF-8")],
    returns: Str,
    returns_independent_storage: true,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Htmlspecialchars,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Converts the HTML special characters in a string into their entities.",
    php_manual: "https://www.php.net/manual/en/function.htmlspecialchars.php",
}
