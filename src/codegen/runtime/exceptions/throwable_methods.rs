//! Purpose:
//! Emits the `__rt_throwable_*` interface-dispatch bodies for the compact
//! Throwable payload: getMessage/getCode/getPrevious/getFile/getLine/
//! getTrace/getTraceAsString/__toString.
//!
//! Called from:
//! - `crate::codegen::runtime::emitters::emit_runtime()` via
//!   `crate::codegen::runtime::exceptions`, and referenced by the
//!   `_class_interface_impl_*` tables emitted in `runtime::data::user`.
//!
//! Key details:
//! - Direct method calls on statically-known throwable receivers lower to
//!   payload reads inline (`lower_throwable_standard_method`); these bodies
//!   exist for DYNAMIC dispatch — Mixed-receiver and interface-typed calls —
//!   which routes through the class interface tables and previously hit
//!   NULL slots (builtin throwable methods have no compiled PHP bodies).
//! - Calling convention matches interface dispatch: receiver in the first
//!   argument register (`x0` / `rdi`), results in the standard result
//!   registers (strings as ptr/len pairs in `x0`/`x1` / `rax`/`rdx`).
//! - Compact payload layout: [0]=class id, [8]=message ptr, [16]=message
//!   len, [24]=code, [32]=previous (raw object pointer or 0).
//! - `getPrevious` returns the Mixed representation (`?Throwable` is a
//!   union): 0 for null, otherwise a freshly boxed tag-6 object cell.
//! - Receivers arriving as tag-6 Mixed boxes (heap kind 5) are unwrapped
//!   once at entry — dynamic dispatch may hand over the box itself.

use crate::codegen::emit::Emitter;
use crate::codegen::platform::Arch;

/// Emits every `__rt_throwable_*` interface-dispatch body.
pub fn emit_throwable_methods(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_throwable_methods_x86_64(emitter);
        return;
    }
    emit_throwable_methods_aarch64(emitter);
}

