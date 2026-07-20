//! Purpose:
//! Dispatches one bounded group of typed builtin runtime targets.
//!
//! Called from:
//! - `super::lower()` while lowering typed EIR runtime calls.
//!
//! Key details:
//! - Dispatch is by enum identity, never by PHP function-name strings.
//! - Extracted bodies remain thin calls into target-aware backend emitters.

use crate::codegen::context::FunctionContext;
use crate::codegen::Result;
use crate::ir::{BuiltinRuntimeTarget, Instruction};
use crate::types::PhpType;

/// Lowers a target owned by bounded dispatch group 12, or returns `None`.
pub(super) fn lower(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    target: BuiltinRuntimeTarget,
) -> Option<Result<()>> {
    match target {
        BuiltinRuntimeTarget::MethodExists => Some({
            crate::codegen::lower_inst::builtins::lower_member_exists(
                ctx,
                inst,
                "method_exists",
            )
        }),
        BuiltinRuntimeTarget::PropertyExists => Some({
            crate::codegen::lower_inst::builtins::lower_member_exists(
                ctx,
                inst,
                "property_exists",
            )
        }),
        BuiltinRuntimeTarget::Strval => Some({
            crate::codegen::lower_inst::builtins::lower_strval(ctx, inst)
        }),
        BuiltinRuntimeTarget::System => Some({
            crate::codegen::lower_inst::builtins::system::lower_system(ctx, inst)
        }),
        BuiltinRuntimeTarget::Time => Some({
            crate::codegen::lower_inst::builtins::system::lower_time(ctx, inst)
        }),
        BuiltinRuntimeTarget::Unserialize => Some({
            crate::codegen::lower_inst::builtins::serialize::lower_unserialize(ctx, inst)
        }),
        BuiltinRuntimeTarget::Usleep => Some({
            crate::codegen::lower_inst::builtins::system::lower_usleep(ctx, inst)
        }),
        BuiltinRuntimeTarget::Boolval => Some({
            crate::codegen::lower_inst::builtins::lower_boolval(ctx, inst)
        }),
        BuiltinRuntimeTarget::Floatval => Some({
            crate::codegen::lower_inst::builtins::lower_floatval(ctx, inst)
        }),
        BuiltinRuntimeTarget::GetResourceId => Some({
            crate::codegen::lower_inst::builtins::types::lower_get_resource_id(ctx, inst)
        }),
        BuiltinRuntimeTarget::GetResourceType => Some({
            crate::codegen::lower_inst::builtins::types::lower_get_resource_type(ctx, inst)
        }),
        BuiltinRuntimeTarget::Gettype => Some({
            crate::codegen::lower_inst::builtins::lower_gettype(ctx, inst)
        }),
        BuiltinRuntimeTarget::Intval => Some({
            crate::codegen::lower_inst::builtins::lower_intval(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsArray => Some({
            crate::codegen::lower_inst::builtins::lower_is_array(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsBool => Some({
            crate::codegen::lower_inst::builtins::lower_static_type_predicate(
                    ctx,
                    inst,
                    "is_bool",
                    PhpType::Bool,
                )
        }),
        BuiltinRuntimeTarget::IsCallable => Some({
            crate::codegen::lower_inst::builtins::lower_is_callable(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsFinite => Some({
            crate::codegen::lower_inst::builtins::math::lower_is_finite(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsFloat => Some({
            crate::codegen::lower_inst::builtins::lower_static_type_predicate(
                    ctx,
                    inst,
                    "is_float",
                    PhpType::Float,
                )
        }),
        BuiltinRuntimeTarget::IsInfinite => Some({
            crate::codegen::lower_inst::builtins::math::lower_is_infinite(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsInt => Some({
            crate::codegen::lower_inst::builtins::lower_static_type_predicate(
                    ctx,
                    inst,
                    "is_int",
                    PhpType::Int,
                )
        }),
        BuiltinRuntimeTarget::IsIterable => Some({
            crate::codegen::lower_inst::builtins::lower_is_iterable(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsNan => Some({
            crate::codegen::lower_inst::builtins::math::lower_is_nan(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsNull => Some({
            crate::codegen::lower_inst::builtins::lower_is_null_builtin(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsNumeric => Some({
            crate::codegen::lower_inst::builtins::is_numeric::lower_is_numeric(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsObject => Some({
            crate::codegen::lower_inst::builtins::lower_is_object(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsResource => Some({
            crate::codegen::lower_inst::builtins::types::lower_is_resource(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsScalar => Some({
            crate::codegen::lower_inst::builtins::lower_is_scalar(ctx, inst)
        }),
        BuiltinRuntimeTarget::IsString => Some({
            crate::codegen::lower_inst::builtins::lower_static_type_predicate(
                    ctx,
                    inst,
                    "is_string",
                    PhpType::Str,
                )
        }),
        BuiltinRuntimeTarget::Settype => Some({
            crate::codegen::lower_inst::builtins::types::lower_settype(ctx, inst)
        }),
        _ => None,
    }
}
