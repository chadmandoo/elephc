//! Purpose:
//! Emits the `__rt_array_merge_str` runtime helper: concatenates two `string`-element indexed
//! arrays into a new one, persisting each 16-byte string descriptor into an owned copy. String
//! array elements are array-owned (`__rt_array_free_deep` frees them), so a raw descriptor copy
//! would share the sources' buffers and double-free them when the result is released — mirror
//! `__rt_array_slice_str`'s per-element `__rt_str_persist` discipline.
//!
//! Called from:
//! - `crate::codegen_support::runtime::emitters::emit_runtime()` via `crate::codegen_support::runtime::arrays`.

use crate::codegen_support::emit::Emitter;
use crate::codegen_support::platform::Arch;

/// Emits the `__rt_array_merge_str` runtime helper.
///
/// # ABI
/// - Input: (x0/rdi) = first array pointer, (x1/rsi) = second array pointer
/// - Output: (x0/rax) = new `string`-element array = arr1 ++ arr2, with owned string copies
pub fn emit_array_merge_str(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_array_merge_str_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: array_merge_str (16-byte string descriptors, per-element persist) ---");
    emitter.label_global("__rt_array_merge_str");

    // stack: [sp,0]=arr1, [sp,8]=arr2, [sp,16]=len1, [sp,24]=len2, [sp,32]=dst, [sp,40]=loop index
    emitter.instruction("sub sp, sp, #96");
    emitter.instruction("stp x29, x30, [sp, #80]");
    emitter.instruction("add x29, sp, #80");
    emitter.instruction("str x0, [sp, #0]");                                    // arr1 pointer
    emitter.instruction("str x1, [sp, #8]");                                    // arr2 pointer
    emitter.instruction("ldr x9, [x0]");
    emitter.instruction("str x9, [sp, #16]");                                   // len1
    emitter.instruction("ldr x10, [x1]");
    emitter.instruction("str x10, [sp, #24]");                                  // len2

    // -- allocate result (16-byte string slots), capacity = len1 + len2 --
    emitter.instruction("add x0, x9, x10");
    emitter.instruction("mov x1, #16");
    emitter.instruction("bl __rt_array_new");
    emitter.instruction("str x0, [sp, #32]");                                   // dst pointer

    // -- copy arr1 (persist each element) into dst[0..len1] --
    emitter.instruction("str xzr, [sp, #40]");                                  // i = 0
    emitter.label("__rt_array_merge_str_copy1");
    emitter.instruction("ldr x6, [sp, #40]");                                   // i
    emitter.instruction("ldr x4, [sp, #16]");                                   // len1
    emitter.instruction("cmp x6, x4");
    emitter.instruction("b.ge __rt_array_merge_str_copy2_setup");
    emitter.instruction("ldr x5, [sp, #0]");                                    // arr1
    emitter.instruction("add x10, x5, x6, lsl #4");
    emitter.instruction("ldp x1, x2, [x10, #24]");                              // load (ptr,len)
    emitter.instruction("bl __rt_str_persist");                                 // owned x1, x2
    emitter.instruction("ldr x6, [sp, #40]");                                   // reload i
    emitter.instruction("ldr x0, [sp, #32]");                                   // dst
    emitter.instruction("add x12, x0, x6, lsl #4");
    emitter.instruction("stp x1, x2, [x12, #24]");                              // dst[i] = owned
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("str x6, [sp, #40]");
    emitter.instruction("b __rt_array_merge_str_copy1");

    // -- copy arr2 (persist each element) into dst[len1..len1+len2] --
    emitter.label("__rt_array_merge_str_copy2_setup");
    emitter.instruction("str xzr, [sp, #40]");                                  // j = 0
    emitter.label("__rt_array_merge_str_copy2");
    emitter.instruction("ldr x6, [sp, #40]");                                   // j
    emitter.instruction("ldr x4, [sp, #24]");                                   // len2
    emitter.instruction("cmp x6, x4");
    emitter.instruction("b.ge __rt_array_merge_str_done");
    emitter.instruction("ldr x5, [sp, #8]");                                    // arr2
    emitter.instruction("add x10, x5, x6, lsl #4");
    emitter.instruction("ldp x1, x2, [x10, #24]");
    emitter.instruction("bl __rt_str_persist");
    emitter.instruction("ldr x6, [sp, #40]");                                   // reload j
    emitter.instruction("ldr x7, [sp, #16]");                                   // len1 (write offset)
    emitter.instruction("add x8, x7, x6");                                      // len1 + j
    emitter.instruction("ldr x0, [sp, #32]");                                   // dst
    emitter.instruction("add x12, x0, x8, lsl #4");
    emitter.instruction("stp x1, x2, [x12, #24]");                              // dst[len1+j] = owned
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("str x6, [sp, #40]");
    emitter.instruction("b __rt_array_merge_str_copy2");

    emitter.label("__rt_array_merge_str_done");
    emitter.instruction("ldr x0, [sp, #32]");                                   // dst
    emitter.instruction("ldr x9, [sp, #16]");
    emitter.instruction("ldr x10, [sp, #24]");
    emitter.instruction("add x9, x9, x10");                                     // total length
    emitter.instruction("str x9, [x0]");                                        // publish length
    emitter.instruction("ldp x29, x30, [sp, #80]");
    emitter.instruction("add sp, sp, #96");
    emitter.instruction("ret");
}

