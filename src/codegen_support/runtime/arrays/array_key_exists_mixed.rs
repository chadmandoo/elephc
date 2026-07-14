//! Purpose:
//! Emits the `__rt_array_key_exists_mixed` runtime helper — the key-presence
//! (`array_key_exists`) equivalent of `__rt_array_get_mixed_key`, for a statically
//! `Array(Mixed)` array whose runtime storage kind (indexed vs hash) is only known
//! at runtime.
//!
//! Called from:
//! - `crate::codegen_support::runtime::emitters::emit_runtime()` via `crate::codegen_support::runtime::arrays`.
//!
//! Key details:
//! - Inputs are the array pointer and a NORMALIZED key pair produced by
//!   `materialize_hash_key` (`key_hi == -1` marks an integer key — including a
//!   numeric string PHP coerces to `int` — and any other `key_hi` marks a genuine
//!   string key). Result is a PHP bool (0/1) in x0/rax.
//! - Dispatches on the array header's packed kind byte (`[ptr-8] & 0xff`): kind 3
//!   is hash storage (delegate to `__rt_hash_get`, whose found flag is already the
//!   `array_key_exists` result and is true for a present-but-null value), kind 2 is
//!   indexed storage (an integer key is a bounds check; a genuine string key is
//!   never present in a pure list). This mirrors `__rt_array_get_mixed_key`'s
//!   dispatch but returns presence instead of the boxed value, so it does NOT
//!   collapse a present `null` entry to "absent".

use crate::codegen_support::emit::Emitter;
use crate::codegen_support::platform::Arch;

/// Emits the mixed-storage `array_key_exists` helper for the current target.
pub fn emit_array_key_exists_mixed(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_array_key_exists_mixed_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: array_key_exists_mixed ---");
    emitter.label_global("__rt_array_key_exists_mixed");
    // Inputs: x0 = array_ptr, x1 = key_lo, x2 = key_hi (-1 marks an integer key).
    emitter.instruction("stp x29, x30, [sp, #-16]!");                           // frame: the hash path calls __rt_hash_get, so save fp/lr
    emitter.instruction("mov x29, sp");                                         // establish a helper frame pointer
    emitter.instruction("cbz x0, __rt_array_key_exists_mixed_absent");          // null array cannot contain the key

    // -- dispatch on array storage kind --
    emitter.instruction("ldr x9, [x0, #-8]");                                   // load packed kind metadata from the array header
    emitter.instruction("and x9, x9, #0xff");                                   // isolate the low byte (kind tag)
    emitter.instruction("cmp x9, #3");                                          // kind 3 = hash storage?
    emitter.instruction("b.eq __rt_array_key_exists_mixed_hash");               // hash storage → probe __rt_hash_get for presence
    emitter.instruction("cmp x9, #2");                                          // kind 2 = indexed storage?
    emitter.instruction("b.ne __rt_array_key_exists_mixed_absent");             // unknown kind cannot contain the key

    // -- indexed storage: only an integer(-coerced) key can be present --
    emitter.instruction("cmn x2, #1");                                          // key_hi == -1 marks an integer key
    emitter.instruction("b.ne __rt_array_key_exists_mixed_absent");             // a genuine string key is never present in a pure list
    emitter.instruction("ldr x9, [x0]");                                        // x9 = array length (header offset 0)
    emitter.instruction("cmp x1, #0");                                          // negative index cannot be present
    emitter.instruction("b.lt __rt_array_key_exists_mixed_absent");             // negative → absent
    emitter.instruction("cmp x1, x9");                                          // index >= length → absent
    emitter.instruction("b.ge __rt_array_key_exists_mixed_absent");             // out of bounds → absent
    emitter.instruction("mov x0, #1");                                          // in-bounds index is present regardless of the stored value (incl. null)
    emitter.instruction("b __rt_array_key_exists_mixed_done");                  // return present

    // -- hash storage: __rt_hash_get's found flag IS the array_key_exists result --
    emitter.label("__rt_array_key_exists_mixed_hash");
    emitter.instruction("bl __rt_hash_get");                                    // x0 = found (already a PHP bool; true even for a present null value)
    emitter.instruction("b __rt_array_key_exists_mixed_done");                  // return the found flag

    emitter.label("__rt_array_key_exists_mixed_absent");
    emitter.instruction("mov x0, #0");                                          // key is absent

    emitter.label("__rt_array_key_exists_mixed_done");
    emitter.instruction("ldp x29, x30, [sp], #16");                             // restore frame pointer and return address
    emitter.instruction("ret");                                                 // return the PHP bool in x0
}

/// Emits the x86_64 variant of `__rt_array_key_exists_mixed`.
fn emit_array_key_exists_mixed_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: array_key_exists_mixed ---");
    emitter.label_global("__rt_array_key_exists_mixed");
    // Inputs: rdi = array_ptr, rsi = key_lo, rdx = key_hi (-1 marks an integer key).
    emitter.instruction("push rbp");                                            // frame: the hash path calls __rt_hash_get
    emitter.instruction("mov rbp, rsp");                                        // establish a helper frame pointer (16-byte aligned for the call)
    emitter.instruction("test rdi, rdi");                                       // null array check
    emitter.instruction("je __rt_array_key_exists_mixed_absent");              // null array cannot contain the key

    // -- dispatch on array storage kind --
    emitter.instruction("mov r9, QWORD PTR [rdi - 8]");                         // load packed kind metadata from the array header
    emitter.instruction("and r9, 0xff");                                        // isolate the low byte (kind tag)
    emitter.instruction("cmp r9, 3");                                           // kind 3 = hash storage?
    emitter.instruction("je __rt_array_key_exists_mixed_hash");                 // hash storage → probe __rt_hash_get for presence
    emitter.instruction("cmp r9, 2");                                           // kind 2 = indexed storage?
    emitter.instruction("jne __rt_array_key_exists_mixed_absent");             // unknown kind cannot contain the key

    // -- indexed storage: only an integer(-coerced) key can be present --
    emitter.instruction("cmp rdx, -1");                                         // key_hi == -1 marks an integer key
    emitter.instruction("jne __rt_array_key_exists_mixed_absent");             // a genuine string key is never present in a pure list
    emitter.instruction("mov r9, QWORD PTR [rdi]");                             // r9 = array length (header offset 0)
    emitter.instruction("test rsi, rsi");                                       // negative index cannot be present
    emitter.instruction("js __rt_array_key_exists_mixed_absent");              // negative → absent
    emitter.instruction("cmp rsi, r9");                                         // index >= length → absent
    emitter.instruction("jge __rt_array_key_exists_mixed_absent");             // out of bounds → absent
    emitter.instruction("mov eax, 1");                                          // in-bounds index is present regardless of the stored value (incl. null)
    emitter.instruction("jmp __rt_array_key_exists_mixed_done");                // return present

    // -- hash storage: __rt_hash_get's found flag IS the array_key_exists result --
    emitter.label("__rt_array_key_exists_mixed_hash");
    emitter.instruction("call __rt_hash_get");                                  // rax = found (already a PHP bool; true even for a present null value)
    emitter.instruction("jmp __rt_array_key_exists_mixed_done");                // return the found flag

    emitter.label("__rt_array_key_exists_mixed_absent");
    emitter.instruction("xor eax, eax");                                        // key is absent

    emitter.label("__rt_array_key_exists_mixed_done");
    emitter.instruction("mov rsp, rbp");                                        // release the helper frame
    emitter.instruction("pop rbp");                                             // restore caller frame pointer
    emitter.instruction("ret");                                                 // return the PHP bool in rax
}
