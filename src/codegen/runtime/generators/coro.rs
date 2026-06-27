//! Purpose:
//! Fiber-backed generator coroutine runtime. A PHP `Generator` is a coroutine
//! object that reuses the Fiber 232-byte layout (so it can reuse
//! `__rt_fiber_switch`/`suspend`/`resume`/`throw`/`start`) plus a small block of
//! generator-specific fields in the otherwise-unused reserved region. This file
//! owns those field offsets and the `yield` suspension primitive
//! `__rt_gen_suspend`.
//!
//! Called from:
//! - `crate::codegen::runtime::generators::emit_generator_runtime()`.
//! - Generated generator bodies call `__rt_gen_suspend` at each `yield`.
//!
//! Key details:
//! - `__rt_gen_suspend` records the yielded key/value into the generator's
//!   persistent `last_key`/`last_value` slots (so repeated `current()`/`key()`
//!   reads are pure loads), then reuses `__rt_fiber_suspend`. Because the fiber
//!   suspend boundary re-raises a scheduled `pending_throw` *inside* the
//!   coroutine's own stack, `Generator::throw()` lands in an in-generator
//!   `try/catch` — the core of issue #329.
//! - Generator fields live at offsets 184..224, inside the Fiber `reserved`
//!   region that `__rt_fiber_construct` already zero-initialises.

// Some field offsets are consumed only by codegen wiring landed in later
// commits of the generators-on-fibers migration; keep them defined alongside
// the layout they describe.
#![allow(dead_code)]

use crate::codegen::emit::Emitter;
use crate::codegen::platform::Arch;

/// Runtime Mixed tag for an integer payload (used to box auto-increment keys).
const INT_TAG: i64 = 0;

// ── Generator-specific field offsets (within the reused Fiber object) ──────
/// Byte offset of `gen_last_key`: boxed Mixed of the most recent yield key.
pub(crate) const GEN_LAST_KEY_OFFSET: i32 = 184;
/// Byte offset of `gen_last_value`: boxed Mixed of the most recent yield value.
pub(crate) const GEN_LAST_VALUE_OFFSET: i32 = 192;
/// Byte offset of `gen_return_value`: boxed Mixed of the body `return` value.
pub(crate) const GEN_RETURN_VALUE_OFFSET: i32 = 200;
/// Byte offset of `gen_auto_key`: next auto-increment integer key (raw i64).
pub(crate) const GEN_AUTO_KEY_OFFSET: i32 = 208;
/// Byte offset of `gen_delegated_iter`: inner iterator for `yield from`.
pub(crate) const GEN_DELEGATED_ITER_OFFSET: i32 = 216;

