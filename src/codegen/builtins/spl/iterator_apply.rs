//! Purpose:
//! Emits PHP `iterator_apply()` calls for Iterator/IteratorAggregate objects.
//! Reuses the statement foreach iterator driver while invoking a callback for each valid position.
//!
//! Called from:
//! - `crate::codegen::builtins::spl::emit()`
//!
//! Key details:
//! - The callback is evaluated once before rewind(), and callback falsehood stops iteration before next().
//! - The returned count includes the callback invocation that requested the stop.

use crate::codegen::abi;
use crate::codegen::builtins::arrays::callback_env;
use crate::codegen::context::Context;
use crate::codegen::data_section::DataSection;
use crate::codegen::emit::Emitter;
use crate::codegen::expr::calls::args as call_args;
use crate::codegen::expr::emit_expr;
use crate::codegen::platform::Arch;
use crate::codegen::stmt::emit_iterator_loop;
use crate::parser::ast::{Expr, ExprKind};
use crate::types::{FunctionSig, PhpType};

use super::iterator_common;

pub fn emit(
    _name: &str,
    args: &[Expr],
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> Option<PhpType> {
    emitter.comment("iterator_apply()");
    let source_ty = emit_expr(&args[0], emitter, ctx, data);
    let Some(class_name) = iterator_common::iterator_object_name(&source_ty).map(str::to_string) else {
        return Some(PhpType::Int);
    };

    abi::emit_push_reg(emitter, abi::int_result_reg(emitter));                  // preserve iterator receiver while resolving the callback

    let call_reg = abi::nested_call_reg(emitter);
    let is_callable_expr = matches!(
        &args[1].kind,
        ExprKind::Closure { .. } | ExprKind::FirstClassCallable(_)
    );
    let precomputed_sig = crate::codegen::callables::callable_sig(&args[1], ctx);
    let captures =
        callback_env::materialize_callback_address(&args[1], call_reg, emitter, ctx, data);
    let sig: Option<FunctionSig> = if is_callable_expr {
        ctx.deferred_closures
            .last()
            .map(|deferred| deferred.sig.clone())
    } else {
        precomputed_sig
    };
    let ret_ty = sig
        .as_ref()
        .map(|sig| sig.return_type.clone())
        .unwrap_or(PhpType::Int);

    abi::emit_load_int_immediate(emitter, abi::int_result_reg(emitter), 0);
    abi::emit_push_reg(emitter, abi::int_result_reg(emitter));                  // save iterator_apply()'s callback-invocation counter
    abi::emit_push_reg(emitter, call_reg);                                      // save the resolved callback address beneath the loop receiver
    abi::emit_load_temporary_stack_slot(emitter, abi::int_result_reg(emitter), 32); // reload the preserved receiver as the loop-driver input

    let callback_args = callback_args(args);
    let loop_start = ctx.next_label("iterator_apply_start");
    let loop_end = ctx.next_label("iterator_apply_end");
    let loop_cont = ctx.next_label("iterator_apply_cont");
    emit_iterator_loop(
        &class_name,
        &loop_start,
        &loop_end,
        &loop_cont,
        emitter,
        ctx,
        data,
        |_, _, _, _| (),
        |_, emitter, ctx, data| {
            emit_callback_invocation(
                &args[1],
                callback_args,
                &captures,
                sig.as_ref(),
                &ret_ty,
                &loop_end,
                emitter,
                ctx,
                data,
            );
        },
        |_, _, _, _| {},
    );
    abi::emit_release_temporary_stack(emitter, 16);                             // discard the saved callback address after iteration
    abi::emit_pop_reg(emitter, abi::int_result_reg(emitter));                   // return the final iterator_apply() invocation count
    abi::emit_release_temporary_stack(emitter, 16);                             // discard the receiver preserved while resolving the callback
    Some(PhpType::Int)
}

fn callback_args(args: &[Expr]) -> &[Expr] {
    match args.get(2).map(|arg| &arg.kind) {
        Some(ExprKind::ArrayLiteral(elems)) => elems.as_slice(),
        _ => &[],
    }
}

fn emit_callback_invocation(
    callback: &Expr,
    callback_args: &[Expr],
    captures: &[(String, PhpType, bool)],
    sig: Option<&FunctionSig>,
    ret_ty: &PhpType,
    loop_end: &str,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) {
    let call_reg = abi::nested_call_reg(emitter);
    abi::emit_load_temporary_stack_slot(emitter, call_reg, 16);

    let save_concat_before_args = emitter.target.arch == Arch::X86_64;
    if save_concat_before_args {
        crate::codegen::expr::save_concat_offset_before_nested_call(emitter, ctx);
    }

    let mut arg_types = Vec::new();
    for (i, arg) in callback_args.iter().enumerate() {
        let target_ty = call_args::declared_target_ty(sig, i);
        let pushed_ty = call_args::push_expr_arg(arg, target_ty, emitter, ctx, data);
        arg_types.push(pushed_ty);
    }

    if let Some(sig) = sig {
        let visible_param_count = sig.params.len();
        let regular_param_count = if sig.variadic.is_some() {
            visible_param_count.saturating_sub(1)
        } else {
            visible_param_count
        };
        for i in arg_types.len()..regular_param_count {
            if let Some(Some(default_expr)) = sig.defaults.get(i) {
                let target_ty = sig.params.get(i).map(|(_, ty)| ty);
                let pushed_ty = call_args::push_expr_arg(default_expr, target_ty, emitter, ctx, data);
                arg_types.push(pushed_ty);
            }
        }
    }
    callback_env::push_captures_as_hidden_args(captures, emitter, ctx, &mut arg_types);

    let assignments = abi::build_outgoing_arg_assignments_for_target(emitter.target, &arg_types, 0);
    let overflow_bytes = abi::materialize_outgoing_args(emitter, &assignments);

    if !save_concat_before_args {
        crate::codegen::expr::save_concat_offset_before_nested_call(emitter, ctx);
    }
    abi::emit_call_reg(emitter, call_reg);
    if save_concat_before_args {
        abi::emit_release_temporary_stack(emitter, overflow_bytes);
        crate::codegen::expr::restore_concat_offset_after_nested_call(emitter, ctx, ret_ty);
    } else {
        crate::codegen::expr::restore_concat_offset_after_nested_call(emitter, ctx, ret_ty);
        abi::emit_release_temporary_stack(emitter, overflow_bytes);
    }

    let _ = callback;
    crate::codegen::expr::coerce_to_truthiness(emitter, ctx, ret_ty);
    iterator_common::emit_increment_saved_count_at_offset(32, emitter);
    emit_branch_if_callback_false(emitter, loop_end);
}

fn emit_branch_if_callback_false(emitter: &mut Emitter, loop_end: &str) {
    match emitter.target.arch {
        Arch::AArch64 => {
            emitter.instruction("cmp x0, #0");                                  // did iterator_apply() callback request iteration stop?
            emitter.instruction(&format!("b.eq {}", loop_end));                 // stop before next() when callback returned false
        }
        Arch::X86_64 => {
            emitter.instruction("test rax, rax");                               // did iterator_apply() callback request iteration stop?
            emitter.instruction(&format!("je {}", loop_end));                   // stop before next() when callback returned false
        }
    }
}
