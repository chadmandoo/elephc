//! Purpose:
//! Function-name probes for eval `function_exists()` and function-like symbol lookup.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols` re-exports.
//!
//! Key details:
//! - Procedural date/time aliases stay visible to eval because static name
//!   resolver rewrites do not run inside runtime eval fragments.

use super::*;

const EVAL_DATE_PROCEDURAL_ALIAS_FUNCTIONS: &[&str] = &[
    "cal_days_in_month",
    "cal_from_jd",
    "cal_info",
    "cal_to_jd",
    "date_add",
    "date_create",
    "date_create_from_format",
    "date_create_immutable",
    "date_create_immutable_from_format",
    "date_date_set",
    "date_diff",
    "date_format",
    "date_get_last_errors",
    "date_interval_create_from_date_string",
    "date_interval_format",
    "date_isodate_set",
    "date_modify",
    "date_offset_get",
    "date_parse",
    "date_parse_from_format",
    "date_sub",
    "date_sun_info",
    "date_sunrise",
    "date_sunset",
    "date_time_set",
    "date_timestamp_get",
    "date_timestamp_set",
    "date_timezone_get",
    "date_timezone_set",
    "easter_date",
    "easter_days",
    "frenchtojd",
    "gettimeofday",
    "gmstrftime",
    "gregoriantojd",
    "idate",
    "jddayofweek",
    "jdmonthname",
    "jdtofrench",
    "jdtogregorian",
    "jdtojewish",
    "jdtojulian",
    "jdtounix",
    "jewishtojd",
    "juliantojd",
    "mktime",
    "gmmktime",
    "strftime",
    "strptime",
    "timezone_abbreviations_list",
    "timezone_identifiers_list",
    "timezone_location_get",
    "timezone_name_from_abbr",
    "timezone_name_get",
    "timezone_offset_get",
    "timezone_open",
    "timezone_transitions_get",
    "timezone_version_get",
    "unixtojd",
];

/// Evaluates `function_exists()` and `is_callable()` inside an eval fragment.
pub(in crate::interpreter) fn eval_builtin_function_probe(
    name: &str,
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    match name {
        "function_exists" => {
            let [value] = args else {
                return Err(EvalStatus::RuntimeFatal);
            };
            let value = eval_expr(value, context, scope, values)?;
            eval_function_probe_result(name, value, context, values)
        }
        "is_callable" => eval_builtin_is_callable(args, context, scope, values),
        _ => Err(EvalStatus::UnsupportedConstruct),
    }
}

/// Evaluates `function_exists()` and `is_callable()` from materialized arguments.
pub(in crate::interpreter) fn eval_function_probe_result(
    name: &str,
    value: RuntimeCellHandle,
    context: &ElephcEvalContext,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let exists = match name {
        "function_exists" => {
            let name = eval_function_probe_name(value, values)?;
            eval_function_probe_exists(context, &name)
        }
        "is_callable" => eval_is_callable_value(value, None, context, values)?,
        _ => return Err(EvalStatus::UnsupportedConstruct),
    };
    values.bool_value(exists)
}

/// Returns true when a PHP function name is visible to eval builtin probes.
pub(in crate::interpreter) fn eval_function_probe_exists(
    context: &ElephcEvalContext,
    name: &str,
) -> bool {
    !name.contains("::")
        && (context.has_function(name)
            || eval_php_visible_builtin_exists(name)
            || eval_date_procedural_alias_exists(name))
}

/// Returns true for DateTime/calendar/timezone aliases that static elephc desugars.
fn eval_date_procedural_alias_exists(name: &str) -> bool {
    let bare = name.rsplit('\\').next().unwrap_or("");
    EVAL_DATE_PROCEDURAL_ALIAS_FUNCTIONS.contains(&bare)
}

/// Reads and normalizes a function-probe string argument.
fn eval_function_probe_name(
    name: RuntimeCellHandle,
    values: &mut impl RuntimeValueOps,
) -> Result<String, EvalStatus> {
    let name = values.string_bytes(name)?;
    let name = String::from_utf8(name).map_err(|_| EvalStatus::RuntimeFatal)?;
    Ok(name.trim_start_matches('\\').to_ascii_lowercase())
}
