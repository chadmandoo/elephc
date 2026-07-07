//! Purpose:
//! Declarative eval registry entry for `htmlentities`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the HTML entity hook.

eval_builtin! {
    name: "htmlentities",
    area: String,
    params: [string],
    direct: HtmlEntity,
    values: HtmlEntity,
}
