//! Purpose:
//! Emits the internal `__elephc_class_parent_name` intrinsic in the frozen
//! legacy direct backend — the parent-name lookup behind
//! `ReflectionClass::getParentClass()`.
//!
//! Called from:
//! - `crate::codegen::builtins::system::emit()`.
//!
//! Key details:
//! - Mirrors the EIR lowering exactly: the class-name string goes to
//!   `__rt_class_parent_name` (a `_classes_by_name` scan mapped through
//!   `_class_parent_ids` and `_class_name_entries`), which returns the parent
//!   name string (empty on miss or no parent).
//! - This legacy emitter exists because synthetic `ReflectionClass` method
//!   bodies can be compiled by the legacy backend (autoload/include
//!   materialization paths); without it the body would fall through to an
//!   unresolved `_fn___elephc_class_parent_name` user-function reference.

use crate::codegen::abi;
use crate::codegen::context::Context;
use crate::codegen::data_section::DataSection;
use crate::codegen::emit::Emitter;
use crate::codegen::expr::{coerce_to_string, emit_expr};
use crate::parser::ast::Expr;
use crate::types::PhpType;

/// Emits code for the internal `__elephc_class_parent_name` intrinsic.
pub fn emit(
    _name: &str,
    args: &[Expr],
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> Option<PhpType> {
    emitter.comment("__elephc_class_parent_name()");
    let ty = emit_expr(&args[0], emitter, ctx, data);
    coerce_to_string(emitter, ctx, data, &ty);
    abi::emit_call_label(emitter, "__rt_class_parent_name");

    Some(PhpType::Str)
}