/// Emits `__rt_gen_suspend`, the `yield` suspension primitive shared by every
/// generated generator body.
///
/// Records the yielded key/value into the current generator's persistent slots
/// (refcount-replacing the previous occupants), then suspends via
/// `__rt_fiber_suspend`. On resume it returns the value delivered by the next
/// `send()`/`next()` (owned by the caller), or — when `Generator::throw()`
/// scheduled a `pending_throw` — the fiber suspend boundary re-raises that
/// exception inside this generator's stack so a local `try/catch` can handle it.
///
/// Input:  `x0`/`rdi` = boxed key cell (NULL → auto-increment integer key);
///         `x1`/`rsi` = boxed value cell (ownership moves into the generator).
/// Output: `x0`/`rax` = boxed value delivered by the next resume (owned).
pub(crate) fn emit_gen_suspend(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_gen_suspend_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: __rt_gen_suspend ---");
    emitter.label_global("__rt_gen_suspend");

    // -- prologue: park the boxed key/value and cache the generator object --
    emitter.instruction("sub sp, sp, #48");                                     // reserve frame plus saved callee registers
    emitter.instruction("stp x29, x30, [sp, #32]");                             // save frame pointer and return address
    emitter.instruction("stp x19, x20, [sp, #16]");                             // preserve callee-saved x19/x20 used as caches
    emitter.instruction("str x21, [sp]");                                       // preserve callee-saved x21 used for the parked key
    emitter.instruction("add x29, sp, #32");                                    // anchor the new frame pointer
    emitter.instruction("mov x20, x1");                                         // x20 = boxed yielded value (ownership moving into the generator)
    emitter.instruction("mov x21, x0");                                         // x21 = boxed key cell (NULL means auto-increment)
    crate::codegen::abi::emit_load_symbol_to_reg(emitter, "x19", "_fiber_current", 0); // x19 = the generator coroutine currently running

    // -- record the yielded value into gen_last_value (refcount-replace) --
    emitter.instruction(&format!("ldr x9, [x19, #{}]", GEN_LAST_VALUE_OFFSET)); // x9 = previous last_value occupant
    emitter.instruction(&format!("str x20, [x19, #{}]", GEN_LAST_VALUE_OFFSET)); // store the freshly yielded value
    emitter.instruction("mov x0, x9");                                          // pass the previous occupant to the releaser
    emitter.instruction("bl __rt_decref_mixed");                                // release the previous last_value (NULL is safe)

    // -- record the key: explicit cell, or a boxed auto-increment integer --
    emitter.instruction("cbz x21, __rt_gen_suspend_auto_key");                  // branch to the auto-key path when no explicit key was supplied
    emitter.instruction(&format!("ldr x9, [x19, #{}]", GEN_LAST_KEY_OFFSET));   // x9 = previous last_key occupant
    emitter.instruction(&format!("str x21, [x19, #{}]", GEN_LAST_KEY_OFFSET));  // store the explicit yielded key
    emitter.instruction("mov x0, x9");                                          // pass the previous key occupant to the releaser
    emitter.instruction("bl __rt_decref_mixed");                                // release the previous last_key (NULL is safe)
    emitter.instruction("b __rt_gen_suspend_yield");                            // skip the auto-key path

    emitter.label("__rt_gen_suspend_auto_key");
    emitter.instruction(&format!("ldr x1, [x19, #{}]", GEN_AUTO_KEY_OFFSET));   // x1 = current auto-increment integer key payload
    emitter.instruction(&format!("mov x0, #{}", INT_TAG));                      // runtime tag 0 = integer
    emitter.instruction("mov x2, #0");                                          // integer payload has no high word
    emitter.instruction("bl __rt_mixed_from_value");                            // x0 = boxed integer key cell (owned)
    emitter.instruction(&format!("ldr x9, [x19, #{}]", GEN_LAST_KEY_OFFSET));   // x9 = previous last_key occupant
    emitter.instruction(&format!("str x0, [x19, #{}]", GEN_LAST_KEY_OFFSET));   // store the freshly boxed auto key
    emitter.instruction("mov x0, x9");                                          // pass the previous key occupant to the releaser
    emitter.instruction("bl __rt_decref_mixed");                                // release the previous last_key (NULL is safe)
    emitter.instruction(&format!("ldr x9, [x19, #{}]", GEN_AUTO_KEY_OFFSET));   // x9 = current auto-increment counter
    emitter.instruction("add x9, x9, #1");                                      // advance the auto-increment counter
    emitter.instruction(&format!("str x9, [x19, #{}]", GEN_AUTO_KEY_OFFSET));   // persist the advanced counter for the next bare yield

    // -- suspend back to the resumer; reuse the fiber suspend boundary --
    emitter.label("__rt_gen_suspend_yield");
    emitter.instruction("mov x0, #0");                                          // generators read last_value, so the fiber transfer value is unused
    emitter.instruction("bl __rt_fiber_suspend");                               // suspend; on resume re-raises a pending throw or returns the sent value

    // -- epilogue: x0 already holds the resumer-delivered value (owned) --
    emitter.instruction("ldr x21, [sp]");                                       // restore caller's x21
    emitter.instruction("ldp x19, x20, [sp, #16]");                             // restore caller's x19/x20
    emitter.instruction("ldp x29, x30, [sp, #32]");                             // restore frame pointer and return address
    emitter.instruction("add sp, sp, #48");                                     // release the helper frame
    emitter.instruction("ret");                                                 // return the sent value to the generator body
}

