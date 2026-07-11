//! Purpose:
//! Emits the `__rt_getmypid` runtime helper assembly for getmypid.
//! Keeps PHP builtin semantics and target-specific ABI variants in one focused emitter.
//!
//! Called from:
//! - `crate::codegen::runtime::emitters::emit_runtime()` via `crate::codegen::runtime::system`.
//!
//! Key details:
//! - Routes through libc `getpid()` on every target (like `__rt_time`'s x86_64/macOS
//!   paths) rather than a raw syscall, so the helper stays free of per-arch syscall
//!   numbers. `pid_t` is a signed 32-bit `int`, so the result is sign-extended to the
//!   64-bit native integer register PHP ints occupy.

use crate::codegen::emit::Emitter;
use crate::codegen::platform::Arch;

/// Emits the `__rt_getmypid` runtime helper for the current platform.
/// Routes to the ARM64 or x86_64 variant. Output: x0 (ARM64) or rax (x86_64) = current process id.
pub(crate) fn emit_getmypid(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_getmypid_x86_64(emitter);
        return;
    }
    emit_getmypid_arm64(emitter);
}

/// Emits `__rt_getmypid` for ARM64 via libc `getpid()`.
/// Output: x0 = current process id (sign-extended from the 32-bit `pid_t`).
fn emit_getmypid_arm64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: getmypid ---");
    emitter.label_global("__rt_getmypid");

    emitter.instruction("sub sp, sp, #16"); // allocate a minimal 16-aligned frame for the libc call
    emitter.instruction("stp x29, x30, [sp]"); // save frame pointer and return address
    emitter.instruction("mov x29, sp"); // establish the new frame pointer
    emitter.bl_c("getpid"); // libc getpid() → w0 = process id
    emitter.instruction("sxtw x0, w0"); // widen the signed 32-bit pid_t to the 64-bit PHP int register
    emitter.instruction("ldp x29, x30, [sp]"); // restore frame pointer and return address
    emitter.instruction("add sp, sp, #16"); // tear down the frame
    emitter.instruction("ret"); // return the pid to the caller
}

/// Emits `__rt_getmypid` for x86_64 via libc `getpid()`.
/// Output: rax = current process id (sign-extended from the 32-bit `pid_t`).
fn emit_getmypid_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: getmypid ---");
    emitter.label_global("__rt_getmypid");

    emitter.instruction("push rbp"); // preserve the caller frame pointer for the libc call
    emitter.instruction("mov rbp, rsp"); // establish a stable frame base (keeps rsp 16-aligned at the call)
    emitter.bl_c("getpid"); // libc getpid() → eax = process id
    emitter.instruction("cdqe"); // sign-extend eax → rax so the 64-bit PHP int register is clean
    emitter.instruction("leave"); // restore the caller frame pointer
    emitter.instruction("ret"); // return the pid to the caller
}
