use crate::codegen::context::{Context, HeapOwnership, LoopLabels};
use crate::codegen::data_section::DataSection;
use crate::codegen::emit::Emitter;
use crate::codegen::expr::objects::dispatch::{
    emit_dispatch_instance_method, emit_dispatch_interface_method,
};
use crate::codegen::platform::Arch;
use crate::codegen::stmt::emit_stmt;
use crate::parser::ast::Stmt;
use crate::types::PhpType;

/// Foreach over an object implementing the Iterator interface.
///
/// On entry, x0 already holds the iterator object pointer (left there by
/// `emit_expr` on the foreach iterable expression).
///
/// Loop shape:
///
/// ```text
/// rewind()
/// loop_start:
///     valid()  ; if !valid jump loop_end
///     key()    ; if requested -> key_var (Mixed)
///     current(); -> value_var (Mixed)
///     <body>
/// loop_cont:
///     next()
///     b loop_start
/// loop_end:
/// ```
///
/// The receiver pointer is parked in a 16-byte stack slot so it survives the
/// nested method calls without burning a callee-saved register. Each method
/// call reloads `x0` from that slot before dispatching through the vtable.
pub(crate) fn emit_iterator_foreach(
    class_name: &str,
    key_var: &Option<String>,
    value_var: &str,
    body: &[Stmt],
    loop_start: &str,
    loop_end: &str,
    loop_cont: &str,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) {
    if emitter.target.arch != Arch::AArch64 {
        unimplemented!("foreach over Iterator object is only implemented for AArch64 in this slice");
    }

    let mut dispatch_target = iterator_dispatch_target(class_name, ctx);
    if !dispatch_target.implements_iterator(ctx) {
        let ret_ty = dispatch_target.dispatch("getIterator", emitter, ctx);
        dispatch_target = iterator_return_dispatch_target(&ret_ty, ctx);
    }

    emitter.instruction("str x0, [sp, #-16]!");                                 // park iterator receiver pointer in a 16-byte stack slot
    if let Some(kv) = key_var {
        reset_iterator_mixed_slot(kv, emitter, ctx);
    }
    reset_iterator_mixed_slot(value_var, emitter, ctx);

    emitter.instruction("ldr x0, [sp]");                                        // reload receiver into x0 for rewind() dispatch
    dispatch_target.dispatch("rewind", emitter, ctx);

    emitter.label(loop_start);

    emitter.instruction("ldr x0, [sp]");                                        // reload receiver into x0 for valid() dispatch
    dispatch_target.dispatch("valid", emitter, ctx);
    emitter.instruction("cmp x0, #0");                                          // valid() returned 0 -> end of iteration
    emitter.instruction(&format!("b.eq {}", loop_end));                         // exit foreach when valid() returns false

    if let Some(kv) = key_var {
        emitter.instruction("ldr x0, [sp]");                                    // reload receiver into x0 for key() dispatch
        let key_ty = dispatch_target.dispatch("key", emitter, ctx);
        if let Some(kvar) = ctx.variables.get(kv) {
            let k_offset = kvar.stack_offset;
            store_iterator_mixed_result(kv, k_offset, &key_ty, emitter, ctx);
        } else {
            emitter.comment(&format!("WARNING: undefined foreach key variable ${}", kv));
        }
    }

    emitter.instruction("ldr x0, [sp]");                                        // reload receiver into x0 for current() dispatch
    let current_ty = dispatch_target.dispatch("current", emitter, ctx);
    if let Some(vvar) = ctx.variables.get(value_var) {
        let v_offset = vvar.stack_offset;
        store_iterator_mixed_result(value_var, v_offset, &current_ty, emitter, ctx);
    } else {
        emitter.comment(&format!("WARNING: undefined foreach value variable ${}", value_var));
    }

    ctx.loop_stack.push(LoopLabels {
        continue_label: loop_cont.to_string(),
        break_label: loop_end.to_string(),
        sp_adjust: 16,
    });
    for s in body {
        emit_stmt(s, emitter, ctx, data);
    }
    ctx.loop_stack.pop();

    emitter.label(loop_cont);
    emitter.instruction("ldr x0, [sp]");                                        // reload receiver into x0 for next() dispatch
    dispatch_target.dispatch("next", emitter, ctx);
    emitter.instruction(&format!("b {}", loop_start));                          // continue the iteration

    emitter.label(loop_end);
    emitter.instruction("add sp, sp, #16");                                     // discard the parked receiver slot
}

