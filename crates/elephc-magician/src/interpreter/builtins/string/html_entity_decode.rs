//! Purpose:
//! Declarative eval registry entry for `html_entity_decode`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the HTML entity hook.

eval_builtin! {
    name: "html_entity_decode",
    area: String,
    params: [string],
    direct: HtmlEntity,
    values: HtmlEntity,
}
