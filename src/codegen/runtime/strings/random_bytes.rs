//! Purpose:
//! Emits the `__rt_random_bytes` runtime helper assembly for PHP `random_bytes`.
//! Keeps the CSPRNG source, string allocation, and target-specific ABI variants in one focused emitter.
//!
//! Called from:
//! - `crate::codegen::runtime::emitters::emit_runtime()` via `crate::codegen::runtime::strings`.
//! - The `random_bytes()` builtin lowering
//!   (`crate::codegen_ir::lower_inst::builtins::strings::lower_random_bytes`).
//!
//! Key details:
//! - Produces an owned byte string of exactly `$length` cryptographically random bytes.
//!   Storage mirrors `__rt_str_repeat`: the 64 KiB concat scratch when it fits, otherwise
//!   a heap allocation stamped as an owned string.
//! - Randomness: the Linux `getrandom` syscall (aarch64 #278, x86_64 #318), filled in a
//!   loop to tolerate short reads; macOS ARM64 uses libc `arc4random_buf`. If `getrandom`
//!   ever reports failure the remaining bytes are zero-filled so the result is always
//!   exactly `$length` defined bytes (never stale scratch/heap contents).

use crate::codegen::emit::Emitter;
use crate::codegen::platform::{Arch, Platform};

/// Owned-string heap kind word, x86_64 variant (matches `__rt_str_repeat`).
const X86_64_HEAP_MAGIC_HI32: u64 = 0x454C5048;

/// Emits the `__rt_random_bytes` runtime helper for the current platform.
///
/// ABI:
///   ARM64:  x0 = length → x1=result_ptr, x2=result_len
///   x86_64: rax = length → rax=result_ptr, rdx=result_len
pub fn emit_random_bytes(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_random_bytes_linux_x86_64(emitter);
        return;
    }
    emit_random_bytes_arm64(emitter);
}

/// Emits the ARM64 variant (Linux `getrandom` or macOS `arc4random_buf`).
fn emit_random_bytes_arm64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: random_bytes ---");
    emitter.label_global("__rt_random_bytes");

    // -- stack frame --
    emitter.instruction("sub sp, sp, #64"); // spill slots for result ptr/len/kind + frame save
    emitter.instruction("stp x29, x30, [sp, #48]"); // save frame pointer and return address
    emitter.instruction("add x29, sp, #48"); // establish the new frame pointer

    emitter.instruction("cbz x0, __rt_random_bytes_empty"); // length 0 → empty string
    emitter.instruction("mov x4, x0"); // x4 = length (result length)
    emitter.instruction("str x4, [sp, #8]"); // save result length

    // -- allocate the result buffer: concat scratch when it fits, else heap --
    crate::codegen::abi::emit_symbol_address(emitter, "x6", "_concat_off");
    emitter.instruction("ldr x12, [x6]"); // current concat scratch write offset
    emitter.instruction("add x13, x12, x4"); // end offset after this append
    emitter.instruction("mov x14, #65536"); // concat scratch capacity
    emitter.instruction("cmp x13, x14"); // does the buffer fit in concat scratch?
    emitter.instruction("b.hi __rt_random_bytes_heap"); // heap fallback on overflow
    crate::codegen::abi::emit_symbol_address(emitter, "x7", "_concat_buf");
    emitter.instruction("add x9, x7, x12"); // dest = concat_buf + off
    emitter.instruction("str x9, [sp, #0]"); // save result start pointer
    emitter.instruction("str xzr, [sp, #16]"); // mark result concat-backed
    emitter.instruction("b __rt_random_bytes_fill"); // skip heap allocation

    emitter.label("__rt_random_bytes_heap");
    emitter.instruction("mov x0, x4"); // requested payload size
    emitter.instruction("bl __rt_heap_alloc"); // allocate owned storage
    emitter.instruction("mov x6, #1"); // heap kind 1 = owned elephc string
    emitter.instruction("str x6, [x0, #-8]"); // stamp the allocation as a string payload
    emitter.instruction("str x0, [sp, #0]"); // save result start pointer
    emitter.instruction("mov x6, #1"); // storage kind = heap
    emitter.instruction("str x6, [sp, #16]"); // mark result heap-backed

    // -- fill with random bytes --
    emitter.label("__rt_random_bytes_fill");
    match emitter.platform {
        Platform::MacOS => {
            emitter.instruction("ldr x0, [sp, #0]"); // buf = result pointer
            emitter.instruction("ldr x1, [sp, #8]"); // len = length
            emitter.bl_c("arc4random_buf"); // fill buf with len CSPRNG bytes (void)
        }
        Platform::Linux => {
            emitter.instruction("ldr x9, [sp, #0]"); // cur = result pointer
            emitter.instruction("ldr x10, [sp, #8]"); // remaining = length
            emitter.label("__rt_random_bytes_getrandom");
            emitter.instruction("cbz x10, __rt_random_bytes_done"); // all bytes filled?
            emitter.instruction("mov x0, x9"); // buf = cur
            emitter.instruction("mov x1, x10"); // count = remaining
            emitter.instruction("mov x2, #0"); // flags = 0
            emitter.instruction("mov x8, #278"); // Linux aarch64 getrandom syscall number
            emitter.instruction("svc #0"); // ask the kernel for random bytes
            emitter.instruction("cmp x0, #0"); // n <= 0 (short read / failure)?
            emitter.instruction("b.le __rt_random_bytes_zerofill"); // zero-fill the remainder on failure
            emitter.instruction("add x9, x9, x0"); // cur += n
            emitter.instruction("sub x10, x10, x0"); // remaining -= n
            emitter.instruction("b __rt_random_bytes_getrandom"); // continue filling
            emitter.label("__rt_random_bytes_zerofill");
            emitter.instruction("cbz x10, __rt_random_bytes_done"); // remainder consumed?
            emitter.instruction("strb wzr, [x9], #1"); // write a zero byte, advance cursor
            emitter.instruction("sub x10, x10, #1"); // decrement the remaining count
            emitter.instruction("b __rt_random_bytes_zerofill"); // continue zero-filling
        }
    }

    // -- finalize: publish concat offset for concat-backed results --
    emitter.label("__rt_random_bytes_done");
    emitter.instruction("ldr x1, [sp, #0]"); // result pointer
    emitter.instruction("ldr x2, [sp, #8]"); // result length
    emitter.instruction("ldr x13, [sp, #16]"); // storage kind (0 concat, 1 heap)
    emitter.instruction("cbnz x13, __rt_random_bytes_return"); // heap results leave concat offset unchanged
    crate::codegen::abi::emit_symbol_address(emitter, "x6", "_concat_off");
    emitter.instruction("ldr x12, [x6]"); // reload concat scratch write offset
    emitter.instruction("add x12, x12, x2"); // advance by the result length
    emitter.instruction("str x12, [x6]"); // publish the updated offset
    emitter.instruction("b __rt_random_bytes_return"); // return the random string

    emitter.label("__rt_random_bytes_empty");
    emitter.instruction("mov x1, #0"); // empty string has no payload pointer
    emitter.instruction("mov x2, #0"); // empty string length is zero

    emitter.label("__rt_random_bytes_return");
    emitter.instruction("ldp x29, x30, [sp, #48]"); // restore frame pointer and return address
    emitter.instruction("add sp, sp, #64"); // tear down the frame
    emitter.instruction("ret"); // return the random string in x1/x2
}

