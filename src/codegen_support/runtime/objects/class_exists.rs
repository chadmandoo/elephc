//! Purpose:
//! Emits the `__rt_class_exists` runtime helper: a boolean existence check over the
//! `_classes_by_name` data table (the same table `__rt_new_by_name` scans). Used by the AOT
//! lowering of `class_exists($dynamic)` when the argument is not a compile-time-constant string,
//! so a runtime class name can be tested against the registered user classes.
//!
//! Called from:
//! - `crate::codegen_support::runtime::emitters::emit_runtime()` via `crate::codegen_support::runtime::objects`.
//!
//! Key details:
//! - Each `_classes_by_name` entry is 32 bytes: name_ptr (8) + name_len (8) + class_id (8) +
//!   obj_size (8). A linear scan compares lengths first, then delegates to `__rt_strcasecmp` for
//!   PHP-style case-insensitive class lookup — identical to `__rt_new_by_name`, but returns a
//!   boolean (1 found / 0 missing) instead of allocating an instance.
//! - Input string ABI matches the rest of the runtime: pointer in the string-result register,
//!   length in the string-length register. Output: 1 (found) or 0 (not found).

use crate::codegen_support::emit::Emitter;
use crate::codegen_support::platform::Arch;
use crate::codegen_support::abi;

/// Emits the `__rt_class_exists` runtime helper.
///
/// ## ARM64 ABI (default)
/// - Input: `x1` = class-name pointer, `x2` = class-name length
/// - Output: `x0` = 1 when a registered class matches (case-insensitively), else 0
/// - Clobbers: `x9`-`x13` and whatever `__rt_strcasecmp` clobbers (frame-preserved)
///
/// ## x86_64 Linux ABI
/// - Input: `rax` = class-name pointer, `rdx` = class-name length
/// - Output: `rax` = 1 when a registered class matches, else 0
pub fn emit_class_exists(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_class_exists_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: class_exists ---");
    emitter.label_global("__rt_class_exists");

    // Frame (48 bytes): [0..16) saved x29/x30, [16) name_ptr, [24) name_len,
    //   [32) entry cursor, [40) entry index saved across __rt_strcasecmp.
    emitter.instruction("sub sp, sp, #48");                                     // helper frame
    emitter.instruction("stp x29, x30, [sp, #0]");                              // save frame pointer and return address
    emitter.instruction("mov x29, sp");                                         // establish the helper frame pointer
    emitter.instruction("str x1, [sp, #16]");                                   // save the name pointer
    emitter.instruction("str x2, [sp, #24]");                                   // save the name length

    // -- load the lookup-table cursor + bound --
    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // x9 = entry count
    emitter.instruction("cbz x9, __rt_ce_miss");                                // empty registry → no match
    abi::emit_symbol_address(emitter, "x10", "_classes_by_name");
    emitter.instruction("str x10, [sp, #32]");                                  // initialise the entry cursor
    emitter.instruction("mov x11, #0");                                         // entry index

    emitter.label("__rt_ce_loop");
    emitter.instruction("cmp x11, x9");                                         // scanned every registered class?
    emitter.instruction("b.ge __rt_ce_miss");                                   // exhausted the table without a match
    emitter.instruction("ldr x10, [sp, #32]");                                  // reload the entry cursor
    emitter.instruction("ldr x13, [x10, #8]");                                  // stored name length
    emitter.instruction("ldr x2, [sp, #24]");                                   // reload the input name length
    emitter.instruction("cmp x13, x2");                                         // length mismatch → skip
    emitter.instruction("b.ne __rt_ce_skip");                                   // skip this class when the name lengths differ
    emitter.instruction("str x11, [sp, #40]");                                  // save the entry index across the string helper
    emitter.instruction("ldr x1, [sp, #16]");                                   // reload the input name pointer
    emitter.instruction("ldr x2, [sp, #24]");                                   // reload the input name length
    emitter.instruction("ldr x3, [x10]");                                       // stored class-name pointer
    emitter.instruction("mov x4, x13");                                         // stored class-name length
    emitter.instruction("bl __rt_strcasecmp");                                  // compare class names case-insensitively
    emitter.instruction("ldr x11, [sp, #40]");                                  // restore the entry index after the string helper
    emitter.instruction("cmp x0, #0");                                          // did the class names match case-insensitively?
    emitter.instruction("b.eq __rt_ce_hit");                                    // full match: the class exists
    emitter.instruction("b __rt_ce_skip");                                      // mismatch: try the next entry

    emitter.label("__rt_ce_skip");
    emitter.instruction("ldr x10, [sp, #32]");                                  // reload the entry cursor
    emitter.instruction("add x10, x10, #32");                                   // advance to the next 32-byte entry
    emitter.instruction("str x10, [sp, #32]");                                  // persist the cursor
    emitter.instruction("add x11, x11, #1");                                    // advance the entry index
    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // reload the count (lost across the table walk)
    emitter.instruction("b __rt_ce_loop");                                      // continue scanning

    emitter.label("__rt_ce_hit");
    emitter.instruction("mov x0, #1");                                          // the class name is registered
    emitter.instruction("ldp x29, x30, [sp, #0]");                              // restore frame pointer and return address
    emitter.instruction("add sp, sp, #48");                                     // release the frame
    emitter.instruction("ret");                                                 // return true

    emitter.label("__rt_ce_miss");
    emitter.instruction("mov x0, #0");                                          // no class with that name
    emitter.instruction("ldp x29, x30, [sp, #0]");                              // restore frame pointer and return address
    emitter.instruction("add sp, sp, #48");                                     // release the frame
    emitter.instruction("ret");                                                 // return false
}

