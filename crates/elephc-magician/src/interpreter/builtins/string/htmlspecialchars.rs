//! Purpose:
//! Declarative eval registry entry for `htmlspecialchars`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the HTML entity hook.

eval_builtin! {
    name: "htmlspecialchars",
    area: String,
    params: [string],
    direct: HtmlEntity,
    values: HtmlEntity,
}
