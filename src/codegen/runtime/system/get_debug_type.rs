//! Purpose:
//! Emits the `__rt_get_debug_type` runtime helper and its `_gdt_*` name table for
//! PHP `get_debug_type()` over boxed `Mixed` values.
//!
//! Called from:
//! - `crate::codegen::runtime::emitters::emit_runtime()` via `crate::codegen::runtime::system`
//!   (helper) and `crate::codegen::runtime::data::fixed` (name table).
//!
//! Key details:
//! - Statically-typed arguments never reach this helper — the EIR frontend folds
//!   them to constant name strings (`lower_static_get_debug_type`). Only Mixed/Union
//!   values arrive here, boxed by the `Mixed` parameter coercion.
//! - Dispatch is heap-header-adaptive like `__rt_mixed_array_get`: raw arrays/hashes
//!   (heap kinds 2/3) and raw objects (kind 4) received as Mixed report directly;
//!   mixed cells (kind 5) dispatch on the runtime value tag (0 int, 1 string,
//!   2 float, 3 bool, 4/5 array, 6 object, 7 nested cell — unwrapped and re-dispatched,
//!   8 null, 9 resource).
//! - Objects resolve their class name by class_id through the `_classes_by_name`
//!   table (48-byte entries; id column at [16..24), name at [0..16)), matching PHP's
//!   FQCN result. Unknown ids fall back to the literal "object".

use crate::codegen::{abi, emit::Emitter, platform::Arch};

/// Returns the `.globl`/`.ascii` assembly for the static type-name strings.
pub(crate) fn emit_get_debug_type_data() -> String {
    let mut out = String::new();
    for (label, value) in [
        ("_gdt_int", "int"),
        ("_gdt_string", "string"),
        ("_gdt_float", "float"),
        ("_gdt_bool", "bool"),
        ("_gdt_array", "array"),
        ("_gdt_null", "null"),
        ("_gdt_object", "object"),
        ("_gdt_resource", "resource (stream)"),
    ] {
        out.push_str(&format!(
            ".globl {0}\n{0}:\n    .ascii \"{1}\"\n",
            label, value
        ));
    }
    out
}

