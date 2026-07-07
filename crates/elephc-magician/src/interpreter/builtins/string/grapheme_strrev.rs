//! Purpose:
//! Declarative eval registry entry for `grapheme_strrev`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the grapheme string reverse hook.

eval_builtin! {
    name: "grapheme_strrev",
    area: String,
    params: [string],
    direct: GraphemeStrrev,
    values: GraphemeStrrev,
}
