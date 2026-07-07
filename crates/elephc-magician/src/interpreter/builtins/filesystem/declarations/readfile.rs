//! Purpose:
//! Declarative eval registry entry for `readfile`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the streaming file output helper.

eval_builtin! {
    name: "readfile",
    area: Filesystem,
    params: [filename],
    direct: Filesystem,
    values: Filesystem,
}