/// get_debug_type: PHP type-name lookup for a boxed Mixed value.
/// Input:  AArch64 x0 = mixed value; x86_64 rdi = mixed value
/// Output: name string — AArch64 (x0 = ptr, x1 = len); x86_64 (rax = ptr, rdx = len)
pub fn emit_get_debug_type(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_get_debug_type_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: get_debug_type ---");
    emitter.label_global("__rt_get_debug_type");

    emitter.instruction("sub sp, sp, #32");                                     // helper frame
    emitter.instruction("stp x29, x30, [sp, #16]");                             // save frame pointer and return address
    emitter.instruction("add x29, sp, #16");                                    // establish the helper frame pointer

    emitter.label("__rt_gdt_dispatch");
    emitter.instruction("cbz x0, __rt_gdt_null");                               // null value → "null"
    emitter.instruction("ldr x9, [x0, #-8]");                                   // load the uniform heap header
    emitter.instruction("and x9, x9, #0xff");                                   // isolate the heap kind byte
    emitter.instruction("cmp x9, #2");                                          // raw indexed array received as Mixed?
    emitter.instruction("b.eq __rt_gdt_array");                                 // report "array"
    emitter.instruction("cmp x9, #3");                                          // raw hash received as Mixed?
    emitter.instruction("b.eq __rt_gdt_array");                                 // report "array"
    emitter.instruction("cmp x9, #4");                                          // raw object received as Mixed?
    emitter.instruction("b.eq __rt_gdt_object_from_x0");                        // resolve the class name from the object
    emitter.instruction("ldr x9, [x0]");                                        // mixed cell: load the runtime value tag
    emitter.instruction("cmp x9, #7");                                          // nested boxed-Mixed entry?
    emitter.instruction("b.ne __rt_gdt_tags");                                  // dispatch plain tags
    emitter.instruction("ldr x0, [x0, #8]");                                    // unwrap the nested cell payload
    emitter.instruction("b __rt_gdt_dispatch");                                 // re-dispatch on the inner value

    emitter.label("__rt_gdt_tags");
    emitter.instruction("cmp x9, #0");                                          // int?
    emitter.instruction("b.eq __rt_gdt_int");
    emitter.instruction("cmp x9, #1");                                          // string?
    emitter.instruction("b.eq __rt_gdt_string");
    emitter.instruction("cmp x9, #2");                                          // float?
    emitter.instruction("b.eq __rt_gdt_float");
    emitter.instruction("cmp x9, #3");                                          // bool?
    emitter.instruction("b.eq __rt_gdt_bool");
    emitter.instruction("cmp x9, #4");                                          // indexed-array payload?
    emitter.instruction("b.eq __rt_gdt_array");
    emitter.instruction("cmp x9, #5");                                          // hash payload?
    emitter.instruction("b.eq __rt_gdt_array");
    emitter.instruction("cmp x9, #6");                                          // object payload?
    emitter.instruction("b.eq __rt_gdt_object_cell");
    emitter.instruction("cmp x9, #9");                                          // resource payload?
    emitter.instruction("b.eq __rt_gdt_resource");
    emitter.instruction("b __rt_gdt_null");                                     // tag 8 and anything unknown → "null"

    emitter.label("__rt_gdt_object_cell");
    emitter.instruction("ldr x0, [x0, #8]");                                    // unwrap the object payload
    emitter.instruction("cbz x0, __rt_gdt_null");                               // defensive null guard
    emitter.label("__rt_gdt_object_from_x0");
    emitter.instruction("ldr x11, [x0]");                                       // x11 = class_id at object offset 0
    abi::emit_symbol_address(emitter, "x9", "_classes_by_name_count");
    emitter.instruction("ldr x9, [x9]");                                        // x9 = entry count
    abi::emit_symbol_address(emitter, "x10", "_classes_by_name");
    emitter.instruction("mov x12, #0");                                         // entry index
    emitter.label("__rt_gdt_class_loop");
    emitter.instruction("cmp x12, x9");                                         // scanned every registered class?
    emitter.instruction("b.ge __rt_gdt_object_fallback");                       // unknown id → literal "object"
    emitter.instruction("ldr x13, [x10, #16]");                                 // entry class_id column
    emitter.instruction("cmp x13, x11");                                        // id match?
    emitter.instruction("b.eq __rt_gdt_class_hit");                             // return the class name
    emitter.instruction("add x10, x10, #48");                                   // advance to the next 48-byte entry
    emitter.instruction("add x12, x12, #1");                                    // advance the entry index
    emitter.instruction("b __rt_gdt_class_loop");                               // continue scanning
    emitter.label("__rt_gdt_class_hit");
    emitter.instruction("ldr x1, [x10, #8]");                                   // name length column
    emitter.instruction("ldr x0, [x10]");                                       // name pointer column
    emitter.instruction("b __rt_gdt_ret");

    emitter.label("__rt_gdt_object_fallback");
    abi::emit_symbol_address(emitter, "x0", "_gdt_object");
    emitter.instruction("mov x1, #6");                                          // strlen("object")
    emitter.instruction("b __rt_gdt_ret");

    for (label, symbol, len) in [
        ("__rt_gdt_int", "_gdt_int", 3),
        ("__rt_gdt_string", "_gdt_string", 6),
        ("__rt_gdt_float", "_gdt_float", 5),
        ("__rt_gdt_bool", "_gdt_bool", 4),
        ("__rt_gdt_array", "_gdt_array", 5),
        ("__rt_gdt_null", "_gdt_null", 4),
        ("__rt_gdt_resource", "_gdt_resource", 17),
    ] {
        emitter.label(label);
        abi::emit_symbol_address(emitter, "x0", symbol);
        emitter.instruction(&format!("mov x1, #{}", len));                      // static name length
        emitter.instruction("b __rt_gdt_ret");
    }

    emitter.label("__rt_gdt_ret");
    emitter.instruction("ldp x29, x30, [sp, #16]");                             // restore frame pointer and return address
    emitter.instruction("add sp, sp, #32");                                     // release the helper frame
    emitter.instruction("ret");                                                 // return the name string in x0/x1
}

