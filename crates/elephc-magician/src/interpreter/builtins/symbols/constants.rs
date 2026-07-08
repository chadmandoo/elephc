//! Purpose:
//! Dynamic constant definition and lookup builtins for runtime eval fragments.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols` re-exports.
//!
//! Key details:
//! - Dynamic constants are stored in the eval context and checked after
//!   predefined constants using PHP's case-sensitive dynamic constant names.

use super::*;

/// Evaluates `define(name, value)` for eval dynamic constant-name registration.
pub(in crate::interpreter) fn eval_builtin_define(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let [name, value] = args else {
        return Err(EvalStatus::RuntimeFatal);
    };
    let name = eval_expr(name, context, scope, values)?;
    let value = eval_expr(value, context, scope, values)?;
    let defined = eval_define_name(name, value, context, values)?;
    values.bool_value(defined)
}

/// Evaluates `defined(name)` against eval dynamic constant names.
pub(in crate::interpreter) fn eval_builtin_defined(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let [name] = args else {
        return Err(EvalStatus::RuntimeFatal);
    };
    let name = eval_expr(name, context, scope, values)?;
    let exists = eval_defined_name(name, context, values)?;
    values.bool_value(exists)
}

/// Evaluates `define(...)` from already materialized call arguments.
pub(in crate::interpreter) fn eval_define_result(
    evaluated_args: &[RuntimeCellHandle],
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let [name, value] = evaluated_args else {
        return Err(EvalStatus::RuntimeFatal);
    };
    let defined = eval_define_name(*name, *value, context, values)?;
    values.bool_value(defined)
}

/// Evaluates `defined(...)` from already materialized call arguments.
pub(in crate::interpreter) fn eval_defined_result(
    evaluated_args: &[RuntimeCellHandle],
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let [name] = evaluated_args else {
        return Err(EvalStatus::RuntimeFatal);
    };
    let exists = eval_defined_name(*name, context, values)?;
    values.bool_value(exists)
}

/// Normalizes and registers one eval dynamic constant name.
pub(in crate::interpreter) fn eval_define_name(
    name: RuntimeCellHandle,
    value: RuntimeCellHandle,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<bool, EvalStatus> {
    let name = eval_constant_name(name, values)?;
    if name.is_empty() {
        return Err(EvalStatus::RuntimeFatal);
    }
    if eval_predefined_constant_value(&name).is_some() || context.has_constant(&name) {
        values.warning(DEFINE_ALREADY_DEFINED_WARNING)?;
        return Ok(false);
    }
    let value = values.retain(value)?;
    if context.define_constant(&name, value) {
        Ok(true)
    } else {
        values.release(value)?;
        Ok(false)
    }
}

/// Normalizes and probes one eval dynamic constant name.
pub(in crate::interpreter) fn eval_defined_name(
    name: RuntimeCellHandle,
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<bool, EvalStatus> {
    let name = eval_constant_name(name, values)?;
    Ok(eval_predefined_constant_value(&name).is_some() || context.has_constant(&name))
}

/// Reads a PHP constant name from a runtime cell without changing case.
pub(in crate::interpreter) fn eval_constant_name(
    name: RuntimeCellHandle,
    values: &mut impl RuntimeValueOps,
) -> Result<String, EvalStatus> {
    let name = values.string_bytes(name)?;
    String::from_utf8(name).map_err(|_| EvalStatus::RuntimeFatal)
}