fn emit_throwable_methods_aarch64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: throwable interface-dispatch bodies ---");

    // Shared receiver unwrap: tag-6 Mixed boxes (heap kind 5) yield their
    // object payload; raw objects pass through. Result in x0.
    emitter.label_global("__rt_throwable_recv");
    emitter.instruction("ldr x9, [x0, #-8]");                                   // load the heap kind word from the uniform header
    emitter.instruction("and x9, x9, #0xff");                                   // isolate the low-byte heap kind tag
    emitter.instruction("cmp x9, #5");                                          // heap kind 5 marks boxed Mixed cells
    emitter.instruction("b.ne __rt_throwable_recv_done");                       // raw object receivers need no unwrap
    emitter.instruction("ldr x9, [x0]");                                        // load the mixed cell's runtime tag
    emitter.instruction("cmp x9, #6");                                          // tag 6 marks object payloads
    emitter.instruction("b.ne __rt_throwable_recv_done");                       // non-object boxes are left untouched
    emitter.instruction("ldr x0, [x0, #8]");                                    // unwrap the boxed object payload pointer
    emitter.label("__rt_throwable_recv_done");
    emitter.instruction("ret");

    emitter.label_global("__rt_throwable_get_message");
    emitter.instruction("str x30, [sp, #-16]!");                                // preserve the caller return address across the receiver unwrap
    emitter.instruction("bl __rt_throwable_recv");                              // normalize boxed receivers to the raw object pointer
    emitter.instruction("ldr x30, [sp], #16");                                  // restore the caller return address
    emitter.instruction("ldr x1, [x0, #16]");                                   // load the Throwable message length
    emitter.instruction("ldr x0, [x0, #8]");                                    // load the Throwable message pointer
    emitter.instruction("ret");

    emitter.label_global("__rt_throwable_get_code");
    emitter.instruction("str x30, [sp, #-16]!");                                // preserve the caller return address across the receiver unwrap
    emitter.instruction("bl __rt_throwable_recv");                              // normalize boxed receivers to the raw object pointer
    emitter.instruction("ldr x30, [sp], #16");                                  // restore the caller return address
    emitter.instruction("ldr x0, [x0, #24]");                                   // load the Throwable code
    emitter.instruction("ret");

    emitter.label_global("__rt_throwable_get_previous");
    emitter.instruction("str x30, [sp, #-16]!");                                // preserve the caller return address across nested helper calls
    emitter.instruction("bl __rt_throwable_recv");                              // normalize boxed receivers to the raw object pointer
    emitter.instruction("ldr x0, [x0, #32]");                                   // load the previous throwable pointer (or 0)
    emitter.instruction("cbz x0, __rt_throwable_get_previous_null");            // null previous keeps the Mixed 0 representation
    emitter.instruction("mov x1, x0");                                          // pass the previous object pointer as the mixed payload low word
    emitter.instruction("mov x2, xzr");                                         // object payloads only use the low word
    emitter.instruction("mov x0, #6");                                          // runtime tag 6 = object
    emitter.instruction("bl __rt_mixed_from_value");                            // retain the previous object and box it into a mixed cell
    emitter.label("__rt_throwable_get_previous_null");
    emitter.instruction("ldr x30, [sp], #16");                                  // restore the caller return address
    emitter.instruction("ret");

    emitter.label_global("__rt_throwable_get_file");
    crate::codegen::abi::emit_symbol_address(emitter, "x0", "_throwable_empty_str");
    emitter.instruction("mov x1, #0");                                          // synthesized file/trace strings are empty
    emitter.instruction("ret");

    emitter.label_global("__rt_throwable_get_line");
    emitter.instruction("mov x0, #0");                                          // synthesized line numbers are zero
    emitter.instruction("ret");

    emitter.label_global("__rt_throwable_get_trace");
    emitter.instruction("str x30, [sp, #-16]!");                                // preserve the caller return address across the allocation
    emitter.instruction("mov x0, #4");                                          // synthesized traces are empty arrays (capacity 4)
    emitter.instruction("mov x1, #8");                                          // 8-byte slots
    emitter.instruction("bl __rt_array_new");                                   // allocate the empty trace array
    emitter.instruction("ldr x9, [x0, #-8]");                                   // load the packed array kind word from the heap header
    emitter.instruction("mov x12, #0x80ff");                                    // preserve the indexed-array kind and persistent COW flag
    emitter.instruction("and x9, x9, x12");                                     // keep only the persistent indexed-array metadata bits
    emitter.instruction("mov x11, #7");                                         // runtime array value_type tag 7 = Mixed elements
    emitter.instruction("lsl x11, x11, #8");                                    // move the value_type tag into the packed kind-word byte lane
    emitter.instruction("orr x9, x9, x11");                                     // combine the heap kind with the array value_type tag
    emitter.instruction("str x9, [x0, #-8]");                                   // persist the packed array kind word in the heap header
    emitter.instruction("ldr x30, [sp], #16");                                  // restore the caller return address
    emitter.instruction("ret");
}

