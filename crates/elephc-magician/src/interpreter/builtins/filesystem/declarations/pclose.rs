//! Purpose:
//! Declarative eval registry entry for `pclose`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the process-pipe close helper.

eval_builtin! {
    name: "pclose",
    area: Filesystem,
    params: [handle],
    direct: Filesystem,
    values: Filesystem,
}
