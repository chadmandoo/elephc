//! Purpose:
//! Type-checks the io PHP builtin family.
//! Validates arity, argument types, warning-producing cases, and inferred return types for direct calls.
//!
//! Called from:
//! - `crate::types::checker::builtins::check_builtin()`
//!
//! Key details:
//! - Signatures, callable aliases, optimizer effects, and codegen builtin dispatch must remain in lockstep.
//! - The `stats` submodule has been fully migrated to the builtin registry (io batch B) and is deleted.
//! - The `files` submodule (`__elephc_phar_*` intrinsics) has been fully migrated to the builtin
//!   registry (io batch C2) and is deleted; these now live in `src/builtins/io/__elephc_phar_*.rs`.
//! - `common` is `pub(crate)` so `fstat`'s check hook in `src/builtins/io/fstat.rs` can call
//!   `ensure_stream_resource`; `streams.rs` continues to use it via `super::common`.

pub(crate) mod common;
mod streams;

use super::super::Checker;
use crate::parser::ast::Expr;
use crate::types::TypeEnv;

use common::BuiltinResult;

/// Type-checks a builtin call by delegating to the `streams` I/O subsystem checker.
///
/// Checks the `streams` submodule. The `debug`, `paths`, `stats`, and `files`
/// submodules have been fully migrated to the builtin registry and are no longer dispatched here.
/// Returns `Ok(Some(result))` if the builtin was recognized by a subsystem,
/// `Ok(None)` if no subsystem handles the name, or an error if validation fails.
pub(super) fn check_builtin(
    checker: &mut Checker,
    name: &str,
    args: &[Expr],
    span: crate::span::Span,
    env: &TypeEnv,
) -> BuiltinResult {
    if let Some(result) = streams::check_builtin(checker, name, args, span, env)? {
        return Ok(Some(result));
    }
    Ok(None)
}
