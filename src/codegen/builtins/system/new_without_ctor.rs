//! Purpose:
//! Emits the internal `__elephc_new_without_ctor` intrinsic in the frozen
//! legacy direct backend — the constructor-less allocator behind
//! `ReflectionClass::newInstanceWithoutConstructor()`.
//!
//! Called from:
//! - `crate::codegen::builtins::system::emit()`.
//!
//! Key details:
//! - Mirrors the EIR lowering exactly: the class-name string goes to
//!   `__rt_new_by_name` (alloc + zero-fill + class-id stamp + property-default
//!   thunk, no constructor), then the raw object pointer is boxed into a Mixed
//!   cell — tag 6 (object) on success, tag 8 (null) for an unknown class.
//! - This legacy emitter exists because synthetic `ReflectionClass` method
//!   bodies can be compiled by the legacy backend (autoload/include
//!   materialization paths); without it the body would fall through to an
//!   unresolved `_fn___elephc_new_without_ctor` user-function reference.

use crate::codegen::abi;
use crate::codegen::context::Context;
use crate::codegen::data_section::DataSection;
use crate::codegen::emit::Emitter;
use crate::codegen::expr::{coerce_to_string, emit_expr};
use crate::codegen::platform::Arch;
use crate::parser::ast::Expr;
use crate::types::PhpType;

/// Emits code for the internal `__elephc_new_without_ctor` intrinsic.
pub fn emit(
    _name: &str,
    args: &[Expr],
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> Option<PhpType> {
    emitter.comment("__elephc_new_without_ctor()");
    let ty = emit_expr(&args[0], emitter, ctx, data);
    coerce_to_string(emitter, ctx, data, &ty);
    abi::emit_call_label(emitter, "__rt_new_by_name");
    let null_label = ctx.next_label("niwc_null");
    let box_label = ctx.next_label("niwc_box");
    match emitter.target.arch {
        Arch::AArch64 => {
            emitter.instruction(&format!("cbz x0, {}", null_label));            // unknown class names box as PHP null
            emitter.instruction("mov x1, x0");                                  // object pointer becomes the mixed payload
            emitter.instruction("mov x2, xzr");                                 // object payloads have no high word
            emitter.instruction("mov x0, #6");                                  // runtime tag 6 = object
            emitter.instruction(&format!("b {}", box_label));                   // box the allocated instance
            emitter.label(&null_label);
            emitter.instruction("mov x1, xzr");                                 // null payload low word
            emitter.instruction("mov x2, xzr");                                 // null payload high word
            emitter.instruction("mov x0, #8");                                  // runtime tag 8 = null
            emitter.label(&box_label);
            abi::emit_call_label(emitter, "__rt_mixed_from_value");
        }
        Arch::X86_64 => {
            emitter.instruction("test rax, rax");                               // did new_by_name find the class?
            emitter.instruction(&format!("jz {}", null_label));                 // unknown class names box as PHP null
            emitter.instruction("mov rdi, rax");                                // object pointer becomes the mixed payload
            emitter.instruction("xor esi, esi");                                // object payloads have no high word
            emitter.instruction("mov eax, 6");                                  // runtime tag 6 = object
            emitter.instruction(&format!("jmp {}", box_label));                 // jump to box the allocated instance
            emitter.label(&null_label);
            emitter.instruction("xor edi, edi");                                // null payload low word
            emitter.instruction("xor esi, esi");                                // null payload high word
            emitter.instruction("mov eax, 8");                                  // runtime tag 8 = null
            emitter.label(&box_label);
            abi::emit_call_label(emitter, "__rt_mixed_from_value");
        }
    }

    Some(PhpType::Mixed)
}
