//! Purpose:
//! Emits the `__rt_strtr` runtime helper assembly for the 3-argument form of PHP `strtr`.
//! Keeps PHP byte-string translation semantics and target-specific ABI variants in one focused emitter.
//!
//! Called from:
//! - `crate::codegen::runtime::emitters::emit_runtime()` via `crate::codegen::runtime::strings`.
//! - The `strtr()` builtin lowering (reuses `lower_string_replace`, so the three string
//!   arguments arrive in the str_replace register layout: subject=arg0, from=arg1, to=arg2).
//!
//! Key details:
//! - Implements `strtr($str, $from, $to)`: each byte of `$str` is translated through a
//!   256-entry table seeded to identity, then overwritten for `j` in `0..min(len($from),
//!   len($to))` with `table[from[j]] = to[j]`. Overwriting in ascending `j` reproduces
//!   PHP's "last mapping wins" for a repeated `$from` byte. The result is exactly the
//!   length of `$str` (1 byte in → 1 byte out).
//! - Result storage mirrors `__rt_str_repeat`: the fixed 64 KiB concat scratch when it
//!   fits, otherwise a heap allocation stamped as an owned string.

use crate::codegen::emit::Emitter;
use crate::codegen::platform::Arch;

/// Owned-string heap kind word, x86_64 variant (matches `__rt_str_repeat`).
const X86_64_HEAP_MAGIC_HI32: u64 = 0x454C5048;

