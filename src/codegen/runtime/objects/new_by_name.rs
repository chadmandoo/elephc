//! Purpose:
//! Emits the `__rt_new_by_name` runtime helper that allocates an instance
//! of a class identified by a string name (Phase 10 step 2). The lookup
//! consults the `_classes_by_name` data table emitted by
//! `crate::codegen::runtime::data::user::emit_classes_by_name_table`.
//!
//! Called from:
//! - `crate::codegen::runtime::emitters::emit_runtime()` via
//!   `crate::codegen::runtime::objects`.
//! - `crate::codegen::expr::objects::emit_new_dynamic` for
//!   `new $variable()` expressions.
//!
//! Key details:
//! - Each `_classes_by_name` entry is 48 bytes: name_ptr (8) + name_len
//!   (8) + class_id (8) + obj_size (8) + file_ptr (8) + file_len (8). A
//!   linear scan compares lengths first, then delegates to
//!   `__rt_strcasecmp` for PHP-style class lookup.
//! - `__rt_class_file_by_name` (also emitted here) runs the same scan but
//!   returns the declaring-file columns for `ReflectionClass::getFileName()`.
//! - On match: allocates obj_size bytes through `__rt_heap_alloc`, stamps
//!   the uniform heap-kind word (heap kind 4 = object) ahead of the
//!   payload, writes the class id at offset 0, and zeroes the property
//!   region so later property-store paths see clean memory.
//! - On miss: returns 0 (null), which `emit_new_dynamic` boxes as PHP
//!   null (`gettype()` reports "NULL").

use crate::codegen::{abi, emit::Emitter, platform::Arch};

const X86_64_HEAP_MAGIC_HI32: u64 = 0x454C5048;

/// new_by_name: instantiate a class by its textual name.
/// Input:  AArch64 x1 = name pointer, x2 = name length
///         x86_64  rax = name pointer, rdx = name length
/// Output: object pointer, or 0 when no class with that name is known.
pub fn emit_new_by_name(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_new_by_name_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: new_by_name ---");
    emitter.label_global("__rt_new_by_name");

    // Frame (64 bytes): [0..16) saved x29/x30, [16) name_ptr, [24) name_len,
    //   [32) matched class_id, [40) matched obj_size, [48) entry cursor,
    //   [56) entry index saved across __rt_strcasecmp.
    emitter.instruction("sub sp, sp, #64");                                     // helper frame
    emitter.instruction("stp x29, x30, [sp, #0]");                              // save frame pointer and return address
    emitter.instruction("mov x29, sp");                                         // establish the helper frame pointer
    emitter.instruction("str x1, [sp, #16]");                                   // save the name pointer
    emitter.instruction("str x2, [sp, #24]");                                   // save the name length

    // -- load the lookup-table cursor + bound --
    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // x9 = entry count
    emitter.instruction("cbz x9, __rt_nbn_miss");                               // empty registry → no match
    abi::emit_symbol_address(emitter, "x10", "_classes_by_name");
    emitter.instruction("str x10, [sp, #48]");                                  // initialise the entry cursor
    emitter.instruction("mov x11, #0");                                         // entry index

    emitter.label("__rt_nbn_loop");
    emitter.instruction("cmp x11, x9");                                         // scanned every registered class?
    emitter.instruction("b.ge __rt_nbn_miss");                                  // exhausted the table without a match
    emitter.instruction("ldr x10, [sp, #48]");                                  // reload the entry cursor
    emitter.instruction("ldr x13, [x10, #8]");                                  // stored name length
    emitter.instruction("ldr x2, [sp, #24]");                                   // reload the input name length
    emitter.instruction("cmp x13, x2");                                         // length mismatch → skip
    emitter.instruction("b.ne __rt_nbn_skip");                                  // skip this class when the name lengths differ
    emitter.instruction("str x11, [sp, #56]");                                  // save the entry index across the string helper
    emitter.instruction("ldr x1, [sp, #16]");                                   // reload the input name pointer
    emitter.instruction("ldr x2, [sp, #24]");                                   // reload the input name length
    emitter.instruction("ldr x3, [x10]");                                       // stored class-name pointer
    emitter.instruction("mov x4, x13");                                         // stored class-name length
    emitter.instruction("bl __rt_strcasecmp");                                  // compare class names case-insensitively
    emitter.instruction("ldr x11, [sp, #56]");                                  // restore the entry index after the string helper
    emitter.instruction("cmp x0, #0");                                          // did the class names match case-insensitively?
    emitter.instruction("b.eq __rt_nbn_match");                                 // full match: allocate the object
    emitter.instruction("b __rt_nbn_skip");                                     // mismatch: try the next entry

    emitter.label("__rt_nbn_skip");
    emitter.instruction("ldr x10, [sp, #48]");                                  // reload the entry cursor
    emitter.instruction("add x10, x10, #48");                                   // advance to the next 48-byte entry
    emitter.instruction("str x10, [sp, #48]");                                  // persist the cursor
    emitter.instruction("add x11, x11, #1");                                    // advance the entry index
    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // reload the count (lost across the table walk)
    emitter.instruction("b __rt_nbn_loop");                                     // continue scanning

    emitter.label("__rt_nbn_match");
    emitter.instruction("ldr x10, [sp, #48]");                                  // reload the matched entry cursor
    emitter.instruction("ldr x12, [x10, #16]");                                 // class_id
    emitter.instruction("ldr x13, [x10, #24]");                                 // obj_size
    emitter.instruction("str x12, [sp, #32]");                                  // save class_id across the heap call
    emitter.instruction("str x13, [sp, #40]");                                  // save obj_size across the heap call

    // -- allocate the object payload --
    emitter.instruction("mov x0, x13");                                         // allocation size
    emitter.instruction("bl __rt_heap_alloc");                                  // x0 = object pointer
    emitter.instruction("mov x9, #4");                                          // heap kind 4 = object instance
    emitter.instruction("str x9, [x0, #-8]");                                   // stamp the uniform heap header
    emitter.instruction("ldr x12, [sp, #32]");                                  // reload class_id
    emitter.instruction("str x12, [x0]");                                       // class_id at offset 0

    // -- zero the property region [obj+8 .. obj+obj_size) --
    emitter.instruction("ldr x13, [sp, #40]");                                  // obj_size
    emitter.instruction("mov x14, #8");                                         // start past the class_id header
    emitter.label("__rt_nbn_zero");
    emitter.instruction("cmp x14, x13");                                        // every byte zeroed?
    emitter.instruction("b.ge __rt_nbn_done");                                  // property region cleared
    emitter.instruction("str xzr, [x0, x14]");                                  // 8-byte zero store
    emitter.instruction("add x14, x14, #8");                                    // advance the zero cursor
    emitter.instruction("b __rt_nbn_zero");                                     // continue zeroing

    emitter.label("__rt_nbn_done");
    // -- run the per-class property-default thunk, if this class has one --
    emitter.instruction("ldr x12, [sp, #32]");                                  // reload the matched class_id
    abi::emit_symbol_address(emitter, "x10", "_class_propinit_ptrs");
    emitter.instruction("ldr x10, [x10, x12, lsl #3]");                         // _class_propinit_ptrs[class_id] (0 = no defaults)
    emitter.instruction("cbz x10, __rt_nbn_no_propinit");                       // class has no property defaults: skip
    emitter.instruction("str x0, [sp, #40]");                                   // save the object across the thunk (obj_size slot is free now)
    emitter.instruction("blr x10");                                             // _class_propinit_<id>(this = object in x0)
    emitter.instruction("ldr x0, [sp, #40]");                                   // restore the object pointer (the thunk may clobber x0)
    emitter.label("__rt_nbn_no_propinit");
    emitter.instruction("ldp x29, x30, [sp, #0]");                              // restore frame pointer and return address
    emitter.instruction("add sp, sp, #64");                                     // release the frame
    emitter.instruction("ret");                                                 // return the object pointer

    emitter.label("__rt_nbn_miss");
    emitter.instruction("mov x0, #0");                                          // no class with that name
    emitter.instruction("ldp x29, x30, [sp, #0]");                              // restore frame pointer and return address
    emitter.instruction("add sp, sp, #64");                                     // release the frame
    emitter.instruction("ret");                                                 // return null
}

