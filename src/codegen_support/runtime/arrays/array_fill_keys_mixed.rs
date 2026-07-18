//! Purpose:
//! Emits the `__rt_hash_set_cast_key` and `__rt_array_fill_keys_mixed` runtime helper assembly
//! for `array_fill_keys()` over a key array whose elements are boxed `Mixed` cells.
//! Keeps PHP array/hash storage, heap ownership, and target-specific ABI variants in one focused emitter.
//!
//! Called from:
//! - `crate::codegen_support::runtime::emitters::emit_runtime()` via `crate::codegen_support::runtime::arrays`.
//!
//! Key details:
//! - A bare `array` parameter carries no element type (elephc discards phpdoc), so a key list
//!   reaching `array_fill_keys()` is frequently `Array(Mixed)`: 8-byte boxed-cell slots rather
//!   than the 16-byte `(ptr, len)` string descriptors `__rt_array_fill_keys` reads. The key type
//!   never existed to be tracked, so this is resolved at the consumer by unboxing each key cell.
//! - `array_fill_keys()` does NOT use the same key cast as a direct `$a[$k] = $v` assignment, and
//!   the difference is observable: verified against PHP 8.5.8, `array_fill_keys([false], 'V')`
//!   yields key `""` where `$a[false] = 'V'` yields key `0`, and `array_fill_keys([2.7], 'V')`
//!   yields key `"2.7"` where `$a[2.7] = 'V'` yields key `2`. PHP stringifies the key and then
//!   applies standard array-key normalization. So this must NOT reuse the tag dispatch in
//!   `__rt_array_set_mixed_key`, which implements the direct-assignment semantics correctly for
//!   its own case (float truncation, `false` -> 0) and would silently mis-key here.
//! - `__rt_mixed_cast_string` already implements exactly PHP's cast (int -> itoa, string ->
//!   persisted copy, float -> ftoa, bool -> "1"/"", null/unsupported -> ""), and its result
//!   registers line up with `__rt_hash_normalize_key`'s inputs, so the pair composes without
//!   register shuffling and without a bespoke tag dispatch to keep in sync.
//! - The cast may return transient scratch bytes (itoa/ftoa buffers), which is safe because
//!   `__rt_hash_set` persists inserted string keys via `__rt_str_persist` before storing them.
//! - The value is passed through untouched as `(value_lo, value_hi, value_tag)`, so the fill
//!   payload keeps its real static type instead of being boxed as a Mixed cell. A caller whose
//!   payload is refcounted must retain it per insertion before calling.

use crate::codegen_support::emit::Emitter;
use crate::codegen_support::platform::Arch;

/// Emits the `__rt_hash_set_cast_key` runtime helper for the current target.
///
/// Inserts one entry into an existing hash using a boxed `Mixed` cell as the key, applying the
/// stringify-then-normalize cast PHP's `array_fill_keys()`/`array_combine()` use for keys. The
/// value words are inserted verbatim.
///
/// Inputs (ARM64): x0=hash, x1=boxed Mixed key cell, x2=value_lo, x3=value_hi, x4=value_tag
/// Inputs (x86_64): rdi=hash, rsi=boxed Mixed key cell, rdx=value_lo, rcx=value_hi, r8=value_tag
/// Output (ARM64): x0=hash pointer (may differ if the table was reallocated)
/// Output (x86_64): rax=hash pointer (may differ if the table was reallocated)
pub fn emit_hash_set_cast_key(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_hash_set_cast_key_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: hash_set_cast_key ---");
    emitter.label_global("__rt_hash_set_cast_key");

    // Stack layout:
    //   [sp, #0]  = destination hash pointer
    //   [sp, #8]  = value low word
    //   [sp, #16] = value high word
    //   [sp, #24] = value_type tag
    //   [sp, #48] = saved x29 / x30
    emitter.instruction("sub sp, sp, #64");                                     // reserve the helper frame for the hash pointer and the value triple
    emitter.instruction("stp x29, x30, [sp, #48]");                             // save frame pointer and return address
    emitter.instruction("add x29, sp, #48");                                    // establish a helper frame pointer
    emitter.instruction("str x0, [sp, #0]");                                    // preserve the destination hash across the cast and normalize calls
    emitter.instruction("str x2, [sp, #8]");                                    // preserve the value low word across the cast and normalize calls
    emitter.instruction("str x3, [sp, #16]");                                   // preserve the value high word across the cast and normalize calls
    emitter.instruction("str x4, [sp, #24]");                                   // preserve the value_type tag across the cast and normalize calls

    emitter.instruction("mov x0, x1");                                          // move the boxed Mixed key cell into the cast argument register
    emitter.instruction("bl __rt_mixed_cast_string");                           // cast the key exactly as array_fill_keys does: x1 = ptr, x2 = len
    emitter.instruction("bl __rt_hash_normalize_key");                          // normalize the cast key (x1/x2) into a hash key pair, folding canonical integer text back to an integer key

    emitter.instruction("ldr x0, [sp, #0]");                                    // reload the destination hash as the set target
    emitter.instruction("ldr x3, [sp, #8]");                                    // reload the value low word for the insert
    emitter.instruction("ldr x4, [sp, #16]");                                   // reload the value high word for the insert
    emitter.instruction("ldr x5, [sp, #24]");                                   // reload the value_type tag for the insert
    emitter.instruction("bl __rt_hash_set");                                    // insert the normalized key with the caller's value triple

    emitter.instruction("ldp x29, x30, [sp, #48]");                             // restore frame pointer and return address
    emitter.instruction("add sp, sp, #64");                                     // release the helper frame
    emitter.instruction("ret");                                                 // return the possibly-grown hash pointer in x0
}

