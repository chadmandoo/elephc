//! Purpose:
//! Emits the `__rt_array_strict_eq` runtime helper: PHP `===` deep value-equality for
//! two arrays (indexed or associative). Recurses into nested arrays; read-only.
//!
//! Called from:
//! - `crate::codegen_support::runtime::emitters::emit_runtime()` via `crate::codegen_support::runtime::arrays`.
//!
//! Key details:
//! - PHP `===` on arrays requires the SAME count and the SAME key=>value pairs in the SAME
//!   insertion order, with values recursively `===`. Objects compare by identity (pointer),
//!   which the scalar/pointer word-compare gives for free; nested arrays recurse.
//! - Packed/indexed operands (heap kind 2) are normalized to a hash via `__rt_array_to_hash`
//!   so both sides walk uniformly through `__rt_hash_iter_next` in insertion order. The
//!   normalized temporaries are released with `__rt_decref_any` before returning.
//! - Read-only: no refcount changes to the operands; borrowed values are only compared.
//! - value tags: 1=string, 4=indexed-array, 5=assoc-array, 8=null; anything else (int/float/
//!   bool/object) compares by its low+high payload words. heap kind at [ptr-8] & 0xff: 2=indexed.

use crate::codegen_support::{abi, emit::Emitter, platform::Arch};

