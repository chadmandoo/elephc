//! Purpose:
//! Emits the output-buffering (`ob_*`) runtime helpers that maintain the stack of
//! capture buffers behind PHP's output-control builtins: `__rt_ob_start`,
//! `__rt_ob_append`, `__rt_ob_contents`, `__rt_ob_length`, `__rt_ob_level`,
//! `__rt_ob_clean`, `__rt_ob_end_clean`, `__rt_ob_flush`, `__rt_ob_end_flush`,
//! and `__rt_ob_flush_all`.
//!
//! Called from:
//! - `crate::codegen_support::runtime::emitters::emit_runtime()` via
//!   `crate::codegen_support::runtime::io`.
//! - `__rt_stdout_write` and `__rt_pr_write` call `__rt_ob_append` while
//!   `_ob_level` is non-zero so every terminal write is captured.
//! - `crate::codegen_support::abi::emit_exit`/`emit_exit_with_result_reg` call
//!   `__rt_ob_flush_all` so still-active buffers reach stdout at process exit.
//!
//! Key details:
//! - State lives in the fixed runtime data section: `_ob_level` plus the 64-slot
//!   parallel arrays `_ob_ptrs`/`_ob_lens`/`_ob_caps` indexed by level-1.
//! - Buffers are heap blocks from `__rt_heap_alloc`; `__rt_ob_append` doubles the
//!   capacity (copy + `__rt_heap_free` of the old block) until the payload fits,
//!   so buffered output is not clamped to a fixed scratch size.
//! - Flushing to the parent sink temporarily decrements `_ob_level` and routes the
//!   bytes back through `__rt_stdout_write`, which re-dispatches to the parent
//!   buffer, the `--web` capture, or the plain `write(1, …)` syscall.
//! - `__rt_ob_flush_all` zeroes `_ob_level` before writing so a fatal-path
//!   re-entry is a no-op and the writes reach the terminal sink directly.

use crate::codegen_support::abi;
use crate::codegen_support::{emit::Emitter, platform::Arch};

/// Maximum output-buffer nesting depth (slots in `_ob_ptrs`/`_ob_lens`/`_ob_caps`).
pub(crate) const OB_MAX_LEVELS: i64 = 64;

/// Initial capacity in bytes of a freshly started output buffer.
const OB_INITIAL_CAPACITY: i64 = 1024;

