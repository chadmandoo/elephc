//! Purpose:
//! Home of the PHP `wordwrap` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   all via `crate::builtins::registry`.
//!
//! Key details:
//! - Accepts a required `string` param plus optional `width`, `break`, and
//!   `cut_long_words` params with PHP-compatible defaults. The `break` param
//!   uses the raw identifier `r#break` because `break` is a Rust keyword.
//! - `lower` is a thin wrapper over the shared `lower_wordwrap` emitter.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "wordwrap",
    area: String,
    params: [
        string: Str,
        width: Int = DefaultSpec::Int(75),
        r#break: Str = DefaultSpec::Str("\n"),
        cut_long_words: Bool = DefaultSpec::Bool(false)
    ],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Wordwrap,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Wraps a string to a given number of characters.",
    php_manual: "https://www.php.net/manual/en/function.wordwrap.php",
}
