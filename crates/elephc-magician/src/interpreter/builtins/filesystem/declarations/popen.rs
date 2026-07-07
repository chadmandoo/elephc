//! Purpose:
//! Declarative eval registry entry for `popen`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the process-pipe open helper.

eval_builtin! {
    name: "popen",
    area: Filesystem,
    params: [command, mode],
    direct: Filesystem,
    values: Filesystem,
}
