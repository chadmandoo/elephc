//! Purpose:
//! Emits the internal `__elephc_class_is_abstract` intrinsic in the frozen
//! legacy direct backend — the abstract-flag lookup behind
//! `ReflectionClass::isAbstract()`.
//!
//! Called from:
//! - `crate::codegen::builtins::system::emit()`.
//!
//! Key details:
//! - Mirrors the EIR lowering exactly: the class-name string goes to
//!   `__rt_class_is_abstract` (the `_classes_by_name` scan indexing the
//!   parallel `_class_is_abstract` flag table), which returns 1 for an
//!   abstract class and 0 for a concrete or unknown one.
//! - This legacy emitter exists because synthetic `ReflectionClass` method
//!   bodies can be compiled by the legacy backend (autoload/include
//!   materialization paths); without it the body would fall through to an
//!   unresolved `_fn___elephc_class_is_abstract` user-function reference.

use crate::codegen::abi;
use crate::codegen::context::Context;
use crate::codegen::data_section::DataSection;
use crate::codegen::emit::Emitter;
use crate::codegen::expr::{coerce_to_string, emit_expr};
use crate::parser::ast::Expr;
use crate::types::PhpType;

/// Emits code for the internal `__elephc_class_is_abstract` intrinsic.
pub fn emit(
    _name: &str,
    args: &[Expr],
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> Option<PhpType> {
    emitter.comment("__elephc_class_is_abstract()");
    let ty = emit_expr(&args[0], emitter, ctx, data);
    coerce_to_string(emitter, ctx, data, &ty);
    abi::emit_call_label(emitter, "__rt_class_is_abstract");

    Some(PhpType::Int)
}
