//! Purpose:
//! Emits the `__rt_str_spaceship` runtime routine implementing PHP 8's string `<=>`:
//! numeric strings compare by numeric value; non-numeric strings compare byte-for-byte.
//! Returns exactly -1, 0, or 1.
//!
//! Called from:
//! - `crate::codegen::runtime::emitters::emit_runtime()` via `crate::codegen::runtime::strings`.
//! - `crate::codegen_ir::lower_inst::comparisons::lower_spaceship` for `Str <=> Str` operands.
//!
//! Key details:
//! - Mirrors `__rt_str_loose_eq`'s structure and input convention (left ptr/len, right ptr/len):
//!   both operands are parsed via `__rt_str_to_number`; only when BOTH parse as PHP numeric
//!   strings do they compare numerically (`"10" <=> "9"` is 1, not lexicographic -1). The
//!   fallback normalizes `__rt_strcmp`'s lexicographic result to a sign.
//! - PHP numeric strings never parse to NaN, so the ordered float conditions are safe.

use crate::codegen::{abi, emit::Emitter, platform::Arch};

/// str_spaceship: PHP `<=>` for two strings.
/// Input:  AArch64 x1/x2 = left ptr/len, x3/x4 = right ptr/len
///         x86_64  rdi/rsi = left ptr/len, rdx/rcx = right ptr/len
/// Output: -1, 0, or 1 in the integer result register.
pub fn emit_str_spaceship(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_str_spaceship_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: str_spaceship ---");
    emitter.label_global("__rt_str_spaceship");

    emitter.instruction("sub sp, sp, #80");                                     // allocate helper slots for both strings and parsed numeric state
    emitter.instruction("stp x29, x30, [sp, #64]");                             // save frame pointer and return address
    emitter.instruction("add x29, sp, #64");                                    // establish a stable helper frame pointer
    emitter.instruction("stp x1, x2, [sp, #0]");                                // save the left string pointer and length
    emitter.instruction("stp x3, x4, [sp, #16]");                               // save the right string pointer and length

    emitter.instruction("bl __rt_str_to_number");                               // parse the left string as a PHP numeric string
    emitter.instruction("str x0, [sp, #32]");                                   // save whether the left string parsed as numeric
    emitter.instruction("str d0, [sp, #40]");                                   // save the parsed left numeric value
    emitter.instruction("ldp x1, x2, [sp, #16]");                               // reload the right string into the parser input registers
    emitter.instruction("bl __rt_str_to_number");                               // parse the right string as a PHP numeric string
    emitter.instruction("ldr x9, [sp, #32]");                                   // reload the left numeric-string flag
    emitter.instruction("cbz x9, __rt_str_spaceship_bytes");                    // non-numeric left strings compare by bytes
    emitter.instruction("cbz x0, __rt_str_spaceship_bytes");                    // non-numeric right strings compare by bytes
    emitter.instruction("ldr d1, [sp, #40]");                                   // reload the parsed left numeric value
    emitter.instruction("fcmp d1, d0");                                         // order the parsed numeric values
    emitter.instruction("cset x9, gt");                                         // 1 when the left value sorts after the right
    emitter.instruction("cset x10, mi");                                        // 1 when the left value sorts before the right
    emitter.instruction("sub x0, x9, x10");                                     // combine into the -1/0/1 spaceship result
    emitter.instruction("b __rt_str_spaceship_done");                           // skip the byte-comparison fallback

    emitter.label("__rt_str_spaceship_bytes");
    emitter.instruction("ldp x1, x2, [sp, #0]");                                // reload the left string pointer and length
    emitter.instruction("ldp x3, x4, [sp, #16]");                               // reload the right string pointer and length
    emitter.instruction("bl __rt_strcmp");                                      // compare non-numeric strings byte-for-byte
    emitter.instruction("cmp x0, #0");                                          // examine the lexicographic comparison result
    emitter.instruction("cset x9, gt");                                         // 1 when the left string sorts after the right
    emitter.instruction("cset x10, lt");                                        // 1 when the left string sorts before the right
    emitter.instruction("sub x0, x9, x10");                                     // normalize the lexicographic result to -1/0/1

    emitter.label("__rt_str_spaceship_done");
    emitter.instruction("ldp x29, x30, [sp, #64]");                             // restore frame pointer and return address
    emitter.instruction("add sp, sp, #80");                                     // release the helper stack frame
    emitter.instruction("ret");                                                 // return the spaceship result
}