/// Emits the Linux x86_64 object runtime helper for new by name.
fn emit_new_by_name_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: new_by_name ---");
    emitter.label_global("__rt_new_by_name");

    // Frame (rbp-relative): [-8) name_ptr [-16) name_len [-24) entry cursor
    //   [-32) class_id stash [-40) obj_size stash [-48) entry index stash.
    emitter.instruction("push rbp");                                            // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish the helper frame pointer
    emitter.instruction("sub rsp, 48");                                         // helper frame
    emitter.instruction("mov QWORD PTR [rbp - 8], rax");                        // save the name pointer (elephc string ABI: rax)
    emitter.instruction("mov QWORD PTR [rbp - 16], rdx");                       // save the name length (elephc string ABI: rdx)

    // -- load the lookup-table cursor + bound --
    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0);   // r9 = entry count
    emitter.instruction("test r9, r9");                                         // empty registry?
    emitter.instruction("jz __rt_nbn_miss_x86");                                // no entries → no match
    abi::emit_symbol_address(emitter, "r10", "_classes_by_name");               // r10 = table base
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // entry cursor
    emitter.instruction("xor r11, r11");                                        // entry index

    emitter.label("__rt_nbn_loop_x86");
    emitter.instruction("cmp r11, r9");                                         // scanned every registered class?
    emitter.instruction("jge __rt_nbn_miss_x86");                               // exhausted the table without a match
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the entry cursor
    emitter.instruction("mov rcx, QWORD PTR [r10 + 8]");                        // stored name length
    emitter.instruction("mov rdx, QWORD PTR [rbp - 16]");                       // reload the input name length
    emitter.instruction("cmp rcx, rdx");                                        // length mismatch?
    emitter.instruction("jne __rt_nbn_skip_x86");                               // skip on length mismatch
    emitter.instruction("mov QWORD PTR [rbp - 48], r11");                       // save the entry index across the string helper
    emitter.instruction("mov rdi, QWORD PTR [rbp - 8]");                        // reload the input name pointer
    emitter.instruction("mov rsi, QWORD PTR [rbp - 16]");                       // reload the input name length
    emitter.instruction("mov rdx, QWORD PTR [r10]");                            // stored class-name pointer
    emitter.instruction("call __rt_strcasecmp");                                // compare class names case-insensitively
    emitter.instruction("mov r11, QWORD PTR [rbp - 48]");                       // restore the entry index after the string helper
    emitter.instruction("test rax, rax");                                       // did the class names match case-insensitively?
    emitter.instruction("je __rt_nbn_match_x86");                               // full match: allocate the object
    emitter.instruction("jmp __rt_nbn_skip_x86");                               // mismatch: try the next entry

    emitter.label("__rt_nbn_skip_x86");
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the entry cursor
    emitter.instruction("add r10, 48");                                         // advance to the next 48-byte entry
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // persist the cursor
    emitter.instruction("add r11, 1");                                          // advance the entry index
    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0);   // reload the count (lost across the table walk)
    emitter.instruction("jmp __rt_nbn_loop_x86");                               // continue scanning

    emitter.label("__rt_nbn_match_x86");
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the matched entry cursor
    emitter.instruction("mov rcx, QWORD PTR [r10 + 16]");                       // class_id
    emitter.instruction("mov rdx, QWORD PTR [r10 + 24]");                       // obj_size
    emitter.instruction("mov QWORD PTR [rbp - 32], rcx");                       // stash class_id
    emitter.instruction("mov QWORD PTR [rbp - 40], rdx");                       // stash obj_size

    // -- allocate the object payload --
    emitter.instruction("mov rax, rdx");                                        // allocation size
    emitter.instruction("call __rt_heap_alloc");                                // rax = object pointer
    emitter.instruction(&format!("mov r10, 0x{:x}", (X86_64_HEAP_MAGIC_HI32 << 32) | 4)); // object heap-kind word with the x86_64 marker
    emitter.instruction("mov QWORD PTR [rax - 8], r10");                        // stamp the uniform heap header
    emitter.instruction("mov rcx, QWORD PTR [rbp - 32]");                       // reload class_id
    emitter.instruction("mov QWORD PTR [rax], rcx");                            // class_id at offset 0

    // -- zero the property region [obj+8 .. obj+obj_size) --
    emitter.instruction("mov rdx, QWORD PTR [rbp - 40]");                       // obj_size
    emitter.instruction("mov rcx, 8");                                          // start past the class_id header
    emitter.label("__rt_nbn_zero_x86");
    emitter.instruction("cmp rcx, rdx");                                        // every byte zeroed?
    emitter.instruction("jge __rt_nbn_done_x86");                               // property region cleared
    emitter.instruction("mov QWORD PTR [rax + rcx], 0");                        // 8-byte zero store
    emitter.instruction("add rcx, 8");                                          // advance the zero cursor
    emitter.instruction("jmp __rt_nbn_zero_x86");                               // continue zeroing

    emitter.label("__rt_nbn_done_x86");
    // -- run the per-class property-default thunk, if this class has one --
    emitter.instruction("mov rcx, QWORD PTR [rbp - 32]");                       // reload the matched class_id
    abi::emit_symbol_address(emitter, "r10", "_class_propinit_ptrs");           // property-init thunk table base
    emitter.instruction("mov r10, QWORD PTR [r10 + rcx*8]");                    // _class_propinit_ptrs[class_id] (0 = no defaults)
    emitter.instruction("test r10, r10");                                       // does this class have a property-init thunk?
    emitter.instruction("jz __rt_nbn_no_propinit_x86");                         // class has no property defaults: skip
    emitter.instruction("mov QWORD PTR [rbp - 40], rax");                       // save the object across the thunk (obj_size slot is free now)
    emitter.instruction("mov rdi, rax");                                        // this = object (first SysV argument)
    emitter.instruction("call r10");                                            // _class_propinit_<id>(this)
    emitter.instruction("mov rax, QWORD PTR [rbp - 40]");                       // restore the object pointer (the thunk may clobber rax)
    emitter.label("__rt_nbn_no_propinit_x86");
    emitter.instruction("add rsp, 48");                                         // release the frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the object pointer

    emitter.label("__rt_nbn_miss_x86");
    emitter.instruction("xor eax, eax");                                        // no class with that name
    emitter.instruction("add rsp, 48");                                         // release the frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return null
}