/// Emits the Linux x86_64 variant of `__rt_get_debug_type`.
fn emit_get_debug_type_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: get_debug_type ---");
    emitter.label_global("__rt_get_debug_type");

    emitter.instruction("push rbp");                                            // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish the helper frame pointer

    emitter.label("__rt_gdt_dispatch_x86");
    emitter.instruction("test rdi, rdi");                                       // null value → "null"
    emitter.instruction("je __rt_gdt_null_x86");
    emitter.instruction("mov r10, QWORD PTR [rdi - 8]");                        // load the uniform heap header
    emitter.instruction("and r10, 0xff");                                       // isolate the heap kind byte
    emitter.instruction("cmp r10, 2");                                          // raw indexed array received as Mixed?
    emitter.instruction("je __rt_gdt_array_x86");
    emitter.instruction("cmp r10, 3");                                          // raw hash received as Mixed?
    emitter.instruction("je __rt_gdt_array_x86");
    emitter.instruction("cmp r10, 4");                                          // raw object received as Mixed?
    emitter.instruction("je __rt_gdt_object_from_rdi_x86");
    emitter.instruction("mov r10, QWORD PTR [rdi]");                            // mixed cell: load the runtime value tag
    emitter.instruction("cmp r10, 7");                                          // nested boxed-Mixed entry?
    emitter.instruction("jne __rt_gdt_tags_x86");
    emitter.instruction("mov rdi, QWORD PTR [rdi + 8]");                        // unwrap the nested cell payload
    emitter.instruction("jmp __rt_gdt_dispatch_x86");                           // re-dispatch on the inner value

    emitter.label("__rt_gdt_tags_x86");
    emitter.instruction("cmp r10, 0");                                          // int?
    emitter.instruction("je __rt_gdt_int_x86");
    emitter.instruction("cmp r10, 1");                                          // string?
    emitter.instruction("je __rt_gdt_string_x86");
    emitter.instruction("cmp r10, 2");                                          // float?
    emitter.instruction("je __rt_gdt_float_x86");
    emitter.instruction("cmp r10, 3");                                          // bool?
    emitter.instruction("je __rt_gdt_bool_x86");
    emitter.instruction("cmp r10, 4");                                          // indexed-array payload?
    emitter.instruction("je __rt_gdt_array_x86");
    emitter.instruction("cmp r10, 5");                                          // hash payload?
    emitter.instruction("je __rt_gdt_array_x86");
    emitter.instruction("cmp r10, 6");                                          // object payload?
    emitter.instruction("je __rt_gdt_object_cell_x86");
    emitter.instruction("cmp r10, 9");                                          // resource payload?
    emitter.instruction("je __rt_gdt_resource_x86");
    emitter.instruction("jmp __rt_gdt_null_x86");                               // tag 8 and anything unknown → "null"

    emitter.label("__rt_gdt_object_cell_x86");
    emitter.instruction("mov rdi, QWORD PTR [rdi + 8]");                        // unwrap the object payload
    emitter.instruction("test rdi, rdi");                                       // defensive null guard
    emitter.instruction("je __rt_gdt_null_x86");
    emitter.label("__rt_gdt_object_from_rdi_x86");
    emitter.instruction("mov r11, QWORD PTR [rdi]");                            // r11 = class_id at object offset 0
    abi::emit_load_symbol_to_reg(emitter, "r9", "_classes_by_name_count", 0);   // r9 = entry count
    abi::emit_symbol_address(emitter, "r10", "_classes_by_name");               // r10 = table base
    emitter.instruction("xor r8, r8");                                          // entry index
    emitter.label("__rt_gdt_class_loop_x86");
    emitter.instruction("cmp r8, r9");                                          // scanned every registered class?
    emitter.instruction("jge __rt_gdt_object_fallback_x86");                    // unknown id → literal "object"
    emitter.instruction("mov rcx, QWORD PTR [r10 + 16]");                       // entry class_id column
    emitter.instruction("cmp rcx, r11");                                        // id match?
    emitter.instruction("je __rt_gdt_class_hit_x86");                           // return the class name
    emitter.instruction("add r10, 48");                                         // advance to the next 48-byte entry
    emitter.instruction("add r8, 1");                                           // advance the entry index
    emitter.instruction("jmp __rt_gdt_class_loop_x86");                         // continue scanning
    emitter.label("__rt_gdt_class_hit_x86");
    emitter.instruction("mov rdx, QWORD PTR [r10 + 8]");                        // name length column
    emitter.instruction("mov rax, QWORD PTR [r10]");                            // name pointer column
    emitter.instruction("jmp __rt_gdt_ret_x86");

    emitter.label("__rt_gdt_object_fallback_x86");
    abi::emit_symbol_address(emitter, "rax", "_gdt_object");
    emitter.instruction("mov rdx, 6");                                          // strlen("object")
    emitter.instruction("jmp __rt_gdt_ret_x86");

    for (label, symbol, len) in [
        ("__rt_gdt_int_x86", "_gdt_int", 3),
        ("__rt_gdt_string_x86", "_gdt_string", 6),
        ("__rt_gdt_float_x86", "_gdt_float", 5),
        ("__rt_gdt_bool_x86", "_gdt_bool", 4),
        ("__rt_gdt_array_x86", "_gdt_array", 5),
        ("__rt_gdt_null_x86", "_gdt_null", 4),
        ("__rt_gdt_resource_x86", "_gdt_resource", 17),
    ] {
        emitter.label(label);
        abi::emit_symbol_address(emitter, "rax", symbol);
        emitter.instruction(&format!("mov rdx, {}", len));                      // static name length
        emitter.instruction("jmp __rt_gdt_ret_x86");
    }

    emitter.label("__rt_gdt_ret_x86");
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the name string in rax/rdx
}
