//! Purpose:
//! Emits the `__rt_array_spaceship` runtime helper assembly for array `<=>` comparison.
//! Keeps PHP array ordering semantics and target-specific ABI variants in one focused emitter.
//!
//! Called from:
//! - `crate::codegen::runtime::emitters::emit_runtime()` via `crate::codegen::runtime::arrays`.
//!
//! Key details:
//! - PHP orders arrays by count first (fewer elements sorts smaller regardless of
//!   content), then pairwise element-by-element, short-circuiting on the first
//!   non-zero comparison — the multi-key usort staple `[$a, $b] <=> [$c, $d]`.
//! - The element layout comes from the caller as a compile-time mode flag derived
//!   from the static element type: mode 0 = 8-byte integer slots compared
//!   numerically; mode 1 = 16-byte string ptr/len slots compared through
//!   `__rt_str_spaceship` (numeric strings numerically, otherwise byte order).
//! - Elements start at +24 past the array header (length word at +0), matching the
//!   indexed-array element base used by the inline array-get lowering.

use crate::codegen::emit::Emitter;
use crate::codegen::platform::Arch;

/// array_spaceship: PHP `<=>` over two packed arrays with a static element mode.
/// Input:  AArch64 x1 = left array, x2 = right array, x3 = mode (0 int, 1 str)
///         x86_64  rdi = left array, rsi = right array, rdx = mode
/// Output: -1 / 0 / 1 in x0 (AArch64) / rax (x86_64)
pub fn emit_array_spaceship(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_array_spaceship_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: array_spaceship ---");
    emitter.label_global("__rt_array_spaceship");

    // -- set up stack frame and preserve callee-saved registers --
    emitter.instruction("sub sp, sp, #64");                                     // allocate the helper frame
    emitter.instruction("stp x29, x30, [sp, #48]");                             // save frame pointer and return address
    emitter.instruction("add x29, sp, #48");                                    // establish the helper frame pointer
    emitter.instruction("stp x19, x20, [sp, #32]");                             // save callee-saved x19/x20
    emitter.instruction("stp x21, x22, [sp, #16]");                             // save callee-saved x21/x22
    emitter.instruction("str x23, [sp, #0]");                                   // save callee-saved x23
    emitter.instruction("mov x19, x1");                                         // x19 = left array pointer
    emitter.instruction("mov x20, x2");                                         // x20 = right array pointer
    emitter.instruction("mov x23, x3");                                         // x23 = element comparison mode

    // -- count comparison decides before any element is inspected --
    emitter.instruction("ldr x21, [x19]");                                      // x21 = left element count
    emitter.instruction("ldr x9, [x20]");                                       // x9 = right element count
    emitter.instruction("cmp x21, x9");                                         // PHP orders by count first
    emitter.instruction("b.lt __rt_arr_ss_less");                               // fewer elements sorts smaller
    emitter.instruction("b.gt __rt_arr_ss_greater");                            // more elements sorts larger
    emitter.instruction("mov x22, #0");                                         // element index

    emitter.label("__rt_arr_ss_loop");
    emitter.instruction("cmp x22, x21");                                        // compared every element pair?
    emitter.instruction("b.ge __rt_arr_ss_equal");                              // all pairs equal → arrays equal
    emitter.instruction("cbnz x23, __rt_arr_ss_str");                           // mode 1 → string element compare

    // -- mode 0: 8-byte integer slots --
    emitter.instruction("add x9, x19, #24");                                    // left element base
    emitter.instruction("ldr x10, [x9, x22, lsl #3]");                          // left element value
    emitter.instruction("add x9, x20, #24");                                    // right element base
    emitter.instruction("ldr x11, [x9, x22, lsl #3]");                          // right element value
    emitter.instruction("cmp x10, x11");                                        // signed integer comparison
    emitter.instruction("b.lt __rt_arr_ss_less");                               // left element smaller
    emitter.instruction("b.gt __rt_arr_ss_greater");                            // left element larger
    emitter.instruction("b __rt_arr_ss_next");                                  // equal pair: advance

    // -- mode 1: 16-byte string ptr/len slots via __rt_str_spaceship --
    emitter.label("__rt_arr_ss_str");
    emitter.instruction("lsl x9, x22, #4");                                     // scale index by 16-byte string slots
    emitter.instruction("add x10, x19, #24");                                   // left element base
    emitter.instruction("add x10, x10, x9");                                    // left slot address
    emitter.instruction("ldr x1, [x10]");                                       // left string pointer
    emitter.instruction("ldr x2, [x10, #8]");                                   // left string length
    emitter.instruction("add x10, x20, #24");                                   // right element base
    emitter.instruction("add x10, x10, x9");                                    // right slot address
    emitter.instruction("ldr x3, [x10]");                                       // right string pointer
    emitter.instruction("ldr x4, [x10, #8]");                                   // right string length
    emitter.instruction("bl __rt_str_spaceship");                               // PHP 8 string spaceship semantics
    emitter.instruction("cbnz x0, __rt_arr_ss_ret");                            // first non-zero pair decides

    emitter.label("__rt_arr_ss_next");
    emitter.instruction("add x22, x22, #1");                                    // advance to the next element pair
    emitter.instruction("b __rt_arr_ss_loop");                                  // continue comparing

    emitter.label("__rt_arr_ss_less");
    emitter.instruction("mov x0, #-1");                                         // left sorts before right
    emitter.instruction("b __rt_arr_ss_ret");                                   // share the epilogue

    emitter.label("__rt_arr_ss_greater");
    emitter.instruction("mov x0, #1");                                          // left sorts after right
    emitter.instruction("b __rt_arr_ss_ret");                                   // share the epilogue

    emitter.label("__rt_arr_ss_equal");
    emitter.instruction("mov x0, #0");                                          // arrays compare equal

    emitter.label("__rt_arr_ss_ret");
    emitter.instruction("ldr x23, [sp, #0]");                                   // restore callee-saved x23
    emitter.instruction("ldp x21, x22, [sp, #16]");                             // restore callee-saved x21/x22
    emitter.instruction("ldp x19, x20, [sp, #32]");                             // restore callee-saved x19/x20
    emitter.instruction("ldp x29, x30, [sp, #48]");                             // restore frame pointer and return address
    emitter.instruction("add sp, sp, #64");                                     // release the helper frame
    emitter.instruction("ret");                                                 // return the -1/0/1 ordering
}