/// class_file_by_name: look up a class's declaring-file path by class name.
/// Runs the same case-insensitive `_classes_by_name` scan as `__rt_new_by_name`
/// but returns the file columns ([32..48) of the matched 48-byte entry).
/// Input:  AArch64 x1 = name pointer, x2 = name length
///         x86_64  rax = name pointer, rdx = name length
/// Output: AArch64 x0 = file pointer, x1 = file length (0/0 on miss)
///         x86_64  rax = file pointer, rdx = file length (0/0 on miss)
pub fn emit_class_file_by_name(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_class_file_by_name_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: class_file_by_name ---");
    emitter.label_global("__rt_class_file_by_name");

    // Frame (48 bytes): [0..16) saved x29/x30, [16) name_ptr, [24) name_len,
    //   [32) entry cursor, [40) entry index saved across __rt_strcasecmp.
    emitter.instruction("sub sp, sp, #48");                                     // helper frame
    emitter.instruction("stp x29, x30, [sp, #0]");                              // save frame pointer and return address
    emitter.instruction("mov x29, sp");                                         // establish the helper frame pointer
    emitter.instruction("str x1, [sp, #16]");                                   // save the name pointer
    emitter.instruction("str x2, [sp, #24]");                                   // save the name length

    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // x9 = entry count
    emitter.instruction("cbz x9, __rt_cfbn_miss");                              // empty registry → no match
    abi::emit_symbol_address(emitter, "x10", "_classes_by_name");
    emitter.instruction("str x10, [sp, #32]");                                  // initialise the entry cursor
    emitter.instruction("mov x11, #0");                                         // entry index

    emitter.label("__rt_cfbn_loop");
    emitter.instruction("cmp x11, x9");                                         // scanned every registered class?
    emitter.instruction("b.ge __rt_cfbn_miss");                                 // exhausted the table without a match
    emitter.instruction("ldr x10, [sp, #32]");                                  // reload the entry cursor
    emitter.instruction("ldr x13, [x10, #8]");                                  // stored name length
    emitter.instruction("ldr x2, [sp, #24]");                                   // reload the input name length
    emitter.instruction("cmp x13, x2");                                         // length mismatch → skip
    emitter.instruction("b.ne __rt_cfbn_skip");                                 // skip this class when the name lengths differ
    emitter.instruction("str x11, [sp, #40]");                                  // save the entry index across the string helper
    emitter.instruction("ldr x1, [sp, #16]");                                   // reload the input name pointer
    emitter.instruction("ldr x2, [sp, #24]");                                   // reload the input name length
    emitter.instruction("ldr x3, [x10]");                                       // stored class-name pointer
    emitter.instruction("mov x4, x13");                                         // stored class-name length
    emitter.instruction("bl __rt_strcasecmp");                                  // compare class names case-insensitively
    emitter.instruction("ldr x11, [sp, #40]");                                  // restore the entry index after the string helper
    emitter.instruction("cmp x0, #0");                                          // did the class names match case-insensitively?
    emitter.instruction("b.eq __rt_cfbn_match");                                // full match: return the file columns
    emitter.instruction("b __rt_cfbn_skip");                                    // mismatch: try the next entry

    emitter.label("__rt_cfbn_skip");
    emitter.instruction("ldr x10, [sp, #32]");                                  // reload the entry cursor
    emitter.instruction("add x10, x10, #48");                                   // advance to the next 48-byte entry
    emitter.instruction("str x10, [sp, #32]");                                  // persist the cursor
    emitter.instruction("add x11, x11, #1");                                    // advance the entry index
    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // reload the count (lost across the table walk)
    emitter.instruction("b __rt_cfbn_loop");                                    // continue scanning

    emitter.label("__rt_cfbn_match");
    emitter.instruction("ldr x10, [sp, #32]");                                  // reload the matched entry cursor
    emitter.instruction("ldr x0, [x10, #32]");                                  // file_ptr column
    emitter.instruction("ldr x1, [x10, #40]");                                  // file_len column
    emitter.instruction("ldp x29, x30, [sp, #0]");                              // restore frame pointer and return address
    emitter.instruction("add sp, sp, #48");                                     // release the frame
    emitter.instruction("ret");                                                 // return the file string

    emitter.label("__rt_cfbn_miss");
    emitter.instruction("mov x0, #0");                                          // no class with that name → null pointer
    emitter.instruction("mov x1, #0");                                          // zero length
    emitter.instruction("ldp x29, x30, [sp, #0]");                              // restore frame pointer and return address
    emitter.instruction("add sp, sp, #48");                                     // release the frame
    emitter.instruction("ret");                                                 // return the empty result
}

/// Emits the Linux x86_64 variant of `__rt_class_file_by_name`.
fn emit_class_file_by_name_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: class_file_by_name ---");
    emitter.label_global("__rt_class_file_by_name");

    // Frame (rbp-relative): [-8) name_ptr [-16) name_len [-24) entry cursor
    //   [-32) entry index stash.
    emitter.instruction("push rbp");                                            // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish the helper frame pointer
    emitter.instruction("sub rsp, 32");                                         // helper frame
    emitter.instruction("mov QWORD PTR [rbp - 8], rax");                        // save the name pointer (elephc string ABI: rax)
    emitter.instruction("mov QWORD PTR [rbp - 16], rdx");                       // save the name length (elephc string ABI: rdx)

    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0);   // r9 = entry count
    emitter.instruction("test r9, r9");                                         // empty registry?
    emitter.instruction("jz __rt_cfbn_miss_x86");                               // no entries → no match
    abi::emit_symbol_address(emitter, "r10", "_classes_by_name");               // r10 = table base
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // entry cursor
    emitter.instruction("xor r11, r11");                                        // entry index

    emitter.label("__rt_cfbn_loop_x86");
    emitter.instruction("cmp r11, r9");                                         // scanned every registered class?
    emitter.instruction("jge __rt_cfbn_miss_x86");                              // exhausted the table without a match
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the entry cursor
    emitter.instruction("mov rcx, QWORD PTR [r10 + 8]");                        // stored name length
    emitter.instruction("mov rdx, QWORD PTR [rbp - 16]");                       // reload the input name length
    emitter.instruction("cmp rcx, rdx");                                        // length mismatch?
    emitter.instruction("jne __rt_cfbn_skip_x86");                              // skip on length mismatch
    emitter.instruction("mov QWORD PTR [rbp - 32], r11");                       // save the entry index across the string helper
    emitter.instruction("mov rdi, QWORD PTR [rbp - 8]");                        // reload the input name pointer
    emitter.instruction("mov rsi, QWORD PTR [rbp - 16]");                       // reload the input name length
    emitter.instruction("mov rdx, QWORD PTR [r10]");                            // stored class-name pointer
    emitter.instruction("call __rt_strcasecmp");                                // compare class names case-insensitively
    emitter.instruction("mov r11, QWORD PTR [rbp - 32]");                       // restore the entry index after the string helper
    emitter.instruction("test rax, rax");                                       // did the class names match case-insensitively?
    emitter.instruction("je __rt_cfbn_match_x86");                              // full match: return the file columns
    emitter.instruction("jmp __rt_cfbn_skip_x86");                              // mismatch: try the next entry

    emitter.label("__rt_cfbn_skip_x86");
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the entry cursor
    emitter.instruction("add r10, 48");                                         // advance to the next 48-byte entry
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // persist the cursor
    emitter.instruction("add r11, 1");                                          // advance the entry index
    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0);   // reload the count (lost across the table walk)
    emitter.instruction("jmp __rt_cfbn_loop_x86");                              // continue scanning

    emitter.label("__rt_cfbn_match_x86");
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the matched entry cursor
    emitter.instruction("mov rax, QWORD PTR [r10 + 32]");                       // file_ptr column
    emitter.instruction("mov rdx, QWORD PTR [r10 + 40]");                       // file_len column
    emitter.instruction("add rsp, 32");                                         // release the frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the file string

    emitter.label("__rt_cfbn_miss_x86");
    emitter.instruction("xor eax, eax");                                        // no class with that name → null pointer
    emitter.instruction("xor edx, edx");                                        // zero length
    emitter.instruction("add rsp, 32");                                         // release the frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the empty result
}

