//! Purpose:
//! Declarative eval registry entry for `mb_strlen`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Counts non-continuation UTF-8 bytes to match the static runtime helper.

eval_builtin! {
    name: "mb_strlen",
    area: String,
    params: [string],
    direct: Strlen,
    values: Strlen,
}

use super::super::super::*;

/// Evaluates the builtin `mb_strlen(...)` for one PHP-coerced string argument.
pub(in crate::interpreter) fn eval_builtin_mb_strlen(
    args: &[EvalExpr],
    context: &mut ElephcEvalContext,
    scope: &mut ElephcEvalScope,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let [value] = args else {
        return Err(EvalStatus::RuntimeFatal);
    };
    let value = eval_expr(value, context, scope, values)?;
    eval_mb_strlen_result(value, values)
}

/// Counts UTF-8 code-point-leading bytes in one materialized eval string.
pub(in crate::interpreter) fn eval_mb_strlen_result(
    value: RuntimeCellHandle,
    values: &mut impl RuntimeValueOps,
) -> Result<RuntimeCellHandle, EvalStatus> {
    let bytes = values.string_bytes(value)?;
    let len = bytes
        .iter()
        .filter(|byte| (**byte & 0xc0) != 0x80)
        .count();
    let len = i64::try_from(len).map_err(|_| EvalStatus::RuntimeFatal)?;
    values.int(len)
}
