//! Purpose:
//! Declarative eval registry entry for `file_put_contents`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the one-shot file write helper.

eval_builtin! {
    name: "file_put_contents",
    area: Filesystem,
    params: [filename, data],
    direct: Filesystem,
    values: Filesystem,
}
