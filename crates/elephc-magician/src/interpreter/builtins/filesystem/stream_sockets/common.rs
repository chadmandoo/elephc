//! Purpose:
//! Shared by-reference writeback helpers for stream socket builtins.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::stream_sockets` submodules.
//!
//! Key details:
//! - Helpers write directly to captured eval reference targets and preserve owned
//!   scope-cell replacement semantics.

use super::*;

/// Writes a socket output string to a captured by-reference target when available.
pub(super) fn eval_write_socket_output_ref_target(
    target: Option<&EvalReferenceTarget>,
    value: Option<String>,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<(), EvalStatus> {
    let Some((target, value)) = target.zip(value) else {
        return Ok(());
    };
    let value = values.string(&value)?;
    eval_write_direct_ref_target(
        target,
        value,
        context,
        values,
        Some(ScopeCellOwnership::Owned),
    )
}

/// Writes a socket output integer to a captured by-reference target when available.
pub(super) fn eval_write_socket_int_output_ref_target(
    target: Option<&EvalReferenceTarget>,
    value: i64,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<(), EvalStatus> {
    let Some(target) = target else {
        return Ok(());
    };
    let value = values.int(value)?;
    eval_write_direct_ref_target(
        target,
        value,
        context,
        values,
        Some(ScopeCellOwnership::Owned),
    )
}
