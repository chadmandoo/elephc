//! Purpose:
//! Declarative eval registry entry for `fgetc`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the unary stream helper.

eval_builtin! {
    name: "fgetc",
    area: Filesystem,
    params: [stream],
    direct: Filesystem,
    values: Filesystem,
}
