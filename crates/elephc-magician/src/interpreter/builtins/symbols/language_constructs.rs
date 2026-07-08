//! Purpose:
//! Eval implementations of PHP language constructs `isset`, `empty`, and `unset`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols` re-exports.
//!
//! Key details:
//! - These constructs receive unevaluated source expressions so missing variables,
//!   properties, array access, and by-reference unset targets keep PHP semantics.

use super::*;

/// Evaluates PHP's `isset(...)` language construct over eval-visible values.
pub(in crate::interpreter) fn eval_builtin_isset(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    if args.is_empty() {
        return Err(EvalStatus::RuntimeFatal);
    }
    for arg in args {
        if !eval_isset_arg(arg, context, scope, values)? {
            return values.bool_value(false);
        }
    }
    values.bool_value(true)
}

/// Evaluates PHP's `empty(...)` language construct over eval-visible values.
pub(in crate::interpreter) fn eval_builtin_empty(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let [arg] = args else {
        return Err(EvalStatus::RuntimeFatal);
    };
    let empty = eval_empty_arg(arg, context, scope, values)?;
    values.bool_value(empty)
}

/// Evaluates direct `unset(...)` calls over eval-visible variables and object properties.
pub(in crate::interpreter) fn eval_builtin_unset(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    if args.is_empty() {
        return Err(EvalStatus::RuntimeFatal);
    }
    for arg in args {
        match arg {
            EvalExpr::LoadVar(name) => {
                if let Some(replaced) = unset_scope_cell(scope, name.clone()) {
                    values.release(replaced)?;
                }
            }
            EvalExpr::PropertyGet { object, property } => {
                let object = eval_expr(object, context, scope, values)?;
                eval_property_unset_result(object, property, context, values)?;
            }
            EvalExpr::DynamicPropertyGet { object, property } => {
                let object = eval_expr(object, context, scope, values)?;
                let property = eval_dynamic_member_name(property, context, scope, values)?;
                eval_property_unset_result(object, &property, context, values)?;
            }
            _ => return Err(EvalStatus::RuntimeFatal),
        }
    }
    values.null()
}

/// Evaluates callable `isset(...)` over already materialized values.
pub(in crate::interpreter) fn eval_isset_result(
    evaluated_args: &[RuntimeCellHandle],
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    if evaluated_args.is_empty() {
        return Err(EvalStatus::RuntimeFatal);
    }
    for value in evaluated_args {
        if values.is_null(*value)? {
            return values.bool_value(false);
        }
    }
    values.bool_value(true)
}

/// Evaluates callable `empty(...)` over one already materialized value.
pub(in crate::interpreter) fn eval_empty_result(
    evaluated_args: &[RuntimeCellHandle],
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let [value] = evaluated_args else {
        return Err(EvalStatus::RuntimeFatal);
    };
    let empty = !values.truthy(*value)?;
    values.bool_value(empty)
}

/// Evaluates callable `unset(...)` after values have already been materialized.
pub(in crate::interpreter) fn eval_unset_result(
    evaluated_args: &[RuntimeCellHandle],
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    if evaluated_args.is_empty() {
        return Err(EvalStatus::RuntimeFatal);
    }
    values.null()
}

