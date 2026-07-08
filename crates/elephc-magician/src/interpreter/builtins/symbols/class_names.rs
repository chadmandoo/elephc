//! Purpose:
//! Class-like symbol existence, aliasing, and declared-symbol array builtins.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols` re-exports.
//!
//! Key details:
//! - Lookup checks eval declarations before generated/AOT runtime metadata.
//! - `class_alias()` records external aliases in the eval context when the
//!   aliased class-like symbol is provided by runtime metadata.

use super::*;

/// Evaluates `class_exists(...)` against dynamic and generated class-name tables.
pub(in crate::interpreter) fn eval_builtin_class_exists(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let name = match args {
        [name] => eval_expr(name, context, scope, values)?,
        [name, autoload] => {
            let name = eval_expr(name, context, scope, values)?;
            let _ = eval_expr(autoload, context, scope, values)?;
            name
        }
        _ => return Err(EvalStatus::RuntimeFatal),
    };
    let exists = eval_class_exists_name(name, context, values)?;
    values.bool_value(exists)
}

/// Evaluates `class_exists(...)` from already materialized call arguments.
pub(in crate::interpreter) fn eval_class_exists_result(
    evaluated_args: &[RuntimeCellHandle],
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let exists = match evaluated_args {
        [name] => eval_class_exists_name(*name, context, values)?,
        [name, _autoload] => eval_class_exists_name(*name, context, values)?,
        _ => return Err(EvalStatus::RuntimeFatal),
    };
    values.bool_value(exists)
}