/// Emits `__rt_ob_start`: push a new output buffer onto the stack.
///
/// No inputs. Returns 1 in `x0`/`rax` on success, 0 when the nesting limit is
/// reached. Allocates the initial heap buffer and records it in the top slot.
pub fn emit_ob_start(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_ob_start_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: ob_start ---");
    emitter.label_global("__rt_ob_start");
    emitter.instruction("sub sp, sp, #32");                                     // allocate the ob_start frame
    emitter.instruction("stp x29, x30, [sp, #16]");                             // save frame pointer and return address
    emitter.instruction("add x29, sp, #16");                                    // establish the ob_start frame pointer
    abi::emit_symbol_address(emitter, "x9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("ldr x10, [x9]");                                       // load the current buffer-stack depth
    emitter.instruction(&format!("cmp x10, #{}", OB_MAX_LEVELS));               // is the nesting limit reached?
    emitter.instruction("b.ge __rt_ob_start_fail");                             // refuse to push past the nesting limit
    emitter.instruction("str x10, [sp, #0]");                                   // save the current depth (the new slot index)
    emitter.instruction(&format!("mov x0, #{}", OB_INITIAL_CAPACITY));          // request the initial buffer capacity
    emitter.instruction("bl __rt_heap_alloc");                                  // allocate the new capture buffer (raw kind, per heap_alloc contract)
    emitter.instruction("ldr x10, [sp, #0]");                                   // reload the new slot index
    abi::emit_symbol_address(emitter, "x11", "_ob_ptrs");                       // materialize the buffer-pointer slot array
    emitter.instruction("str x0, [x11, x10, lsl #3]");                          // record the new buffer base pointer
    abi::emit_symbol_address(emitter, "x11", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("str xzr, [x11, x10, lsl #3]");                         // the new buffer starts empty
    abi::emit_symbol_address(emitter, "x11", "_ob_caps");                       // materialize the capacity slot array
    emitter.instruction(&format!("mov x12, #{}", OB_INITIAL_CAPACITY));         // load the initial capacity value
    emitter.instruction("str x12, [x11, x10, lsl #3]");                         // record the initial capacity
    emitter.instruction("add x10, x10, #1");                                    // the stack is now one level deeper
    abi::emit_symbol_address(emitter, "x9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("str x10, [x9]");                                       // publish the new depth
    emitter.instruction("mov x0, #1");                                          // report success
    emitter.instruction("b __rt_ob_start_done");                                // skip the failure path
    emitter.label("__rt_ob_start_fail");
    emitter.instruction("mov x0, #0");                                          // report failure (nesting limit reached)
    emitter.label("__rt_ob_start_done");
    emitter.instruction("ldp x29, x30, [sp, #16]");                             // restore frame pointer and return address
    emitter.instruction("add sp, sp, #32");                                     // release the ob_start frame
    emitter.instruction("ret");                                                 // return the success flag
}

/// Emits the Linux x86_64 variant of `__rt_ob_start`.
fn emit_ob_start_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: ob_start ---");
    emitter.label_global("__rt_ob_start");
    emitter.instruction("push rbp");                                            // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish the ob_start frame pointer
    emitter.instruction("sub rsp, 16");                                         // reserve an aligned slot for the new slot index
    abi::emit_symbol_address(emitter, "r9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("mov r10, QWORD PTR [r9]");                             // load the current buffer-stack depth
    emitter.instruction(&format!("cmp r10, {}", OB_MAX_LEVELS));                // is the nesting limit reached?
    emitter.instruction("jge __rt_ob_start_fail_x86");                          // refuse to push past the nesting limit
    emitter.instruction("mov QWORD PTR [rbp - 8], r10");                        // save the current depth (the new slot index)
    emitter.instruction(&format!("mov rax, {}", OB_INITIAL_CAPACITY));          // request the initial buffer capacity
    emitter.instruction("call __rt_heap_alloc");                                // allocate the new capture buffer (raw kind, per heap_alloc contract)
    emitter.instruction("mov r10, QWORD PTR [rbp - 8]");                        // reload the new slot index
    abi::emit_symbol_address(emitter, "r11", "_ob_ptrs");                       // materialize the buffer-pointer slot array
    emitter.instruction("mov QWORD PTR [r11 + r10*8], rax");                    // record the new buffer base pointer
    abi::emit_symbol_address(emitter, "r11", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("mov QWORD PTR [r11 + r10*8], 0");                      // the new buffer starts empty
    abi::emit_symbol_address(emitter, "r11", "_ob_caps");                       // materialize the capacity slot array
    emitter.instruction(&format!(
        "mov QWORD PTR [r11 + r10*8], {}",
        OB_INITIAL_CAPACITY
    ));                                                                         // record the initial capacity
    emitter.instruction("add r10, 1");                                          // the stack is now one level deeper
    abi::emit_symbol_address(emitter, "r9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("mov QWORD PTR [r9], r10");                             // publish the new depth
    emitter.instruction("mov eax, 1");                                          // report success
    emitter.instruction("jmp __rt_ob_start_done_x86");                          // skip the failure path
    emitter.label("__rt_ob_start_fail_x86");
    emitter.instruction("xor eax, eax");                                        // report failure (nesting limit reached)
    emitter.label("__rt_ob_start_done_x86");
    emitter.instruction("add rsp, 16");                                         // release the ob_start frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the success flag
}

/// Emits `__rt_ob_append`: append bytes to the top output buffer, growing it as
/// needed.
///
/// Inputs: AArch64 `x0`=src pointer, `x1`=length / x86_64 `rdi`=src, `rsi`=length.
/// No result. A zero `_ob_level` makes the call a defensive no-op. Growth doubles
/// the capacity until the payload fits, copies the used prefix, and frees the old
/// block.
pub fn emit_ob_append(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_ob_append_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: ob_append ---");
    emitter.label_global("__rt_ob_append");
    // frame: [0]=src, [8]=len, [16]=slot index, [24]=used, [32]=new capacity
    emitter.instruction("sub sp, sp, #64");                                     // allocate the ob_append frame
    emitter.instruction("stp x29, x30, [sp, #48]");                             // save frame pointer and return address
    emitter.instruction("add x29, sp, #48");                                    // establish the ob_append frame pointer
    emitter.instruction("str x0, [sp, #0]");                                    // save the source byte pointer
    emitter.instruction("str x1, [sp, #8]");                                    // save the source byte length
    abi::emit_symbol_address(emitter, "x9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("ldr x10, [x9]");                                       // load the current buffer-stack depth
    emitter.instruction("cbz x10, __rt_ob_append_done");                        // no active buffer — defensive no-op
    emitter.instruction("sub x10, x10, #1");                                    // top slot index = depth - 1
    emitter.instruction("str x10, [sp, #16]");                                  // save the top slot index
    abi::emit_symbol_address(emitter, "x11", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("ldr x12, [x11, x10, lsl #3]");                         // load the top buffer's used byte count
    emitter.instruction("str x12, [sp, #24]");                                  // save the used byte count
    abi::emit_symbol_address(emitter, "x13", "_ob_caps");                       // materialize the capacity slot array
    emitter.instruction("ldr x14, [x13, x10, lsl #3]");                         // load the top buffer's capacity
    emitter.instruction("ldr x15, [sp, #8]");                                   // reload the incoming byte length
    emitter.instruction("add x15, x12, x15");                                   // needed bytes = used + incoming length
    emitter.instruction("cmp x15, x14");                                        // does the payload fit the current capacity?
    emitter.instruction("b.ls __rt_ob_append_copy");                            // fits — skip the growth path
    // -- grow: double the capacity until the payload fits --
    emitter.label("__rt_ob_append_grow_size");
    emitter.instruction("lsl x14, x14, #1");                                    // double the candidate capacity
    emitter.instruction("cmp x14, x15");                                        // does the doubled capacity fit the payload?
    emitter.instruction("b.lo __rt_ob_append_grow_size");                       // keep doubling until it fits
    emitter.instruction("str x14, [sp, #32]");                                  // save the new capacity
    emitter.instruction("mov x0, x14");                                         // request the new capacity from the allocator
    emitter.instruction("bl __rt_heap_alloc");                                  // allocate the replacement buffer (raw kind, per heap_alloc contract)
    // -- copy the used prefix from the old buffer into the replacement --
    emitter.instruction("ldr x10, [sp, #16]");                                  // reload the top slot index
    abi::emit_symbol_address(emitter, "x11", "_ob_ptrs");                       // materialize the buffer-pointer slot array
    emitter.instruction("ldr x12, [x11, x10, lsl #3]");                         // load the old buffer base pointer
    emitter.instruction("ldr x13, [sp, #24]");                                  // reload the used byte count
    emitter.instruction("mov x14, #0");                                         // start the copy at offset zero
    emitter.label("__rt_ob_append_grow_copy");
    emitter.instruction("cmp x14, x13");                                        // copied all used bytes?
    emitter.instruction("b.ge __rt_ob_append_grow_swap");                       // yes — publish the replacement buffer
    emitter.instruction("ldrb w15, [x12, x14]");                                // load the next byte from the old buffer
    emitter.instruction("strb w15, [x0, x14]");                                 // store the byte into the replacement buffer
    emitter.instruction("add x14, x14, #1");                                    // advance the copy cursor
    emitter.instruction("b __rt_ob_append_grow_copy");                          // continue copying the used prefix
    emitter.label("__rt_ob_append_grow_swap");
    emitter.instruction("str x0, [x11, x10, lsl #3]");                          // publish the replacement buffer base pointer
    abi::emit_symbol_address(emitter, "x13", "_ob_caps");                       // materialize the capacity slot array
    emitter.instruction("ldr x14, [sp, #32]");                                  // reload the new capacity
    emitter.instruction("str x14, [x13, x10, lsl #3]");                         // publish the new capacity
    emitter.instruction("mov x0, x12");                                         // pass the old buffer to the deallocator
    emitter.instruction("bl __rt_heap_free");                                   // release the old buffer block
    // -- copy the incoming bytes to buffer base + used --
    emitter.label("__rt_ob_append_copy");
    emitter.instruction("ldr x10, [sp, #16]");                                  // reload the top slot index
    abi::emit_symbol_address(emitter, "x11", "_ob_ptrs");                       // materialize the buffer-pointer slot array
    emitter.instruction("ldr x12, [x11, x10, lsl #3]");                         // load the (possibly replaced) buffer base pointer
    emitter.instruction("ldr x13, [sp, #24]");                                  // reload the used byte count
    emitter.instruction("add x12, x12, x13");                                   // destination cursor = base + used
    emitter.instruction("ldr x14, [sp, #0]");                                   // reload the source byte pointer
    emitter.instruction("ldr x15, [sp, #8]");                                   // reload the source byte length
    emitter.label("__rt_ob_append_copy_loop");
    emitter.instruction("cbz x15, __rt_ob_append_publish");                     // no bytes left — publish the new length
    emitter.instruction("ldrb w9, [x14]");                                      // load the next source byte
    emitter.instruction("strb w9, [x12]");                                      // store the byte at the destination cursor
    emitter.instruction("add x14, x14, #1");                                    // advance the source cursor
    emitter.instruction("add x12, x12, #1");                                    // advance the destination cursor
    emitter.instruction("sub x15, x15, #1");                                    // one byte fewer to copy
    emitter.instruction("b __rt_ob_append_copy_loop");                          // continue copying the incoming bytes
    emitter.label("__rt_ob_append_publish");
    emitter.instruction("ldr x13, [sp, #24]");                                  // reload the used byte count
    emitter.instruction("ldr x14, [sp, #8]");                                   // reload the incoming byte length
    emitter.instruction("add x13, x13, x14");                                   // new used count = used + incoming length
    emitter.instruction("ldr x10, [sp, #16]");                                  // reload the top slot index
    abi::emit_symbol_address(emitter, "x11", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("str x13, [x11, x10, lsl #3]");                         // publish the new used byte count
    emitter.label("__rt_ob_append_done");
    emitter.instruction("ldp x29, x30, [sp, #48]");                             // restore frame pointer and return address
    emitter.instruction("add sp, sp, #64");                                     // release the ob_append frame
    emitter.instruction("ret");                                                 // return to caller
}

/// Emits the Linux x86_64 variant of `__rt_ob_append`.
fn emit_ob_append_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: ob_append ---");
    emitter.label_global("__rt_ob_append");
    // frame: [rbp-8]=src, [rbp-16]=len, [rbp-24]=slot index, [rbp-32]=used, [rbp-40]=new capacity
    emitter.instruction("push rbp");                                            // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish the ob_append frame pointer
    emitter.instruction("sub rsp, 48");                                         // reserve the ob_append local slots (16-aligned)
    emitter.instruction("mov QWORD PTR [rbp - 8], rdi");                        // save the source byte pointer
    emitter.instruction("mov QWORD PTR [rbp - 16], rsi");                       // save the source byte length
    abi::emit_symbol_address(emitter, "r9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("mov r10, QWORD PTR [r9]");                             // load the current buffer-stack depth
    emitter.instruction("test r10, r10");                                       // is any buffer active?
    emitter.instruction("jz __rt_ob_append_done_x86");                          // no active buffer — defensive no-op
    emitter.instruction("sub r10, 1");                                          // top slot index = depth - 1
    emitter.instruction("mov QWORD PTR [rbp - 24], r10");                       // save the top slot index
    abi::emit_symbol_address(emitter, "r11", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("mov rcx, QWORD PTR [r11 + r10*8]");                    // load the top buffer's used byte count
    emitter.instruction("mov QWORD PTR [rbp - 32], rcx");                       // save the used byte count
    abi::emit_symbol_address(emitter, "r11", "_ob_caps");                       // materialize the capacity slot array
    emitter.instruction("mov rdx, QWORD PTR [r11 + r10*8]");                    // load the top buffer's capacity
    emitter.instruction("mov r8, QWORD PTR [rbp - 16]");                        // reload the incoming byte length
    emitter.instruction("add r8, rcx");                                         // needed bytes = used + incoming length
    emitter.instruction("cmp r8, rdx");                                         // does the payload fit the current capacity?
    emitter.instruction("jbe __rt_ob_append_copy_x86");                         // fits — skip the growth path
    // -- grow: double the capacity until the payload fits --
    emitter.label("__rt_ob_append_grow_size_x86");
    emitter.instruction("shl rdx, 1");                                          // double the candidate capacity
    emitter.instruction("cmp rdx, r8");                                         // does the doubled capacity fit the payload?
    emitter.instruction("jb __rt_ob_append_grow_size_x86");                     // keep doubling until it fits
    emitter.instruction("mov QWORD PTR [rbp - 40], rdx");                       // save the new capacity
    emitter.instruction("mov rax, rdx");                                        // request the new capacity from the allocator
    emitter.instruction("call __rt_heap_alloc");                                // allocate the replacement buffer (raw kind, per heap_alloc contract)
    // -- copy the used prefix from the old buffer into the replacement --
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the top slot index
    abi::emit_symbol_address(emitter, "r11", "_ob_ptrs");                       // materialize the buffer-pointer slot array
    emitter.instruction("mov rcx, QWORD PTR [r11 + r10*8]");                    // load the old buffer base pointer
    emitter.instruction("mov rdx, QWORD PTR [rbp - 32]");                       // reload the used byte count
    emitter.instruction("xor r8d, r8d");                                        // start the copy at offset zero
    emitter.label("__rt_ob_append_grow_copy_x86");
    emitter.instruction("cmp r8, rdx");                                         // copied all used bytes?
    emitter.instruction("jge __rt_ob_append_grow_swap_x86");                    // yes — publish the replacement buffer
    emitter.instruction("mov r9b, BYTE PTR [rcx + r8]");                        // load the next byte from the old buffer
    emitter.instruction("mov BYTE PTR [rax + r8], r9b");                        // store the byte into the replacement buffer
    emitter.instruction("add r8, 1");                                           // advance the copy cursor
    emitter.instruction("jmp __rt_ob_append_grow_copy_x86");                    // continue copying the used prefix
    emitter.label("__rt_ob_append_grow_swap_x86");
    emitter.instruction("mov QWORD PTR [r11 + r10*8], rax");                    // publish the replacement buffer base pointer
    abi::emit_symbol_address(emitter, "r9", "_ob_caps");                        // materialize the capacity slot array
    emitter.instruction("mov rdx, QWORD PTR [rbp - 40]");                       // reload the new capacity
    emitter.instruction("mov QWORD PTR [r9 + r10*8], rdx");                     // publish the new capacity
    emitter.instruction("mov rax, rcx");                                        // pass the old buffer to the deallocator
    emitter.instruction("call __rt_heap_free");                                 // release the old buffer block
    // -- copy the incoming bytes to buffer base + used --
    emitter.label("__rt_ob_append_copy_x86");
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the top slot index
    abi::emit_symbol_address(emitter, "r11", "_ob_ptrs");                       // materialize the buffer-pointer slot array
    emitter.instruction("mov rcx, QWORD PTR [r11 + r10*8]");                    // load the (possibly replaced) buffer base pointer
    emitter.instruction("add rcx, QWORD PTR [rbp - 32]");                       // destination cursor = base + used
    emitter.instruction("mov rdx, QWORD PTR [rbp - 8]");                        // reload the source byte pointer
    emitter.instruction("mov r8, QWORD PTR [rbp - 16]");                        // reload the source byte length
    emitter.label("__rt_ob_append_copy_loop_x86");
    emitter.instruction("test r8, r8");                                         // any bytes left to copy?
    emitter.instruction("jz __rt_ob_append_publish_x86");                       // no bytes left — publish the new length
    emitter.instruction("mov r9b, BYTE PTR [rdx]");                             // load the next source byte
    emitter.instruction("mov BYTE PTR [rcx], r9b");                             // store the byte at the destination cursor
    emitter.instruction("add rdx, 1");                                          // advance the source cursor
    emitter.instruction("add rcx, 1");                                          // advance the destination cursor
    emitter.instruction("sub r8, 1");                                           // one byte fewer to copy
    emitter.instruction("jmp __rt_ob_append_copy_loop_x86");                    // continue copying the incoming bytes
    emitter.label("__rt_ob_append_publish_x86");
    emitter.instruction("mov rcx, QWORD PTR [rbp - 32]");                       // reload the used byte count
    emitter.instruction("add rcx, QWORD PTR [rbp - 16]");                       // new used count = used + incoming length
    emitter.instruction("mov r10, QWORD PTR [rbp - 24]");                       // reload the top slot index
    abi::emit_symbol_address(emitter, "r11", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("mov QWORD PTR [r11 + r10*8], rcx");                    // publish the new used byte count
    emitter.label("__rt_ob_append_done_x86");
    emitter.instruction("add rsp, 48");                                         // release the ob_append local slots
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return to caller
}

/// Emits `__rt_ob_contents`: return a persisted copy of the top buffer contents.
///
/// No inputs. Returns the platform string result pair (AArch64 `x1`=ptr, `x2`=len /
/// x86_64 `rax`=ptr, `rdx`=len); a null pointer signals "no active buffer" so the
/// caller can box PHP `false`.
pub fn emit_ob_contents(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_ob_contents_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: ob_contents ---");
    emitter.label_global("__rt_ob_contents");
    emitter.instruction("stp x29, x30, [sp, #-16]!");                           // save frame pointer and return address
    emitter.instruction("mov x29, sp");                                         // establish a frame pointer for the persist call
    abi::emit_symbol_address(emitter, "x9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("ldr x10, [x9]");                                       // load the current buffer-stack depth
    emitter.instruction("cbz x10, __rt_ob_contents_none");                      // no active buffer — return the null failure pair
    emitter.instruction("sub x10, x10, #1");                                    // top slot index = depth - 1
    abi::emit_symbol_address(emitter, "x11", "_ob_ptrs");                       // materialize the buffer-pointer slot array
    emitter.instruction("ldr x1, [x11, x10, lsl #3]");                          // load the top buffer base pointer
    abi::emit_symbol_address(emitter, "x11", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("ldr x2, [x11, x10, lsl #3]");                          // load the top buffer's used byte count
    emitter.instruction("bl __rt_str_persist");                                 // copy the contents into an owned heap string
    emitter.instruction("b __rt_ob_contents_done");                             // return the persisted string pair
    emitter.label("__rt_ob_contents_none");
    emitter.instruction("mov x1, #0");                                          // null pointer signals "no active buffer"
    emitter.instruction("mov x2, #0");                                          // zero length for the failure pair
    emitter.label("__rt_ob_contents_done");
    emitter.instruction("ldp x29, x30, [sp], #16");                             // restore frame pointer and return address
    emitter.instruction("ret");                                                 // return the string result pair
}

/// Emits the Linux x86_64 variant of `__rt_ob_contents`.
fn emit_ob_contents_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: ob_contents ---");
    emitter.label_global("__rt_ob_contents");
    emitter.instruction("push rbp");                                            // preserve the caller frame pointer across the persist call
    emitter.instruction("mov rbp, rsp");                                        // establish a frame pointer for the persist call
    abi::emit_symbol_address(emitter, "r9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("mov r10, QWORD PTR [r9]");                             // load the current buffer-stack depth
    emitter.instruction("test r10, r10");                                       // is any buffer active?
    emitter.instruction("jz __rt_ob_contents_none_x86");                        // no active buffer — return the null failure pair
    emitter.instruction("sub r10, 1");                                          // top slot index = depth - 1
    abi::emit_symbol_address(emitter, "r11", "_ob_ptrs");                       // materialize the buffer-pointer slot array
    emitter.instruction("mov rax, QWORD PTR [r11 + r10*8]");                    // load the top buffer base pointer
    abi::emit_symbol_address(emitter, "r11", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("mov rdx, QWORD PTR [r11 + r10*8]");                    // load the top buffer's used byte count
    emitter.instruction("call __rt_str_persist");                               // copy the contents into an owned heap string
    emitter.instruction("jmp __rt_ob_contents_done_x86");                       // return the persisted string pair
    emitter.label("__rt_ob_contents_none_x86");
    emitter.instruction("xor eax, eax");                                        // null pointer signals "no active buffer"
    emitter.instruction("xor edx, edx");                                        // zero length for the failure pair
    emitter.label("__rt_ob_contents_done_x86");
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the string result pair
}

/// Emits `__rt_ob_length` and `__rt_ob_level`: the buffer-stack integer queries.
///
/// `__rt_ob_length` returns the top buffer's used byte count in `x0`/`rax`, or -1
/// when no buffer is active. `__rt_ob_level` returns the nesting depth. Neither
/// takes inputs nor calls other helpers.
pub fn emit_ob_queries(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_ob_queries_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: ob_length / ob_level ---");
    emitter.label_global("__rt_ob_length");
    abi::emit_symbol_address(emitter, "x9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("ldr x10, [x9]");                                       // load the current buffer-stack depth
    emitter.instruction("cbz x10, __rt_ob_length_none");                        // no active buffer — return the -1 sentinel
    emitter.instruction("sub x10, x10, #1");                                    // top slot index = depth - 1
    abi::emit_symbol_address(emitter, "x11", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("ldr x0, [x11, x10, lsl #3]");                          // return the top buffer's used byte count
    emitter.instruction("ret");                                                 // return the length
    emitter.label("__rt_ob_length_none");
    emitter.instruction("mov x0, #-1");                                         // -1 signals "no active buffer"
    emitter.instruction("ret");                                                 // return the failure sentinel

    emitter.blank();
    emitter.label_global("__rt_ob_level");
    abi::emit_symbol_address(emitter, "x9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("ldr x0, [x9]");                                        // return the current nesting depth
    emitter.instruction("ret");                                                 // return the level
}

/// Emits the Linux x86_64 variants of `__rt_ob_length` and `__rt_ob_level`.
fn emit_ob_queries_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: ob_length / ob_level ---");
    emitter.label_global("__rt_ob_length");
    abi::emit_symbol_address(emitter, "r9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("mov r10, QWORD PTR [r9]");                             // load the current buffer-stack depth
    emitter.instruction("test r10, r10");                                       // is any buffer active?
    emitter.instruction("jz __rt_ob_length_none_x86");                          // no active buffer — return the -1 sentinel
    emitter.instruction("sub r10, 1");                                          // top slot index = depth - 1
    abi::emit_symbol_address(emitter, "r11", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("mov rax, QWORD PTR [r11 + r10*8]");                    // return the top buffer's used byte count
    emitter.instruction("ret");                                                 // return the length
    emitter.label("__rt_ob_length_none_x86");
    emitter.instruction("mov rax, -1");                                         // -1 signals "no active buffer"
    emitter.instruction("ret");                                                 // return the failure sentinel

    emitter.blank();
    emitter.label_global("__rt_ob_level");
    abi::emit_symbol_address(emitter, "r9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("mov rax, QWORD PTR [r9]");                             // return the current nesting depth
    emitter.instruction("ret");                                                 // return the level
}

/// Emits `__rt_ob_clean`: discard the top buffer's contents but keep the buffer.
///
/// No inputs. Returns 1 in `x0`/`rax` on success, 0 when no buffer is active.
pub fn emit_ob_clean(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_ob_clean_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: ob_clean ---");
    emitter.label_global("__rt_ob_clean");
    abi::emit_symbol_address(emitter, "x9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("ldr x10, [x9]");                                       // load the current buffer-stack depth
    emitter.instruction("cbz x10, __rt_ob_clean_fail");                         // no active buffer — report failure
    emitter.instruction("sub x10, x10, #1");                                    // top slot index = depth - 1
    abi::emit_symbol_address(emitter, "x11", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("str xzr, [x11, x10, lsl #3]");                         // truncate the top buffer to zero bytes
    emitter.instruction("mov x0, #1");                                          // report success
    emitter.instruction("ret");                                                 // return the success flag
    emitter.label("__rt_ob_clean_fail");
    emitter.instruction("mov x0, #0");                                          // report failure (no active buffer)
    emitter.instruction("ret");                                                 // return the failure flag
}

/// Emits the Linux x86_64 variant of `__rt_ob_clean`.
fn emit_ob_clean_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: ob_clean ---");
    emitter.label_global("__rt_ob_clean");
    abi::emit_symbol_address(emitter, "r9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("mov r10, QWORD PTR [r9]");                             // load the current buffer-stack depth
    emitter.instruction("test r10, r10");                                       // is any buffer active?
    emitter.instruction("jz __rt_ob_clean_fail_x86");                           // no active buffer — report failure
    emitter.instruction("sub r10, 1");                                          // top slot index = depth - 1
    abi::emit_symbol_address(emitter, "r11", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("mov QWORD PTR [r11 + r10*8], 0");                      // truncate the top buffer to zero bytes
    emitter.instruction("mov eax, 1");                                          // report success
    emitter.instruction("ret");                                                 // return the success flag
    emitter.label("__rt_ob_clean_fail_x86");
    emitter.instruction("xor eax, eax");                                        // report failure (no active buffer)
    emitter.instruction("ret");                                                 // return the failure flag
}

/// Emits `__rt_ob_end_clean`: discard the top buffer and pop the stack.
///
/// No inputs. Returns 1 in `x0`/`rax` on success, 0 when no buffer is active.
/// Publishes the decremented depth before freeing so a fatal inside the
/// deallocator can no longer observe the dying buffer.
pub fn emit_ob_end_clean(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_ob_end_clean_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: ob_end_clean ---");
    emitter.label_global("__rt_ob_end_clean");
    emitter.instruction("stp x29, x30, [sp, #-16]!");                           // save frame pointer and return address
    emitter.instruction("mov x29, sp");                                         // establish a frame pointer for the free call
    abi::emit_symbol_address(emitter, "x9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("ldr x10, [x9]");                                       // load the current buffer-stack depth
    emitter.instruction("cbz x10, __rt_ob_end_clean_fail");                     // no active buffer — report failure
    emitter.instruction("sub x10, x10, #1");                                    // new depth = old depth - 1 (also the dying slot index)
    emitter.instruction("str x10, [x9]");                                       // publish the popped depth before freeing
    abi::emit_symbol_address(emitter, "x11", "_ob_ptrs");                       // materialize the buffer-pointer slot array
    emitter.instruction("ldr x0, [x11, x10, lsl #3]");                          // load the dying buffer base pointer
    emitter.instruction("bl __rt_heap_free");                                   // release the dying buffer block
    emitter.instruction("mov x0, #1");                                          // report success
    emitter.instruction("b __rt_ob_end_clean_done");                            // skip the failure path
    emitter.label("__rt_ob_end_clean_fail");
    emitter.instruction("mov x0, #0");                                          // report failure (no active buffer)
    emitter.label("__rt_ob_end_clean_done");
    emitter.instruction("ldp x29, x30, [sp], #16");                             // restore frame pointer and return address
    emitter.instruction("ret");                                                 // return the success flag
}

/// Emits the Linux x86_64 variant of `__rt_ob_end_clean`.
fn emit_ob_end_clean_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: ob_end_clean ---");
    emitter.label_global("__rt_ob_end_clean");
    emitter.instruction("push rbp");                                            // preserve the caller frame pointer across the free call
    emitter.instruction("mov rbp, rsp");                                        // establish a frame pointer for the free call
    abi::emit_symbol_address(emitter, "r9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("mov r10, QWORD PTR [r9]");                             // load the current buffer-stack depth
    emitter.instruction("test r10, r10");                                       // is any buffer active?
    emitter.instruction("jz __rt_ob_end_clean_fail_x86");                       // no active buffer — report failure
    emitter.instruction("sub r10, 1");                                          // new depth = old depth - 1 (also the dying slot index)
    emitter.instruction("mov QWORD PTR [r9], r10");                             // publish the popped depth before freeing
    abi::emit_symbol_address(emitter, "r11", "_ob_ptrs");                       // materialize the buffer-pointer slot array
    emitter.instruction("mov rax, QWORD PTR [r11 + r10*8]");                    // load the dying buffer base pointer
    emitter.instruction("call __rt_heap_free");                                 // release the dying buffer block
    emitter.instruction("mov eax, 1");                                          // report success
    emitter.instruction("jmp __rt_ob_end_clean_done_x86");                      // skip the failure path
    emitter.label("__rt_ob_end_clean_fail_x86");
    emitter.instruction("xor eax, eax");                                        // report failure (no active buffer)
    emitter.label("__rt_ob_end_clean_done_x86");
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the success flag
}

/// Emits `__rt_ob_flush`: write the top buffer's contents to the parent sink and
/// truncate the buffer, keeping the level.
///
/// No inputs. Returns 1 in `x0`/`rax` on success, 0 when no buffer is active.
/// Temporarily decrements `_ob_level` around the `__rt_stdout_write` call so the
/// bytes route to the parent buffer, the `--web` capture, or the terminal.
pub fn emit_ob_flush(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_ob_flush_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: ob_flush ---");
    emitter.label_global("__rt_ob_flush");
    emitter.instruction("sub sp, sp, #32");                                     // allocate the ob_flush frame
    emitter.instruction("stp x29, x30, [sp, #16]");                             // save frame pointer and return address
    emitter.instruction("add x29, sp, #16");                                    // establish the ob_flush frame pointer
    abi::emit_symbol_address(emitter, "x9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("ldr x10, [x9]");                                       // load the current buffer-stack depth
    emitter.instruction("cbz x10, __rt_ob_flush_fail");                         // no active buffer — report failure
    emitter.instruction("str x10, [sp, #0]");                                   // save the depth across the write call
    emitter.instruction("sub x11, x10, #1");                                    // top slot index = depth - 1
    abi::emit_symbol_address(emitter, "x12", "_ob_ptrs");                       // materialize the buffer-pointer slot array
    emitter.instruction("ldr x0, [x12, x11, lsl #3]");                          // pass the top buffer base pointer to the writer
    abi::emit_symbol_address(emitter, "x12", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("ldr x1, [x12, x11, lsl #3]");                          // pass the top buffer's used byte count to the writer
    emitter.instruction("cbz x1, __rt_ob_flush_truncate");                      // empty buffer — nothing to write
    emitter.instruction("str x11, [x9]");                                       // temporarily pop the level so the write routes to the parent sink
    emitter.instruction("bl __rt_stdout_write");                                // write the buffered bytes to the parent sink
    abi::emit_symbol_address(emitter, "x9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("ldr x10, [sp, #0]");                                   // reload the saved depth
    emitter.instruction("str x10, [x9]");                                       // restore the buffer-stack depth
    emitter.label("__rt_ob_flush_truncate");
    emitter.instruction("ldr x10, [sp, #0]");                                   // reload the saved depth
    emitter.instruction("sub x11, x10, #1");                                    // top slot index = depth - 1
    abi::emit_symbol_address(emitter, "x12", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("str xzr, [x12, x11, lsl #3]");                         // truncate the flushed buffer to zero bytes
    emitter.instruction("mov x0, #1");                                          // report success
    emitter.instruction("b __rt_ob_flush_done");                                // skip the failure path
    emitter.label("__rt_ob_flush_fail");
    emitter.instruction("mov x0, #0");                                          // report failure (no active buffer)
    emitter.label("__rt_ob_flush_done");
    emitter.instruction("ldp x29, x30, [sp, #16]");                             // restore frame pointer and return address
    emitter.instruction("add sp, sp, #32");                                     // release the ob_flush frame
    emitter.instruction("ret");                                                 // return the success flag
}

/// Emits the Linux x86_64 variant of `__rt_ob_flush`.
fn emit_ob_flush_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: ob_flush ---");
    emitter.label_global("__rt_ob_flush");
    emitter.instruction("push rbp");                                            // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish the ob_flush frame pointer
    emitter.instruction("sub rsp, 16");                                         // reserve an aligned slot for the saved depth
    abi::emit_symbol_address(emitter, "r9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("mov r10, QWORD PTR [r9]");                             // load the current buffer-stack depth
    emitter.instruction("test r10, r10");                                       // is any buffer active?
    emitter.instruction("jz __rt_ob_flush_fail_x86");                           // no active buffer — report failure
    emitter.instruction("mov QWORD PTR [rbp - 8], r10");                        // save the depth across the write call
    emitter.instruction("mov r11, r10");                                        // copy the depth for slot indexing
    emitter.instruction("sub r11, 1");                                          // top slot index = depth - 1
    abi::emit_symbol_address(emitter, "r8", "_ob_ptrs");                        // materialize the buffer-pointer slot array
    emitter.instruction("mov rdi, QWORD PTR [r8 + r11*8]");                     // pass the top buffer base pointer to the writer
    abi::emit_symbol_address(emitter, "r8", "_ob_lens");                        // materialize the used-bytes slot array
    emitter.instruction("mov rsi, QWORD PTR [r8 + r11*8]");                     // pass the top buffer's used byte count to the writer
    emitter.instruction("test rsi, rsi");                                       // is there anything to write?
    emitter.instruction("jz __rt_ob_flush_truncate_x86");                       // empty buffer — nothing to write
    emitter.instruction("mov QWORD PTR [r9], r11");                             // temporarily pop the level so the write routes to the parent sink
    emitter.instruction("call __rt_stdout_write");                              // write the buffered bytes to the parent sink
    abi::emit_symbol_address(emitter, "r9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("mov r10, QWORD PTR [rbp - 8]");                        // reload the saved depth
    emitter.instruction("mov QWORD PTR [r9], r10");                             // restore the buffer-stack depth
    emitter.label("__rt_ob_flush_truncate_x86");
    emitter.instruction("mov r10, QWORD PTR [rbp - 8]");                        // reload the saved depth
    emitter.instruction("sub r10, 1");                                          // top slot index = depth - 1
    abi::emit_symbol_address(emitter, "r11", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("mov QWORD PTR [r11 + r10*8], 0");                      // truncate the flushed buffer to zero bytes
    emitter.instruction("mov eax, 1");                                          // report success
    emitter.instruction("jmp __rt_ob_flush_done_x86");                          // skip the failure path
    emitter.label("__rt_ob_flush_fail_x86");
    emitter.instruction("xor eax, eax");                                        // report failure (no active buffer)
    emitter.label("__rt_ob_flush_done_x86");
    emitter.instruction("add rsp, 16");                                         // release the ob_flush frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the success flag
}

/// Emits `__rt_ob_end_flush`: flush the top buffer to the parent sink, then pop
/// and free it.
///
/// No inputs. Returns 1 in `x0`/`rax` on success, 0 when no buffer is active.
/// Composes `__rt_ob_flush` (write + truncate) with `__rt_ob_end_clean` (pop + free).
pub fn emit_ob_end_flush(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_ob_end_flush_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: ob_end_flush ---");
    emitter.label_global("__rt_ob_end_flush");
    emitter.instruction("stp x29, x30, [sp, #-16]!");                           // save frame pointer and return address
    emitter.instruction("mov x29, sp");                                         // establish a frame pointer for the helper calls
    emitter.instruction("bl __rt_ob_flush");                                    // write the top buffer's contents to the parent sink
    emitter.instruction("cbz x0, __rt_ob_end_flush_done");                      // no active buffer — propagate the failure flag
    emitter.instruction("bl __rt_ob_end_clean");                                // pop and free the flushed buffer
    emitter.label("__rt_ob_end_flush_done");
    emitter.instruction("ldp x29, x30, [sp], #16");                             // restore frame pointer and return address
    emitter.instruction("ret");                                                 // return the success flag
}

/// Emits the Linux x86_64 variant of `__rt_ob_end_flush`.
fn emit_ob_end_flush_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: ob_end_flush ---");
    emitter.label_global("__rt_ob_end_flush");
    emitter.instruction("push rbp");                                            // preserve the caller frame pointer across the helper calls
    emitter.instruction("mov rbp, rsp");                                        // establish a frame pointer for the helper calls
    emitter.instruction("call __rt_ob_flush");                                  // write the top buffer's contents to the parent sink
    emitter.instruction("test rax, rax");                                       // did the flush find an active buffer?
    emitter.instruction("jz __rt_ob_end_flush_done_x86");                       // no active buffer — propagate the failure flag
    emitter.instruction("call __rt_ob_end_clean");                              // pop and free the flushed buffer
    emitter.label("__rt_ob_end_flush_done_x86");
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the success flag
}

/// Emits `__rt_ob_flush_all`: write every still-active buffer to the terminal
/// sink, bottom-up, and reset the stack.
///
/// No inputs, no result. Zeroes `_ob_level` first so the writes route past the
/// buffer stack and any fatal-path re-entry is a no-op; buffers are not freed
/// because the process is about to exit.
pub fn emit_ob_flush_all(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_ob_flush_all_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: ob_flush_all (process-exit drain) ---");
    emitter.label_global("__rt_ob_flush_all");
    emitter.instruction("sub sp, sp, #32");                                     // allocate the flush_all frame
    emitter.instruction("stp x29, x30, [sp, #16]");                             // save frame pointer and return address
    emitter.instruction("add x29, sp, #16");                                    // establish the flush_all frame pointer
    abi::emit_symbol_address(emitter, "x9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("ldr x10, [x9]");                                       // load the current buffer-stack depth
    emitter.instruction("cbz x10, __rt_ob_flush_all_done");                     // nothing buffered — done
    emitter.instruction("str xzr, [x9]");                                       // zero the depth: route writes to the terminal and guard re-entry
    emitter.instruction("str x10, [sp, #8]");                                   // save the buffer count
    emitter.instruction("str xzr, [sp, #0]");                                   // start draining at the bottom slot
    emitter.label("__rt_ob_flush_all_loop");
    emitter.instruction("ldr x10, [sp, #0]");                                   // reload the drain cursor
    emitter.instruction("ldr x11, [sp, #8]");                                   // reload the buffer count
    emitter.instruction("cmp x10, x11");                                        // drained every buffer?
    emitter.instruction("b.ge __rt_ob_flush_all_done");                         // yes — done
    abi::emit_symbol_address(emitter, "x12", "_ob_ptrs");                       // materialize the buffer-pointer slot array
    emitter.instruction("ldr x0, [x12, x10, lsl #3]");                          // pass the buffer base pointer to the writer
    abi::emit_symbol_address(emitter, "x12", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("ldr x1, [x12, x10, lsl #3]");                          // pass the buffer's used byte count to the writer
    emitter.instruction("cbz x1, __rt_ob_flush_all_next");                      // empty buffer — skip the write
    emitter.instruction("bl __rt_stdout_write");                                // write the buffered bytes to the terminal sink
    emitter.label("__rt_ob_flush_all_next");
    emitter.instruction("ldr x10, [sp, #0]");                                   // reload the drain cursor
    emitter.instruction("add x10, x10, #1");                                    // advance to the next buffer
    emitter.instruction("str x10, [sp, #0]");                                   // save the advanced drain cursor
    emitter.instruction("b __rt_ob_flush_all_loop");                            // continue draining
    emitter.label("__rt_ob_flush_all_done");
    emitter.instruction("ldp x29, x30, [sp, #16]");                             // restore frame pointer and return address
    emitter.instruction("add sp, sp, #32");                                     // release the flush_all frame
    emitter.instruction("ret");                                                 // return to caller
}

/// Emits the Linux x86_64 variant of `__rt_ob_flush_all`.
fn emit_ob_flush_all_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: ob_flush_all (process-exit drain) ---");
    emitter.label_global("__rt_ob_flush_all");
    emitter.instruction("push rbp");                                            // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish the flush_all frame pointer
    emitter.instruction("sub rsp, 16");                                         // reserve slots for the drain cursor and buffer count
    abi::emit_symbol_address(emitter, "r9", "_ob_level");                       // materialize the address of the buffer-stack depth
    emitter.instruction("mov r10, QWORD PTR [r9]");                             // load the current buffer-stack depth
    emitter.instruction("test r10, r10");                                       // is anything buffered?
    emitter.instruction("jz __rt_ob_flush_all_done_x86");                       // nothing buffered — done
    emitter.instruction("mov QWORD PTR [r9], 0");                               // zero the depth: route writes to the terminal and guard re-entry
    emitter.instruction("mov QWORD PTR [rbp - 8], r10");                        // save the buffer count
    emitter.instruction("mov QWORD PTR [rbp - 16], 0");                         // start draining at the bottom slot
    emitter.label("__rt_ob_flush_all_loop_x86");
    emitter.instruction("mov r10, QWORD PTR [rbp - 16]");                       // reload the drain cursor
    emitter.instruction("cmp r10, QWORD PTR [rbp - 8]");                        // drained every buffer?
    emitter.instruction("jge __rt_ob_flush_all_done_x86");                      // yes — done
    abi::emit_symbol_address(emitter, "r11", "_ob_ptrs");                       // materialize the buffer-pointer slot array
    emitter.instruction("mov rdi, QWORD PTR [r11 + r10*8]");                    // pass the buffer base pointer to the writer
    abi::emit_symbol_address(emitter, "r11", "_ob_lens");                       // materialize the used-bytes slot array
    emitter.instruction("mov rsi, QWORD PTR [r11 + r10*8]");                    // pass the buffer's used byte count to the writer
    emitter.instruction("test rsi, rsi");                                       // is there anything to write?
    emitter.instruction("jz __rt_ob_flush_all_next_x86");                       // empty buffer — skip the write
    emitter.instruction("call __rt_stdout_write");                              // write the buffered bytes to the terminal sink
    emitter.label("__rt_ob_flush_all_next_x86");
    emitter.instruction("mov r10, QWORD PTR [rbp - 16]");                       // reload the drain cursor
    emitter.instruction("add r10, 1");                                          // advance to the next buffer
    emitter.instruction("mov QWORD PTR [rbp - 16], r10");                       // save the advanced drain cursor
    emitter.instruction("jmp __rt_ob_flush_all_loop_x86");                      // continue draining
    emitter.label("__rt_ob_flush_all_done_x86");
    emitter.instruction("add rsp, 16");                                         // release the flush_all frame
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return to caller
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen_support::platform::{Arch, Platform, Target};

    /// Renders every ob_* buffer helper for one target.
    fn render(platform: Platform, arch: Arch) -> String {
        let mut emitter = Emitter::new(Target::new(platform, arch));
        emit_ob_start(&mut emitter);
        emit_ob_append(&mut emitter);
        emit_ob_contents(&mut emitter);
        emit_ob_queries(&mut emitter);
        emit_ob_clean(&mut emitter);
        emit_ob_end_clean(&mut emitter);
        emit_ob_flush(&mut emitter);
        emit_ob_end_flush(&mut emitter);
        emit_ob_flush_all(&mut emitter);
        emitter.output()
    }

    /// Verifies every target exports all ob_* buffer helper labels.
    #[test]
    fn emits_global_labels_for_all_targets() {
        for (platform, arch) in [
            (Platform::MacOS, Arch::AArch64),
            (Platform::Linux, Arch::AArch64),
            (Platform::Linux, Arch::X86_64),
        ] {
            let asm = render(platform, arch);
            for label in [
                "__rt_ob_start",
                "__rt_ob_append",
                "__rt_ob_contents",
                "__rt_ob_length",
                "__rt_ob_level",
                "__rt_ob_clean",
                "__rt_ob_end_clean",
                "__rt_ob_flush",
                "__rt_ob_end_flush",
                "__rt_ob_flush_all",
            ] {
                assert!(
                    asm.contains(&format!(".globl {label}\n")),
                    "missing {label} for {:?}/{:?}",
                    platform,
                    arch
                );
            }
        }
    }

    /// Verifies the growth path allocates a replacement block and frees the old one.
    #[test]
    fn append_grows_through_heap_alloc_and_free() {
        let mac = render(Platform::MacOS, Arch::AArch64);
        assert!(mac.contains("bl __rt_heap_alloc"));
        assert!(mac.contains("bl __rt_heap_free"));
        let linux_x86 = render(Platform::Linux, Arch::X86_64);
        assert!(linux_x86.contains("call __rt_heap_alloc"));
        assert!(linux_x86.contains("call __rt_heap_free"));
    }

    /// Verifies flush routes buffered bytes back through the stdout funnel so
    /// nested buffers and `--web` capture keep working.
    #[test]
    fn flush_paths_route_through_stdout_write() {
        let mac = render(Platform::MacOS, Arch::AArch64);
        assert!(mac.contains("bl __rt_stdout_write"));
        let linux_x86 = render(Platform::Linux, Arch::X86_64);
        assert!(linux_x86.contains("call __rt_stdout_write"));
    }

    /// Verifies ob_get_contents persists the buffer through `__rt_str_persist`.
    #[test]
    fn contents_persists_the_buffer() {
        let mac = render(Platform::MacOS, Arch::AArch64);
        assert!(mac.contains("bl __rt_str_persist"));
        let linux_x86 = render(Platform::Linux, Arch::X86_64);
        assert!(linux_x86.contains("call __rt_str_persist"));
    }
}