/// Emits the Linux x86_64 cast-key hash insert helper.
///
/// Input registers: rdi=hash, rsi=boxed Mixed key cell, rdx=value_lo, rcx=value_hi, r8=value_tag.
/// Output: rax=hash pointer (may differ if the table was reallocated).
fn emit_hash_set_cast_key_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: hash_set_cast_key ---");
    emitter.label_global("__rt_hash_set_cast_key");

    emitter.instruction("push rbp");                                            // preserve the caller frame pointer
    emitter.instruction("mov rbp, rsp");                                        // establish a stable helper frame
    emitter.instruction("sub rsp, 48");                                         // reserve aligned spill slots for the hash pointer and the value triple
    emitter.instruction("mov QWORD PTR [rbp - 8], rdi");                        // preserve the destination hash across the cast and normalize calls
    emitter.instruction("mov QWORD PTR [rbp - 16], rdx");                       // preserve the value low word across the cast and normalize calls
    emitter.instruction("mov QWORD PTR [rbp - 24], rcx");                       // preserve the value high word across the cast and normalize calls
    emitter.instruction("mov QWORD PTR [rbp - 32], r8");                        // preserve the value_type tag across the cast and normalize calls

    // `__rt_mixed_cast_string`'s doc comment names RDI as its input, but on x86_64 it opens with
    // `call __rt_mixed_unbox`, whose input register is RAX — so RAX is the register that actually
    // carries the boxed cell in. Passing it in RDI unboxes whatever RAX happened to hold and
    // segfaults. Same doc-versus-implementation asymmetry that caused the #640 miscompile.
    emitter.instruction("mov rax, rsi");                                        // move the boxed Mixed key cell into the cast input register
    emitter.instruction("call __rt_mixed_cast_string");                         // cast the key exactly as array_fill_keys does: rax = ptr, rdx = len
    emitter.instruction("call __rt_hash_normalize_key");                        // normalize the cast key (rax/rdx) into a hash key pair, folding canonical integer text back to an integer key
    emitter.instruction("mov rsi, rax");                                        // publish the normalized key low word as the hash key low word

    emitter.instruction("mov rdi, QWORD PTR [rbp - 8]");                        // reload the destination hash as the set target
    emitter.instruction("mov rcx, QWORD PTR [rbp - 16]");                       // reload the value low word for the insert
    emitter.instruction("mov r8, QWORD PTR [rbp - 24]");                        // reload the value high word for the insert
    emitter.instruction("mov r9, QWORD PTR [rbp - 32]");                        // reload the value_type tag for the insert
    emitter.instruction("call __rt_hash_set");                                  // insert the normalized key with the caller's value triple

    emitter.instruction("add rsp, 48");                                         // release the helper spill slots before returning
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer
    emitter.instruction("ret");                                                 // return the possibly-grown hash pointer in rax
}