/// Evaluates one `empty` operand without warning or failing on missing variables.
pub(in crate::interpreter) fn eval_empty_arg(
    arg: &EvalExpr,
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<bool, EvalStatus> {
    if let EvalExpr::LoadVar(name) = arg {
        let Some(value) = visible_scope_cell(context, scope, name) else {
            return Ok(true);
        };
        return Ok(!values.truthy(value)?);
    }
    if let EvalExpr::PropertyGet { object, property } = arg {
        let object = eval_expr(object, context, scope, values)?;
        if !eval_property_isset_result(object, property, context, values)? {
            return Ok(true);
        }
        let value = eval_property_get_result(object, property, context, values)?;
        return Ok(!values.truthy(value)?);
    }
    if let EvalExpr::DynamicPropertyGet { object, property } = arg {
        let object = eval_expr(object, context, scope, values)?;
        let property = eval_dynamic_member_name(property, context, scope, values)?;
        if !eval_property_isset_result(object, &property, context, values)? {
            return Ok(true);
        }
        let value = eval_property_get_result(object, &property, context, values)?;
        return Ok(!values.truthy(value)?);
    }
    if let EvalExpr::NullsafePropertyGet { object, property } = arg {
        let object = eval_expr(object, context, scope, values)?;
        if values.is_null(object)? {
            return Ok(true);
        }
        if !eval_property_isset_result(object, property, context, values)? {
            return Ok(true);
        }
        let value = eval_property_get_result(object, property, context, values)?;
        return Ok(!values.truthy(value)?);
    }
    if let EvalExpr::NullsafeDynamicPropertyGet { object, property } = arg {
        let object = eval_expr(object, context, scope, values)?;
        if values.is_null(object)? {
            return Ok(true);
        }
        let property = eval_dynamic_member_name(property, context, scope, values)?;
        if !eval_property_isset_result(object, &property, context, values)? {
            return Ok(true);
        }
        let value = eval_property_get_result(object, &property, context, values)?;
        return Ok(!values.truthy(value)?);
    }
    if let EvalExpr::StaticPropertyGet {
        class_name,
        property,
    } = arg
    {
        if !eval_static_property_isset_result(class_name, property, context, values)? {
            return Ok(true);
        }
        let value = eval_static_property_get_result(class_name, property, context, values)?;
        return Ok(!values.truthy(value)?);
    }
    if let EvalExpr::DynamicStaticPropertyGet {
        class_name,
        property,
    } = arg
    {
        let class_name = eval_expr(class_name, context, scope, values)?;
        let class_name = eval_dynamic_class_name(class_name, context, values)?;
        if !eval_static_property_isset_result(&class_name, property, context, values)? {
            return Ok(true);
        }
        let value = eval_static_property_get_result(&class_name, property, context, values)?;
        return Ok(!values.truthy(value)?);
    }
    if let EvalExpr::DynamicStaticPropertyNameGet {
        class_name,
        property,
    } = arg
    {
        let class_name = eval_expr(class_name, context, scope, values)?;
        let class_name = eval_dynamic_class_name(class_name, context, values)?;
        let property = eval_dynamic_member_name(property, context, scope, values)?;
        if !eval_static_property_isset_result(&class_name, &property, context, values)? {
            return Ok(true);
        }
        let value = eval_static_property_get_result(&class_name, &property, context, values)?;
        return Ok(!values.truthy(value)?);
    }
    if let EvalExpr::ArrayGet { array, index } = arg {
        let array = eval_expr(array, context, scope, values)?;
        let index = eval_expr(index, context, scope, values)?;
        if values.type_tag(array)? == EVAL_TAG_OBJECT {
            return eval_array_access_empty_result(array, index, context, values);
        }
        let value = values.array_get(array, index)?;
        return Ok(!values.truthy(value)?);
    }
    let value = eval_expr(arg, context, scope, values)?;
    Ok(!values.truthy(value)?)
}

/// Evaluates one `isset` operand without allocating a null cell for missing variables.
pub(in crate::interpreter) fn eval_isset_arg(
    arg: &EvalExpr,
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<bool, EvalStatus> {
    if let EvalExpr::LoadVar(name) = arg {
        let Some(value) = visible_scope_cell(context, scope, name) else {
            return Ok(false);
        };
        return Ok(!values.is_null(value)?);
    }
    if let EvalExpr::PropertyGet { object, property } = arg {
        let object = eval_expr(object, context, scope, values)?;
        return eval_property_isset_result(object, property, context, values);
    }
    if let EvalExpr::DynamicPropertyGet { object, property } = arg {
        let object = eval_expr(object, context, scope, values)?;
        let property = eval_dynamic_member_name(property, context, scope, values)?;
        return eval_property_isset_result(object, &property, context, values);
    }
    if let EvalExpr::NullsafePropertyGet { object, property } = arg {
        let object = eval_expr(object, context, scope, values)?;
        if values.is_null(object)? {
            return Ok(false);
        }
        return eval_property_isset_result(object, property, context, values);
    }
    if let EvalExpr::NullsafeDynamicPropertyGet { object, property } = arg {
        let object = eval_expr(object, context, scope, values)?;
        if values.is_null(object)? {
            return Ok(false);
        }
        let property = eval_dynamic_member_name(property, context, scope, values)?;
        return eval_property_isset_result(object, &property, context, values);
    }
    if let EvalExpr::StaticPropertyGet {
        class_name,
        property,
    } = arg
    {
        return eval_static_property_isset_result(class_name, property, context, values);
    }
    if let EvalExpr::DynamicStaticPropertyGet {
        class_name,
        property,
    } = arg
    {
        let class_name = eval_expr(class_name, context, scope, values)?;
        let class_name = eval_dynamic_class_name(class_name, context, values)?;
        return eval_static_property_isset_result(&class_name, property, context, values);
    }
    if let EvalExpr::DynamicStaticPropertyNameGet {
        class_name,
        property,
    } = arg
    {
        let class_name = eval_expr(class_name, context, scope, values)?;
        let class_name = eval_dynamic_class_name(class_name, context, values)?;
        let property = eval_dynamic_member_name(property, context, scope, values)?;
        return eval_static_property_isset_result(&class_name, &property, context, values);
    }
    if let EvalExpr::ArrayGet { array, index } = arg {
        let array = eval_expr(array, context, scope, values)?;
        let index = eval_expr(index, context, scope, values)?;
        if values.type_tag(array)? == EVAL_TAG_OBJECT {
            return eval_array_access_isset_result(array, index, context, values);
        }
        let value = values.array_get(array, index)?;
        return Ok(!values.is_null(value)?);
    }
    let value = eval_expr(arg, context, scope, values)?;
    Ok(!values.is_null(value)?)
}

/// Evaluates `empty($object[$key])` through `ArrayAccess::offsetExists()` and `offsetGet()`.
fn eval_array_access_empty_result(
    object: RuntimeCellHandle,
    index: RuntimeCellHandle,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<bool, EvalStatus> {
    if !eval_array_access_isset_result(object, index, context, values)? {
        return Ok(true);
    }
    let value = eval_array_get_result(object, index, context, values)?;
    Ok(!values.truthy(value)?)
}

/// Evaluates `isset($object[$key])` through `ArrayAccess::offsetExists()`.
fn eval_array_access_isset_result(
    object: RuntimeCellHandle,
    index: RuntimeCellHandle,
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<bool, EvalStatus> {
    if !eval_array_access_object_matches(object, context, values)? {
        return Err(EvalStatus::RuntimeFatal);
    }
    let result = eval_method_call_result(object, "offsetExists", vec![index], context, values)?;
    let exists = values.truthy(result)?;
    values.release(result)?;
    Ok(exists)
}