/// Emits the `__rt_random_bytes` runtime helper for Linux x86_64 using the `getrandom` syscall.
///
/// ABI: rax = length → rax=result_ptr, rdx=result_len. The `syscall` instruction clobbers
/// rcx/r11, so the fill cursor and remaining count are kept on the stack across each call.
fn emit_random_bytes_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: random_bytes ---");
    emitter.label_global("__rt_random_bytes");

    emitter.instruction("push rbp"); // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp"); // establish a stable frame base
    emitter.instruction("sub rsp, 48"); // spill slots: result ptr/len/kind + fill cursor/remaining

    emitter.instruction("test rax, rax"); // length 0?
    emitter.instruction("jz __rt_random_bytes_empty_linux_x86_64"); // → empty string
    emitter.instruction("mov rcx, rax"); // rcx = length
    emitter.instruction("mov QWORD PTR [rbp - 16], rcx"); // save result length

    // -- allocate the result buffer: concat scratch when it fits, else heap --
    crate::codegen::abi::emit_symbol_address(emitter, "r8", "_concat_off");
    emitter.instruction("mov r9, QWORD PTR [r8]"); // current concat scratch write offset
    emitter.instruction("mov rdx, r9"); // copy before adding the result length
    emitter.instruction("add rdx, rcx"); // end offset after this append
    emitter.instruction("cmp rdx, 65536"); // does the buffer fit in concat scratch?
    emitter.instruction("ja __rt_random_bytes_heap_linux_x86_64"); // heap fallback on overflow
    crate::codegen::abi::emit_symbol_address(emitter, "r10", "_concat_buf");
    emitter.instruction("lea r11, [r10 + r9]"); // dest = concat_buf + off
    emitter.instruction("mov QWORD PTR [rbp - 8], r11"); // save result start pointer
    emitter.instruction("mov QWORD PTR [rbp - 24], 0"); // mark result concat-backed
    emitter.instruction("jmp __rt_random_bytes_fill_linux_x86_64"); // skip heap allocation

    emitter.label("__rt_random_bytes_heap_linux_x86_64");
    emitter.instruction("mov rax, rcx"); // requested payload size
    emitter.instruction("call __rt_heap_alloc"); // allocate owned storage
    emitter.instruction(&format!(
        "mov r10, 0x{:x}",
        (X86_64_HEAP_MAGIC_HI32 << 32) | 1
    )); // owned-string heap kind word with the x86_64 marker
    emitter.instruction("mov QWORD PTR [rax - 8], r10"); // stamp the allocation as a string payload
    emitter.instruction("mov QWORD PTR [rbp - 8], rax"); // save result start pointer
    emitter.instruction("mov QWORD PTR [rbp - 24], 1"); // mark result heap-backed

    // -- fill with random bytes via getrandom (cursor/remaining spilled across the syscall) --
    emitter.label("__rt_random_bytes_fill_linux_x86_64");
    emitter.instruction("mov r8, QWORD PTR [rbp - 8]"); // cur = result pointer
    emitter.instruction("mov r9, QWORD PTR [rbp - 16]"); // remaining = length
    emitter.instruction("mov QWORD PTR [rbp - 32], r8"); // save fill cursor
    emitter.instruction("mov QWORD PTR [rbp - 40], r9"); // save remaining count
    emitter.label("__rt_random_bytes_getrandom_linux_x86_64");
    emitter.instruction("mov r9, QWORD PTR [rbp - 40]"); // reload remaining
    emitter.instruction("test r9, r9"); // all bytes filled?
    emitter.instruction("jz __rt_random_bytes_done_linux_x86_64"); // yes → finalize
    emitter.instruction("mov rdi, QWORD PTR [rbp - 32]"); // buf = cur
    emitter.instruction("mov rsi, r9"); // count = remaining
    emitter.instruction("xor edx, edx"); // flags = 0
    emitter.instruction("mov eax, 318"); // Linux x86_64 getrandom syscall number
    emitter.instruction("syscall"); // ask the kernel for random bytes
    emitter.instruction("test rax, rax"); // n <= 0 (short read / failure)?
    emitter.instruction("jle __rt_random_bytes_zerofill_linux_x86_64"); // zero-fill the remainder on failure
    emitter.instruction("mov r8, QWORD PTR [rbp - 32]"); // reload cursor
    emitter.instruction("add r8, rax"); // cur += n
    emitter.instruction("mov QWORD PTR [rbp - 32], r8"); // save cursor
    emitter.instruction("mov r9, QWORD PTR [rbp - 40]"); // reload remaining
    emitter.instruction("sub r9, rax"); // remaining -= n
    emitter.instruction("mov QWORD PTR [rbp - 40], r9"); // save remaining
    emitter.instruction("jmp __rt_random_bytes_getrandom_linux_x86_64"); // continue filling

    emitter.label("__rt_random_bytes_zerofill_linux_x86_64");
    emitter.instruction("mov r8, QWORD PTR [rbp - 32]"); // cur
    emitter.instruction("mov r9, QWORD PTR [rbp - 40]"); // remaining
    emitter.label("__rt_random_bytes_zloop_linux_x86_64");
    emitter.instruction("test r9, r9"); // remainder consumed?
    emitter.instruction("jz __rt_random_bytes_done_linux_x86_64"); // yes → finalize
    emitter.instruction("mov BYTE PTR [r8], 0"); // write a zero byte
    emitter.instruction("inc r8"); // advance cursor
    emitter.instruction("dec r9"); // decrement remaining
    emitter.instruction("jmp __rt_random_bytes_zloop_linux_x86_64"); // continue zero-filling

    // -- finalize: publish concat offset for concat-backed results --
    emitter.label("__rt_random_bytes_done_linux_x86_64");
    emitter.instruction("mov rax, QWORD PTR [rbp - 8]"); // result pointer
    emitter.instruction("mov rdx, QWORD PTR [rbp - 16]"); // result length
    emitter.instruction("mov r8, QWORD PTR [rbp - 24]"); // storage kind (0 concat, 1 heap)
    emitter.instruction("test r8, r8"); // heap-backed?
    emitter.instruction("jnz __rt_random_bytes_return_linux_x86_64"); // heap results leave concat offset unchanged
    crate::codegen::abi::emit_symbol_address(emitter, "r8", "_concat_off");
    emitter.instruction("mov r9, QWORD PTR [r8]"); // reload concat scratch write offset
    emitter.instruction("add r9, rdx"); // advance by the result length
    emitter.instruction("mov QWORD PTR [r8], r9"); // publish the updated offset
    emitter.instruction("jmp __rt_random_bytes_return_linux_x86_64"); // return the random string

    emitter.label("__rt_random_bytes_empty_linux_x86_64");
    emitter.instruction("xor rax, rax"); // empty string has no payload pointer
    emitter.instruction("xor rdx, rdx"); // empty string length is zero

    emitter.label("__rt_random_bytes_return_linux_x86_64");
    emitter.instruction("add rsp, 48"); // tear down the frame
    emitter.instruction("pop rbp"); // restore the caller frame pointer
    emitter.instruction("ret"); // return the random string in rax/rdx
}
