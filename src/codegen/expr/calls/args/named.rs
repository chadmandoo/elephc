use crate::codegen::emit::Emitter;
use crate::codegen::{abi, context::Context, data_section::DataSection};
use crate::parser::ast::{Expr, ExprKind};
use crate::span::Span;
use crate::types::call_args;
use crate::types::{FunctionSig, PhpType};

use super::{
    array_element_stride, declared_target_ty, emit_array_length_bounds_check,
    emit_empty_variadic_array_arg, emit_named_spread_length_abort, emit_ref_arg_variable_address,
    load_array_element_to_result, push_arg_value, push_expr_arg, push_loaded_array_element_arg,
    spread_source_elem_ty, store_current_array_element, variadic_container_elem_ty,
    EmittedCallArgs,
};

#[derive(Clone)]
enum FinalArgSource {
    SourceTemp(usize),
    PrefixElement {
        prefix_temp_idx: usize,
        element_idx: usize,
        default: Option<Expr>,
    },
    Default(Expr),
}

#[derive(Clone)]
struct VariadicArgSource {
    key: Option<String>,
    source: FinalArgSource,
}

pub(super) fn emit_source_order_named_call_args(
    args_exprs: &[Expr],
    sig: &FunctionSig,
    regular_param_count: usize,
    ref_arg_context_label: &str,
    retain_non_variable_ref_args: bool,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> EmittedCallArgs {
    let plan = call_args::plan_call_args_with_regular_param_count(
        sig,
        args_exprs,
        Span::dummy(),
        regular_param_count,
        false,
        true,
    )
        .expect("codegen received invalid named call arguments after type checking");
    debug_assert!(plan.has_named_args());

    if plan.has_spread_args() {
        return emit_source_order_named_spread_call_args(
            &plan,
            sig,
            regular_param_count,
            Span::dummy(),
            emitter,
            ctx,
            data,
        );
    }

    emit_source_order_named_non_spread_call_args(
        &plan,
        sig,
        regular_param_count,
        ref_arg_context_label,
        retain_non_variable_ref_args,
        emitter,
        ctx,
        data,
    )
}

fn emit_source_order_named_non_spread_call_args(
    plan: &call_args::CallArgPlan,
    sig: &FunctionSig,
    regular_param_count: usize,
    ref_arg_context_label: &str,
    retain_non_variable_ref_args: bool,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> EmittedCallArgs {
    let mut slot_sources: Vec<Option<FinalArgSource>> = vec![None; regular_param_count];
    let mut variadic_sources = Vec::new();
    let mut source_temp_types = Vec::new();
    let mut source_temp_by_index: Vec<Option<usize>> = vec![None; plan.source_args.len()];

    for source in &plan.source_values {
        match source {
            call_args::PlannedSourceValue::Regular {
                source_index,
                param_idx,
                expr,
            } => {
                let temp_idx = emit_source_temp_arg(
                    expr,
                    sig,
                    Some(*param_idx),
                    ref_arg_context_label,
                    retain_non_variable_ref_args,
                    &mut source_temp_types,
                    emitter,
                    ctx,
                    data,
                );
                source_temp_by_index[*source_index] = Some(temp_idx);
            }
            call_args::PlannedSourceValue::Variadic {
                source_index,
                key,
                expr,
            } => {
                let temp_idx = emit_source_temp_arg(
                    expr,
                    sig,
                    None,
                    ref_arg_context_label,
                    retain_non_variable_ref_args,
                    &mut source_temp_types,
                    emitter,
                    ctx,
                    data,
                );
                source_temp_by_index[*source_index] = Some(temp_idx);
                variadic_sources.push(VariadicArgSource {
                    key: key.clone(),
                    source: FinalArgSource::SourceTemp(temp_idx),
                });
            }
        }
    }

    for (idx, planned) in plan.regular_args.iter().enumerate() {
        match planned {
            call_args::PlannedRegularArg::Source { source_index, .. } => {
                let temp_idx = source_temp_by_index[*source_index]
                    .expect("planned regular source was not evaluated");
                slot_sources[idx] = Some(FinalArgSource::SourceTemp(temp_idx));
            }
            call_args::PlannedRegularArg::Default(default) => {
                slot_sources[idx] = Some(FinalArgSource::Default(default.clone()));
            }
            call_args::PlannedRegularArg::SpreadElement { .. } => {
                unreachable!("non-spread named call plan contained a spread element");
            }
        }
    }

    push_final_call_args_from_sources(
        slot_sources,
        variadic_sources,
        sig,
        regular_param_count,
        &source_temp_types,
        emitter,
        ctx,
        data,
    )
}

fn emit_source_order_named_spread_call_args(
    plan: &call_args::CallArgPlan,
    sig: &FunctionSig,
    regular_param_count: usize,
    call_span: Span,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> EmittedCallArgs {
    let first_named_pos = plan.first_named_pos.unwrap_or(plan.source_args.len());
    let prefix_args = &plan.source_args[..first_named_pos];
    let prefix_span = prefix_args
        .first()
        .map(|arg| arg.span)
        .unwrap_or_else(Span::dummy);
    let prefix_expr = plan
        .positional_prefix_expr(call_span)
        .unwrap_or_else(|| Expr::new(ExprKind::ArrayLiteral(Vec::new()), prefix_span));
    let mut source_temp_types = Vec::new();
    emitter.comment("evaluate named-call positional prefix");
    let prefix_ty = push_expr_arg(&prefix_expr, None, emitter, ctx, data);
    let prefix_temp_idx = push_source_temp_type(&mut source_temp_types, prefix_ty);

    let mut source_temp_by_index: Vec<Option<usize>> = vec![None; plan.source_args.len()];
    let mut variadic_sources = Vec::new();
    for source in &plan.source_values {
        if source.source_index() < first_named_pos {
            continue;
        }
        let param_idx = source.param_idx();
        let temp_idx = emit_source_temp_arg(
            source.expr(),
            sig,
            param_idx,
            if param_idx.is_some() {
                "named arg"
            } else {
                "named variadic arg"
            },
            false,
            &mut source_temp_types,
            emitter,
            ctx,
            data,
        );
        source_temp_by_index[source.source_index()] = Some(temp_idx);
        if param_idx.is_none() {
            variadic_sources.push(VariadicArgSource {
                key: source.key().map(str::to_string),
                source: FinalArgSource::SourceTemp(temp_idx),
            });
        }
    }

    let max_prefix_len = plan
        .regular_args
        .iter()
        .filter_map(|planned| match planned {
            call_args::PlannedRegularArg::SpreadElement {
                prefix_element_idx,
                ..
            } => Some(prefix_element_idx + 1),
            _ => None,
        })
        .max()
        .unwrap_or(0);
    let min_prefix_len = plan
        .regular_args
        .iter()
        .filter_map(|planned| match planned {
            call_args::PlannedRegularArg::SpreadElement {
                prefix_element_idx,
                default,
                ..
            } if default.is_none() => Some(prefix_element_idx + 1),
            _ => None,
        })
        .max()
        .unwrap_or(0);
    emit_prefix_array_length_check(
        prefix_temp_idx,
        &source_temp_types,
        min_prefix_len,
        max_prefix_len,
        emitter,
        ctx,
        data,
    );

    let mut slot_sources = Vec::new();
    for planned in &plan.regular_args {
        match planned {
            call_args::PlannedRegularArg::Source { source_index, .. } => {
                let temp_idx = source_temp_by_index[*source_index]
                    .expect("planned named source was not evaluated");
                slot_sources.push(Some(FinalArgSource::SourceTemp(temp_idx)));
            }
            call_args::PlannedRegularArg::SpreadElement {
                prefix_element_idx,
                default,
                ..
            } => {
                slot_sources.push(Some(FinalArgSource::PrefixElement {
                    prefix_temp_idx,
                    element_idx: *prefix_element_idx,
                    default: default.clone(),
                }));
            }
            call_args::PlannedRegularArg::Default(default) => {
                slot_sources.push(Some(FinalArgSource::Default(default.clone())));
            }
        }
    }

    push_final_call_args_from_sources(
        slot_sources,
        variadic_sources,
        sig,
        regular_param_count,
        &source_temp_types,
        emitter,
        ctx,
        data,
    )
}

fn push_source_temp_type(source_temp_types: &mut Vec<PhpType>, ty: PhpType) -> usize {
    let idx = source_temp_types.len();
    source_temp_types.push(ty);
    idx
}

#[allow(clippy::too_many_arguments)]
fn emit_source_temp_arg(
    arg: &Expr,
    sig: &FunctionSig,
    param_idx: Option<usize>,
    ref_arg_context_label: &str,
    retain_non_variable_ref_args: bool,
    source_temp_types: &mut Vec<PhpType>,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> usize {
    let is_ref = param_idx
        .and_then(|idx| sig.ref_params.get(idx))
        .copied()
        .unwrap_or(false);
    let pushed_ty = if is_ref {
        if let ExprKind::Variable(var_name) = &arg.kind {
            emit_ref_arg_variable_address(var_name, ref_arg_context_label, emitter, ctx);
        } else {
            let source_ty = super::super::super::emit_expr(arg, emitter, ctx, data);
            if retain_non_variable_ref_args {
                super::super::super::retain_borrowed_heap_arg(emitter, arg, &source_ty);
            }
        }
        push_arg_value(emitter, &PhpType::Int);
        PhpType::Int
    } else {
        let target_ty = param_idx.and_then(|idx| declared_target_ty(Some(sig), idx));
        push_expr_arg(arg, target_ty, emitter, ctx, data)
    };
    push_source_temp_type(source_temp_types, pushed_ty)
}

fn push_final_call_args_from_sources(
    slot_sources: Vec<Option<FinalArgSource>>,
    variadic_sources: Vec<VariadicArgSource>,
    sig: &FunctionSig,
    regular_param_count: usize,
    source_temp_types: &[PhpType],
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> EmittedCallArgs {
    let source_temp_bytes = pushed_temp_bytes(source_temp_types);
    let mut arg_types = Vec::new();
    let mut final_pushed_bytes = 0usize;

    for (idx, source) in slot_sources.into_iter().enumerate().take(regular_param_count) {
        let target_ty = declared_target_ty(Some(sig), idx);
        let pushed_ty = match source {
            Some(FinalArgSource::SourceTemp(temp_idx)) => {
                push_saved_source_temp_arg(
                    temp_idx,
                    source_temp_types,
                    final_pushed_bytes,
                    emitter,
                )
            }
            Some(FinalArgSource::PrefixElement {
                prefix_temp_idx,
                element_idx,
                default,
            }) => push_prefix_array_element_arg(
                prefix_temp_idx,
                element_idx,
                default.as_ref(),
                target_ty,
                source_temp_types,
                final_pushed_bytes,
                emitter,
                ctx,
                data,
            ),
            Some(FinalArgSource::Default(default)) => {
                push_expr_arg(&default, target_ty, emitter, ctx, data)
            }
            None => continue,
        };
        final_pushed_bytes += temp_slot_size(&pushed_ty);
        arg_types.push(pushed_ty);
    }

    if sig.variadic.is_some() {
        let variadic_ty = if variadic_sources.is_empty() {
            emit_empty_variadic_array_arg("empty variadic array", emitter)
        } else {
            emit_variadic_array_arg_from_sources(
                &variadic_sources,
                source_temp_types,
                final_pushed_bytes,
                emitter,
                ctx,
                data,
            )
        };
        arg_types.push(variadic_ty);
    }

    EmittedCallArgs {
        arg_types,
        source_temp_bytes,
    }
}

fn temp_slot_size(ty: &PhpType) -> usize {
    if matches!(ty, PhpType::Void | PhpType::Never) {
        0
    } else {
        16
    }
}

pub(crate) fn pushed_temp_bytes(types: &[PhpType]) -> usize {
    types.iter().map(temp_slot_size).sum()
}

fn temp_offsets(types: &[PhpType]) -> Vec<usize> {
    let mut offsets = vec![0usize; types.len()];
    let mut running = 0usize;
    for idx in (0..types.len()).rev() {
        offsets[idx] = running;
        running += temp_slot_size(&types[idx]);
    }
    offsets
}

fn source_temp_offset(source_temp_types: &[PhpType], temp_idx: usize, extra_bytes: usize) -> usize {
    extra_bytes + temp_offsets(source_temp_types)[temp_idx]
}

fn load_source_temp_to_result(
    temp_idx: usize,
    source_temp_types: &[PhpType],
    extra_bytes: usize,
    emitter: &mut Emitter,
) -> PhpType {
    let ty = source_temp_types[temp_idx].clone();
    let offset = source_temp_offset(source_temp_types, temp_idx, extra_bytes);
    match ty.codegen_repr() {
        PhpType::Float => {
            abi::emit_load_temporary_stack_slot(emitter, abi::float_result_reg(emitter), offset);
        }
        PhpType::Str => {
            let (ptr_reg, len_reg) = abi::string_result_regs(emitter);
            abi::emit_load_temporary_stack_slot(emitter, ptr_reg, offset);
            abi::emit_load_temporary_stack_slot(emitter, len_reg, offset + 8);
        }
        PhpType::Void | PhpType::Never => {}
        _ => {
            abi::emit_load_temporary_stack_slot(emitter, abi::int_result_reg(emitter), offset);
        }
    }
    ty
}

fn push_saved_source_temp_arg(
    temp_idx: usize,
    source_temp_types: &[PhpType],
    final_pushed_bytes: usize,
    emitter: &mut Emitter,
) -> PhpType {
    let ty = load_source_temp_to_result(temp_idx, source_temp_types, final_pushed_bytes, emitter);
    push_arg_value(emitter, &ty);
    ty
}

fn emit_prefix_array_length_check(
    prefix_temp_idx: usize,
    source_temp_types: &[PhpType],
    min_len: usize,
    max_len: usize,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) {
    let ok_label = ctx.next_label("named_prefix_len_ok");
    let fail_label = ctx.next_label("named_prefix_len_fail");
    emitter.comment("validate named-argument positional prefix length");
    let prefix_offset = source_temp_offset(source_temp_types, prefix_temp_idx, 0);
    match emitter.target.arch {
        crate::codegen::platform::Arch::AArch64 => {
            abi::emit_load_temporary_stack_slot(emitter, "x8", prefix_offset);
            emitter.instruction("ldr x9, [x8]");                                // load the evaluated positional-prefix array length
            emit_array_length_bounds_check("x9", min_len, max_len, &fail_label, &ok_label, emitter);
        }
        crate::codegen::platform::Arch::X86_64 => {
            abi::emit_load_temporary_stack_slot(emitter, "r8", prefix_offset);
            emitter.instruction("mov r10, QWORD PTR [r8]");                     // load the evaluated positional-prefix array length
            emit_array_length_bounds_check("r10", min_len, max_len, &fail_label, &ok_label, emitter);
        }
    }
    emitter.label(&fail_label);
    emit_named_spread_length_abort(emitter, data);
    emitter.label(&ok_label);
}

fn push_prefix_array_element_arg(
    prefix_temp_idx: usize,
    element_idx: usize,
    default: Option<&Expr>,
    target_ty: Option<&PhpType>,
    source_temp_types: &[PhpType],
    final_pushed_bytes: usize,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> PhpType {
    if let Some(default) = default {
        let use_default = ctx.next_label("named_prefix_default");
        let done = ctx.next_label("named_prefix_done");
        emit_branch_if_prefix_element_missing(
            prefix_temp_idx,
            element_idx,
            source_temp_types,
            final_pushed_bytes,
            &use_default,
            emitter,
        );
        let loaded_ty = push_existing_prefix_array_element_arg(
            prefix_temp_idx,
            element_idx,
            target_ty,
            source_temp_types,
            final_pushed_bytes,
            emitter,
            ctx,
            data,
        );
        abi::emit_jump(emitter, &done);
        emitter.label(&use_default);
        let default_ty = push_expr_arg(default, target_ty, emitter, ctx, data);
        emitter.label(&done);
        return super::super::super::widen_codegen_type(&loaded_ty, &default_ty);
    }

    push_existing_prefix_array_element_arg(
        prefix_temp_idx,
        element_idx,
        target_ty,
        source_temp_types,
        final_pushed_bytes,
        emitter,
        ctx,
        data,
    )
}

fn emit_branch_if_prefix_element_missing(
    prefix_temp_idx: usize,
    element_idx: usize,
    source_temp_types: &[PhpType],
    final_pushed_bytes: usize,
    label: &str,
    emitter: &mut Emitter,
) {
    let prefix_offset = source_temp_offset(source_temp_types, prefix_temp_idx, final_pushed_bytes);
    match emitter.target.arch {
        crate::codegen::platform::Arch::AArch64 => {
            abi::emit_load_temporary_stack_slot(emitter, "x8", prefix_offset);
            emitter.instruction("ldr x9, [x8]");                                // load prefix length before choosing spread element or default
            abi::emit_load_int_immediate(emitter, "x10", element_idx as i64);
            emitter.instruction("cmp x9, x10");                                 // check whether this optional prefix element exists
            emitter.instruction(&format!("b.le {}", label));                    // use the default when the prefix is too short for this slot
        }
        crate::codegen::platform::Arch::X86_64 => {
            abi::emit_load_temporary_stack_slot(emitter, "r8", prefix_offset);
            emitter.instruction("mov r10, QWORD PTR [r8]");                     // load prefix length before choosing spread element or default
            abi::emit_load_int_immediate(emitter, "r11", element_idx as i64);
            emitter.instruction("cmp r10, r11");                                // check whether this optional prefix element exists
            emitter.instruction(&format!("jle {}", label));                     // use the default when the prefix is too short for this slot
        }
    }
}

fn push_existing_prefix_array_element_arg(
    prefix_temp_idx: usize,
    element_idx: usize,
    target_ty: Option<&PhpType>,
    source_temp_types: &[PhpType],
    final_pushed_bytes: usize,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> PhpType {
    let prefix_ty = source_temp_types[prefix_temp_idx].clone();
    let source_elem_ty = spread_source_elem_ty(&prefix_ty);
    let elem_stride = array_element_stride(&source_elem_ty);
    let prefix_offset = source_temp_offset(source_temp_types, prefix_temp_idx, final_pushed_bytes);
    let array_data_reg = match emitter.target.arch {
        crate::codegen::platform::Arch::AArch64 => "x20",
        crate::codegen::platform::Arch::X86_64 => "r10",
    };
    abi::emit_load_temporary_stack_slot(emitter, array_data_reg, prefix_offset);
    match emitter.target.arch {
        crate::codegen::platform::Arch::AArch64 => {
            emitter.instruction(&format!("add {}, {}, #24", array_data_reg, array_data_reg)); // address the positional-prefix array payload
        }
        crate::codegen::platform::Arch::X86_64 => {
            emitter.instruction(&format!("add {}, 24", array_data_reg));        // address the positional-prefix array payload
        }
    }
    load_array_element_to_result(emitter, &source_elem_ty, array_data_reg, element_idx * elem_stride);
    push_loaded_array_element_arg(&source_elem_ty, target_ty, emitter, ctx, data)
}

fn emit_variadic_array_arg_from_sources(
    variadic_sources: &[VariadicArgSource],
    source_temp_types: &[PhpType],
    final_pushed_bytes: usize,
    emitter: &mut Emitter,
    _ctx: &mut Context,
    data: &mut DataSection,
) -> PhpType {
    if variadic_sources.iter().any(|source| source.key.is_some()) {
        return emit_variadic_assoc_arg_from_sources(
            variadic_sources,
            source_temp_types,
            final_pushed_bytes,
            emitter,
            data,
        );
    }

    let elem_count = variadic_sources.len();
    let first_elem_ty = match variadic_sources.first() {
        Some(VariadicArgSource {
            source: FinalArgSource::SourceTemp(temp_idx),
            ..
        }) => source_temp_types[*temp_idx].clone(),
        _ => PhpType::Int,
    };
    let container_elem_ty = variadic_container_elem_ty(&first_elem_ty);
    let elem_size = match container_elem_ty.codegen_repr() {
        PhpType::Str => 16,
        _ => 8,
    };
    let (capacity_reg, elem_size_reg, peek_reg, len_reg) = match emitter.target.arch {
        crate::codegen::platform::Arch::AArch64 => ("x0", "x1", "x9", "x10"),
        crate::codegen::platform::Arch::X86_64 => ("rdi", "rsi", "r11", "r10"),
    };

    emitter.comment(&format!("build variadic array ({} elements)", elem_count));
    abi::emit_load_int_immediate(emitter, capacity_reg, elem_count as i64);
    abi::emit_load_int_immediate(emitter, elem_size_reg, elem_size as i64);
    abi::emit_call_label(emitter, "__rt_array_new");
    abi::emit_push_result_value(emitter, &PhpType::Array(Box::new(container_elem_ty.clone())));

    for (idx, source) in variadic_sources.iter().enumerate() {
        let mut elem_ty = match &source.source {
            FinalArgSource::SourceTemp(temp_idx) => load_source_temp_to_result(
                *temp_idx,
                source_temp_types,
                final_pushed_bytes + 16,
                emitter,
            ),
            _ => PhpType::Int,
        };
        let boxed_for_container = if matches!(container_elem_ty, PhpType::Mixed)
            && !matches!(elem_ty, PhpType::Mixed | PhpType::Union(_))
        {
            crate::codegen::emit_box_current_value_as_mixed(emitter, &elem_ty);
            elem_ty = PhpType::Mixed;
            true
        } else {
            false
        };
        if !boxed_for_container {
            abi::emit_incref_if_refcounted(emitter, &elem_ty.codegen_repr());
        }
        match emitter.target.arch {
            crate::codegen::platform::Arch::AArch64 => {
                emitter.instruction(&format!("ldr {}, [sp]", peek_reg));        // peek the variadic array pointer without removing it from the stack
            }
            crate::codegen::platform::Arch::X86_64 => {
                emitter.instruction(&format!("mov {}, QWORD PTR [rsp]", peek_reg)); // peek the variadic array pointer without removing it from the stack
            }
        }
        if idx == 0 {
            super::super::super::arrays::emit_array_value_type_stamp(emitter, peek_reg, &elem_ty);
        }
        store_current_array_element(emitter, peek_reg, idx, &elem_ty);
        abi::emit_load_int_immediate(emitter, len_reg, (idx + 1) as i64);
        abi::emit_store_to_address(emitter, len_reg, peek_reg, 0);
    }

    PhpType::Array(Box::new(container_elem_ty))
}

fn emit_variadic_assoc_arg_from_sources(
    variadic_sources: &[VariadicArgSource],
    source_temp_types: &[PhpType],
    final_pushed_bytes: usize,
    emitter: &mut Emitter,
    data: &mut DataSection,
) -> PhpType {
    let elem_count = variadic_sources.len();
    let first_elem_ty = match variadic_sources.first() {
        Some(VariadicArgSource {
            source: FinalArgSource::SourceTemp(temp_idx),
            ..
        }) => source_temp_types[*temp_idx].clone(),
        _ => PhpType::Int,
    };
    let container_elem_ty = variadic_container_elem_ty(&first_elem_ty);
    let hash_capacity_reg = abi::int_arg_reg_name(emitter.target, 0);
    let key_ptr_reg = abi::int_arg_reg_name(emitter.target, 1);
    let key_len_reg = abi::int_arg_reg_name(emitter.target, 2);
    let value_lo_reg = abi::int_arg_reg_name(emitter.target, 3);
    let value_hi_reg = abi::int_arg_reg_name(emitter.target, 4);
    let value_tag_reg = abi::int_arg_reg_name(emitter.target, 5);
    let tag_reg = abi::int_arg_reg_name(emitter.target, 1);
    let result_reg = abi::int_result_reg(emitter);
    let stack_reg = match emitter.target.arch {
        crate::codegen::platform::Arch::AArch64 => "sp",
        crate::codegen::platform::Arch::X86_64 => "rsp",
    };
    let zero_reg = match emitter.target.arch {
        crate::codegen::platform::Arch::AArch64 => "xzr",
        crate::codegen::platform::Arch::X86_64 => "0",
    };

    emitter.comment(&format!("build named variadic array ({} elements)", elem_count));
    abi::emit_load_int_immediate(
        emitter,
        hash_capacity_reg,
        std::cmp::max(elem_count * 2, 16) as i64,
    );
    abi::emit_load_int_immediate(
        emitter,
        tag_reg,
        crate::codegen::runtime_value_tag(&container_elem_ty) as i64,
    );
    abi::emit_call_label(emitter, "__rt_hash_new");
    abi::emit_push_result_value(emitter, &PhpType::AssocArray {
        key: Box::new(PhpType::Mixed),
        value: Box::new(container_elem_ty.clone()),
    });

    for (idx, source) in variadic_sources.iter().enumerate() {
        match &source.key {
            Some(key) => {
                let (key_label, key_len) = data.add_string(key.as_bytes());
                abi::emit_symbol_address(emitter, key_ptr_reg, &key_label);
                abi::emit_load_int_immediate(emitter, key_len_reg, key_len as i64);
            }
            None => {
                abi::emit_load_int_immediate(emitter, key_ptr_reg, idx as i64);
                abi::emit_load_int_immediate(emitter, key_len_reg, -1);
            }
        }
        abi::emit_push_reg_pair(emitter, key_ptr_reg, key_len_reg);             // preserve the variadic hash key while loading the saved argument value
        let mut elem_ty = match &source.source {
            FinalArgSource::SourceTemp(temp_idx) => load_source_temp_to_result(
                *temp_idx,
                source_temp_types,
                final_pushed_bytes + 32,
                emitter,
            ),
            _ => PhpType::Int,
        };
        let boxed_for_container = if matches!(container_elem_ty, PhpType::Mixed)
            && !matches!(elem_ty, PhpType::Mixed | PhpType::Union(_))
        {
            crate::codegen::emit_box_current_value_as_mixed(emitter, &elem_ty);
            elem_ty = PhpType::Mixed;
            true
        } else {
            false
        };
        if !boxed_for_container && matches!(elem_ty, PhpType::Str) {
            abi::emit_call_label(emitter, "__rt_str_persist");                  // persist variadic strings before storing them in the hash table
        } else if !boxed_for_container {
            abi::emit_incref_if_refcounted(emitter, &elem_ty.codegen_repr());
        }
        let (val_lo, val_hi) = match elem_ty.codegen_repr() {
            PhpType::Float => {
                let bits_reg = abi::temp_int_reg(emitter.target);
                match emitter.target.arch {
                    crate::codegen::platform::Arch::AArch64 => {
                        emitter.instruction(&format!("fmov {}, {}", bits_reg, abi::float_result_reg(emitter))); // move variadic float bits into the hash value register
                    }
                    crate::codegen::platform::Arch::X86_64 => {
                        emitter.instruction(&format!("movq {}, {}", bits_reg, abi::float_result_reg(emitter))); // move variadic float bits into the hash value register
                    }
                }
                (bits_reg, zero_reg)
            }
            PhpType::Str => {
                let (ptr_reg, len_reg) = abi::string_result_regs(emitter);
                (ptr_reg, len_reg)
            }
            _ => (result_reg, zero_reg),
        };
        emitter.instruction(&format!("mov {}, {}", value_lo_reg, val_lo));      // move the variadic value low word into the hash-set ABI register
        emitter.instruction(&format!("mov {}, {}", value_hi_reg, val_hi));      // move the variadic value high word into the hash-set ABI register
        abi::emit_load_int_immediate(
            emitter,
            value_tag_reg,
            crate::codegen::runtime_value_tag(&elem_ty) as i64,
        );
        abi::emit_pop_reg_pair(emitter, key_ptr_reg, key_len_reg);              // restore the variadic hash key into the hash-set ABI registers
        abi::emit_load_temporary_stack_slot(emitter, hash_capacity_reg, 0);
        abi::emit_call_label(emitter, "__rt_hash_set");
        abi::emit_store_to_address(emitter, result_reg, stack_reg, 0);
    }

    PhpType::AssocArray {
        key: Box::new(PhpType::Mixed),
        value: Box::new(container_elem_ty),
    }
}