/// Emits the `__rt_strtr` runtime helper for the current platform.
///
/// ABI (matches the `lower_string_replace` 3-string layout):
///   ARM64:  x1/x2=subject, x3/x4=from, x5/x6=to  → x1=result_ptr, x2=result_len
///   x86_64: rax/rdx=subject, rdi/rsi=from, rcx/r8=to → rax=result_ptr, rdx=result_len
pub fn emit_strtr(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_strtr_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: strtr (3-arg char map) ---");
    emitter.label_global("__rt_strtr");

    // -- stack frame: 256-byte table at [sp+0], metadata above it --
    emitter.instruction("sub sp, sp, #320"); // 256 table + result metadata + frame save
    emitter.instruction("stp x29, x30, [sp, #304]"); // save frame pointer and return address
    emitter.instruction("add x29, sp, #304"); // establish the new frame pointer
    emitter.instruction("stp x1, x2, [sp, #280]"); // preserve subject ptr/len across heap_alloc

    emitter.instruction("cbz x2, __rt_strtr_empty"); // empty subject → empty result

    // -- build a 256-entry identity table at [sp+0] --
    emitter.instruction("add x8, sp, #0"); // x8 = table base
    emitter.instruction("mov x9, #0"); // k = 0
    emitter.label("__rt_strtr_init");
    emitter.instruction("strb w9, [x8, x9]"); // table[k] = k
    emitter.instruction("add x9, x9, #1"); // advance k
    emitter.instruction("cmp x9, #256"); // filled all 256 entries?
    emitter.instruction("b.lo __rt_strtr_init"); // continue seeding identity

    // -- apply overrides: table[from[j]] = to[j] for j in 0..min(from_len, to_len) --
    emitter.instruction("cmp x4, x6"); // from_len vs to_len
    emitter.instruction("csel x7, x4, x6, ls"); // x7 = min(from_len, to_len)
    emitter.instruction("mov x9, #0"); // j = 0
    emitter.label("__rt_strtr_ovr");
    emitter.instruction("cmp x9, x7"); // processed the whole overlap?
    emitter.instruction("b.hs __rt_strtr_ovr_done"); // yes → table is built
    emitter.instruction("ldrb w10, [x3, x9]"); // from[j]
    emitter.instruction("ldrb w11, [x5, x9]"); // to[j]
    emitter.instruction("strb w11, [x8, x10]"); // table[from[j]] = to[j] (ascending j → last wins)
    emitter.instruction("add x9, x9, #1"); // advance j
    emitter.instruction("b __rt_strtr_ovr"); // continue applying overrides
    emitter.label("__rt_strtr_ovr_done");

    // -- allocate the result buffer (result length == subject length) --
    emitter.instruction("ldr x4, [sp, #288]"); // x4 = subject length = result length
    emitter.instruction("str x4, [sp, #264]"); // save result length for finalization
    crate::codegen::abi::emit_symbol_address(emitter, "x6", "_concat_off");
    emitter.instruction("ldr x12, [x6]"); // current concat scratch write offset
    emitter.instruction("add x13, x12, x4"); // end offset after this append
    emitter.instruction("mov x14, #65536"); // concat scratch capacity
    emitter.instruction("cmp x13, x14"); // does the result fit in concat scratch?
    emitter.instruction("b.hi __rt_strtr_heap"); // use heap fallback when it would overflow
    crate::codegen::abi::emit_symbol_address(emitter, "x7", "_concat_buf");
    emitter.instruction("add x9, x7, x12"); // dest = concat_buf + off
    emitter.instruction("str x9, [sp, #256]"); // save result start pointer
    emitter.instruction("str xzr, [sp, #272]"); // mark result concat-backed
    emitter.instruction("b __rt_strtr_fill"); // skip heap allocation

    emitter.label("__rt_strtr_heap");
    emitter.instruction("mov x0, x4"); // requested payload size
    emitter.instruction("bl __rt_heap_alloc"); // allocate owned storage
    emitter.instruction("mov x6, #1"); // heap kind 1 = owned elephc string
    emitter.instruction("str x6, [x0, #-8]"); // stamp the allocation as a string payload
    emitter.instruction("str x0, [sp, #256]"); // save result start pointer
    emitter.instruction("mov x6, #1"); // storage kind = heap
    emitter.instruction("str x6, [sp, #272]"); // mark result heap-backed

    // -- fill: dest[i] = table[subject[i]] --
    emitter.label("__rt_strtr_fill");
    emitter.instruction("add x8, sp, #0"); // reload table base (heap_alloc clobbered x8)
    emitter.instruction("ldr x1, [sp, #280]"); // subject ptr
    emitter.instruction("ldr x2, [sp, #288]"); // subject len
    emitter.instruction("ldr x9, [sp, #256]"); // dest ptr
    emitter.instruction("mov x10, #0"); // i = 0
    emitter.label("__rt_strtr_fill_loop");
    emitter.instruction("cmp x10, x2"); // translated every byte?
    emitter.instruction("b.hs __rt_strtr_done"); // yes → finalize
    emitter.instruction("ldrb w11, [x1, x10]"); // subject byte
    emitter.instruction("ldrb w12, [x8, x11]"); // table[byte]
    emitter.instruction("strb w12, [x9, x10]"); // dest[i] = translated byte
    emitter.instruction("add x10, x10, #1"); // advance i
    emitter.instruction("b __rt_strtr_fill_loop"); // continue translating

    // -- finalize: publish concat offset for concat-backed results --
    emitter.label("__rt_strtr_done");
    emitter.instruction("ldr x1, [sp, #256]"); // result pointer
    emitter.instruction("ldr x2, [sp, #264]"); // result length
    emitter.instruction("ldr x13, [sp, #272]"); // storage kind (0 concat, 1 heap)
    emitter.instruction("cbnz x13, __rt_strtr_return"); // heap results leave concat offset unchanged
    crate::codegen::abi::emit_symbol_address(emitter, "x6", "_concat_off");
    emitter.instruction("ldr x12, [x6]"); // reload concat scratch write offset
    emitter.instruction("add x12, x12, x2"); // advance by the result length
    emitter.instruction("str x12, [x6]"); // publish the updated offset
    emitter.instruction("b __rt_strtr_return"); // return the translated string

    emitter.label("__rt_strtr_empty");
    emitter.instruction("mov x1, #0"); // empty string has no payload pointer
    emitter.instruction("mov x2, #0"); // empty string length is zero

    emitter.label("__rt_strtr_return");
    emitter.instruction("ldp x29, x30, [sp, #304]"); // restore frame pointer and return address
    emitter.instruction("add sp, sp, #320"); // tear down the frame
    emitter.instruction("ret"); // return the translated string in x1/x2
}