fn emit_throwable_methods_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: throwable interface-dispatch bodies ---");

    // Shared receiver unwrap: tag-6 Mixed boxes (heap kind 5) yield their
    // object payload; raw objects pass through. Receiver in rdi, result in rdi.
    emitter.label_global("__rt_throwable_recv");
    emitter.instruction("mov r10, QWORD PTR [rdi - 8]");                        // load the heap kind word from the uniform header
    emitter.instruction("and r10, 0xff");                                       // isolate the low-byte heap kind tag
    emitter.instruction("cmp r10, 5");                                          // heap kind 5 marks boxed Mixed cells
    emitter.instruction("jne __rt_throwable_recv_done");                        // raw object receivers need no unwrap
    emitter.instruction("mov r10, QWORD PTR [rdi]");                            // load the mixed cell's runtime tag
    emitter.instruction("cmp r10, 6");                                          // tag 6 marks object payloads
    emitter.instruction("jne __rt_throwable_recv_done");                        // non-object boxes are left untouched
    emitter.instruction("mov rdi, QWORD PTR [rdi + 8]");                        // unwrap the boxed object payload pointer
    emitter.label("__rt_throwable_recv_done");
    emitter.instruction("ret");

    emitter.label_global("__rt_throwable_get_message");
    emitter.instruction("call __rt_throwable_recv");                            // normalize boxed receivers to the raw object pointer
    emitter.instruction("mov rax, QWORD PTR [rdi + 8]");                        // load the Throwable message pointer
    emitter.instruction("mov rdx, QWORD PTR [rdi + 16]");                       // load the Throwable message length
    emitter.instruction("ret");

    emitter.label_global("__rt_throwable_get_code");
    emitter.instruction("call __rt_throwable_recv");                            // normalize boxed receivers to the raw object pointer
    emitter.instruction("mov rax, QWORD PTR [rdi + 24]");                       // load the Throwable code
    emitter.instruction("ret");

    emitter.label_global("__rt_throwable_get_previous");
    emitter.instruction("call __rt_throwable_recv");                            // normalize boxed receivers to the raw object pointer
    emitter.instruction("mov rax, QWORD PTR [rdi + 32]");                       // load the previous throwable pointer (or 0)
    emitter.instruction("test rax, rax");                                       // null previous keeps the Mixed 0 representation
    emitter.instruction("jz __rt_throwable_get_previous_null");
    emitter.instruction("push rbp");                                            // align the stack for the nested boxing helper call
    emitter.instruction("mov rbp, rsp");
    emitter.instruction("mov rdi, rax");                                        // pass the previous object pointer as the mixed payload low word
    emitter.instruction("xor rsi, rsi");                                        // object payloads only use the low word
    emitter.instruction("mov rax, 6");                                          // runtime tag 6 = object
    emitter.instruction("call __rt_mixed_from_value");                          // retain the previous object and box it into a mixed cell
    emitter.instruction("mov rsp, rbp");                                        // release the alignment frame
    emitter.instruction("pop rbp");
    emitter.label("__rt_throwable_get_previous_null");
    emitter.instruction("ret");

    emitter.label_global("__rt_throwable_get_file");
    emitter.instruction("lea rax, [rip + _throwable_empty_str]");               // synthesized file/trace strings are empty
    emitter.instruction("xor rdx, rdx");
    emitter.instruction("ret");

    emitter.label_global("__rt_throwable_get_line");
    emitter.instruction("xor rax, rax");                                        // synthesized line numbers are zero
    emitter.instruction("ret");

    emitter.label_global("__rt_throwable_get_trace");
    emitter.instruction("push rbp");                                            // align the stack for the nested allocation call
    emitter.instruction("mov rbp, rsp");
    emitter.instruction("mov rdi, 4");                                          // synthesized traces are empty arrays (capacity 4)
    emitter.instruction("mov rsi, 8");                                          // 8-byte slots
    emitter.instruction("call __rt_array_new");                                 // allocate the empty trace array
    emitter.instruction("mov r10, QWORD PTR [rax - 8]");                        // load the packed array kind word from the heap header
    emitter.instruction("mov r11, 0xffffffff000080ff");                         // preserve the heap magic marker plus the indexed-array kind and COW flag
    emitter.instruction("and r10, r11");                                        // keep only the persistent indexed-array metadata bits
    emitter.instruction("mov r11, 7");                                          // runtime array value_type tag 7 = Mixed elements
    emitter.instruction("shl r11, 8");                                          // move the value_type tag into the packed kind-word byte lane
    emitter.instruction("or r10, r11");                                         // combine the heap kind with the array value_type tag
    emitter.instruction("mov QWORD PTR [rax - 8], r10");                        // persist the packed array kind word in the heap header
    emitter.instruction("mov rsp, rbp");                                        // release the alignment frame
    emitter.instruction("pop rbp");
    emitter.instruction("ret");
}
