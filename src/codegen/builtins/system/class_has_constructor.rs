//! Purpose:
//! Emits the internal `__elephc_class_has_constructor` intrinsic in the frozen
//! legacy direct backend — the constructor-presence lookup behind
//! `ReflectionClass::getConstructor()`.
//!
//! Called from:
//! - `crate::codegen::builtins::system::emit()`.
//!
//! Key details:
//! - Mirrors the EIR lowering exactly: the class-name string goes to
//!   `__rt_class_has_constructor` (the `_classes_by_name` scan indexing the
//!   parallel `_class_has_ctor` flag table), which returns 1 when the class
//!   declares or inherits `__construct` and 0 otherwise.
//! - This legacy emitter exists because synthetic `ReflectionClass` method
//!   bodies can be compiled by the legacy backend (autoload/include
//!   materialization paths); without it the body would fall through to an
//!   unresolved `_fn___elephc_class_has_constructor` user-function reference.

use crate::codegen::abi;
use crate::codegen::context::Context;
use crate::codegen::data_section::DataSection;
use crate::codegen::emit::Emitter;
use crate::codegen::expr::{coerce_to_string, emit_expr};
use crate::parser::ast::Expr;
use crate::types::PhpType;

/// Emits code for the internal `__elephc_class_has_constructor` intrinsic.
pub fn emit(
    _name: &str,
    args: &[Expr],
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> Option<PhpType> {
    emitter.comment("__elephc_class_has_constructor()");
    let ty = emit_expr(&args[0], emitter, ctx, data);
    coerce_to_string(emitter, ctx, data, &ty);
    abi::emit_call_label(emitter, "__rt_class_has_constructor");

    Some(PhpType::Int)
}