/// Emits the Linux x86_64 variant of `__rt_array_spaceship`.
fn emit_array_spaceship_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: array_spaceship ---");
    emitter.label_global("__rt_array_spaceship");

    emitter.instruction("push rbp");                                            // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish the helper frame pointer
    emitter.instruction("push rbx");                                            // save callee-saved rbx (left array)
    emitter.instruction("push r12");                                            // save callee-saved r12 (right array)
    emitter.instruction("push r13");                                            // save callee-saved r13 (element count)
    emitter.instruction("push r14");                                            // save callee-saved r14 (element index)
    emitter.instruction("push r15");                                            // save callee-saved r15 (element mode)
    emitter.instruction("sub rsp, 8");                                          // keep rsp 16-aligned for the string helper's libc calls
    emitter.instruction("mov rbx, rdi");                                        // rbx = left array pointer
    emitter.instruction("mov r12, rsi");                                        // r12 = right array pointer
    emitter.instruction("mov r15, rdx");                                        // r15 = element comparison mode

    emitter.instruction("mov r13, QWORD PTR [rbx]");                            // r13 = left element count
    emitter.instruction("mov r9, QWORD PTR [r12]");                             // r9 = right element count
    emitter.instruction("cmp r13, r9");                                         // PHP orders by count first
    emitter.instruction("jl __rt_arr_ss_less_x86");                             // fewer elements sorts smaller
    emitter.instruction("jg __rt_arr_ss_greater_x86");                          // more elements sorts larger
    emitter.instruction("xor r14, r14");                                        // element index

    emitter.label("__rt_arr_ss_loop_x86");
    emitter.instruction("cmp r14, r13");                                        // compared every element pair?
    emitter.instruction("jge __rt_arr_ss_equal_x86");                           // all pairs equal → arrays equal
    emitter.instruction("test r15, r15");                                       // mode 1 → string element compare
    emitter.instruction("jnz __rt_arr_ss_str_x86");                             // dispatch to the string pair path

    // -- mode 0: 8-byte integer slots --
    emitter.instruction("mov r10, QWORD PTR [rbx + r14*8 + 24]");               // left element value
    emitter.instruction("mov r11, QWORD PTR [r12 + r14*8 + 24]");               // right element value
    emitter.instruction("cmp r10, r11");                                        // signed integer comparison
    emitter.instruction("jl __rt_arr_ss_less_x86");                             // left element smaller
    emitter.instruction("jg __rt_arr_ss_greater_x86");                          // left element larger
    emitter.instruction("jmp __rt_arr_ss_next_x86");                            // equal pair: advance

    // -- mode 1: 16-byte string ptr/len slots via __rt_str_spaceship --
    emitter.label("__rt_arr_ss_str_x86");
    emitter.instruction("mov r10, r14");                                        // copy element index before scaling
    emitter.instruction("shl r10, 4");                                          // scale index by 16-byte string slots
    emitter.instruction("mov rdi, QWORD PTR [rbx + r10 + 24]");                 // left string pointer
    emitter.instruction("mov rsi, QWORD PTR [rbx + r10 + 32]");                 // left string length
    emitter.instruction("mov rdx, QWORD PTR [r12 + r10 + 24]");                 // right string pointer
    emitter.instruction("mov rcx, QWORD PTR [r12 + r10 + 32]");                 // right string length
    emitter.instruction("call __rt_str_spaceship");                             // PHP 8 string spaceship semantics
    emitter.instruction("test rax, rax");                                       // first non-zero pair decides
    emitter.instruction("jnz __rt_arr_ss_ret_x86");                             // propagate the element ordering

    emitter.label("__rt_arr_ss_next_x86");
    emitter.instruction("add r14, 1");                                          // advance to the next element pair
    emitter.instruction("jmp __rt_arr_ss_loop_x86");                            // continue comparing

    emitter.label("__rt_arr_ss_less_x86");
    emitter.instruction("mov rax, -1");                                         // left sorts before right
    emitter.instruction("jmp __rt_arr_ss_ret_x86");                             // share the epilogue

    emitter.label("__rt_arr_ss_greater_x86");
    emitter.instruction("mov eax, 1");                                          // left sorts after right
    emitter.instruction("jmp __rt_arr_ss_ret_x86");                             // share the epilogue

    emitter.label("__rt_arr_ss_equal_x86");
    emitter.instruction("xor eax, eax");                                        // arrays compare equal

    emitter.label("__rt_arr_ss_ret_x86");
    emitter.instruction("add rsp, 8");                                          // release the alignment pad
    emitter.instruction("pop r15");                                             // restore callee-saved r15
    emitter.instruction("pop r14");                                             // restore callee-saved r14
    emitter.instruction("pop r13");                                             // restore callee-saved r13
    emitter.instruction("pop r12");                                             // restore callee-saved r12
    emitter.instruction("pop rbx");                                             // restore callee-saved rbx
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the -1/0/1 ordering
}