/// Emits the `__rt_strtr` runtime helper for Linux x86_64.
///
/// ABI: rax/rdx=subject, rdi/rsi=from, rcx/r8=to → rax=result_ptr, rdx=result_len.
/// Mirrors the ARM64 variant: identity table, ascending overrides (last wins), concat
/// scratch with heap fallback.
fn emit_strtr_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: strtr (3-arg char map) ---");
    emitter.label_global("__rt_strtr");

    emitter.instruction("push rbp"); // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp"); // establish a stable frame base
    emitter.instruction("sub rsp, 320"); // 256-byte table + result metadata
    emitter.instruction("mov QWORD PTR [rbp - 288], rax"); // preserve subject ptr across heap_alloc
    emitter.instruction("mov QWORD PTR [rbp - 296], rdx"); // preserve subject len across heap_alloc

    emitter.instruction("test rdx, rdx"); // empty subject?
    emitter.instruction("jz __rt_strtr_empty_linux_x86_64"); // → empty result

    // -- build a 256-entry identity table at [rbp-256] --
    emitter.instruction("lea r9, [rbp - 256]"); // r9 = table base
    emitter.instruction("xor r10, r10"); // k = 0
    emitter.label("__rt_strtr_init_linux_x86_64");
    emitter.instruction("mov BYTE PTR [r9 + r10], r10b"); // table[k] = k
    emitter.instruction("inc r10"); // advance k
    emitter.instruction("cmp r10, 256"); // filled all 256 entries?
    emitter.instruction("jb __rt_strtr_init_linux_x86_64"); // continue seeding identity

    // -- min(from_len, to_len) into rax --
    emitter.instruction("mov rax, rsi"); // from_len
    emitter.instruction("cmp rax, r8"); // vs to_len
    emitter.instruction("jbe __rt_strtr_min_ok_linux_x86_64"); // rax already the min
    emitter.instruction("mov rax, r8"); // otherwise min = to_len
    emitter.label("__rt_strtr_min_ok_linux_x86_64");

    // -- apply overrides: table[from[j]] = to[j] for j in 0..min --
    emitter.instruction("xor r10, r10"); // j = 0
    emitter.label("__rt_strtr_ovr_linux_x86_64");
    emitter.instruction("cmp r10, rax"); // processed the whole overlap?
    emitter.instruction("jae __rt_strtr_ovr_done_linux_x86_64"); // yes → table built
    emitter.instruction("movzx edx, BYTE PTR [rdi + r10]"); // from[j]
    emitter.instruction("movzx r11d, BYTE PTR [rcx + r10]"); // to[j]
    emitter.instruction("mov BYTE PTR [r9 + rdx], r11b"); // table[from[j]] = to[j] (last wins)
    emitter.instruction("inc r10"); // advance j
    emitter.instruction("jmp __rt_strtr_ovr_linux_x86_64"); // continue applying overrides
    emitter.label("__rt_strtr_ovr_done_linux_x86_64");

    // -- allocate the result buffer (result length == subject length) --
    emitter.instruction("mov rcx, QWORD PTR [rbp - 296]"); // subject length = result length
    emitter.instruction("mov QWORD PTR [rbp - 272], rcx"); // save result length
    crate::codegen::abi::emit_symbol_address(emitter, "r8", "_concat_off");
    emitter.instruction("mov r9, QWORD PTR [r8]"); // current concat scratch write offset
    emitter.instruction("mov rdx, r9"); // copy before adding the result length
    emitter.instruction("add rdx, rcx"); // end offset after this append
    emitter.instruction("cmp rdx, 65536"); // does the result fit in concat scratch?
    emitter.instruction("ja __rt_strtr_heap_linux_x86_64"); // heap fallback on overflow
    crate::codegen::abi::emit_symbol_address(emitter, "r10", "_concat_buf");
    emitter.instruction("lea r11, [r10 + r9]"); // dest = concat_buf + off
    emitter.instruction("mov QWORD PTR [rbp - 264], r11"); // save result start pointer
    emitter.instruction("mov QWORD PTR [rbp - 280], 0"); // mark result concat-backed
    emitter.instruction("jmp __rt_strtr_fill_linux_x86_64"); // skip heap allocation

    emitter.label("__rt_strtr_heap_linux_x86_64");
    emitter.instruction("mov rax, rcx"); // requested payload size
    emitter.instruction("call __rt_heap_alloc"); // allocate owned storage
    emitter.instruction(&format!(
        "mov r10, 0x{:x}",
        (X86_64_HEAP_MAGIC_HI32 << 32) | 1
    )); // owned-string heap kind word with the x86_64 marker
    emitter.instruction("mov QWORD PTR [rax - 8], r10"); // stamp the allocation as a string payload
    emitter.instruction("mov QWORD PTR [rbp - 264], rax"); // save result start pointer
    emitter.instruction("mov QWORD PTR [rbp - 280], 1"); // mark result heap-backed

    // -- fill: dest[i] = table[subject[i]] --
    emitter.label("__rt_strtr_fill_linux_x86_64");
    emitter.instruction("lea r8, [rbp - 256]"); // reload table base
    emitter.instruction("mov rdi, QWORD PTR [rbp - 288]"); // subject ptr
    emitter.instruction("mov rsi, QWORD PTR [rbp - 296]"); // subject len
    emitter.instruction("mov r9, QWORD PTR [rbp - 264]"); // dest ptr
    emitter.instruction("xor r10, r10"); // i = 0
    emitter.label("__rt_strtr_fill_loop_linux_x86_64");
    emitter.instruction("cmp r10, rsi"); // translated every byte?
    emitter.instruction("jae __rt_strtr_done_linux_x86_64"); // yes → finalize
    emitter.instruction("movzx eax, BYTE PTR [rdi + r10]"); // subject byte
    emitter.instruction("movzx eax, BYTE PTR [r8 + rax]"); // table[byte]
    emitter.instruction("mov BYTE PTR [r9 + r10], al"); // dest[i] = translated byte
    emitter.instruction("inc r10"); // advance i
    emitter.instruction("jmp __rt_strtr_fill_loop_linux_x86_64"); // continue translating

    // -- finalize: publish concat offset for concat-backed results --
    emitter.label("__rt_strtr_done_linux_x86_64");
    emitter.instruction("mov rax, QWORD PTR [rbp - 264]"); // result pointer
    emitter.instruction("mov rdx, QWORD PTR [rbp - 272]"); // result length
    emitter.instruction("mov r8, QWORD PTR [rbp - 280]"); // storage kind (0 concat, 1 heap)
    emitter.instruction("test r8, r8"); // heap-backed?
    emitter.instruction("jnz __rt_strtr_return_linux_x86_64"); // heap results leave concat offset unchanged
    crate::codegen::abi::emit_symbol_address(emitter, "r8", "_concat_off");
    emitter.instruction("mov r9, QWORD PTR [r8]"); // reload concat scratch write offset
    emitter.instruction("add r9, rdx"); // advance by the result length
    emitter.instruction("mov QWORD PTR [r8], r9"); // publish the updated offset
    emitter.instruction("jmp __rt_strtr_return_linux_x86_64"); // return the translated string

    emitter.label("__rt_strtr_empty_linux_x86_64");
    emitter.instruction("xor rax, rax"); // empty string has no payload pointer
    emitter.instruction("xor rdx, rdx"); // empty string length is zero

    emitter.label("__rt_strtr_return_linux_x86_64");
    emitter.instruction("add rsp, 320"); // tear down the frame
    emitter.instruction("pop rbp"); // restore the caller frame pointer
    emitter.instruction("ret"); // return the translated string in rax/rdx
}