/// class_is_abstract: look up a class's abstract flag by class name.
/// Runs the same case-insensitive `_classes_by_name` scan as
/// `__rt_class_file_by_name` but returns the matched entry's flag from the
/// parallel position-indexed `_class_is_abstract` table.
/// Input:  AArch64 x1 = name pointer, x2 = name length
///         x86_64  rax = name pointer, rdx = name length
/// Output: AArch64 x0 = 1 when abstract, 0 when concrete or unknown
///         x86_64  rax = 1 when abstract, 0 when concrete or unknown
pub fn emit_class_is_abstract_by_name(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_class_is_abstract_by_name_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: class_is_abstract ---");
    emitter.label_global("__rt_class_is_abstract");

    // Frame (48 bytes): [0..16) saved x29/x30, [16) name_ptr, [24) name_len,
    //   [32) entry cursor, [40) entry index saved across __rt_strcasecmp.
    emitter.instruction("sub sp, sp, #48");                                     // helper frame
    emitter.instruction("stp x29, x30, [sp, #0]");                              // save frame pointer and return address
    emitter.instruction("mov x29, sp");                                         // establish the helper frame pointer
    emitter.instruction("str x1, [sp, #16]");                                   // save the name pointer
    emitter.instruction("str x2, [sp, #24]");                                   // save the name length

    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // x9 = entry count
    emitter.instruction("cbz x9, __rt_ciab_miss");                              // empty registry → not abstract
    abi::emit_symbol_address(emitter, "x10", "_classes_by_name");
    emitter.instruction("str x10, [sp, #32]");                                  // initialise the entry cursor
    emitter.instruction("mov x11, #0");                                         // entry index

    emitter.label("__rt_ciab_loop");
    emitter.instruction("cmp x11, x9");                                         // scanned every registered class?
    emitter.instruction("b.ge __rt_ciab_miss");                                 // exhausted the table without a match
    emitter.instruction("ldr x10, [sp, #32]");                                  // reload the entry cursor
    emitter.instruction("ldr x13, [x10, #8]");                                  // stored name length
    emitter.instruction("ldr x2, [sp, #24]");                                   // reload the input name length
    emitter.instruction("cmp x13, x2");                                         // length mismatch → skip
    emitter.instruction("b.ne __rt_ciab_skip");                                 // skip this class when the name lengths differ
    emitter.instruction("str x11, [sp, #40]");                                  // save the entry index across the string helper
    emitter.instruction("ldr x1, [sp, #16]");                                   // reload the input name pointer
    emitter.instruction("ldr x2, [sp, #24]");                                   // reload the input name length
    emitter.instruction("ldr x3, [x10]");                                       // stored class-name pointer
    emitter.instruction("mov x4, x13");                                         // stored class-name length
    emitter.instruction("bl __rt_strcasecmp");                                  // compare class names case-insensitively
    emitter.instruction("ldr x11, [sp, #40]");                                  // restore the entry index after the string helper
    emitter.instruction("cmp x0, #0");                                          // did the class names match case-insensitively?
    emitter.instruction("b.eq __rt_ciab_match");                                // full match: return the abstract flag
    emitter.instruction("b __rt_ciab_skip");                                    // mismatch: try the next entry

    emitter.label("__rt_ciab_skip");
    emitter.instruction("ldr x10, [sp, #32]");                                  // reload the entry cursor
    emitter.instruction("add x10, x10, #48");                                   // advance to the next 48-byte entry
    emitter.instruction("str x10, [sp, #32]");                                  // persist the cursor
    emitter.instruction("add x11, x11, #1");                                    // advance the entry index
    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // reload the count (lost across the table walk)
    emitter.instruction("b __rt_ciab_loop");                                    // continue scanning

    emitter.label("__rt_ciab_match");
    abi::emit_symbol_address(emitter, "x10", "_class_is_abstract");
    emitter.instruction("ldr x0, [x10, x11, lsl #3]");                          // abstract flag at the matched entry index
    emitter.instruction("ldp x29, x30, [sp, #0]");                              // restore frame pointer and return address
    emitter.instruction("add sp, sp, #48");                                     // release the frame
    emitter.instruction("ret");                                                 // return the flag

    emitter.label("__rt_ciab_miss");
    emitter.instruction("mov x0, #0");                                          // unknown class → not abstract
    emitter.instruction("ldp x29, x30, [sp, #0]");                              // restore frame pointer and return address
    emitter.instruction("add sp, sp, #48");                                     // release the frame
    emitter.instruction("ret");                                                 // return the zero flag
}

