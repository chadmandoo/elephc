//! Purpose:
//! Emits the `__rt_class_name_of` runtime helper — the Mixed-receiver arm of the
//! dynamic `$expr::class` resolver (`__elephc_class_name_of`).
//!
//! Called from:
//! - `crate::codegen::runtime::emitters::emit_runtime()` via `crate::codegen::runtime::system`.
//!
//! Key details:
//! - Object and String receivers are handled statically by the lowering; only a
//!   Mixed/Union value reaches here, always as a boxed Mixed cell. An object
//!   payload (tag 6) resolves its class name through the dense
//!   `_class_name_entries` table (class_id → (ptr, len) 16-byte rows); a string
//!   payload (tag 1) is its own class name (PHP `::class` on a string returns the
//!   string). Anything else (or null) returns the empty class name.

use crate::codegen::{abi, emit::Emitter, platform::Arch};

/// class_name_of: PHP `$expr::class` for a boxed Mixed receiver.
/// Input:  AArch64 x0 = mixed cell; x86_64 rdi = mixed cell
/// Output: string — AArch64 (x1 = ptr, x2 = len); x86_64 (rax = ptr, rdx = len)
pub fn emit_class_name_of(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_class_name_of_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: class_name_of ---");
    emitter.label_global("__rt_class_name_of");

    emitter.instruction("cbz x0, __rt_cno_empty");                              // null value → empty class name
    emitter.instruction("ldr x9, [x0]");                                        // load the boxed value tag
    emitter.instruction("cmp x9, #6");                                          // tag 6 = object payload?
    emitter.instruction("b.eq __rt_cno_object");                                // resolve the object's class name
    emitter.instruction("cmp x9, #1");                                          // tag 1 = string payload?
    emitter.instruction("b.eq __rt_cno_string");                               // a string is its own class name
    emitter.instruction("b __rt_cno_empty");                                    // any other payload → empty

    emitter.label("__rt_cno_object");
    emitter.instruction("ldr x0, [x0, #8]");                                    // unwrap the object pointer from the cell
    emitter.instruction("cbz x0, __rt_cno_empty");                              // defensive null guard
    emitter.instruction("ldr x9, [x0]");                                        // load the object's runtime class id
    abi::emit_symbol_address(emitter, "x10", "_class_name_count");
    emitter.instruction("ldr x10, [x10]");                                      // load the dense class-name row count
    emitter.instruction("cmp x9, x10");                                         // validate the class id before indexing
    emitter.instruction("b.hs __rt_cno_empty");                                 // unknown class id → empty
    abi::emit_symbol_address(emitter, "x11", "_class_name_entries");
    emitter.instruction("lsl x12, x9, #4");                                     // scale the class id by the 16-byte row size
    emitter.instruction("add x11, x11, x12");                                   // point at the selected class-name row
    emitter.instruction("ldr x1, [x11]");                                       // class-name string pointer → result
    emitter.instruction("ldr x2, [x11, #8]");                                   // class-name string length → result
    emitter.instruction("ret");                                                 // return the class name in x1/x2

    emitter.label("__rt_cno_string");
    emitter.instruction("ldr x1, [x0, #8]");                                    // string payload pointer → result
    emitter.instruction("ldr x2, [x0, #16]");                                   // string payload length → result
    emitter.instruction("ret");                                                 // return the string in x1/x2

    emitter.label("__rt_cno_empty");
    abi::emit_symbol_address(emitter, "x1", "_class_name_missing");
    emitter.instruction("mov x2, #0");                                          // empty class name has zero length
    emitter.instruction("ret");                                                 // return the empty class name
}

/// Emits the Linux x86_64 variant of `__rt_class_name_of`.
fn emit_class_name_of_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: class_name_of ---");
    emitter.label_global("__rt_class_name_of");

    emitter.instruction("test rdi, rdi");                                       // null value → empty class name
    emitter.instruction("je __rt_cno_empty_x86");
    emitter.instruction("mov r10, QWORD PTR [rdi]");                            // load the boxed value tag
    emitter.instruction("cmp r10, 6");                                          // tag 6 = object payload?
    emitter.instruction("je __rt_cno_object_x86");                              // resolve the object's class name
    emitter.instruction("cmp r10, 1");                                          // tag 1 = string payload?
    emitter.instruction("je __rt_cno_string_x86");                             // a string is its own class name
    emitter.instruction("jmp __rt_cno_empty_x86");                              // any other payload → empty

    emitter.label("__rt_cno_object_x86");
    emitter.instruction("mov rdi, QWORD PTR [rdi + 8]");                        // unwrap the object pointer from the cell
    emitter.instruction("test rdi, rdi");                                       // defensive null guard
    emitter.instruction("je __rt_cno_empty_x86");
    emitter.instruction("mov r10, QWORD PTR [rdi]");                            // load the object's runtime class id
    abi::emit_load_symbol_to_reg(emitter, "r11", "_class_name_count", 0);       // dense class-name row count
    emitter.instruction("cmp r10, r11");                                        // validate the class id before indexing
    emitter.instruction("jae __rt_cno_empty_x86");                              // unknown class id → empty
    abi::emit_symbol_address(emitter, "r11", "_class_name_entries");
    emitter.instruction("shl r10, 4");                                          // scale the class id by the 16-byte row size
    emitter.instruction("add r11, r10");                                        // point at the selected class-name row
    emitter.instruction("mov rax, QWORD PTR [r11]");                            // class-name string pointer → result
    emitter.instruction("mov rdx, QWORD PTR [r11 + 8]");                        // class-name string length → result
    emitter.instruction("ret");                                                 // return the class name in rax/rdx

    emitter.label("__rt_cno_string_x86");
    emitter.instruction("mov rax, QWORD PTR [rdi + 8]");                        // string payload pointer → result
    emitter.instruction("mov rdx, QWORD PTR [rdi + 16]");                       // string payload length → result
    emitter.instruction("ret");                                                 // return the string in rax/rdx

    emitter.label("__rt_cno_empty_x86");
    abi::emit_symbol_address(emitter, "rax", "_class_name_missing");
    emitter.instruction("xor edx, edx");                                        // empty class name has zero length
    emitter.instruction("ret");                                                 // return the empty class name
}