/// Normalizes a PHP class-name cell and probes dynamic names before generated classes.
pub(in crate::interpreter) fn eval_class_exists_name(
    name: RuntimeCellHandle,
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<bool, EvalStatus> {
    let name = values.string_bytes(name)?;
    let name = String::from_utf8(name).map_err(|_| EvalStatus::RuntimeFatal)?;
    let name = name.trim_start_matches('\\');
    if name.eq_ignore_ascii_case("Closure") {
        return Ok(true);
    }
    if context.has_class(name) {
        return Ok(true);
    }
    values.class_exists(name)
}

/// Evaluates `class_alias(class, alias, autoload?)` against eval and generated class tables.
pub(in crate::interpreter) fn eval_builtin_class_alias(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let (class, alias) = match args {
        [class, alias] => (
            eval_expr(class, context, scope, values)?,
            eval_expr(alias, context, scope, values)?,
        ),
        [class, alias, autoload] => {
            let class = eval_expr(class, context, scope, values)?;
            let alias = eval_expr(alias, context, scope, values)?;
            let _ = eval_expr(autoload, context, scope, values)?;
            (class, alias)
        }
        _ => return Err(EvalStatus::RuntimeFatal),
    };
    eval_class_alias_result(&[class, alias], context, values)
}

/// Evaluates `class_alias(...)` from already materialized call arguments.
pub(in crate::interpreter) fn eval_class_alias_result(
    evaluated_args: &[RuntimeCellHandle],
    context: &mut ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let (class, alias) = match evaluated_args {
        [class, alias] => (*class, *alias),
        [class, alias, _autoload] => (*class, *alias),
        _ => return Err(EvalStatus::RuntimeFatal),
    };
    let class = eval_class_alias_name(class, values)?;
    let alias = eval_class_alias_name(alias, values)?;
    if alias.is_empty()
        || context.resolve_class_like_name(&alias).is_some()
        || values.class_exists(&alias)?
        || eval_runtime_interface_exists(&alias, values)?
        || values.trait_exists(&alias)?
        || values.enum_exists(&alias)?
    {
        return values.bool_value(false);
    }
    let aliased = if context.resolve_class_like_name(&class).is_some() {
        context.define_class_alias(&class, &alias)
    } else if values.enum_exists(&class)? {
        context.define_external_enum_alias(&class, &alias)
    } else if values.class_exists(&class)? {
        context.define_external_class_alias(&class, &alias)
    } else if eval_runtime_interface_exists(&class, values)? {
        context.define_external_interface_alias(&class, &alias)
    } else if values.trait_exists(&class)? {
        context.define_external_trait_alias(&class, &alias)
    } else {
        false
    };
    values.bool_value(aliased)
}

/// Reads and normalizes one `class_alias()` class-name argument.
fn eval_class_alias_name(
    name: RuntimeCellHandle,
    values: &mut impl RuntimeValueOps,
) -> Result<String, EvalStatus> {
    let name = values.string_bytes(name)?;
    let name = String::from_utf8(name).map_err(|_| EvalStatus::RuntimeFatal)?;
    Ok(name.trim_start_matches('\\').to_string())
}

/// Evaluates `get_declared_classes/interfaces/traits()` for eval-visible declarations.
pub(in crate::interpreter) fn eval_builtin_get_declared_symbols(
    name: &str,
    args: &[EvalExpr],
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    if !args.is_empty() {
        return Err(EvalStatus::RuntimeFatal);
    }
    eval_get_declared_symbols_result(name, context, values)
}

/// Builds an indexed array for eval-visible declared class-like names.
pub(in crate::interpreter) fn eval_get_declared_symbols_result(
    name: &str,
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    match name {
        "get_declared_classes" => {
            eval_dynamic_string_array_result(context.declared_class_names(), values)
        }
        "get_declared_interfaces" => {
            eval_dynamic_string_array_result(context.declared_interface_names(), values)
        }
        "get_declared_traits" => {
            eval_dynamic_string_array_result(context.declared_trait_names(), values)
        }
        _ => Err(EvalStatus::RuntimeFatal),
    }
}

/// Builds one indexed PHP array from runtime-owned strings.
fn eval_dynamic_string_array_result(
    items: &[String],
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let mut result = values.array_new(items.len())?;
    for (index, item) in items.iter().enumerate() {
        let index = i64::try_from(index).map_err(|_| EvalStatus::RuntimeFatal)?;
        let key = values.int(index)?;
        let value = values.string(item)?;
        result = values.array_set(result, key, value)?;
    }
    Ok(result)
}

/// Evaluates `interface_exists(...)` against generated interface-name metadata.
pub(in crate::interpreter) fn eval_builtin_interface_exists(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let name = match args {
        [name] => eval_expr(name, context, scope, values)?,
        [name, autoload] => {
            let name = eval_expr(name, context, scope, values)?;
            let _ = eval_expr(autoload, context, scope, values)?;
            name
        }
        _ => return Err(EvalStatus::RuntimeFatal),
    };
    let exists = eval_interface_exists_name(name, context, values)?;
    values.bool_value(exists)
}

/// Evaluates `interface_exists(...)` from already materialized call arguments.
pub(in crate::interpreter) fn eval_interface_exists_result(
    evaluated_args: &[RuntimeCellHandle],
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let exists = match evaluated_args {
        [name] => eval_interface_exists_name(*name, context, values)?,
        [name, _autoload] => eval_interface_exists_name(*name, context, values)?,
        _ => return Err(EvalStatus::RuntimeFatal),
    };
    values.bool_value(exists)
}

/// Normalizes a PHP interface-name cell and probes eval and generated interface metadata.
pub(in crate::interpreter) fn eval_interface_exists_name(
    name: RuntimeCellHandle,
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<bool, EvalStatus> {
    let name = values.string_bytes(name)?;
    let name = String::from_utf8(name).map_err(|_| EvalStatus::RuntimeFatal)?;
    let name = name.trim_start_matches('\\');
    Ok(context.has_interface(name) || eval_runtime_interface_exists(name, values)?)
}

/// Evaluates `trait_exists(...)` and `enum_exists(...)` against generated metadata.
pub(in crate::interpreter) fn eval_builtin_class_like_exists(
    name: &str,
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let symbol = match args {
        [symbol] => eval_expr(symbol, context, scope, values)?,
        [symbol, autoload] => {
            let symbol = eval_expr(symbol, context, scope, values)?;
            let _ = eval_expr(autoload, context, scope, values)?;
            symbol
        }
        _ => return Err(EvalStatus::RuntimeFatal),
    };
    let exists = eval_class_like_exists_name(name, symbol, context, values)?;
    values.bool_value(exists)
}

/// Evaluates materialized `trait_exists(...)` or `enum_exists(...)` arguments.
pub(in crate::interpreter) fn eval_class_like_exists_result(
    name: &str,
    evaluated_args: &[RuntimeCellHandle],
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let exists = match evaluated_args {
        [symbol] => eval_class_like_exists_name(name, *symbol, context, values)?,
        [symbol, _autoload] => eval_class_like_exists_name(name, *symbol, context, values)?,
        _ => return Err(EvalStatus::RuntimeFatal),
    };
    values.bool_value(exists)
}

/// Normalizes a PHP class-like name cell and probes generated trait or enum metadata.
pub(in crate::interpreter) fn eval_class_like_exists_name(
    name: &str,
    symbol: RuntimeCellHandle,
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<bool, EvalStatus> {
    let symbol = values.string_bytes(symbol)?;
    let symbol = String::from_utf8(symbol).map_err(|_| EvalStatus::RuntimeFatal)?;
    let symbol = symbol.trim_start_matches('\\');
    match name {
        "trait_exists" => Ok(context.has_trait(symbol) || values.trait_exists(symbol)?),
        "enum_exists" => Ok(context.has_enum(symbol) || values.enum_exists(symbol)?),
        _ => Err(EvalStatus::UnsupportedConstruct),
    }
}