/// Emits the Linux x86_64 variant of `__rt_class_is_abstract`.
fn emit_class_is_abstract_by_name_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: class_is_abstract ---");
    emitter.label_global("__rt_class_is_abstract");

    // Frame (rbp-relative): [-8) name_ptr [-16) name_len [-24) entry cursor
    //   [-32) entry index stash.
    emitter.instruction("push rbp");                                            // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish the helper frame pointer
    emitter.instruction("sub rsp, 32");                                         // helper frame
    emitter.instruction("mov QWORD PTR [rbp - 8], rax");                        // save the name pointer (elephc string ABI: rax)
    emitter.instruction("mov QWORD PTR [rbp - 16], rdx");                       // save the name length (elephc string ABI: rdx)

    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0);   // r9 = entry count
    emitter.instruction("test r9, r9");                                         // empty registry?
    emitter.instruction("jz __rt_ciab_miss_x86");                               // no entries → not abstract
    abi::emit_symbol_address(emitter, "r10", "_classes_by_name");               // r10 = table base
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // entry cursor
    emitter.instruction("xor r11, r11");                                        // entry index

    emitter.label("__rt_ciab_loop_x86");
    emitter.instruction("cmp r11, r9");                                         // scanned every registered class?
    emitter.instruction("jge __rt_ciab_miss_x86");                              // exhausted the table without a match
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the entry cursor
    emitter.instruction("mov rcx, QWORD PTR [r10 + 8]");                        // stored name length
    emitter.instruction("mov rdx, QWORD PTR [rbp - 16]");                       // reload the input name length
    emitter.instruction("cmp rcx, rdx");                                        // length mismatch?
    emitter.instruction("jne __rt_ciab_skip_x86");                              // skip on length mismatch
    emitter.instruction("mov QWORD PTR [rbp - 32], r11");                       // save the entry index across the string helper
    emitter.instruction("mov rdi, QWORD PTR [rbp - 8]");                        // reload the input name pointer
    emitter.instruction("mov rsi, QWORD PTR [rbp - 16]");                       // reload the input name length
    emitter.instruction("mov rdx, QWORD PTR [r10]");                            // stored class-name pointer
    emitter.instruction("call __rt_strcasecmp");                                // compare class names case-insensitively
    emitter.instruction("mov r11, QWORD PTR [rbp - 32]");                       // restore the entry index after the string helper
    emitter.instruction("test rax, rax");                                       // did the class names match case-insensitively?
    emitter.instruction("je __rt_ciab_match_x86");                              // full match: return the abstract flag
    emitter.instruction("jmp __rt_ciab_skip_x86");                              // mismatch: try the next entry

    emitter.label("__rt_ciab_skip_x86");
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the entry cursor
    emitter.instruction("add r10, 48");                                         // advance to the next 48-byte entry
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // persist the cursor
    emitter.instruction("add r11, 1");                                          // advance the entry index
    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0);   // reload the count (lost across the table walk)
    emitter.instruction("jmp __rt_ciab_loop_x86");                              // continue scanning

    emitter.label("__rt_ciab_match_x86");
    abi::emit_symbol_address(emitter, "r10", "_class_is_abstract");             // r10 = flag-table base
    emitter.instruction("mov rax, QWORD PTR [r10 + r11*8]");                    // abstract flag at the matched entry index
    emitter.instruction("add rsp, 32");                                         // release the frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the flag

    emitter.label("__rt_ciab_miss_x86");
    emitter.instruction("xor eax, eax");                                        // unknown class → not abstract
    emitter.instruction("add rsp, 32");                                         // release the frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the zero flag
}

/// class_exists: report whether a class name is registered at runtime.
/// Runs the same case-insensitive `_classes_by_name` scan as
/// `__rt_class_file_by_name`; a match returns 1, a miss returns 0. Codegen
/// retains every declared class when a module contains a dynamic
/// `class_exists()` call, so the table answers exactly like the compile-time
/// fold does for literal names. Builtin classes registered in the table
/// (Exception, ReflectionClass, ...) report 1, matching PHP.
/// Input:  AArch64 x1 = name pointer, x2 = name length
///         x86_64  rax = name pointer, rdx = name length
/// Output: AArch64 x0 = 1 when the class exists, else 0
///         x86_64  rax = 1 when the class exists, else 0
pub fn emit_class_exists_by_name(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_class_exists_by_name_linux_x86_64(emitter);
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

    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // x9 = entry count
    emitter.instruction("cbz x9, __rt_cex_miss");                               // empty registry → no match
    abi::emit_symbol_address(emitter, "x10", "_classes_by_name");
    emitter.instruction("str x10, [sp, #32]");                                  // initialise the entry cursor
    emitter.instruction("mov x11, #0");                                         // entry index

    emitter.label("__rt_cex_loop");
    emitter.instruction("cmp x11, x9");                                         // scanned every registered class?
    emitter.instruction("b.ge __rt_cex_miss");                                  // exhausted the table without a match
    emitter.instruction("ldr x10, [sp, #32]");                                  // reload the entry cursor
    emitter.instruction("ldr x13, [x10, #8]");                                  // stored name length
    emitter.instruction("ldr x2, [sp, #24]");                                   // reload the input name length
    emitter.instruction("cmp x13, x2");                                         // length mismatch → skip
    emitter.instruction("b.ne __rt_cex_skip");                                  // skip this class when the name lengths differ
    emitter.instruction("str x11, [sp, #40]");                                  // save the entry index across the string helper
    emitter.instruction("ldr x1, [sp, #16]");                                   // reload the input name pointer
    emitter.instruction("ldr x2, [sp, #24]");                                   // reload the input name length
    emitter.instruction("ldr x3, [x10]");                                       // stored class-name pointer
    emitter.instruction("mov x4, x13");                                         // stored class-name length
    emitter.instruction("bl __rt_strcasecmp");                                  // compare class names case-insensitively
    emitter.instruction("ldr x11, [sp, #40]");                                  // restore the entry index after the string helper
    emitter.instruction("cmp x0, #0");                                          // did the class names match case-insensitively?
    emitter.instruction("b.eq __rt_cex_match");                                 // full match: the class exists
    emitter.instruction("b __rt_cex_skip");                                     // mismatch: try the next entry

    emitter.label("__rt_cex_skip");
    emitter.instruction("ldr x10, [sp, #32]");                                  // reload the entry cursor
    emitter.instruction("add x10, x10, #48");                                   // advance to the next 48-byte entry
    emitter.instruction("str x10, [sp, #32]");                                  // persist the cursor
    emitter.instruction("add x11, x11, #1");                                    // advance the entry index
    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // reload the count (lost across the table walk)
    emitter.instruction("b __rt_cex_loop");                                     // continue scanning

    emitter.label("__rt_cex_match");
    emitter.instruction("mov x0, #1");                                          // the class exists
    emitter.instruction("ldp x29, x30, [sp, #0]");                              // restore frame pointer and return address
    emitter.instruction("add sp, sp, #48");                                     // release the frame
    emitter.instruction("ret");                                                 // return true

    emitter.label("__rt_cex_miss");
    emitter.instruction("mov x0, #0");                                          // no class with that name
    emitter.instruction("ldp x29, x30, [sp, #0]");                              // restore frame pointer and return address
    emitter.instruction("add sp, sp, #48");                                     // release the frame
    emitter.instruction("ret");                                                 // return false
}