/// Compares two array pointers for PHP strict (`===`) value-equality, recursing into nested
/// arrays. Input: x0/rdi = left array, x1/rsi = right array. Returns 1 in x0/rax when strictly
/// equal, 0 otherwise. Read-only; normalized packed temporaries are freed before return.
pub fn emit_array_strict_eq(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_array_strict_eq_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: array_strict_eq ---");
    emitter.label_global("__rt_array_strict_eq");
    // frame slots (sp-relative): 0=a_h 8=b_h 16=a_tmp 24=b_tmp 32=cur_a 40=cur_b 48=result
    // 56=ka_lo 64=ka_hi 72=va_lo 80=va_hi 88=va_tag 96=vb_lo 104=vb_hi 112=vb_tag; 128=x29/x30
    emitter.instruction("sub sp, sp, #144");                                    // allocate the compare stack frame
    emitter.instruction("stp x29, x30, [sp, #128]");                            // save frame pointer and return address
    emitter.instruction("add x29, sp, #128");                                   // establish the frame
    emitter.instruction("cmp x0, x1");                                          // fast path: same array pointer is strictly equal
    emitter.instruction("b.eq __rt_array_strict_eq_true_quick");
    emitter.instruction("str x0, [sp, #0]");                                    // a_h = left (tentative)
    emitter.instruction("str x1, [sp, #8]");                                    // b_h = right (tentative)
    emitter.instruction("str xzr, [sp, #16]");                                  // a_tmp = 0
    emitter.instruction("str xzr, [sp, #24]");                                  // b_tmp = 0
    // normalize left if packed/indexed (heap kind 2)
    emitter.instruction("ldr x9, [x0, #-8]");                                   // left heap-kind header word
    emitter.instruction("and x9, x9, #0xff");                                   // isolate the low-byte heap kind
    emitter.instruction("cmp x9, #2");                                          // is left a packed indexed array?
    emitter.instruction("b.ne __rt_array_strict_eq_a_norm_done");
    emitter.instruction("ldr x0, [sp, #0]");                                    // reload left pointer
    emitter.instruction("bl __rt_array_to_hash");                              // convert packed -> owned hash, x0 = temp
    emitter.instruction("str x0, [sp, #0]");                                    // a_h = temp hash
    emitter.instruction("str x0, [sp, #16]");                                   // a_tmp = temp (free later)
    emitter.label("__rt_array_strict_eq_a_norm_done");
    // normalize right if packed/indexed
    emitter.instruction("ldr x0, [sp, #8]");                                    // right pointer
    emitter.instruction("ldr x9, [x0, #-8]");                                   // right heap-kind header word
    emitter.instruction("and x9, x9, #0xff");
    emitter.instruction("cmp x9, #2");
    emitter.instruction("b.ne __rt_array_strict_eq_b_norm_done");
    emitter.instruction("ldr x0, [sp, #8]");
    emitter.instruction("bl __rt_array_to_hash");
    emitter.instruction("str x0, [sp, #8]");                                    // b_h = temp hash
    emitter.instruction("str x0, [sp, #24]");                                   // b_tmp = temp
    emitter.label("__rt_array_strict_eq_b_norm_done");
    // length check
    emitter.instruction("ldr x0, [sp, #0]");
    emitter.instruction("ldr x0, [x0]");                                        // len(a_h) at header offset 0
    emitter.instruction("ldr x1, [sp, #8]");
    emitter.instruction("ldr x1, [x1]");                                        // len(b_h)
    emitter.instruction("cmp x0, x1");
    emitter.instruction("b.ne __rt_array_strict_eq_false");
    emitter.instruction("str xzr, [sp, #32]");                                  // cursor_a = 0
    emitter.instruction("str xzr, [sp, #40]");                                  // cursor_b = 0
    emitter.label("__rt_array_strict_eq_loop");
    emitter.instruction("ldr x0, [sp, #0]");                                    // a_h
    emitter.instruction("ldr x1, [sp, #32]");                                   // cursor_a
    emitter.instruction("bl __rt_hash_iter_next");                             // x0=cur,x1=kp,x2=kl,x3=vlo,x4=vhi,x5=vtag
    emitter.instruction("cmn x0, #1");                                          // returned cursor == -1 -> done (all matched)
    emitter.instruction("b.eq __rt_array_strict_eq_true");
    emitter.instruction("str x0, [sp, #32]");                                   // save next cursor_a
    emitter.instruction("str x1, [sp, #56]");                                   // ka_lo (key ptr / int)
    emitter.instruction("str x2, [sp, #64]");                                   // ka_hi (key len / -1)
    emitter.instruction("str x3, [sp, #72]");                                   // va_lo
    emitter.instruction("str x4, [sp, #80]");                                   // va_hi
    emitter.instruction("str x5, [sp, #88]");                                   // va_tag
    emitter.instruction("ldr x0, [sp, #8]");                                    // b_h
    emitter.instruction("ldr x1, [sp, #40]");                                   // cursor_b
    emitter.instruction("bl __rt_hash_iter_next");                             // x0=cur,x1=kb_lo,x2=kb_hi,x3=vb_lo,x4=vb_hi,x5=vb_tag
    emitter.instruction("str x0, [sp, #40]");                                   // save next cursor_b
    emitter.instruction("str x3, [sp, #96]");                                   // vb_lo
    emitter.instruction("str x4, [sp, #104]");                                  // vb_hi
    emitter.instruction("str x5, [sp, #112]");                                  // vb_tag
    // compare keys: hash_key_eq(x1=ka_lo,x2=ka_hi,x3=kb_lo,x4=kb_hi)
    emitter.instruction("mov x3, x1");                                          // x3 = kb_lo (from iter b, currently in x1)
    emitter.instruction("mov x4, x2");                                          // x4 = kb_hi
    emitter.instruction("ldr x1, [sp, #56]");                                   // x1 = ka_lo
    emitter.instruction("ldr x2, [sp, #64]");                                   // x2 = ka_hi
    emitter.instruction("bl __rt_hash_key_eq");                                 // x0 = 1 if keys equal
    emitter.instruction("cbz x0, __rt_array_strict_eq_false");                  // differing keys -> not equal
    // compare value tags
    emitter.instruction("ldr x0, [sp, #88]");                                   // va_tag
    emitter.instruction("ldr x1, [sp, #112]");                                  // vb_tag
    emitter.instruction("cmp x0, x1");
    emitter.instruction("b.ne __rt_array_strict_eq_false");
    emitter.instruction("cmp x0, #8");                                          // null?
    emitter.instruction("b.eq __rt_array_strict_eq_loop");                      // matching null tags are equal
    emitter.instruction("cmp x0, #1");                                          // string?
    emitter.instruction("b.eq __rt_array_strict_eq_cmp_string");
    emitter.instruction("cmp x0, #4");                                          // indexed-array value?
    emitter.instruction("b.eq __rt_array_strict_eq_cmp_array");
    emitter.instruction("cmp x0, #5");                                          // assoc-array value?
    emitter.instruction("b.eq __rt_array_strict_eq_cmp_array");
    emitter.instruction("cmp x0, #7");                                          // boxed mixed cell (heterogeneous element)?
    emitter.instruction("b.eq __rt_array_strict_eq_cmp_mixed");
    // else: scalar/object -> compare low + high payload words
    emitter.instruction("ldr x0, [sp, #72]");                                   // va_lo
    emitter.instruction("ldr x1, [sp, #96]");                                   // vb_lo
    emitter.instruction("cmp x0, x1");
    emitter.instruction("b.ne __rt_array_strict_eq_false");
    emitter.instruction("ldr x0, [sp, #80]");                                   // va_hi
    emitter.instruction("ldr x1, [sp, #104]");                                  // vb_hi
    emitter.instruction("cmp x0, x1");
    emitter.instruction("b.ne __rt_array_strict_eq_false");
    emitter.instruction("b __rt_array_strict_eq_loop");
    emitter.label("__rt_array_strict_eq_cmp_string");
    emitter.instruction("ldr x1, [sp, #72]");                                   // left string ptr
    emitter.instruction("ldr x2, [sp, #80]");                                   // left string len
    emitter.instruction("ldr x3, [sp, #96]");                                   // right string ptr
    emitter.instruction("ldr x4, [sp, #104]");                                  // right string len
    emitter.instruction("bl __rt_str_eq");
    emitter.instruction("cbz x0, __rt_array_strict_eq_false");
    emitter.instruction("b __rt_array_strict_eq_loop");
    emitter.label("__rt_array_strict_eq_cmp_array");
    emitter.instruction("ldr x0, [sp, #72]");                                   // left nested array ptr
    emitter.instruction("ldr x1, [sp, #96]");                                   // right nested array ptr
    emitter.instruction("bl __rt_array_strict_eq");                            // recurse into nested arrays
    emitter.instruction("cbz x0, __rt_array_strict_eq_false");
    emitter.instruction("b __rt_array_strict_eq_loop");
    emitter.label("__rt_array_strict_eq_cmp_mixed");
    emitter.instruction("ldr x0, [sp, #72]");                                   // va_lo (boxed mixed pointer)
    emitter.instruction("ldr x1, [sp, #96]");                                   // vb_lo (boxed mixed pointer)
    emitter.instruction("bl __rt_mixed_strict_eq");                            // unbox + compare a heterogeneous element
    emitter.instruction("cbz x0, __rt_array_strict_eq_false");
    emitter.instruction("b __rt_array_strict_eq_loop");
    emitter.label("__rt_array_strict_eq_true");
    emitter.instruction("mov x9, #1");
    emitter.instruction("str x9, [sp, #48]");                                   // result = 1
    emitter.instruction("b __rt_array_strict_eq_cleanup");
    emitter.label("__rt_array_strict_eq_false");
    emitter.instruction("str xzr, [sp, #48]");                                  // result = 0
    emitter.label("__rt_array_strict_eq_cleanup");
    emitter.instruction("ldr x0, [sp, #16]");                                   // a_tmp
    emitter.instruction("cbz x0, __rt_array_strict_eq_no_a_tmp");
    emitter.instruction("bl __rt_decref_any");                                 // free the normalized left temp
    emitter.label("__rt_array_strict_eq_no_a_tmp");
    emitter.instruction("ldr x0, [sp, #24]");                                   // b_tmp
    emitter.instruction("cbz x0, __rt_array_strict_eq_no_b_tmp");
    emitter.instruction("bl __rt_decref_any");
    emitter.label("__rt_array_strict_eq_no_b_tmp");
    emitter.instruction("ldr x0, [sp, #48]");                                   // result
    emitter.instruction("ldp x29, x30, [sp, #128]");
    emitter.instruction("add sp, sp, #144");
    emitter.instruction("ret");
    emitter.label("__rt_array_strict_eq_true_quick");
    emitter.instruction("mov x0, #1");                                          // identical pointers -> equal, no temps allocated
    emitter.instruction("ldp x29, x30, [sp, #128]");
    emitter.instruction("add sp, sp, #144");
    emitter.instruction("ret");
}