/// Emits the `__rt_array_merge_str` runtime helper for x86_64 Linux.
fn emit_array_merge_str_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: array_merge_str (16-byte string descriptors, per-element persist) ---");
    emitter.label_global("__rt_array_merge_str");

    // stack: [rbp-8]=arr1, [rbp-16]=arr2, [rbp-24]=len1, [rbp-32]=len2, [rbp-40]=dst, [rbp-48]=index
    emitter.instruction("push rbp");
    emitter.instruction("mov rbp, rsp");
    emitter.instruction("sub rsp, 64");
    emitter.instruction("mov QWORD PTR [rbp - 8], rdi");                        // arr1
    emitter.instruction("mov QWORD PTR [rbp - 16], rsi");                       // arr2
    emitter.instruction("mov rax, QWORD PTR [rdi]");
    emitter.instruction("mov QWORD PTR [rbp - 24], rax");                       // len1
    emitter.instruction("mov rax, QWORD PTR [rsi]");
    emitter.instruction("mov QWORD PTR [rbp - 32], rax");                       // len2

    // -- allocate result (16-byte string slots), capacity = len1 + len2 --
    emitter.instruction("mov rdi, QWORD PTR [rbp - 24]");
    emitter.instruction("add rdi, QWORD PTR [rbp - 32]");
    emitter.instruction("mov rsi, 16");
    emitter.instruction("call __rt_array_new");
    emitter.instruction("mov QWORD PTR [rbp - 40], rax");                       // dst

    // -- copy arr1 (persist each element) into dst[0..len1] --
    emitter.instruction("mov QWORD PTR [rbp - 48], 0");                         // i = 0
    emitter.label("__rt_array_merge_str_copy1_x86");
    emitter.instruction("mov rax, QWORD PTR [rbp - 48]");                       // i
    emitter.instruction("cmp rax, QWORD PTR [rbp - 24]");                       // i < len1?
    emitter.instruction("jge __rt_array_merge_str_copy2_setup_x86");
    emitter.instruction("shl rax, 4");                                          // i*16
    emitter.instruction("mov r8, QWORD PTR [rbp - 8]");                         // arr1
    emitter.instruction("add r8, rax");
    emitter.instruction("mov rax, QWORD PTR [r8 + 24]");                        // ptr
    emitter.instruction("mov rdx, QWORD PTR [r8 + 32]");                        // len
    emitter.instruction("call __rt_str_persist");                              // rax=owned ptr, rdx=owned len
    emitter.instruction("mov rcx, QWORD PTR [rbp - 48]");                       // reload i
    emitter.instruction("shl rcx, 4");                                          // i*16
    emitter.instruction("mov r8, QWORD PTR [rbp - 40]");                        // dst
    emitter.instruction("add r8, rcx");
    emitter.instruction("mov QWORD PTR [r8 + 24], rax");                        // dst[i] = owned ptr
    emitter.instruction("mov QWORD PTR [r8 + 32], rdx");                        // dst[i].len = owned len
    emitter.instruction("inc QWORD PTR [rbp - 48]");
    emitter.instruction("jmp __rt_array_merge_str_copy1_x86");

    // -- copy arr2 (persist each element) into dst[len1..len1+len2] --
    emitter.label("__rt_array_merge_str_copy2_setup_x86");
    emitter.instruction("mov QWORD PTR [rbp - 48], 0");                         // j = 0
    emitter.label("__rt_array_merge_str_copy2_x86");
    emitter.instruction("mov rax, QWORD PTR [rbp - 48]");                       // j
    emitter.instruction("cmp rax, QWORD PTR [rbp - 32]");                       // j < len2?
    emitter.instruction("jge __rt_array_merge_str_done_x86");
    emitter.instruction("shl rax, 4");                                          // j*16
    emitter.instruction("mov r8, QWORD PTR [rbp - 16]");                        // arr2
    emitter.instruction("add r8, rax");
    emitter.instruction("mov rax, QWORD PTR [r8 + 24]");                        // ptr
    emitter.instruction("mov rdx, QWORD PTR [r8 + 32]");                        // len
    emitter.instruction("call __rt_str_persist");
    emitter.instruction("mov rcx, QWORD PTR [rbp - 48]");                       // reload j
    emitter.instruction("add rcx, QWORD PTR [rbp - 24]");                       // len1 + j
    emitter.instruction("shl rcx, 4");                                          // (len1+j)*16
    emitter.instruction("mov r8, QWORD PTR [rbp - 40]");                        // dst
    emitter.instruction("add r8, rcx");
    emitter.instruction("mov QWORD PTR [r8 + 24], rax");                        // dst[len1+j] = owned ptr
    emitter.instruction("mov QWORD PTR [r8 + 32], rdx");                        // .len = owned len
    emitter.instruction("inc QWORD PTR [rbp - 48]");
    emitter.instruction("jmp __rt_array_merge_str_copy2_x86");

    emitter.label("__rt_array_merge_str_done_x86");
    emitter.instruction("mov rax, QWORD PTR [rbp - 40]");                       // dst
    emitter.instruction("mov rcx, QWORD PTR [rbp - 24]");
    emitter.instruction("add rcx, QWORD PTR [rbp - 32]");                       // total length
    emitter.instruction("mov QWORD PTR [rax], rcx");                            // publish length
    emitter.instruction("add rsp, 64");
    emitter.instruction("pop rbp");
    emitter.instruction("ret");
}
