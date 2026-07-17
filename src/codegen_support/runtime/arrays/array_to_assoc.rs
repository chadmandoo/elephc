//! Purpose:
//! Emits `__rt_array_to_assoc`: coerce an array value to associative (hash) storage,
//! returning an owned pointer. A packed/indexed source (heap kind 2) is converted to a
//! hash via `__rt_array_to_hash` (integer keys 0..n-1); an already-hash source is an
//! owned identity retain (`__rt_incref`).
//!
//! Called from:
//! - `crate::codegen_support::runtime::emitters::emit_runtime()` via `crate::codegen_support::runtime::arrays`.
//!
//! Key details:
//! - Used to lower a `runtime_call` coercion from a statically-indexed `Array(_)` (whose
//!   runtime storage may be packed or hash) to a declared `AssocArray` type, so associative
//!   consumers always receive hash storage. The result is owned; the source is not mutated.

use crate::codegen_support::{abi, emit::Emitter, platform::Arch};

/// Coerces an array pointer to owned associative (hash) storage.
/// Input: x0/rdi = array pointer. Output: x0/rax = owned hash pointer.
pub fn emit_array_to_assoc(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_array_to_assoc_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: array_to_assoc ---");
    emitter.label_global("__rt_array_to_assoc");
    emitter.instruction("sub sp, sp, #16");                                     // frame for the helper calls
    emitter.instruction("str x30, [sp]");                                       // save return address
    emitter.instruction("ldr x9, [x0, #-8]");                                   // load the heap-kind header word
    emitter.instruction("and x9, x9, #0xff");                                   // isolate the low-byte heap kind
    emitter.instruction("cmp x9, #2");                                          // is the source a packed indexed array?
    emitter.instruction("b.ne __rt_array_to_assoc_already_hash");
    emitter.instruction("bl __rt_array_to_hash");                              // convert packed -> owned hash (x0 in/out)
    emitter.instruction("b __rt_array_to_assoc_done");
    emitter.label("__rt_array_to_assoc_already_hash");
    emitter.instruction("bl __rt_incref");                                     // retain the already-hash source (x0 preserved)
    emitter.label("__rt_array_to_assoc_done");
    emitter.instruction("ldr x30, [sp]");                                       // restore return address
    emitter.instruction("add sp, sp, #16");                                     // release the frame
    emitter.instruction("ret");                                                 // x0 = owned hash pointer
}

/// x86_64 Linux implementation. Input: rdi = array pointer. Output: rax = owned hash pointer.
fn emit_array_to_assoc_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: array_to_assoc ---");
    emitter.label_global("__rt_array_to_assoc");
    emitter.instruction("push rbp");
    emitter.instruction("mov rbp, rsp");
    emitter.instruction("sub rsp, 16");
    emitter.instruction("mov QWORD PTR [rbp - 8], rdi");                        // save the source array pointer
    emitter.instruction("mov rax, QWORD PTR [rdi - 8]");                        // load the heap-kind header word
    emitter.instruction("and rax, 0xff");                                       // isolate the low-byte heap kind
    emitter.instruction("cmp rax, 2");                                          // is the source a packed indexed array?
    emitter.instruction("jne __rt_array_to_assoc_already_hash");
    emitter.instruction("mov rdi, QWORD PTR [rbp - 8]");                        // packed source -> array_to_hash input
    abi::emit_call_label(emitter, "__rt_array_to_hash");                        // rax = owned hash
    emitter.instruction("add rsp, 16");
    emitter.instruction("pop rbp");
    emitter.instruction("ret");
    emitter.label("__rt_array_to_assoc_already_hash");
    emitter.instruction("mov rax, QWORD PTR [rbp - 8]");                        // already-hash source -> __rt_incref input (rax) and return value
    abi::emit_call_label(emitter, "__rt_incref");                              // retain; rax preserved = source pointer
    emitter.instruction("add rsp, 16");
    emitter.instruction("pop rbp");
    emitter.instruction("ret");                                                 // rax = owned hash pointer
}
