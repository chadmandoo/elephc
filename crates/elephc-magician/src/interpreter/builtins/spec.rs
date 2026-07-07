//! Purpose:
//! Declarative builtin specifications for the eval interpreter.
//! Each spec owns PHP-visible metadata plus optional direct and evaluated-arg
//! dispatch hooks for one builtin.
//!
//! Called from:
//! - `crate::interpreter::builtins::registry` lookup and metadata helpers.
//! - `eval_builtin!` submissions in per-builtin home files.
//!
//! Key details:
//! - Specs are collected with `inventory` to let builtin files register
//!   themselves without growing a central match.
//! - Hook enums keep calls monomorphized over `RuntimeValueOps`.

use super::super::{
    eval_builtin_ceil, eval_builtin_clamp, eval_builtin_count, eval_builtin_float_binary,
    eval_builtin_float_pair, eval_builtin_float_unary, eval_builtin_floor, eval_builtin_gettype,
    eval_builtin_intdiv, eval_builtin_log, eval_builtin_min_max, eval_builtin_number_format,
    eval_builtin_pi, eval_builtin_pow, eval_builtin_round, eval_builtin_sqrt, eval_builtin_strlen,
    eval_builtin_type_predicate, eval_count_result, ElephcEvalContext, ElephcEvalScope, EvalExpr,
    EvalStatus, RuntimeCellHandle, RuntimeValueOps,
};
use super::{
    eval_builtin_abs, eval_builtin_cast, eval_builtin_strrev, eval_cast_result,
    eval_clamp_result, eval_float_binary_result, eval_float_pair_result, eval_float_unary_result,
    eval_gettype_result, eval_intdiv_result, eval_log_result, eval_min_max_result,
    eval_number_format_result, eval_type_predicate_result,
};
pub(in crate::interpreter) use super::registry::EvalBuiltinDefaultValue;

/// Broad domain used to group eval builtin home files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::interpreter) enum EvalArea {
    /// Array and collection builtins.
    Array,
    /// Formatting and display-oriented numeric builtins.
    Formatting,
    /// Numeric and mathematical builtins.
    Math,
    /// String-processing builtins.
    String,
    /// Scalar conversion and type-related builtins.
    Types,
}

/// Parameter metadata for one eval builtin argument.
#[derive(Clone, Copy)]
pub(in crate::interpreter) struct EvalParamSpec {
    /// PHP-visible parameter name.
    pub(in crate::interpreter) name: &'static str,
    /// Optional PHP default value.
    pub(in crate::interpreter) default: Option<EvalBuiltinDefaultValue>,
    /// Whether this parameter must bind to caller storage.
    pub(in crate::interpreter) by_ref: bool,
}

/// Direct expression-level dispatch hooks for migrated builtins.
#[derive(Clone, Copy)]
pub(in crate::interpreter) enum EvalDirectHook {
    /// Dispatches `abs(...)`.
    Abs,
    /// Dispatches scalar cast builtins.
    Cast,
    /// Dispatches `ceil(...)`.
    Ceil,
    /// Dispatches `clamp(...)`.
    Clamp,
    /// Dispatches `count(...)`.
    Count,
    /// Dispatches binary floating-point builtins.
    FloatBinary,
    /// Dispatches paired floating-point builtins.
    FloatPair,
    /// Dispatches unary floating-point builtins.
    FloatUnary,
    /// Dispatches `floor(...)`.
    Floor,
    /// Dispatches `gettype(...)`.
    Gettype,
    /// Dispatches `intdiv(...)`.
    Intdiv,
    /// Dispatches `log(...)`.
    Log,
    /// Dispatches `min(...)` and `max(...)`.
    MinMax,
    /// Dispatches `number_format(...)`.
    NumberFormat,
    /// Dispatches `pi()`.
    Pi,
    /// Dispatches `pow(...)`.
    Pow,
    /// Dispatches `round(...)`.
    Round,
    /// Dispatches `sqrt(...)`.
    Sqrt,
    /// Dispatches `strlen(...)`.
    Strlen,
    /// Dispatches `strrev(...)`.
    Strrev,
    /// Dispatches scalar and container type predicates.
    TypePredicate,
}

