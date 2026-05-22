//! Purpose:
//! Emits PHP `iterator_count()` calls for arrays and Iterator/IteratorAggregate objects.
//! Reuses the statement foreach iterator driver for object traversal.
//!
//! Called from:
//! - `crate::codegen::builtins::spl::emit()`
//!
//! Key details:
//! - Object iteration calls rewind(), valid(), and next() just like PHP and leaves the iterator exhausted.
//! - The saved count lives beneath the loop driver's receiver stack slot.

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
    emitter.comment("iterator_count()");
    let source_ty = emit_expr(&args[0], emitter, ctx, data);
    if iterator_common::emit_count_loaded_array(&source_ty, emitter) {
        return Some(PhpType::Int);
    }

    let Some(class_name) = iterator_common::iterator_object_name(&source_ty) else {
        return Some(PhpType::Int);
    };

    let receiver_reg = abi::nested_call_reg(emitter);
    emitter.instruction(&format!(
        "mov {}, {}",
        receiver_reg,
        abi::int_result_reg(emitter)
    )); // preserve iterator receiver while initializing the count slot
    abi::emit_load_int_immediate(emitter, abi::int_result_reg(emitter), 0);
    abi::emit_push_reg(emitter, abi::int_result_reg(emitter));                  // save iterator_count()'s counter underneath the loop receiver
    iterator_common::emit_restore_receiver_from_preserved_reg(emitter, receiver_reg);

    let loop_start = ctx.next_label("iterator_count_start");
    let loop_end = ctx.next_label("iterator_count_end");
    let loop_cont = ctx.next_label("iterator_count_cont");
    emit_iterator_loop(
        class_name,
        &loop_start,
        &loop_end,
        &loop_cont,
        emitter,
        ctx,
        data,
        |_, _, _, _| (),
        |_, emitter, _, _| iterator_common::emit_increment_saved_count(emitter),
        |_, _, _, _| {},
    );
    abi::emit_pop_reg(emitter, abi::int_result_reg(emitter));                   // return the final saved iterator_count() counter
    Some(PhpType::Int)
}
