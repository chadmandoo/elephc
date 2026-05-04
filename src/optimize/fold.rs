mod casts;
mod expr;
mod ops;
mod scalar;

pub(super) use expr::{fold_enum_case, fold_expr, fold_method, fold_params, fold_property};
pub(super) use scalar::{assigned_scalar_value, scalar_value, ScalarValue};