/// Emits the Linux x86_64 variant of `__rt_class_exists`.
fn emit_class_exists_by_name_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: class_exists ---");
    emitter.label_global("__rt_class_exists");

    // Frame (rbp-relative): [-8) name_ptr [-16) name_len [-24) entry cursor
    //   [-32) entry index stash.
    emitter.instruction("push rbp");                                            // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish the helper frame pointer
    emitter.instruction("sub rsp, 32");                                         // helper frame
    emitter.instruction("mov QWORD PTR [rbp - 8], rax");                        // save the name pointer (elephc string ABI: rax)
    emitter.instruction("mov QWORD PTR [rbp - 16], rdx");                       // save the name length (elephc string ABI: rdx)

    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0);   // r9 = entry count
    emitter.instruction("test r9, r9");                                         // empty registry?
    emitter.instruction("jz __rt_cex_miss_x86");                                // no entries → no match
    abi::emit_symbol_address(emitter, "r10", "_classes_by_name");               // r10 = table base
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // entry cursor
    emitter.instruction("xor r11, r11");                                        // entry index

    emitter.label("__rt_cex_loop_x86");
    emitter.instruction("cmp r11, r9");                                         // scanned every registered class?
    emitter.instruction("jge __rt_cex_miss_x86");                               // exhausted the table without a match
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the entry cursor
    emitter.instruction("mov rcx, QWORD PTR [r10 + 8]");                        // stored name length
    emitter.instruction("mov rdx, QWORD PTR [rbp - 16]");                       // reload the input name length
    emitter.instruction("cmp rcx, rdx");                                        // length mismatch?
    emitter.instruction("jne __rt_cex_skip_x86");                               // skip on length mismatch
    emitter.instruction("mov QWORD PTR [rbp - 32], r11");                       // save the entry index across the string helper
    emitter.instruction("mov rdi, QWORD PTR [rbp - 8]");                        // reload the input name pointer
    emitter.instruction("mov rsi, QWORD PTR [rbp - 16]");                       // reload the input name length
    emitter.instruction("mov rdx, QWORD PTR [r10]");                            // stored class-name pointer
    emitter.instruction("call __rt_strcasecmp");                                // compare class names case-insensitively
    emitter.instruction("mov r11, QWORD PTR [rbp - 32]");                       // restore the entry index after the string helper
    emitter.instruction("test rax, rax");                                       // did the class names match case-insensitively?
    emitter.instruction("je __rt_cex_match_x86");                               // full match: the class exists
    emitter.instruction("jmp __rt_cex_skip_x86");                               // mismatch: try the next entry

    emitter.label("__rt_cex_skip_x86");
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the entry cursor
    emitter.instruction("add r10, 48");                                         // advance to the next 48-byte entry
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // persist the cursor
    emitter.instruction("add r11, 1");                                          // advance the entry index
    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0);   // reload the count (lost across the table walk)
    emitter.instruction("jmp __rt_cex_loop_x86");                               // continue scanning

    emitter.label("__rt_cex_match_x86");
    emitter.instruction("mov eax, 1");                                          // the class exists
    emitter.instruction("add rsp, 32");                                         // release the frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return true

    emitter.label("__rt_cex_miss_x86");
    emitter.instruction("xor eax, eax");                                        // no class with that name
    emitter.instruction("add rsp, 32");                                         // release the frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return false
}

/// class_has_constructor: report whether a class declares/inherits __construct.
/// Runs the `_classes_by_name` scan and returns the matched entry's flag from
/// the parallel position-indexed `_class_has_ctor` table
/// (ReflectionClass::getConstructor()'s null case).
/// Input:  AArch64 x1 = name pointer, x2 = name length
///         x86_64  rax = name pointer, rdx = name length
/// Output: 1 when the class has a constructor, 0 when not or unknown
pub fn emit_class_has_constructor_by_name(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_class_has_constructor_by_name_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: class_has_constructor ---");
    emitter.label_global("__rt_class_has_constructor");

    // Frame (48 bytes): [0..16) saved x29/x30, [16) name_ptr, [24) name_len,
    //   [32) entry cursor, [40) entry index saved across __rt_strcasecmp.
    emitter.instruction("sub sp, sp, #48");                                     // helper frame
    emitter.instruction("stp x29, x30, [sp, #0]");                              // save frame pointer and return address
    emitter.instruction("mov x29, sp");                                         // establish the helper frame pointer
    emitter.instruction("str x1, [sp, #16]");                                   // save the name pointer
    emitter.instruction("str x2, [sp, #24]");                                   // save the name length

    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // x9 = entry count
    emitter.instruction("cbz x9, __rt_chc_miss");                               // empty registry → no constructor
    abi::emit_symbol_address(emitter, "x10", "_classes_by_name");
    emitter.instruction("str x10, [sp, #32]");                                  // initialise the entry cursor
    emitter.instruction("mov x11, #0");                                         // entry index

    emitter.label("__rt_chc_loop");
    emitter.instruction("cmp x11, x9");                                         // scanned every registered class?
    emitter.instruction("b.ge __rt_chc_miss");                                  // exhausted the table without a match
    emitter.instruction("ldr x10, [sp, #32]");                                  // reload the entry cursor
    emitter.instruction("ldr x13, [x10, #8]");                                  // stored name length
    emitter.instruction("ldr x2, [sp, #24]");                                   // reload the input name length
    emitter.instruction("cmp x13, x2");                                         // length mismatch → skip
    emitter.instruction("b.ne __rt_chc_skip");                                  // skip this class when the name lengths differ
    emitter.instruction("str x11, [sp, #40]");                                  // save the entry index across the string helper
    emitter.instruction("ldr x1, [sp, #16]");                                   // reload the input name pointer
    emitter.instruction("ldr x2, [sp, #24]");                                   // reload the input name length
    emitter.instruction("ldr x3, [x10]");                                       // stored class-name pointer
    emitter.instruction("mov x4, x13");                                         // stored class-name length
    emitter.instruction("bl __rt_strcasecmp");                                  // compare class names case-insensitively
    emitter.instruction("ldr x11, [sp, #40]");                                  // restore the entry index after the string helper
    emitter.instruction("cmp x0, #0");                                          // did the class names match case-insensitively?
    emitter.instruction("b.eq __rt_chc_match");                                 // full match: return the constructor flag
    emitter.instruction("b __rt_chc_skip");                                     // mismatch: try the next entry

    emitter.label("__rt_chc_skip");
    emitter.instruction("ldr x10, [sp, #32]");                                  // reload the entry cursor
    emitter.instruction("add x10, x10, #48");                                   // advance to the next 48-byte entry
    emitter.instruction("str x10, [sp, #32]");                                  // persist the cursor
    emitter.instruction("add x11, x11, #1");                                    // advance the entry index
    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // reload the count (lost across the table walk)
    emitter.instruction("b __rt_chc_loop");                                     // continue scanning

    emitter.label("__rt_chc_match");
    abi::emit_symbol_address(emitter, "x10", "_class_has_ctor");
    emitter.instruction("ldr x0, [x10, x11, lsl #3]");                          // constructor flag at the matched entry index
    emitter.instruction("ldp x29, x30, [sp, #0]");                              // restore frame pointer and return address
    emitter.instruction("add sp, sp, #48");                                     // release the frame
    emitter.instruction("ret");                                                 // return the flag

    emitter.label("__rt_chc_miss");
    emitter.instruction("mov x0, #0");                                          // unknown class → no constructor
    emitter.instruction("ldp x29, x30, [sp, #0]");                              // restore frame pointer and return address
    emitter.instruction("add sp, sp, #48");                                     // release the frame
    emitter.instruction("ret");                                                 // return the zero flag
}

