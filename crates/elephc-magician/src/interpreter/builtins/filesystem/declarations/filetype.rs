//! Purpose:
//! Declarative eval registry entry for `filetype`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the filetype helper.

eval_builtin! {
    name: "filetype",
    area: Filesystem,
    params: [filename],
    direct: Filesystem,
    values: Filesystem,
}
