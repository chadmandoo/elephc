//! Purpose:
//! Emits PHP `iterator_to_array()` calls for arrays and Iterator/IteratorAggregate objects.
//! Reuses the statement foreach iterator driver for object traversal.
//!
//! Called from:
//! - `crate::codegen::builtins::spl::emit()`
//!
//! Key details:
//! - `$preserve_keys=false` appends current() values without calling key().
//! - `$preserve_keys=true` normalizes key() results through the associative-array hash ABI.

use crate::codegen::abi;
use crate::codegen::context::Context;
use crate::codegen::data_section::DataSection;
use crate::codegen::emit::Emitter;
use crate::codegen::expr::emit_expr;
use crate::codegen::stmt::emit_iterator_loop;
use crate::parser::ast::Expr;
use crate::types::PhpType;

use super::iterator_common;

pub fn emit(
    _name: &str,
    args: &[Expr],
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> Option<PhpType> {
    emitter.comment("iterator_to_array()");
    let preserve_keys = iterator_common::preserve_keys_arg(args);
    let source_ty = emit_expr(&args[0], emitter, ctx, data);
    if let Some(cloned_ty) = iterator_common::emit_clone_loaded_array(&source_ty, emitter) {
        return Some(cloned_ty);
    }

    let Some(class_name) = iterator_common::iterator_object_name(&source_ty) else {
        return Some(result_ty(preserve_keys));
    };

    let receiver_reg = abi::nested_call_reg(emitter);
    emitter.instruction(&format!(
        "mov {}, {}",
        receiver_reg,
        abi::int_result_reg(emitter)
    )); // preserve iterator receiver while allocating iterator_to_array()'s result
    if preserve_keys {
        iterator_common::emit_new_mixed_hash(emitter);
    } else {
        iterator_common::emit_new_mixed_indexed_array(emitter);
    }
    iterator_common::emit_save_result_under_receiver(emitter);
    iterator_common::emit_restore_receiver_from_preserved_reg(emitter, receiver_reg);

    let loop_start = ctx.next_label("iterator_to_array_start");
    let loop_end = ctx.next_label("iterator_to_array_end");
    let loop_cont = ctx.next_label("iterator_to_array_cont");
    emit_iterator_loop(
        class_name,
        &loop_start,
        &loop_end,
        &loop_cont,
        emitter,
        ctx,
        data,
        |_, _, _, _| (),
        |dispatch_target, emitter, ctx, data| {
            if preserve_keys {
                iterator_common::emit_insert_current_with_iterator_key(
                    dispatch_target,
                    emitter,
                    ctx,
                    data,
                );
            } else {
                iterator_common::emit_append_current_to_saved_array(
                    dispatch_target,
                    emitter,
                    ctx,
                );
            }
        },
        |_, _, _, _| {},
    );
    abi::emit_pop_reg(emitter, abi::int_result_reg(emitter));                   // return the completed iterator_to_array() result container
    Some(result_ty(preserve_keys))
}

fn result_ty(preserve_keys: bool) -> PhpType {
    if preserve_keys {
        PhpType::AssocArray {
            key: Box::new(PhpType::Mixed),
            value: Box::new(PhpType::Mixed),
        }
    } else {
        PhpType::Array(Box::new(PhpType::Mixed))
    }
}