/// Evaluated-argument dispatch hooks for migrated builtins.
#[derive(Clone, Copy)]
pub(in crate::interpreter) enum EvalValuesHook {
    /// Dispatches `abs(...)`.
    Abs,
    /// Dispatches scalar cast builtins.
    Cast,
    /// Dispatches `ceil(...)`.
    Ceil,
    /// Dispatches `clamp(...)`.
    Clamp,
    /// Dispatches `count(...)`.
    Count,
    /// Dispatches binary floating-point builtins.
    FloatBinary,
    /// Dispatches paired floating-point builtins.
    FloatPair,
    /// Dispatches unary floating-point builtins.
    FloatUnary,
    /// Dispatches `floor(...)`.
    Floor,
    /// Dispatches `gettype(...)`.
    Gettype,
    /// Dispatches `intdiv(...)`.
    Intdiv,
    /// Dispatches `log(...)`.
    Log,
    /// Dispatches `min(...)` and `max(...)`.
    MinMax,
    /// Dispatches `number_format(...)`.
    NumberFormat,
    /// Dispatches `pi()`.
    Pi,
    /// Dispatches `pow(...)`.
    Pow,
    /// Dispatches `round(...)`.
    Round,
    /// Dispatches `sqrt(...)`.
    Sqrt,
    /// Dispatches `strlen(...)`.
    Strlen,
    /// Dispatches `strrev(...)`.
    Strrev,
    /// Dispatches scalar and container type predicates.
    TypePredicate,
}

/// Static declaration for one PHP-visible eval builtin.
pub(in crate::interpreter) struct EvalBuiltinSpec {
    /// Canonical lowercase PHP builtin name.
    pub(in crate::interpreter) name: &'static str,
    /// Builtin family used by the file layout.
    pub(in crate::interpreter) area: EvalArea,
    /// Parameter names in PHP call order.
    pub(in crate::interpreter) param_names: &'static [&'static str],
    /// Parameter metadata in PHP call order.
    pub(in crate::interpreter) params: &'static [EvalParamSpec],
    /// Variadic parameter name, when supported.
    pub(in crate::interpreter) variadic: Option<&'static str>,
    /// Parameter names that must bind by reference.
    pub(in crate::interpreter) by_ref_params: &'static [&'static str],
    /// Direct expression-level dispatch hook.
    pub(in crate::interpreter) direct: Option<EvalDirectHook>,
    /// Evaluated-argument dispatch hook.
    pub(in crate::interpreter) values: Option<EvalValuesHook>,
}

impl EvalBuiltinSpec {
    /// Returns this builtin's file-layout area.
    pub(in crate::interpreter) fn area(&self) -> EvalArea {
        self.area
    }

    /// Returns the number of required leading parameters.
    pub(in crate::interpreter) fn required_param_count(&self) -> usize {
        self.params
            .iter()
            .take_while(|param| param.default.is_none())
            .count()
    }

    /// Returns the number of parameters that define defaults.
    pub(in crate::interpreter) fn default_param_count(&self) -> usize {
        let fixed_defaults = self
            .params
            .iter()
            .filter(|param| param.default.is_some())
            .count();
        fixed_defaults + usize::from(self.variadic.is_some())
    }

    /// Returns by-reference parameter names, checking they agree with param flags in debug builds.
    pub(in crate::interpreter) fn by_ref_param_names(&self) -> &'static [&'static str] {
        debug_assert!(self
            .params
            .iter()
            .filter(|param| param.by_ref)
            .all(|param| self.by_ref_params.contains(&param.name)));
        self.by_ref_params
    }

    /// Returns the default value for one PHP parameter slot.
    pub(in crate::interpreter) fn default_value(
        &self,
        param_index: usize,
    ) -> Option<EvalBuiltinDefaultValue> {
        self.params.get(param_index).and_then(|param| param.default)
    }
}

impl EvalDirectHook {
    /// Runs a direct expression-level builtin call through the migrated hook.
    pub(in crate::interpreter) fn call(
        self,
        name: &str,
        args: &[EvalExpr],
        context: &mut ElephcEvalContext,
        scope: &mut ElephcEvalScope,
        values: &mut impl RuntimeValueOps,
    ) -> Result<RuntimeCellHandle, EvalStatus> {
        match self {
            Self::Abs => eval_builtin_abs(args, context, scope, values),
            Self::Cast => eval_builtin_cast(name, args, context, scope, values),
            Self::Ceil => eval_builtin_ceil(args, context, scope, values),
            Self::Clamp => eval_builtin_clamp(args, context, scope, values),
            Self::Count => eval_builtin_count(args, context, scope, values),
            Self::FloatBinary => eval_builtin_float_binary(name, args, context, scope, values),
            Self::FloatPair => eval_builtin_float_pair(name, args, context, scope, values),
            Self::FloatUnary => eval_builtin_float_unary(name, args, context, scope, values),
            Self::Floor => eval_builtin_floor(args, context, scope, values),
            Self::Gettype => eval_builtin_gettype(args, context, scope, values),
            Self::Intdiv => eval_builtin_intdiv(args, context, scope, values),
            Self::Log => eval_builtin_log(args, context, scope, values),
            Self::MinMax => eval_builtin_min_max(name, args, context, scope, values),
            Self::NumberFormat => eval_builtin_number_format(args, context, scope, values),
            Self::Pi => eval_builtin_pi(args, values),
            Self::Pow => eval_builtin_pow(args, context, scope, values),
            Self::Round => eval_builtin_round(args, context, scope, values),
            Self::Sqrt => eval_builtin_sqrt(args, context, scope, values),
            Self::Strlen => eval_builtin_strlen(args, context, scope, values),
            Self::Strrev => eval_builtin_strrev(args, context, scope, values),
            Self::TypePredicate => eval_builtin_type_predicate(name, args, context, scope, values),
        }
    }
}