/// Emits the Linux x86_64 variant of `__rt_class_exists`.
fn emit_class_exists_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: class_exists ---");
    emitter.label_global("__rt_class_exists");

    // Frame (rbp-relative): [-8) name_ptr [-16) name_len [-24) entry cursor [-32) entry index stash.
    emitter.instruction("push rbp");                                            // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish the helper frame pointer
    emitter.instruction("sub rsp, 32");                                         // helper frame
    emitter.instruction("mov QWORD PTR [rbp - 8], rax");                        // save the name pointer (elephc string ABI: rax)
    emitter.instruction("mov QWORD PTR [rbp - 16], rdx");                       // save the name length (elephc string ABI: rdx)

    // -- load the lookup-table cursor + bound --
    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0); // r9 = entry count
    emitter.instruction("test r9, r9");                                         // empty registry?
    emitter.instruction("jz __rt_ce_miss_x86");                                 // no entries → no match
    abi::emit_symbol_address(emitter, "r10", "_classes_by_name"); // r10 = table base
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // entry cursor
    emitter.instruction("xor r11, r11");                                        // entry index

    emitter.label("__rt_ce_loop_x86");
    emitter.instruction("cmp r11, r9");                                         // scanned every registered class?
    emitter.instruction("jge __rt_ce_miss_x86");                               // exhausted the table without a match
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the entry cursor
    emitter.instruction("mov rcx, QWORD PTR [r10 + 8]");                        // stored name length
    emitter.instruction("mov rdx, QWORD PTR [rbp - 16]");                       // reload the input name length
    emitter.instruction("cmp rcx, rdx");                                        // length mismatch?
    emitter.instruction("jne __rt_ce_skip_x86");                               // skip on length mismatch
    emitter.instruction("mov QWORD PTR [rbp - 32], r11");                       // save the entry index across the string helper
    emitter.instruction("mov rdi, QWORD PTR [rbp - 8]");                        // reload the input name pointer
    emitter.instruction("mov rsi, QWORD PTR [rbp - 16]");                       // reload the input name length
    emitter.instruction("mov rdx, QWORD PTR [r10]");                            // stored class-name pointer
    emitter.instruction("call __rt_strcasecmp");                                // compare class names case-insensitively
    emitter.instruction("mov r11, QWORD PTR [rbp - 32]");                       // restore the entry index after the string helper
    emitter.instruction("test rax, rax");                                       // did the class names match case-insensitively?
    emitter.instruction("je __rt_ce_hit_x86");                                 // full match: the class exists
    emitter.instruction("jmp __rt_ce_skip_x86");                               // mismatch: try the next entry

    emitter.label("__rt_ce_skip_x86");
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the entry cursor
    emitter.instruction("add r10, 32");                                         // advance to the next 32-byte entry
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // persist the cursor
    emitter.instruction("add r11, 1");                                          // advance the entry index
    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0); // reload the count (lost across the table walk)
    emitter.instruction("jmp __rt_ce_loop_x86");                               // continue scanning

    emitter.label("__rt_ce_hit_x86");
    emitter.instruction("mov eax, 1");                                          // the class name is registered
    emitter.instruction("add rsp, 32");                                         // release the frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return true

    emitter.label("__rt_ce_miss_x86");
    emitter.instruction("xor eax, eax");                                        // no class with that name
    emitter.instruction("add rsp, 32");                                         // release the frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return false
}