/// Linux x86_64 variant of `__rt_str_spaceship`.
fn emit_str_spaceship_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: str_spaceship ---");
    emitter.label_global("__rt_str_spaceship");

    emitter.instruction("push rbp");                                            // save the caller frame pointer before nested runtime calls
    emitter.instruction("mov rbp, rsp");                                        // establish a stable helper frame pointer
    emitter.instruction("sub rsp, 80");                                         // allocate aligned helper slots for both strings and parsed numeric state
    emitter.instruction("mov QWORD PTR [rbp - 8], rdi");                        // save the left string pointer
    emitter.instruction("mov QWORD PTR [rbp - 16], rsi");                       // save the left string length
    emitter.instruction("mov QWORD PTR [rbp - 24], rdx");                       // save the right string pointer
    emitter.instruction("mov QWORD PTR [rbp - 32], rcx");                       // save the right string length

    emitter.instruction("mov rax, rdi");                                        // move the left string pointer into the parser input register
    emitter.instruction("mov rdx, rsi");                                        // move the left string length into the parser input register
    abi::emit_call_label(emitter, "__rt_str_to_number");                        // parse the left string as a PHP numeric string
    emitter.instruction("mov QWORD PTR [rbp - 40], rax");                       // save whether the left string parsed as numeric
    emitter.instruction("movsd QWORD PTR [rbp - 48], xmm0");                    // save the parsed left numeric value
    emitter.instruction("mov rax, QWORD PTR [rbp - 24]");                       // reload the right string pointer into the parser input register
    emitter.instruction("mov rdx, QWORD PTR [rbp - 32]");                       // reload the right string length into the parser input register
    abi::emit_call_label(emitter, "__rt_str_to_number");                        // parse the right string as a PHP numeric string
    emitter.instruction("cmp QWORD PTR [rbp - 40], 0");                         // did the left string parse as numeric?
    emitter.instruction("je __rt_str_spaceship_bytes_linux_x86_64");            // non-numeric left strings compare by bytes
    emitter.instruction("test rax, rax");                                       // did the right string parse as numeric?
    emitter.instruction("je __rt_str_spaceship_bytes_linux_x86_64");            // non-numeric right strings compare by bytes
    emitter.instruction("movsd xmm1, QWORD PTR [rbp - 48]");                    // reload the parsed left numeric value
    emitter.instruction("ucomisd xmm1, xmm0");                                  // order the parsed numeric values
    emitter.instruction("seta r10b");                                           // 1 when the left value sorts after the right
    emitter.instruction("setb r11b");                                           // 1 when the left value sorts before the right
    emitter.instruction("movzx r10, r10b");                                     // widen the greater-than flag
    emitter.instruction("movzx r11, r11b");                                     // widen the less-than flag
    emitter.instruction("mov rax, r10");                                        // start from the greater-than flag
    emitter.instruction("sub rax, r11");                                        // combine into the -1/0/1 spaceship result
    emitter.instruction("jmp __rt_str_spaceship_done_linux_x86_64");            // skip the byte-comparison fallback

    emitter.label("__rt_str_spaceship_bytes_linux_x86_64");
    emitter.instruction("mov rdi, QWORD PTR [rbp - 8]");                        // reload the left string pointer
    emitter.instruction("mov rsi, QWORD PTR [rbp - 16]");                       // reload the left string length
    emitter.instruction("mov rdx, QWORD PTR [rbp - 24]");                       // reload the right string pointer
    emitter.instruction("mov rcx, QWORD PTR [rbp - 32]");                       // reload the right string length
    abi::emit_call_label(emitter, "__rt_strcmp");                               // compare non-numeric strings byte-for-byte
    emitter.instruction("cmp rax, 0");                                          // examine the lexicographic comparison result
    emitter.instruction("setg r10b");                                           // 1 when the left string sorts after the right
    emitter.instruction("setl r11b");                                           // 1 when the left string sorts before the right
    emitter.instruction("movzx r10, r10b");                                     // widen the greater-than flag
    emitter.instruction("movzx r11, r11b");                                     // widen the less-than flag
    emitter.instruction("mov rax, r10");                                        // start from the greater-than flag
    emitter.instruction("sub rax, r11");                                        // normalize the lexicographic result to -1/0/1

    emitter.label("__rt_str_spaceship_done_linux_x86_64");
    emitter.instruction("add rsp, 80");                                         // release the helper stack frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the spaceship result
}
