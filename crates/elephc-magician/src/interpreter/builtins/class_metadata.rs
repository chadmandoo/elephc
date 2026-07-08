//! Purpose:
//! Hosts shared class metadata helpers and OOP introspection support.
//!
//! Called from:
//! - `crate::interpreter::expressions::eval_positional_expr_call()`.
//! - Dynamic callable dispatch under `builtins::registry::dispatch`.
//!
//! Key details:
//! - Focused symbol builtin files own PHP-visible declarations and eval behavior.
//! - OOP introspection modules use these helpers for class-name normalization
//!   and eval/runtime class-like existence checks.

use super::super::*;

mod oop_introspection;

pub(in crate::interpreter) use oop_introspection::*;

/// Returns whether one normalized class-like name exists in eval or runtime metadata.
pub(in crate::interpreter) fn eval_class_relation_name_exists(
    name: &str,
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<bool, EvalStatus> {
    if context.has_class(name)
        || context.has_interface(name)
        || context.has_trait(name)
        || context.has_enum(name)
        || values.class_exists(name)?
        || eval_runtime_interface_exists(name, values)?
        || values.trait_exists(name)?
    {
        return Ok(true);
    }
    values.enum_exists(name)
}

/// Reads and normalizes one class metadata string argument.
pub(in crate::interpreter) fn eval_class_metadata_name(
    name: RuntimeCellHandle,
    values: &mut impl RuntimeValueOps,
) -> Result<String, EvalStatus> {
    let name = values.string_bytes(name)?;
    let name = String::from_utf8(name).map_err(|_| EvalStatus::RuntimeFatal)?;
    Ok(name.trim_start_matches('\\').to_string())
}