/// Emits the `__rt_array_fill_keys_mixed` runtime helper for the current target.
///
/// The boxed-`Mixed` key sibling of `__rt_array_fill_keys`: identical loop and capacity policy,
/// but each key slot is one pointer wide (a boxed cell) instead of a 16-byte string descriptor,
/// and the key cast is delegated to `__rt_hash_set_cast_key`.
///
/// Inputs (ARM64): x0=keys_array (Mixed-cell array), x1=fill value lo, x2=value_type_tag
/// Inputs (x86_64): rdi=keys_array, rsi=fill value lo, rdx=value_type_tag
/// Output (ARM64): x0=new hash table pointer
/// Output (x86_64): rax=new hash table pointer
pub fn emit_array_fill_keys_mixed(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_array_fill_keys_mixed_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: array_fill_keys_mixed ---");
    emitter.label_global("__rt_array_fill_keys_mixed");

    // Stack layout:
    //   [sp, #0]  = keys array pointer
    //   [sp, #8]  = fill value
    //   [sp, #16] = hash table pointer (result)
    //   [sp, #24] = loop index i
    //   [sp, #32] = fill value_type tag
    //   [sp, #48] = saved x29 / x30
    emitter.instruction("sub sp, sp, #64");                                     // allocate the helper frame
    emitter.instruction("stp x29, x30, [sp, #48]");                             // save frame pointer and return address
    emitter.instruction("add x29, sp, #48");                                    // set up a helper frame pointer
    emitter.instruction("str x0, [sp, #0]");                                    // save keys array pointer
    emitter.instruction("str x1, [sp, #8]");                                    // save fill value
    emitter.instruction("str x2, [sp, #32]");                                   // save result value_type tag

    // -- create hash table with capacity = length * 2 --
    emitter.instruction("ldr x0, [x0]");                                        // x0 = keys array length
    emitter.instruction("lsl x0, x0, #1");                                      // x0 = length * 2 (capacity with headroom)
    emitter.instruction("mov x9, #16");                                         // x9 = minimum capacity
    emitter.instruction("cmp x0, x9");                                          // compare with minimum
    emitter.instruction("csel x0, x9, x0, lt");                                 // if length*2 < 16, use 16
    emitter.instruction("ldr x1, [sp, #32]");                                   // x1 = requested result value_type tag
    emitter.instruction("bl __rt_hash_new");                                    // create hash table, x0 = hash ptr
    emitter.instruction("str x0, [sp, #16]");                                   // save hash table pointer

    // -- loop over keys --
    emitter.instruction("str xzr, [sp, #24]");                                  // i = 0

    emitter.label("__rt_array_fill_keys_mixed_loop");
    emitter.instruction("ldr x0, [sp, #0]");                                    // reload keys array pointer
    emitter.instruction("ldr x3, [x0]");                                        // x3 = keys array length
    emitter.instruction("ldr x4, [sp, #24]");                                   // x4 = i
    emitter.instruction("cmp x4, x3");                                          // compare i with length
    emitter.instruction("b.ge __rt_array_fill_keys_mixed_done");                // if i >= length, done

    // -- load the boxed Mixed key cell from keys[i] (8 bytes per Mixed element) --
    emitter.instruction("lsl x5, x4, #3");                                      // x5 = i * 8 (byte offset for a pointer-wide Mixed slot)
    emitter.instruction("add x5, x0, x5");                                      // x5 = keys_array + byte offset
    emitter.instruction("add x5, x5, #24");                                     // x5 = skip header to data region
    emitter.instruction("ldr x1, [x5]");                                        // x1 = boxed Mixed key cell

    // -- insert the cast key with the caller's fill payload --
    emitter.instruction("ldr x0, [sp, #16]");                                   // x0 = hash table pointer
    emitter.instruction("ldr x2, [sp, #8]");                                    // x2 = value_lo = fill value
    emitter.instruction("mov x3, #0");                                          // x3 = value_hi = 0
    emitter.instruction("ldr x4, [sp, #32]");                                   // x4 = value_tag for the fill payload
    emitter.instruction("bl __rt_hash_set_cast_key");                           // cast, normalize, and insert the key/value pair
    emitter.instruction("str x0, [sp, #16]");                                   // update hash table pointer after possible growth

    // -- advance loop --
    emitter.instruction("ldr x4, [sp, #24]");                                   // reload i
    emitter.instruction("add x4, x4, #1");                                      // i += 1
    emitter.instruction("str x4, [sp, #24]");                                   // save updated i
    emitter.instruction("b __rt_array_fill_keys_mixed_loop");                   // continue loop

    // -- return hash table --
    emitter.label("__rt_array_fill_keys_mixed_done");
    emitter.instruction("ldr x0, [sp, #16]");                                   // x0 = hash table pointer
    emitter.instruction("ldp x29, x30, [sp, #48]");                             // restore frame pointer and return address
    emitter.instruction("add sp, sp, #64");                                     // deallocate stack frame
    emitter.instruction("ret");                                                 // return with x0 = hash table
}

