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

/// Lowers a target owned by bounded dispatch group 07, or returns `None`.
pub(super) fn lower(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    target: BuiltinRuntimeTarget,
) -> Option<Result<()>> {
    match target {
        BuiltinRuntimeTarget::Touch => Some({
            crate::codegen::lower_inst::builtins::io::lower_touch(ctx, inst)
        }),
        BuiltinRuntimeTarget::Umask => Some({
            crate::codegen::lower_inst::builtins::io::lower_umask(ctx, inst)
        }),
        BuiltinRuntimeTarget::Unlink => Some({
            crate::codegen::lower_inst::builtins::io::lower_unlink(ctx, inst)
        }),
        BuiltinRuntimeTarget::VarDump => Some({
            crate::codegen::lower_inst::builtins::debug::lower_var_dump(ctx, inst)
        }),
        BuiltinRuntimeTarget::Vfprintf => Some({
            crate::codegen::lower_inst::builtins::io::lower_vfprintf(ctx, inst)
        }),
        BuiltinRuntimeTarget::Abs => Some({
            crate::codegen::lower_inst::builtins::math::lower_abs(ctx, inst)
        }),
        BuiltinRuntimeTarget::Acos => Some({
            crate::codegen::lower_inst::builtins::math::lower_unary_libm(ctx, inst, "acos")
        }),
        BuiltinRuntimeTarget::Asin => Some({
            crate::codegen::lower_inst::builtins::math::lower_unary_libm(ctx, inst, "asin")
        }),
        BuiltinRuntimeTarget::Atan => Some({
            crate::codegen::lower_inst::builtins::math::lower_unary_libm(ctx, inst, "atan")
        }),
        BuiltinRuntimeTarget::Atan2 => Some({
            crate::codegen::lower_inst::builtins::math::lower_atan2(ctx, inst)
        }),
        BuiltinRuntimeTarget::Ceil => Some({
            crate::codegen::lower_inst::builtins::math::lower_ceil(ctx, inst)
        }),
        BuiltinRuntimeTarget::Clamp => Some({
            crate::codegen::lower_inst::builtins::math::lower_clamp(ctx, inst)
        }),
        BuiltinRuntimeTarget::Cos => Some({
            crate::codegen::lower_inst::builtins::math::lower_unary_libm(ctx, inst, "cos")
        }),
        BuiltinRuntimeTarget::Cosh => Some({
            crate::codegen::lower_inst::builtins::math::lower_unary_libm(ctx, inst, "cosh")
        }),
        BuiltinRuntimeTarget::Deg2rad => Some({
            crate::codegen::lower_inst::builtins::math::lower_deg2rad(ctx, inst)
        }),
        BuiltinRuntimeTarget::Exp => Some({
            crate::codegen::lower_inst::builtins::math::lower_unary_libm(ctx, inst, "exp")
        }),
        BuiltinRuntimeTarget::Fdiv => Some({
            crate::codegen::lower_inst::builtins::math::lower_fdiv(ctx, inst)
        }),
        BuiltinRuntimeTarget::Floor => Some({
            crate::codegen::lower_inst::builtins::math::lower_floor(ctx, inst)
        }),
        BuiltinRuntimeTarget::Fmod => Some({
            crate::codegen::lower_inst::builtins::math::lower_fmod(ctx, inst)
        }),
        BuiltinRuntimeTarget::Hypot => Some({
            crate::codegen::lower_inst::builtins::math::lower_hypot(ctx, inst)
        }),
        BuiltinRuntimeTarget::Intdiv => Some({
            crate::codegen::lower_inst::builtins::math::lower_intdiv(ctx, inst)
        }),
        BuiltinRuntimeTarget::Log => Some({
            crate::codegen::lower_inst::builtins::math::lower_log(ctx, inst)
        }),
        BuiltinRuntimeTarget::Log10 => Some({
            crate::codegen::lower_inst::builtins::math::lower_unary_libm(ctx, inst, "log10")
        }),
        BuiltinRuntimeTarget::Log2 => Some({
            crate::codegen::lower_inst::builtins::math::lower_unary_libm(ctx, inst, "log2")
        }),
        BuiltinRuntimeTarget::Max => Some({
            crate::codegen::lower_inst::builtins::math::lower_min_max(ctx, inst, true)
        }),
        BuiltinRuntimeTarget::Min => Some({
            crate::codegen::lower_inst::builtins::math::lower_min_max(ctx, inst, false)
        }),
        BuiltinRuntimeTarget::MtRand => Some({
            crate::codegen::lower_inst::builtins::math::lower_rand(ctx, inst, "mt_rand")
        }),
        BuiltinRuntimeTarget::Pi => Some({
            crate::codegen::lower_inst::builtins::math::lower_pi(ctx, inst)
        }),
        BuiltinRuntimeTarget::Pow => Some({
            crate::codegen::lower_inst::builtins::math::lower_pow(ctx, inst)
        }),
        BuiltinRuntimeTarget::Rad2deg => Some({
            crate::codegen::lower_inst::builtins::math::lower_rad2deg(ctx, inst)
        }),
        BuiltinRuntimeTarget::Rand => Some({
            crate::codegen::lower_inst::builtins::math::lower_rand(ctx, inst, "rand")
        }),
        BuiltinRuntimeTarget::RandomInt => Some({
            crate::codegen::lower_inst::builtins::math::lower_random_int(ctx, inst)
        }),
        BuiltinRuntimeTarget::Round => Some({
            crate::codegen::lower_inst::builtins::math::lower_round(ctx, inst)
        }),
        BuiltinRuntimeTarget::Sin => Some({
            crate::codegen::lower_inst::builtins::math::lower_unary_libm(ctx, inst, "sin")
        }),
        BuiltinRuntimeTarget::Sinh => Some({
            crate::codegen::lower_inst::builtins::math::lower_unary_libm(ctx, inst, "sinh")
        }),
        _ => None,
    }
}
