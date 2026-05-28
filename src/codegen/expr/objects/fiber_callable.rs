//! Purpose:
//! Materializes PHP callable shapes passed to `new Fiber(...)` as runtime callable descriptors.
//! Keeps Fiber constructor lowering focused on object allocation while callable selection stays here.
//!
//! Called from:
//! - `crate::codegen::expr::objects::allocation`
//!
//! Key details:
//! - The Fiber object stores one callable descriptor pointer and a generated wrapper pointer.
//! - Raw string callbacks use runtime descriptor-name dispatch; receiver-bound shapes are
//!   converted to first-class-callable descriptors so receiver environments live in captures.

use crate::codegen::callable_dispatch::{RuntimeCallableCase, RuntimeCallableSelector};
use crate::codegen::context::Context;
use crate::codegen::data_section::DataSection;
use crate::codegen::emit::Emitter;
use crate::codegen::platform::Arch;
use crate::codegen::{abi, callable_dispatch};
use crate::names::{php_symbol_key, Name};
use crate::parser::ast::{CallableTarget, Expr, ExprKind, StaticReceiver};
use crate::types::PhpType;

/// Emits a Fiber callback descriptor and returns the wrapper label that can invoke it.
///
/// Existing descriptor-valued expressions delegate to the ordinary Fiber wrapper planner.
/// Raw string callbacks and callable arrays are materialized into descriptor pointers first,
/// then use the generic descriptor-invoker Fiber wrapper.
pub(super) fn emit_fiber_callable_descriptor(
    callable_expr: &Expr,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> Option<String> {
    if emit_callable_array_descriptor(callable_expr, emitter, ctx, data)
        || emit_invokable_object_descriptor(callable_expr, emitter, ctx, data)
        || emit_string_callable_descriptor(callable_expr, emitter, ctx, data)
    {
        return Some(super::fiber_wrapper::prepare_descriptor_invoker_wrapper(ctx));
    }

    crate::codegen::expr::emit_expr(callable_expr, emitter, ctx, data);
    super::fiber_wrapper::prepare_fiber_wrapper(callable_expr, ctx)
}

/// Emits a first-class-callable descriptor for a callable-array Fiber callback.
fn emit_callable_array_descriptor(
    callable_expr: &Expr,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> bool {
    if let Some(target) = callable_array_literal_target(callable_expr, ctx) {
        return emit_synthetic_first_class_callable(target, callable_expr, emitter, ctx, data);
    }

    let ExprKind::Variable(var_name) = &callable_expr.kind else {
        return false;
    };
    let Some(target) = ctx.callable_array_targets.get(var_name).cloned() else {
        return false;
    };
    if !callable_array_variable_target_supported(&target) {
        return false;
    }
    emit_synthetic_first_class_callable(target, callable_expr, emitter, ctx, data)
}

/// Emits a first-class-callable descriptor for an object with public `__invoke()`.
fn emit_invokable_object_descriptor(
    callable_expr: &Expr,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> bool {
    if !simple_receiver_expr(callable_expr) {
        return false;
    }
    let callable_ty = crate::codegen::functions::infer_contextual_type(callable_expr, ctx);
    let Some(class_name) = crate::codegen::functions::singular_object_class(&callable_ty) else {
        return false;
    };
    if !ctx
        .classes
        .get(class_name)
        .is_some_and(|class_info| class_info.methods.contains_key("__invoke"))
    {
        return false;
    }

    let target = CallableTarget::Method {
        object: Box::new(callable_expr.clone()),
        method: "__invoke".to_string(),
    };
    emit_synthetic_first_class_callable(target, callable_expr, emitter, ctx, data)
}

/// Emits runtime string-name descriptor selection for a Fiber callback.
fn emit_string_callable_descriptor(
    callable_expr: &Expr,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> bool {
    let callable_ty = crate::codegen::functions::infer_contextual_type(callable_expr, ctx);
    if !matches!(callable_ty.codegen_repr(), PhpType::Str) {
        return false;
    }

    crate::codegen::expr::emit_expr(callable_expr, emitter, ctx, data);
    emit_select_loaded_string_descriptor(emitter, ctx, data);
    true
}

/// Emits a synthetic first-class callable expression and leaves its descriptor in the result register.
fn emit_synthetic_first_class_callable(
    target: CallableTarget,
    source_expr: &Expr,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> bool {
    let fcc_expr = Expr::new(ExprKind::FirstClassCallable(target), source_expr.span);
    crate::codegen::expr::emit_expr(&fcc_expr, emitter, ctx, data);
    true
}

/// Selects a callable descriptor for the currently loaded string callback name.
fn emit_select_loaded_string_descriptor(
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) {
    emitter.comment("fiber callable string descriptor selection");
    let (ptr_reg, len_reg) = abi::string_result_regs(emitter);
    abi::emit_push_reg_pair(emitter, ptr_reg, len_reg);                         // preserve the Fiber string callback name during descriptor selection

    let cases = callable_dispatch::runtime_callable_cases(ctx, data, &[], None);
    let call_reg = abi::nested_call_reg(emitter);
    let done_label = ctx.next_label("fiber_string_callable_done");
    for case in cases.iter().filter(|case| case.has_invoker) {
        emit_string_case_selection(case, call_reg, &done_label, emitter, ctx, data);
    }
    emit_fiber_callable_no_match_abort(emitter, data);
    emitter.label(&done_label);
    abi::emit_release_temporary_stack(emitter, 16);                             // discard the saved Fiber string callback name after descriptor selection
}

/// Emits one runtime string descriptor case for `new Fiber($callback)`.
fn emit_string_case_selection(
    case: &RuntimeCallableCase,
    call_reg: &str,
    done_label: &str,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) {
    let next_case = ctx.next_label("fiber_string_callable_next");
    let selector = RuntimeCallableSelector::StringNameStack {
        ptr_offset: 0,
        len_offset: 8,
        call_reg,
    };
    callable_dispatch::emit_branch_if_callable_case_mismatch(
        &selector, case, &next_case, emitter, ctx, data,
    );
    let result_reg = abi::int_result_reg(emitter);
    if call_reg != result_reg {
        emitter.instruction(&format!("mov {}, {}", result_reg, call_reg));      // return the selected Fiber callable descriptor
    }
    abi::emit_jump(emitter, done_label);
    emitter.label(&next_case);
}

/// Returns a callable target for a two-slot literal callable array supported by Fiber.
fn callable_array_literal_target(expr: &Expr, ctx: &Context) -> Option<CallableTarget> {
    let (receiver, method) = callable_array_parts(expr)?;
    if let Some(receiver) = static_callable_receiver(receiver, ctx) {
        return Some(CallableTarget::StaticMethod {
            receiver,
            method: method.to_string(),
        });
    }
    if !simple_receiver_expr(receiver) {
        return None;
    }
    Some(CallableTarget::Method {
        object: Box::new(receiver.clone()),
        method: method.to_string(),
    })
}

/// Returns true when a tracked callable-array variable can be lowered without hidden temps.
fn callable_array_variable_target_supported(target: &CallableTarget) -> bool {
    match target {
        CallableTarget::StaticMethod { .. } => true,
        CallableTarget::Method { .. } => false,
        CallableTarget::Function(_) => false,
    }
}

/// Returns receiver and method from `[receiver, "method"]`.
fn callable_array_parts(expr: &Expr) -> Option<(&Expr, &str)> {
    let ExprKind::ArrayLiteral(elems) = &expr.kind else {
        return None;
    };
    if elems.len() != 2 {
        return None;
    }
    let ExprKind::StringLiteral(method) = &elems[1].kind else {
        return None;
    };
    Some((&elems[0], method.as_str()))
}

/// Resolves a literal callable-array receiver to a static class target.
fn static_callable_receiver(receiver: &Expr, ctx: &Context) -> Option<StaticReceiver> {
    let class_name = match &receiver.kind {
        ExprKind::StringLiteral(class_name) => resolve_class_name(ctx, class_name)?.to_string(),
        ExprKind::ClassConstant { receiver } => resolve_static_receiver_class(receiver, ctx)?,
        _ => return None,
    };
    Some(StaticReceiver::Named(Name::from(class_name)))
}

/// Resolves a scoped receiver to a concrete class name.
fn resolve_static_receiver_class(receiver: &StaticReceiver, ctx: &Context) -> Option<String> {
    match receiver {
        StaticReceiver::Named(name) => resolve_class_name(ctx, name.as_str()).map(str::to_string),
        StaticReceiver::Self_ | StaticReceiver::Static => ctx.current_class.clone(),
        StaticReceiver::Parent => ctx
            .current_class
            .as_ref()
            .and_then(|class_name| ctx.classes.get(class_name))
            .and_then(|class_info| class_info.parent.clone()),
    }
}

/// Resolves a class name case-insensitively against known codegen classes.
fn resolve_class_name<'a>(ctx: &'a Context, class_name: &str) -> Option<&'a str> {
    let class_key = php_symbol_key(class_name.trim_start_matches('\\'));
    ctx.classes
        .keys()
        .find(|existing| php_symbol_key(existing) == class_key)
        .map(String::as_str)
}

/// Returns true when first-class callable lowering can capture the receiver without a temp slot.
fn simple_receiver_expr(expr: &Expr) -> bool {
    matches!(&expr.kind, ExprKind::Variable(_) | ExprKind::This)
}

/// Emits a fatal diagnostic when a runtime Fiber callable name has no descriptor case.
fn emit_fiber_callable_no_match_abort(emitter: &mut Emitter, data: &mut DataSection) {
    let (message_label, message_len) = data.add_string(
        b"Fatal error: Fiber callback string did not resolve to an invokable target\n",
    );
    match emitter.target.arch {
        Arch::AArch64 => {
            emitter.instruction("mov x0, #2");                                  // write the Fiber callable diagnostic to stderr
            emitter.adrp("x1", &message_label);
            emitter.add_lo12("x1", "x1", &message_label);
            emitter.instruction(&format!("mov x2, #{}", message_len));          // pass the Fiber callable diagnostic byte length to write()
            emitter.syscall(4);
            abi::emit_exit(emitter, 1);
        }
        Arch::X86_64 => {
            emitter.instruction("mov edi, 2");                                  // write the Fiber callable diagnostic to stderr
            abi::emit_symbol_address(emitter, "rsi", &message_label);
            emitter.instruction(&format!("mov edx, {}", message_len));          // pass the Fiber callable diagnostic byte length to write()
            emitter.instruction("mov eax, 1");                                  // Linux x86_64 syscall 1 = write
            emitter.instruction("syscall");                                     // emit the Fiber callable diagnostic
            abi::emit_exit(emitter, 1);
        }
    }
}