/// Emits the x86_64 Linux variant of `__rt_array_fill_keys_mixed`.
///
/// - rdi: keys_array (input, spilled across helper calls)
/// - rsi: fill value lo (input, spilled across helper calls)
/// - rdx: value_type_tag (input, spilled across helper calls)
/// - rax: returns the new hash table pointer
fn emit_array_fill_keys_mixed_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: array_fill_keys_mixed ---");
    emitter.label_global("__rt_array_fill_keys_mixed");

    emitter.instruction("push rbp");                                            // preserve the caller frame pointer before reserving associative-array fill spill slots
    emitter.instruction("mov rbp, rsp");                                        // establish a stable frame base for keys, fill payload, hash pointer, loop index, and value tag bookkeeping
    emitter.instruction("sub rsp, 48");                                         // reserve aligned spill slots for keys, fill payload, hash pointer, loop index, and value tag bookkeeping
    emitter.instruction("mov QWORD PTR [rbp - 8], rdi");                        // preserve the indexed array of boxed keys across hash allocation and repeated insert helper calls
    emitter.instruction("mov QWORD PTR [rbp - 16], rsi");                       // preserve the scalar fill payload across hash allocation and repeated insert helper calls
    emitter.instruction("mov QWORD PTR [rbp - 40], rdx");                       // preserve the requested associative-array value_type tag across hash allocation and repeated insert helper calls
    emitter.instruction("mov rdi, QWORD PTR [rdi]");                            // load the indexed array key count and place it in the first x86_64 hash-constructor argument register
    emitter.instruction("shl rdi, 1");                                          // double the indexed array key count to provide the associative-array constructor some insertion headroom
    emitter.instruction("cmp rdi, 16");                                         // clamp the requested associative-array capacity to the minimum bucket count expected by the hash runtime
    emitter.instruction("jge __rt_array_fill_keys_mixed_capacity_x86");         // keep the doubled key-count capacity when it already meets the minimum bucket count
    emitter.instruction("mov rdi, 16");                                         // fall back to the minimum associative-array bucket count for very small key arrays
    emitter.label("__rt_array_fill_keys_mixed_capacity_x86");
    emitter.instruction("mov rsi, QWORD PTR [rbp - 40]");                       // pass the requested associative-array value_type tag to the x86_64 hash constructor
    emitter.instruction("call __rt_hash_new");                                  // allocate the destination associative array through the shared x86_64 hash constructor
    emitter.instruction("mov QWORD PTR [rbp - 24], rax");                       // preserve the destination associative-array pointer across repeated insert helper calls
    emitter.instruction("mov QWORD PTR [rbp - 32], 0");                         // initialize the boxed-key loop index to the first key slot
    emitter.label("__rt_array_fill_keys_mixed_loop_x86");
    emitter.instruction("mov rcx, QWORD PTR [rbp - 32]");                       // reload the boxed-key loop index before loading the next Mixed cell slot
    emitter.instruction("mov r10, QWORD PTR [rbp - 8]");                        // reload the indexed array of boxed keys before reading the key-count loop bound
    emitter.instruction("cmp rcx, QWORD PTR [r10]");                            // compare the current key loop index against the indexed-array key count
    emitter.instruction("jge __rt_array_fill_keys_mixed_done_x86");             // finish once every boxed key slot has been inserted into the associative array
    emitter.instruction("mov r11, rcx");                                        // copy the boxed-key loop index before scaling it to the pointer-wide Mixed slot size
    emitter.instruction("shl r11, 3");                                          // scale the boxed-key loop index by the 8-byte Mixed slot size
    emitter.instruction("add r10, r11");                                        // advance from the indexed-array base pointer to the selected Mixed key slot
    emitter.instruction("add r10, 24");                                         // skip the indexed-array header to reach the selected Mixed key slot payload
    emitter.instruction("mov rdi, QWORD PTR [rbp - 24]");                       // reload the destination associative-array pointer before inserting the current key/value pair
    emitter.instruction("mov rsi, QWORD PTR [r10]");                            // load the current boxed Mixed key cell from the selected slot
    emitter.instruction("mov rdx, QWORD PTR [rbp - 16]");                       // reload the scalar fill payload into the insert helper low-word register
    emitter.instruction("xor ecx, ecx");                                        // clear the insert helper high-word register because scalar fills use only the low payload word
    emitter.instruction("mov r8, QWORD PTR [rbp - 40]");                        // reload the requested associative-array value_type tag into the insert helper tag register
    emitter.instruction("call __rt_hash_set_cast_key");                         // cast, normalize, and insert the key plus scalar fill payload
    emitter.instruction("mov QWORD PTR [rbp - 24], rax");                       // persist the possibly-grown destination associative-array pointer after hash insertion
    emitter.instruction("mov r10, QWORD PTR [rbp - 32]");                       // reload the boxed-key loop index after hash insertion clobbered caller-saved registers
    emitter.instruction("add r10, 1");                                          // advance the boxed-key loop index after inserting one key/value pair
    emitter.instruction("mov QWORD PTR [rbp - 32], r10");                       // persist the updated boxed-key loop index across the next insertion helper call
    emitter.instruction("jmp __rt_array_fill_keys_mixed_loop_x86");             // continue inserting boxed Mixed keys into the destination associative array
    emitter.label("__rt_array_fill_keys_mixed_done_x86");
    emitter.instruction("mov rax, QWORD PTR [rbp - 24]");                       // return the filled associative-array pointer in the standard x86_64 integer result register
    emitter.instruction("add rsp, 48");                                         // release the associative-array fill spill slots before returning
    emitter.instruction("pop rbp");                                             // restore the caller frame pointer before returning
    emitter.instruction("ret");                                                 // return the filled associative-array pointer in rax
}
