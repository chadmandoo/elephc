//! Purpose:
//! Declarative eval registry entries and dispatch adapters for raw-memory builtins.
//!
//! Called from:
//! - `crate::interpreter::builtins::raw_memory` module loading.
//! - `crate::interpreter::builtins::hooks` for migrated raw-memory dispatch.
//!
//! Key details:
//! - Direct calls delegate to the existing source-sensitive helper so
//!   `buffer_free($local)` can null the source variable.
//! - Values calls keep the by-value `ptr(...)` unsupported behavior.

use super::super::super::*;
use super::{eval_builtin_raw_memory, eval_raw_memory_builtin_result};

mod buffer_free;
mod buffer_len;
mod buffer_new;
mod ptr;
mod ptr_get;
mod ptr_is_null;
mod ptr_null;
mod ptr_offset;
mod ptr_read16;
mod ptr_read32;
mod ptr_read8;
mod ptr_read_string;
mod ptr_set;
mod ptr_sizeof;
mod ptr_write16;
mod ptr_write32;
mod ptr_write8;
mod ptr_write_string;

/// Dispatches direct expression-level calls for raw-memory builtins.
pub(in crate::interpreter) fn eval_builtin_raw_memory_call(
    name: &str,
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    eval_builtin_raw_memory(name, args, context, scope, values)
}

/// Dispatches evaluated-argument calls for raw-memory builtins.
pub(in crate::interpreter) fn eval_raw_memory_values_result(
    name: &str,
    evaluated_args: &[RuntimeCellHandle],
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    eval_raw_memory_builtin_result(name, evaluated_args, context, values)
}