impl EvalValuesHook {
    /// Runs an evaluated-argument builtin call through the migrated hook.
    pub(in crate::interpreter) fn call(
        self,
        name: &str,
        evaluated_args: &[RuntimeCellHandle],
        context: &mut ElephcEvalContext,
        values: &mut impl RuntimeValueOps,
    ) -> Result<RuntimeCellHandle, EvalStatus> {
        match self {
            Self::Abs => {
                let [value] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                values.abs(*value)
            }
            Self::Cast => {
                let [value] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                eval_cast_result(name, *value, context, values)
            }
            Self::Ceil => {
                let [value] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                values.ceil(*value)
            }
            Self::Clamp => {
                let [value, min, max] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                eval_clamp_result(*value, *min, *max, values)
            }
            Self::Count => match evaluated_args {
                [value] => eval_count_result(*value, None, context, values),
                [value, mode] => eval_count_result(*value, Some(*mode), context, values),
                _ => Err(EvalStatus::RuntimeFatal),
            },
            Self::FloatBinary => {
                let [left, right] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                eval_float_binary_result(name, *left, *right, values)
            }
            Self::FloatPair => {
                let [left, right] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                eval_float_pair_result(name, *left, *right, values)
            }
            Self::FloatUnary => {
                let [value] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                eval_float_unary_result(name, *value, values)
            }
            Self::Floor => {
                let [value] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                values.floor(*value)
            }
            Self::Gettype => {
                let [value] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                eval_gettype_result(*value, values)
            }
            Self::Intdiv => {
                let [left, right] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                eval_intdiv_result(*left, *right, values)
            }
            Self::Log => match evaluated_args {
                [num] => eval_log_result(*num, None, values),
                [num, base] => eval_log_result(*num, Some(*base), values),
                _ => Err(EvalStatus::RuntimeFatal),
            },
            Self::MinMax => eval_min_max_result(name, evaluated_args, values),
            Self::NumberFormat => match evaluated_args {
                [value] => eval_number_format_result(*value, None, None, None, values),
                [value, decimals] => {
                    eval_number_format_result(*value, Some(*decimals), None, None, values)
                }
                [value, decimals, decimal_separator] => eval_number_format_result(
                    *value,
                    Some(*decimals),
                    Some(*decimal_separator),
                    None,
                    values,
                ),
                [value, decimals, decimal_separator, thousands_separator] => {
                    eval_number_format_result(
                        *value,
                        Some(*decimals),
                        Some(*decimal_separator),
                        Some(*thousands_separator),
                        values,
                    )
                }
                _ => Err(EvalStatus::RuntimeFatal),
            },
            Self::Pi => {
                if !evaluated_args.is_empty() {
                    return Err(EvalStatus::RuntimeFatal);
                }
                values.float(std::f64::consts::PI)
            }
            Self::Pow => {
                let [left, right] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                values.pow(*left, *right)
            }
            Self::Round => match evaluated_args {
                [value] => values.round(*value, None),
                [value, precision] => values.round(*value, Some(*precision)),
                _ => Err(EvalStatus::RuntimeFatal),
            },
            Self::Sqrt => {
                let [value] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                values.sqrt(*value)
            }
            Self::Strlen => {
                let [value] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                let bytes = values.string_bytes(*value)?;
                let len = i64::try_from(bytes.len()).map_err(|_| EvalStatus::RuntimeFatal)?;
                values.int(len)
            }
            Self::Strrev => {
                let [value] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                values.strrev(*value)
            }
            Self::TypePredicate => {
                let [value] = evaluated_args else {
                    return Err(EvalStatus::RuntimeFatal);
                };
                eval_type_predicate_result(name, *value, context, values)
            }
        }
    }
}

inventory::collect!(EvalBuiltinSpec);
