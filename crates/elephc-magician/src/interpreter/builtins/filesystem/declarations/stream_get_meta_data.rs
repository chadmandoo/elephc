//! Purpose:
//! Declarative eval registry entry for `stream_get_meta_data`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the unary stream helper.

eval_builtin! {
    name: "stream_get_meta_data",
    area: Filesystem,
    params: [stream],
    direct: Filesystem,
    values: Filesystem,
}
