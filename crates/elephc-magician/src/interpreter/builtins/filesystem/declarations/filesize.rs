//! Purpose:
//! Declarative eval registry entry for `filesize`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the filesize helper.

eval_builtin! {
    name: "filesize",
    area: Filesystem,
    params: [filename],
    direct: Filesystem,
    values: Filesystem,
}
