//! Purpose:
//! Declarative eval registry entry for `is_writeable`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the file-probe helper.

eval_builtin! {
    name: "is_writeable",
    area: Filesystem,
    params: [filename],
    direct: Filesystem,
    values: Filesystem,
}
