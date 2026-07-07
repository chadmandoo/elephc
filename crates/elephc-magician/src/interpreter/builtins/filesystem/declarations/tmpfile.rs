//! Purpose:
//! Declarative eval registry entry for `tmpfile`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the temporary stream helper.

eval_builtin! {
    name: "tmpfile",
    area: Filesystem,
    params: [],
    direct: Filesystem,
    values: Filesystem,
}
