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

/// Lowers a target owned by bounded dispatch group 01, or returns `None`.
pub(super) fn lower(
    ctx: &mut FunctionContext<'_>,
    inst: &Instruction,
    target: BuiltinRuntimeTarget,
) -> Option<Result<()>> {
    match target {
        BuiltinRuntimeTarget::ArrayShift => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_array_shift(ctx, inst)
        }),
        BuiltinRuntimeTarget::ArraySlice => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_array_slice(ctx, inst)
        }),
        BuiltinRuntimeTarget::ArraySplice => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_array_splice(ctx, inst)
        }),
        BuiltinRuntimeTarget::ArraySum => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_array_sum(ctx, inst)
        }),
        BuiltinRuntimeTarget::ArrayUdiff => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_array_udiff(ctx, inst)
        }),
        BuiltinRuntimeTarget::ArrayUintersect => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_array_uintersect(ctx, inst)
        }),
        BuiltinRuntimeTarget::ArrayUnique => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_array_unique(ctx, inst)
        }),
        BuiltinRuntimeTarget::ArrayUnshift => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_array_unshift(ctx, inst)
        }),
        BuiltinRuntimeTarget::ArrayValues => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_array_values(ctx, inst)
        }),
        BuiltinRuntimeTarget::ArrayWalk => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_array_walk(ctx, inst)
        }),
        BuiltinRuntimeTarget::ArrayWalkRecursive => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_array_walk_recursive(ctx, inst)
        }),
        BuiltinRuntimeTarget::Arsort => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_arsort(ctx, inst)
        }),
        BuiltinRuntimeTarget::Asort => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_asort(ctx, inst)
        }),
        BuiltinRuntimeTarget::Count => Some({
            crate::codegen::lower_inst::builtins::lower_count(ctx, inst)
        }),
        BuiltinRuntimeTarget::InArray => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_in_array(ctx, inst)
        }),
        BuiltinRuntimeTarget::Krsort => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_krsort(ctx, inst)
        }),
        BuiltinRuntimeTarget::Ksort => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_ksort(ctx, inst)
        }),
        BuiltinRuntimeTarget::Natcasesort => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_natcasesort(ctx, inst)
        }),
        BuiltinRuntimeTarget::Natsort => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_natsort(ctx, inst)
        }),
        BuiltinRuntimeTarget::Range => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_range(ctx, inst)
        }),
        BuiltinRuntimeTarget::Rsort => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_rsort(ctx, inst)
        }),
        BuiltinRuntimeTarget::Shuffle => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_shuffle(ctx, inst)
        }),
        BuiltinRuntimeTarget::Sort => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_sort(ctx, inst)
        }),
        BuiltinRuntimeTarget::Uasort => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_uasort(ctx, inst)
        }),
        BuiltinRuntimeTarget::Uksort => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_uksort(ctx, inst)
        }),
        BuiltinRuntimeTarget::Usort => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_usort(ctx, inst)
        }),
        BuiltinRuntimeTarget::CallUserFunc => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_call_user_func_builtin_escape(ctx, inst, "call_user_func")
        }),
        BuiltinRuntimeTarget::CallUserFuncArray => Some({
            crate::codegen::lower_inst::builtins::arrays::lower_call_user_func_builtin_escape(ctx, inst, "call_user_func_array")
        }),
        BuiltinRuntimeTarget::ClassAlias => Some({
            crate::codegen::lower_inst::builtins::types::lower_class_alias(ctx, inst)
        }),
        BuiltinRuntimeTarget::ClassExists => Some({
            crate::codegen::lower_inst::builtins::lower_class_like_exists(ctx, inst, "class_exists")
        }),
        BuiltinRuntimeTarget::ClassImplements => Some({
            crate::codegen::lower_inst::builtins::class_relations::lower_class_relation(
                    ctx,
                    inst,
                    "class_implements",
                )
        }),
        BuiltinRuntimeTarget::ClassParents => Some({
            crate::codegen::lower_inst::builtins::class_relations::lower_class_relation(
                    ctx,
                    inst,
                    "class_parents",
                )
        }),
        BuiltinRuntimeTarget::ClassUses => Some({
            crate::codegen::lower_inst::builtins::class_relations::lower_class_relation(
                    ctx,
                    inst,
                    "class_uses",
                )
        }),
        BuiltinRuntimeTarget::EnumExists => Some({
            crate::codegen::lower_inst::builtins::lower_class_like_exists(ctx, inst, "enum_exists")
        }),
        BuiltinRuntimeTarget::FunctionExists => Some({
            crate::codegen::lower_inst::builtins::lower_function_exists(ctx, inst)
        }),
        _ => None,
    }
}