/// Emits the Linux x86_64 variant of `__rt_class_has_constructor`.
fn emit_class_has_constructor_by_name_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: class_has_constructor ---");
    emitter.label_global("__rt_class_has_constructor");

    // Frame (rbp-relative): [-8) name_ptr [-16) name_len [-24) entry cursor
    //   [-32) entry index stash.
    emitter.instruction("push rbp");                                            // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish the helper frame pointer
    emitter.instruction("sub rsp, 32");                                         // helper frame
    emitter.instruction("mov QWORD PTR [rbp - 8], rax");                        // save the name pointer (elephc string ABI: rax)
    emitter.instruction("mov QWORD PTR [rbp - 16], rdx");                       // save the name length (elephc string ABI: rdx)

    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0);   // r9 = entry count
    emitter.instruction("test r9, r9");                                         // empty registry?
    emitter.instruction("jz __rt_chc_miss_x86");                                // no entries → no constructor
    abi::emit_symbol_address(emitter, "r10", "_classes_by_name");               // r10 = table base
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // entry cursor
    emitter.instruction("xor r11, r11");                                        // entry index

    emitter.label("__rt_chc_loop_x86");
    emitter.instruction("cmp r11, r9");                                         // scanned every registered class?
    emitter.instruction("jge __rt_chc_miss_x86");                               // exhausted the table without a match
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the entry cursor
    emitter.instruction("mov rcx, QWORD PTR [r10 + 8]");                        // stored name length
    emitter.instruction("mov rdx, QWORD PTR [rbp - 16]");                       // reload the input name length
    emitter.instruction("cmp rcx, rdx");                                        // length mismatch?
    emitter.instruction("jne __rt_chc_skip_x86");                               // skip on length mismatch
    emitter.instruction("mov QWORD PTR [rbp - 32], r11");                       // save the entry index across the string helper
    emitter.instruction("mov rdi, QWORD PTR [rbp - 8]");                        // reload the input name pointer
    emitter.instruction("mov rsi, QWORD PTR [rbp - 16]");                       // reload the input name length
    emitter.instruction("mov rdx, QWORD PTR [r10]");                            // stored class-name pointer
    emitter.instruction("call __rt_strcasecmp");                                // compare class names case-insensitively
    emitter.instruction("mov r11, QWORD PTR [rbp - 32]");                       // restore the entry index after the string helper
    emitter.instruction("test rax, rax");                                       // did the class names match case-insensitively?
    emitter.instruction("je __rt_chc_match_x86");                               // full match: return the constructor flag
    emitter.instruction("jmp __rt_chc_skip_x86");                               // mismatch: try the next entry

    emitter.label("__rt_chc_skip_x86");
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the entry cursor
    emitter.instruction("add r10, 48");                                         // advance to the next 48-byte entry
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // persist the cursor
    emitter.instruction("add r11, 1");                                          // advance the entry index
    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0);   // reload the count (lost across the table walk)
    emitter.instruction("jmp __rt_chc_loop_x86");                               // continue scanning

    emitter.label("__rt_chc_match_x86");
    abi::emit_symbol_address(emitter, "r10", "_class_has_ctor");                // r10 = flag-table base
    emitter.instruction("mov rax, QWORD PTR [r10 + r11*8]");                    // constructor flag at the matched entry index
    emitter.instruction("add rsp, 32");                                         // release the frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the flag

    emitter.label("__rt_chc_miss_x86");
    emitter.instruction("xor eax, eax");                                        // unknown class → no constructor
    emitter.instruction("add rsp, 32");                                         // release the frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the zero flag
}

/// class_parent_name: look up a class's parent-class name by class name.
/// Runs the `_classes_by_name` scan; a match reads the entry's class_id,
/// maps it through the dense `_class_parent_ids` table, and resolves the
/// parent id through the dense `_class_name_entries` name table
/// (ReflectionClass::getParentClass()).
/// Input:  AArch64 x1 = name pointer, x2 = name length
///         x86_64  rax = name pointer, rdx = name length
/// Output: AArch64 x0 = parent-name pointer, x1 = parent-name length (0/0
///         when the class is unknown or has no parent); x86_64 rax/rdx.
pub fn emit_class_parent_name_by_name(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_class_parent_name_by_name_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: class_parent_name ---");
    emitter.label_global("__rt_class_parent_name");

    // Frame (48 bytes): [0..16) saved x29/x30, [16) name_ptr, [24) name_len,
    //   [32) entry cursor, [40) entry index saved across __rt_strcasecmp.
    emitter.instruction("sub sp, sp, #48");                                     // helper frame
    emitter.instruction("stp x29, x30, [sp, #0]");                              // save frame pointer and return address
    emitter.instruction("mov x29, sp");                                         // establish the helper frame pointer
    emitter.instruction("str x1, [sp, #16]");                                   // save the name pointer
    emitter.instruction("str x2, [sp, #24]");                                   // save the name length

    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // x9 = entry count
    emitter.instruction("cbz x9, __rt_cpn_miss");                               // empty registry → no parent
    abi::emit_symbol_address(emitter, "x10", "_classes_by_name");
    emitter.instruction("str x10, [sp, #32]");                                  // initialise the entry cursor
    emitter.instruction("mov x11, #0");                                         // entry index

    emitter.label("__rt_cpn_loop");
    emitter.instruction("cmp x11, x9");                                         // scanned every registered class?
    emitter.instruction("b.ge __rt_cpn_miss");                                  // exhausted the table without a match
    emitter.instruction("ldr x10, [sp, #32]");                                  // reload the entry cursor
    emitter.instruction("ldr x13, [x10, #8]");                                  // stored name length
    emitter.instruction("ldr x2, [sp, #24]");                                   // reload the input name length
    emitter.instruction("cmp x13, x2");                                         // length mismatch → skip
    emitter.instruction("b.ne __rt_cpn_skip");                                  // skip this class when the name lengths differ
    emitter.instruction("str x11, [sp, #40]");                                  // save the entry index across the string helper
    emitter.instruction("ldr x1, [sp, #16]");                                   // reload the input name pointer
    emitter.instruction("ldr x2, [sp, #24]");                                   // reload the input name length
    emitter.instruction("ldr x3, [x10]");                                       // stored class-name pointer
    emitter.instruction("mov x4, x13");                                         // stored class-name length
    emitter.instruction("bl __rt_strcasecmp");                                  // compare class names case-insensitively
    emitter.instruction("ldr x11, [sp, #40]");                                  // restore the entry index after the string helper
    emitter.instruction("cmp x0, #0");                                          // did the class names match case-insensitively?
    emitter.instruction("b.eq __rt_cpn_match");                                 // full match: resolve the parent name
    emitter.instruction("b __rt_cpn_skip");                                     // mismatch: try the next entry

    emitter.label("__rt_cpn_skip");
    emitter.instruction("ldr x10, [sp, #32]");                                  // reload the entry cursor
    emitter.instruction("add x10, x10, #48");                                   // advance to the next 48-byte entry
    emitter.instruction("str x10, [sp, #32]");                                  // persist the cursor
    emitter.instruction("add x11, x11, #1");                                    // advance the entry index
    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // reload the count (lost across the table walk)
    emitter.instruction("b __rt_cpn_loop");                                     // continue scanning

    emitter.label("__rt_cpn_match");
    emitter.instruction("ldr x10, [sp, #32]");                                  // reload the matched entry cursor
    emitter.instruction("ldr x9, [x10, #16]");                                  // class_id column of the matched entry
    abi::emit_symbol_address(emitter, "x10", "_class_name_count");
    emitter.instruction("ldr x10, [x10]");                                      // x10 = dense class-name row count
    emitter.instruction("cmp x9, x10");                                         // validate the class id before the parent lookup
    emitter.instruction("b.hs __rt_cpn_miss");                                  // out-of-range ids have no reportable parent
    abi::emit_symbol_address(emitter, "x12", "_class_parent_ids");
    emitter.instruction("ldr x9, [x12, x9, lsl #3]");                           // parent class id (or -1)
    emitter.instruction("cmn x9, #1");                                          // parentless sentinel?
    emitter.instruction("b.eq __rt_cpn_miss");                                  // no parent → empty result
    emitter.instruction("cmp x9, x10");                                         // validate the parent id before the name lookup
    emitter.instruction("b.hs __rt_cpn_miss");                                  // out-of-range parent ids have no name row
    abi::emit_symbol_address(emitter, "x12", "_class_name_entries");
    emitter.instruction("add x12, x12, x9, lsl #4");                            // parent's 16-byte class-name metadata row
    emitter.instruction("ldr x0, [x12]");                                       // parent-name string pointer
    emitter.instruction("ldr x1, [x12, #8]");                                   // parent-name string length
    emitter.instruction("ldp x29, x30, [sp, #0]");                              // restore frame pointer and return address
    emitter.instruction("add sp, sp, #48");                                     // release the frame
    emitter.instruction("ret");                                                 // return the parent name

    emitter.label("__rt_cpn_miss");
    emitter.instruction("mov x0, #0");                                          // unknown class or no parent → null pointer
    emitter.instruction("mov x1, #0");                                          // zero length
    emitter.instruction("ldp x29, x30, [sp, #0]");                              // restore frame pointer and return address
    emitter.instruction("add sp, sp, #48");                                     // release the frame
    emitter.instruction("ret");                                                 // return the empty result
}