/// x86_64 implementation of `__rt_gen_suspend`.
///
/// Mirrors the ARM64 version using the System V ABI: generator object cached in
/// `r12`, parked key/value in `r13`/`r14`, Mixed boxing via `rax`=tag,
/// `rdi`=low, `rsi`=high.
fn emit_gen_suspend_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: __rt_gen_suspend ---");
    emitter.label_global("__rt_gen_suspend");

    // -- prologue: park the boxed key/value and cache the generator object --
    emitter.instruction("push rbp");                                            // save caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish the helper frame pointer
    emitter.instruction("push r12");                                            // preserve r12 (generator object cache)
    emitter.instruction("push r13");                                            // preserve r13 (parked boxed value)
    emitter.instruction("push r14");                                            // preserve r14 (parked boxed key)
    emitter.instruction("sub rsp, 8");                                          // keep the stack 16-byte aligned across nested calls
    emitter.instruction("mov r14, rdi");                                        // r14 = boxed key cell (NULL means auto-increment)
    emitter.instruction("mov r13, rsi");                                        // r13 = boxed yielded value (ownership moving into the generator)
    crate::codegen::abi::emit_load_symbol_to_reg(emitter, "r12", "_fiber_current", 0); // r12 = the generator coroutine currently running

    // -- record the yielded value into gen_last_value (refcount-replace) --
    emitter.instruction(&format!("mov rax, QWORD PTR [r12 + {}]", GEN_LAST_VALUE_OFFSET)); // rax = previous last_value occupant
    emitter.instruction(&format!("mov QWORD PTR [r12 + {}], r13", GEN_LAST_VALUE_OFFSET)); // store the freshly yielded value
    emitter.instruction("mov rdi, rax");                                        // pass the previous occupant to the releaser
    emitter.instruction("call __rt_decref_mixed");                              // release the previous last_value (NULL is safe)

    // -- record the key: explicit cell, or a boxed auto-increment integer --
    emitter.instruction("test r14, r14");                                       // was an explicit key supplied?
    emitter.instruction("jz __rt_gen_suspend_auto_key");                        // branch to the auto-key path when no explicit key was supplied
    emitter.instruction(&format!("mov rax, QWORD PTR [r12 + {}]", GEN_LAST_KEY_OFFSET)); // rax = previous last_key occupant
    emitter.instruction(&format!("mov QWORD PTR [r12 + {}], r14", GEN_LAST_KEY_OFFSET)); // store the explicit yielded key
    emitter.instruction("mov rdi, rax");                                        // pass the previous key occupant to the releaser
    emitter.instruction("call __rt_decref_mixed");                              // release the previous last_key (NULL is safe)
    emitter.instruction("jmp __rt_gen_suspend_yield");                          // skip the auto-key path

    emitter.label("__rt_gen_suspend_auto_key");
    emitter.instruction(&format!("mov rdi, QWORD PTR [r12 + {}]", GEN_AUTO_KEY_OFFSET)); // rdi = current auto-increment integer key payload
    emitter.instruction(&format!("mov rax, {}", INT_TAG));                      // runtime tag 0 = integer
    emitter.instruction("xor esi, esi");                                        // integer payload has no high word
    emitter.instruction("call __rt_mixed_from_value");                          // rax = boxed integer key cell (owned)
    emitter.instruction(&format!("mov rcx, QWORD PTR [r12 + {}]", GEN_LAST_KEY_OFFSET)); // rcx = previous last_key occupant
    emitter.instruction(&format!("mov QWORD PTR [r12 + {}], rax", GEN_LAST_KEY_OFFSET)); // store the freshly boxed auto key
    emitter.instruction("mov rdi, rcx");                                        // pass the previous key occupant to the releaser
    emitter.instruction("call __rt_decref_mixed");                              // release the previous last_key (NULL is safe)
    emitter.instruction(&format!("mov rax, QWORD PTR [r12 + {}]", GEN_AUTO_KEY_OFFSET)); // rax = current auto-increment counter
    emitter.instruction("add rax, 1");                                          // advance the auto-increment counter
    emitter.instruction(&format!("mov QWORD PTR [r12 + {}], rax", GEN_AUTO_KEY_OFFSET)); // persist the advanced counter for the next bare yield

    // -- suspend back to the resumer; reuse the fiber suspend boundary --
    emitter.label("__rt_gen_suspend_yield");
    emitter.instruction("xor edi, edi");                                        // generators read last_value, so the fiber transfer value is unused
    emitter.instruction("call __rt_fiber_suspend");                             // suspend; on resume re-raises a pending throw or returns the sent value

    // -- epilogue: rax already holds the resumer-delivered value (owned) --
    emitter.instruction("add rsp, 8");                                          // release the alignment pad
    emitter.instruction("pop r14");                                             // restore caller's r14
    emitter.instruction("pop r13");                                             // restore caller's r13
    emitter.instruction("pop r12");                                             // restore caller's r12
    emitter.instruction("pop rbp");                                             // restore caller frame pointer
    emitter.instruction("ret");                                                 // return the sent value to the generator body
}