/// x86_64 Linux implementation of `__rt_array_strict_eq`.
/// Input: rdi = left array, rsi = right array. Returns boolean in rax.
fn emit_array_strict_eq_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: array_strict_eq ---");
    emitter.label_global("__rt_array_strict_eq");
    // rbp frame slots: -8=a_h -16=b_h -24=a_tmp -32=b_tmp -40=cur_a -48=cur_b -56=result
    // -64=ka_lo -72=ka_hi -80=va_lo -88=va_hi -96=va_tag -104=vb_lo -112=vb_hi -120=vb_tag
    emitter.instruction("push rbp");
    emitter.instruction("mov rbp, rsp");
    emitter.instruction("sub rsp, 128");
    emitter.instruction("cmp rdi, rsi");                                        // fast path: identical pointers
    emitter.instruction("je __rt_array_strict_eq_true_quick");
    emitter.instruction("mov QWORD PTR [rbp - 8], rdi");                        // a_h = left (tentative)
    emitter.instruction("mov QWORD PTR [rbp - 16], rsi");                       // b_h = right (tentative)
    emitter.instruction("mov QWORD PTR [rbp - 24], 0");                         // a_tmp = 0
    emitter.instruction("mov QWORD PTR [rbp - 32], 0");                         // b_tmp = 0
    // normalize left if packed (heap kind 2)
    emitter.instruction("mov rax, QWORD PTR [rdi - 8]");                        // left heap-kind header word
    emitter.instruction("and rax, 0xff");
    emitter.instruction("cmp rax, 2");
    emitter.instruction("jne __rt_array_strict_eq_a_norm_done");
    emitter.instruction("mov rdi, QWORD PTR [rbp - 8]");
    abi::emit_call_label(emitter, "__rt_array_to_hash");                        // packed -> owned hash in rax
    emitter.instruction("mov QWORD PTR [rbp - 8], rax");
    emitter.instruction("mov QWORD PTR [rbp - 24], rax");
    emitter.label("__rt_array_strict_eq_a_norm_done");
    // normalize right if packed
    emitter.instruction("mov rdi, QWORD PTR [rbp - 16]");
    emitter.instruction("mov rax, QWORD PTR [rdi - 8]");
    emitter.instruction("and rax, 0xff");
    emitter.instruction("cmp rax, 2");
    emitter.instruction("jne __rt_array_strict_eq_b_norm_done");
    emitter.instruction("mov rdi, QWORD PTR [rbp - 16]");
    abi::emit_call_label(emitter, "__rt_array_to_hash");
    emitter.instruction("mov QWORD PTR [rbp - 16], rax");
    emitter.instruction("mov QWORD PTR [rbp - 32], rax");
    emitter.label("__rt_array_strict_eq_b_norm_done");
    // length check
    emitter.instruction("mov rax, QWORD PTR [rbp - 8]");
    emitter.instruction("mov rax, QWORD PTR [rax]");                            // len(a_h)
    emitter.instruction("mov rcx, QWORD PTR [rbp - 16]");
    emitter.instruction("mov rcx, QWORD PTR [rcx]");                            // len(b_h)
    emitter.instruction("cmp rax, rcx");
    emitter.instruction("jne __rt_array_strict_eq_false");
    emitter.instruction("mov QWORD PTR [rbp - 40], 0");                         // cursor_a = 0
    emitter.instruction("mov QWORD PTR [rbp - 48], 0");                         // cursor_b = 0
    emitter.label("__rt_array_strict_eq_loop");
    emitter.instruction("mov rdi, QWORD PTR [rbp - 8]");                        // a_h
    emitter.instruction("mov rsi, QWORD PTR [rbp - 40]");                       // cursor_a
    abi::emit_call_label(emitter, "__rt_hash_iter_next");                       // rax=cur,rdi=kp,rdx=kl,rcx=vlo,r8=vhi,r9=vtag
    emitter.instruction("cmp rax, -1");                                         // done?
    emitter.instruction("je __rt_array_strict_eq_true");
    emitter.instruction("mov QWORD PTR [rbp - 40], rax");                       // save next cursor_a
    emitter.instruction("mov QWORD PTR [rbp - 64], rdi");                       // ka_lo
    emitter.instruction("mov QWORD PTR [rbp - 72], rdx");                       // ka_hi
    emitter.instruction("mov QWORD PTR [rbp - 80], rcx");                       // va_lo
    emitter.instruction("mov QWORD PTR [rbp - 88], r8");                        // va_hi
    emitter.instruction("mov QWORD PTR [rbp - 96], r9");                        // va_tag
    emitter.instruction("mov rdi, QWORD PTR [rbp - 16]");                       // b_h
    emitter.instruction("mov rsi, QWORD PTR [rbp - 48]");                       // cursor_b
    abi::emit_call_label(emitter, "__rt_hash_iter_next");                       // rax=cur,rdi=kb_lo,rdx=kb_hi,rcx=vb_lo,r8=vb_hi,r9=vb_tag
    emitter.instruction("mov QWORD PTR [rbp - 48], rax");                       // save next cursor_b
    emitter.instruction("mov QWORD PTR [rbp - 104], rcx");                      // vb_lo
    emitter.instruction("mov QWORD PTR [rbp - 112], r8");                       // vb_hi
    emitter.instruction("mov QWORD PTR [rbp - 120], r9");                       // vb_tag
    // compare keys: hash_key_eq(rdi=ka_lo,rsi=ka_hi,rdx=kb_lo,rcx=kb_hi)
    emitter.instruction("mov rcx, rdx");                                        // rcx = kb_hi
    emitter.instruction("mov rdx, rdi");                                        // rdx = kb_lo (iter-b key ptr)
    emitter.instruction("mov rdi, QWORD PTR [rbp - 64]");                       // rdi = ka_lo
    emitter.instruction("mov rsi, QWORD PTR [rbp - 72]");                       // rsi = ka_hi
    abi::emit_call_label(emitter, "__rt_hash_key_eq");                          // rax = 1 if keys equal
    emitter.instruction("test rax, rax");
    emitter.instruction("je __rt_array_strict_eq_false");
    // compare value tags
    emitter.instruction("mov rax, QWORD PTR [rbp - 96]");                       // va_tag
    emitter.instruction("mov rcx, QWORD PTR [rbp - 120]");                      // vb_tag
    emitter.instruction("cmp rax, rcx");
    emitter.instruction("jne __rt_array_strict_eq_false");
    emitter.instruction("cmp rax, 8");                                          // null?
    emitter.instruction("je __rt_array_strict_eq_loop");
    emitter.instruction("cmp rax, 1");                                          // string?
    emitter.instruction("je __rt_array_strict_eq_cmp_string");
    emitter.instruction("cmp rax, 4");                                          // indexed-array value?
    emitter.instruction("je __rt_array_strict_eq_cmp_array");
    emitter.instruction("cmp rax, 5");                                          // assoc-array value?
    emitter.instruction("je __rt_array_strict_eq_cmp_array");
    emitter.instruction("cmp rax, 7");                                          // boxed mixed cell (heterogeneous element)?
    emitter.instruction("je __rt_array_strict_eq_cmp_mixed");
    // else scalar/object -> compare low + high words
    emitter.instruction("mov rax, QWORD PTR [rbp - 80]");                       // va_lo
    emitter.instruction("cmp rax, QWORD PTR [rbp - 104]");                      // vb_lo
    emitter.instruction("jne __rt_array_strict_eq_false");
    emitter.instruction("mov rax, QWORD PTR [rbp - 88]");                       // va_hi
    emitter.instruction("cmp rax, QWORD PTR [rbp - 112]");                      // vb_hi
    emitter.instruction("jne __rt_array_strict_eq_false");
    emitter.instruction("jmp __rt_array_strict_eq_loop");
    emitter.label("__rt_array_strict_eq_cmp_string");
    emitter.instruction("mov rdi, QWORD PTR [rbp - 80]");                       // left string ptr
    emitter.instruction("mov rsi, QWORD PTR [rbp - 88]");                       // left string len
    emitter.instruction("mov rdx, QWORD PTR [rbp - 104]");                      // right string ptr
    emitter.instruction("mov rcx, QWORD PTR [rbp - 112]");                      // right string len
    abi::emit_call_label(emitter, "__rt_str_eq");
    emitter.instruction("test rax, rax");
    emitter.instruction("je __rt_array_strict_eq_false");
    emitter.instruction("jmp __rt_array_strict_eq_loop");
    emitter.label("__rt_array_strict_eq_cmp_array");
    emitter.instruction("mov rdi, QWORD PTR [rbp - 80]");                       // left nested array ptr
    emitter.instruction("mov rsi, QWORD PTR [rbp - 104]");                      // right nested array ptr
    abi::emit_call_label(emitter, "__rt_array_strict_eq");                      // recurse
    emitter.instruction("test rax, rax");
    emitter.instruction("je __rt_array_strict_eq_false");
    emitter.instruction("jmp __rt_array_strict_eq_loop");
    emitter.label("__rt_array_strict_eq_cmp_mixed");
    emitter.instruction("mov rdi, QWORD PTR [rbp - 80]");                       // va_lo (boxed mixed pointer)
    emitter.instruction("mov rsi, QWORD PTR [rbp - 104]");                      // vb_lo (boxed mixed pointer)
    abi::emit_call_label(emitter, "__rt_mixed_strict_eq");                      // unbox + compare a heterogeneous element
    emitter.instruction("test rax, rax");
    emitter.instruction("je __rt_array_strict_eq_false");
    emitter.instruction("jmp __rt_array_strict_eq_loop");
    emitter.label("__rt_array_strict_eq_true");
    emitter.instruction("mov QWORD PTR [rbp - 56], 1");                         // result = 1
    emitter.instruction("jmp __rt_array_strict_eq_cleanup");
    emitter.label("__rt_array_strict_eq_false");
    emitter.instruction("mov QWORD PTR [rbp - 56], 0");                         // result = 0
    emitter.label("__rt_array_strict_eq_cleanup");
    emitter.instruction("mov rdi, QWORD PTR [rbp - 24]");                       // a_tmp
    emitter.instruction("test rdi, rdi");
    emitter.instruction("jz __rt_array_strict_eq_no_a_tmp");
    abi::emit_call_label(emitter, "__rt_decref_any");                          // free normalized left temp
    emitter.label("__rt_array_strict_eq_no_a_tmp");
    emitter.instruction("mov rdi, QWORD PTR [rbp - 32]");                       // b_tmp
    emitter.instruction("test rdi, rdi");
    emitter.instruction("jz __rt_array_strict_eq_no_b_tmp");
    abi::emit_call_label(emitter, "__rt_decref_any");
    emitter.label("__rt_array_strict_eq_no_b_tmp");
    emitter.instruction("mov rax, QWORD PTR [rbp - 56]");                       // result
    emitter.instruction("add rsp, 128");
    emitter.instruction("pop rbp");
    emitter.instruction("ret");
    emitter.label("__rt_array_strict_eq_true_quick");
    emitter.instruction("mov rax, 1");                                          // identical pointers -> equal
    emitter.instruction("add rsp, 128");
    emitter.instruction("pop rbp");
    emitter.instruction("ret");
}