/// Emits the Linux x86_64 variant of `__rt_class_parent_name`.
fn emit_class_parent_name_by_name_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: class_parent_name ---");
    emitter.label_global("__rt_class_parent_name");

    // Frame (rbp-relative): [-8) name_ptr [-16) name_len [-24) entry cursor
    //   [-32) entry index stash.
    emitter.instruction("push rbp");                                            // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish the helper frame pointer
    emitter.instruction("sub rsp, 32");                                         // helper frame
    emitter.instruction("mov QWORD PTR [rbp - 8], rax");                        // save the name pointer (elephc string ABI: rax)
    emitter.instruction("mov QWORD PTR [rbp - 16], rdx");                       // save the name length (elephc string ABI: rdx)

    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0);   // r9 = entry count
    emitter.instruction("test r9, r9");                                         // empty registry?
    emitter.instruction("jz __rt_cpn_miss_x86");                                // no entries → no parent
    abi::emit_symbol_address(emitter, "r10", "_classes_by_name");               // r10 = table base
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // entry cursor
    emitter.instruction("xor r11, r11");                                        // entry index

    emitter.label("__rt_cpn_loop_x86");
    emitter.instruction("cmp r11, r9");                                         // scanned every registered class?
    emitter.instruction("jge __rt_cpn_miss_x86");                               // exhausted the table without a match
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the entry cursor
    emitter.instruction("mov rcx, QWORD PTR [r10 + 8]");                        // stored name length
    emitter.instruction("mov rdx, QWORD PTR [rbp - 16]");                       // reload the input name length
    emitter.instruction("cmp rcx, rdx");                                        // length mismatch?
    emitter.instruction("jne __rt_cpn_skip_x86");                               // skip on length mismatch
    emitter.instruction("mov QWORD PTR [rbp - 32], r11");                       // save the entry index across the string helper
    emitter.instruction("mov rdi, QWORD PTR [rbp - 8]");                        // reload the input name pointer
    emitter.instruction("mov rsi, QWORD PTR [rbp - 16]");                       // reload the input name length
    emitter.instruction("mov rdx, QWORD PTR [r10]");                            // stored class-name pointer
    emitter.instruction("call __rt_strcasecmp");                                // compare class names case-insensitively
    emitter.instruction("mov r11, QWORD PTR [rbp - 32]");                       // restore the entry index after the string helper
    emitter.instruction("test rax, rax");                                       // did the class names match case-insensitively?
    emitter.instruction("je __rt_cpn_match_x86");                               // full match: resolve the parent name
    emitter.instruction("jmp __rt_cpn_skip_x86");                               // mismatch: try the next entry

    emitter.label("__rt_cpn_skip_x86");
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the entry cursor
    emitter.instruction("add r10, 48");                                         // advance to the next 48-byte entry
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // persist the cursor
    emitter.instruction("add r11, 1");                                          // advance the entry index
    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0);   // reload the count (lost across the table walk)
    emitter.instruction("jmp __rt_cpn_loop_x86");                               // continue scanning

    emitter.label("__rt_cpn_match_x86");
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the matched entry cursor
    emitter.instruction("mov r9, QWORD PTR [r10 + 16]");                        // class_id column of the matched entry
    abi::emit_load_symbol_to_reg(emitter, "r10", "_class_name_count", 0);       // r10 = dense class-name row count
    emitter.instruction("cmp r9, r10");                                         // validate the class id before the parent lookup
    emitter.instruction("jae __rt_cpn_miss_x86");                               // out-of-range ids have no reportable parent
    abi::emit_symbol_address(emitter, "r11", "_class_parent_ids");
    emitter.instruction("mov r9, QWORD PTR [r11 + r9*8]");                      // parent class id (or -1)
    emitter.instruction("cmp r9, -1");                                          // parentless sentinel?
    emitter.instruction("je __rt_cpn_miss_x86");                                // no parent → empty result
    emitter.instruction("cmp r9, r10");                                         // validate the parent id before the name lookup
    emitter.instruction("jae __rt_cpn_miss_x86");                               // out-of-range parent ids have no name row
    abi::emit_symbol_address(emitter, "r11", "_class_name_entries");
    emitter.instruction("shl r9, 4");                                           // scale the parent id by the 16-byte name row size
    emitter.instruction("add r11, r9");                                         // parent's class-name metadata row
    emitter.instruction("mov rax, QWORD PTR [r11]");                            // parent-name string pointer
    emitter.instruction("mov rdx, QWORD PTR [r11 + 8]");                        // parent-name string length
    emitter.instruction("add rsp, 32");                                         // release the frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the parent name

    emitter.label("__rt_cpn_miss_x86");
    emitter.instruction("xor eax, eax");                                        // unknown class or no parent → null pointer
    emitter.instruction("xor edx, edx");                                        // zero length
    emitter.instruction("add rsp, 32");                                         // release the frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the empty result
}