#[derive(Clone)]
enum IteratorDispatchTarget {
    Class(String),
    Interface(String),
}

impl IteratorDispatchTarget {
    fn dispatch(&self, method: &str, emitter: &mut Emitter, ctx: &mut Context) -> PhpType {
        match self {
            IteratorDispatchTarget::Class(class_name) => {
                emit_dispatch_instance_method(class_name, method, emitter, ctx)
            }
            IteratorDispatchTarget::Interface(interface_name) => {
                emit_dispatch_interface_method(interface_name, method, emitter, ctx)
            }
        }
    }

    fn implements_iterator(&self, ctx: &Context) -> bool {
        match self {
            IteratorDispatchTarget::Class(class_name) => {
                class_implements_interface(class_name, "Iterator", ctx)
            }
            IteratorDispatchTarget::Interface(interface_name) => {
                interface_extends_interface(interface_name, "Iterator", ctx)
            }
        }
    }
}

fn iterator_dispatch_target(name: &str, ctx: &Context) -> IteratorDispatchTarget {
    if ctx.interfaces.contains_key(name) {
        IteratorDispatchTarget::Interface(name.to_string())
    } else {
        IteratorDispatchTarget::Class(name.to_string())
    }
}

fn iterator_return_dispatch_target(ret_ty: &PhpType, ctx: &Context) -> IteratorDispatchTarget {
    match ret_ty {
        PhpType::Object(name) if ctx.interfaces.contains_key(name) => {
            IteratorDispatchTarget::Interface(name.clone())
        }
        PhpType::Object(name) => IteratorDispatchTarget::Class(name.clone()),
        _ => IteratorDispatchTarget::Interface("Iterator".to_string()),
    }
}

fn class_implements_interface(class_name: &str, interface_name: &str, ctx: &Context) -> bool {
    ctx.classes.get(class_name).is_some_and(|class_info| {
        class_info
            .interfaces
            .iter()
            .any(|name| name == interface_name)
    })
}

fn interface_extends_interface(interface_name: &str, ancestor_name: &str, ctx: &Context) -> bool {
    if interface_name == ancestor_name {
        return true;
    }
    let mut stack = vec![interface_name.to_string()];
    let mut seen = std::collections::HashSet::new();
    while let Some(current_name) = stack.pop() {
        if !seen.insert(current_name.clone()) {
            continue;
        }
        let Some(interface_info) = ctx.interfaces.get(&current_name) else {
            continue;
        };
        for parent_name in &interface_info.parents {
            if parent_name == ancestor_name {
                return true;
            }
            stack.push(parent_name.clone());
        }
    }
    false
}

fn store_iterator_mixed_result(
    var_name: &str,
    offset: usize,
    result_ty: &PhpType,
    emitter: &mut Emitter,
    ctx: &mut Context,
) {
    crate::codegen::emit_box_current_value_as_mixed(emitter, &result_ty.codegen_repr());
    emitter.instruction("str x0, [sp, #-16]!");                                 // preserve the freshly returned mixed value across previous-value cleanup
    crate::codegen::abi::load_at_offset_scratch(emitter, "x0", offset, "x10");
    emitter.instruction("bl __rt_decref_mixed");                                // release the previous owned foreach mixed value before overwriting it
    emitter.instruction("ldr x0, [sp], #16");                                   // restore the new mixed value after cleanup
    crate::codegen::abi::store_at_offset_scratch(emitter, "x0", offset, "x10");
    ctx.update_var_type_and_ownership(var_name, PhpType::Mixed, HeapOwnership::Owned);
}

fn reset_iterator_mixed_slot(var_name: &str, emitter: &mut Emitter, ctx: &Context) {
    let Some(var) = ctx.variables.get(var_name) else {
        return;
    };
    if var.ownership == HeapOwnership::Owned && var.ty.is_refcounted() {
        crate::codegen::abi::load_at_offset_scratch(
            emitter,
            "x0",
            var.stack_offset,
            "x10",
        );
        crate::codegen::abi::emit_decref_if_refcounted(emitter, &var.ty);
    }
    crate::codegen::abi::emit_store_zero_to_local_slot(emitter, var.stack_offset);
}
