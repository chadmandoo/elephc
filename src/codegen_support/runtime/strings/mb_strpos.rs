//! Purpose:
//! Emits the `__rt_mb_strpos` runtime helper assembly for `mb_strpos`.
//! Keeps PHP multibyte position semantics and target-specific ABI variants in one focused emitter.
//!
//! Called from:
//! - `crate::codegen::runtime::emitters::emit_runtime()` via `crate::codegen::runtime::strings`.
//! - The `mb_strpos()` builtin lowering
//!   (`crate::codegen_ir::lower_inst::builtins::strings::lower_string_position`).
//!
//! Key details:
//! - Shares `__rt_strpos`'s two-level byte search (same ptr/len ABI, so the builtin reuses
//!   the `lower_string_position` emitter), but returns the UTF-8 CODE-POINT index of the
//!   first match rather than the byte offset: on a hit at byte offset `pos`, it counts the
//!   code-point leading bytes (top two bits not `10`) in `haystack[0..pos]` — the same scan
//!   `__rt_mb_strlen` uses. Empty needle → 0; not found → -1 (the `box_search_result`
//!   sentinel the emitter maps to PHP `false`).

use crate::codegen::emit::Emitter;
use crate::codegen::platform::Arch;

/// Emits the `__rt_mb_strpos` runtime helper for the current platform.
///
/// ABI (matches `__rt_strpos`):
///   ARM64:  x1=haystack_ptr, x2=haystack_len, x3=needle_ptr, x4=needle_len → x0 = code-point index or -1
///   x86_64: rdi=haystack_ptr, rsi=haystack_len, rdx=needle_ptr, rcx=needle_len → rax = code-point index or -1
pub fn emit_mb_strpos(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_mb_strpos_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: mb_strpos (UTF-8 code-point position) ---");
    emitter.label_global("__rt_mb_strpos");

    // -- edge cases --
    emitter.instruction("cbz x4, __rt_mb_strpos_empty"); // empty needle always matches at code-point 0
    emitter.instruction("cmp x4, x2"); // compare needle length with haystack length
    emitter.instruction("b.gt __rt_mb_strpos_notfound"); // needle longer than haystack, can't match
    emitter.instruction("mov x5, #0"); // initialize byte search position to 0

    // -- outer loop: try matching needle at each byte position --
    emitter.label("__rt_mb_strpos_outer");
    emitter.instruction("sub x9, x2, x4"); // last valid start = haystack_len - needle_len
    emitter.instruction("cmp x5, x9"); // check if position exceeds last valid start
    emitter.instruction("b.gt __rt_mb_strpos_notfound"); // past end, needle not found

    // -- inner loop: compare needle bytes at current position --
    emitter.instruction("mov x6, #0"); // needle comparison index = 0
    emitter.label("__rt_mb_strpos_inner");
    emitter.instruction("cmp x6, x4"); // check if all needle bytes matched
    emitter.instruction("b.ge __rt_mb_strpos_found"); // all matched, byte offset is x5
    emitter.instruction("add x7, x5, x6"); // compute haystack index = pos + needle_idx
    emitter.instruction("ldrb w8, [x1, x7]"); // load haystack byte at computed index
    emitter.instruction("ldrb w9, [x3, x6]"); // load needle byte at current index
    emitter.instruction("cmp w8, w9"); // compare haystack and needle bytes
    emitter.instruction("b.ne __rt_mb_strpos_next"); // mismatch, try next position
    emitter.instruction("add x6, x6, #1"); // advance needle index
    emitter.instruction("b __rt_mb_strpos_inner"); // continue comparing

    // -- advance to next haystack position --
    emitter.label("__rt_mb_strpos_next");
    emitter.instruction("add x5, x5, #1"); // increment byte search position
    emitter.instruction("b __rt_mb_strpos_outer"); // retry from new position

    // -- match at byte offset x5: convert to a code-point index --
    emitter.label("__rt_mb_strpos_found");
    emitter.instruction("mov x0, #0"); // code-point counter (return register)
    emitter.instruction("mov x6, #0"); // byte index
    emitter.label("__rt_mb_strpos_count");
    emitter.instruction("cmp x6, x5"); // reached the match byte offset?
    emitter.instruction("b.hs __rt_mb_strpos_count_done"); // yes → x0 holds the code-point index
    emitter.instruction("ldrb w8, [x1, x6]"); // load the current byte
    emitter.instruction("and w9, w8, #0xC0"); // isolate the top two bits
    emitter.instruction("cmp w9, #0x80"); // continuation byte (10xxxxxx)?
    emitter.instruction("b.eq __rt_mb_strpos_count_skip"); // yes → not a code-point start
    emitter.instruction("add x0, x0, #1"); // count a code-point-leading byte
    emitter.label("__rt_mb_strpos_count_skip");
    emitter.instruction("add x6, x6, #1"); // advance to the next byte
    emitter.instruction("b __rt_mb_strpos_count"); // continue counting
    emitter.label("__rt_mb_strpos_count_done");
    emitter.instruction("ret"); // return the code-point index

    // -- return sentinels --
    emitter.label("__rt_mb_strpos_empty");
    emitter.instruction("mov x0, #0"); // empty needle found at code-point 0
    emitter.instruction("ret"); // return to caller
    emitter.label("__rt_mb_strpos_notfound");
    emitter.instruction("mov x0, #-1"); // return -1 (not found → PHP false)
    emitter.instruction("ret"); // return to caller
}

/// Emits the `__rt_mb_strpos` runtime helper for Linux x86_64.
///
/// ABI: rdi=haystack_ptr, rsi=haystack_len, rdx=needle_ptr, rcx=needle_len → rax = code-point index or -1.
/// Mirrors `__rt_strpos`'s x86_64 byte search, then counts code-point leading bytes up to the hit.
fn emit_mb_strpos_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: mb_strpos (UTF-8 code-point position) ---");
    emitter.label_global("__rt_mb_strpos");

    emitter.instruction("test rcx, rcx"); // empty needles match immediately at code-point zero
    emitter.instruction("jz __rt_mb_strpos_empty_linux_x86_64"); // return zero for an empty needle
    emitter.instruction("cmp rcx, rsi"); // reject searches whose needle is longer than the haystack
    emitter.instruction("jg __rt_mb_strpos_notfound_linux_x86_64"); // needle cannot fit
    emitter.instruction("mov r10, rsi"); // copy the haystack length to compute the last valid start
    emitter.instruction("sub r10, rcx"); // last haystack byte offset where the full needle still fits
    emitter.instruction("mov r8, rdi"); // seed the candidate pointer at the start of the haystack
    emitter.instruction("xor r9d, r9d"); // start scanning from haystack byte offset zero

    emitter.label("__rt_mb_strpos_outer_linux_x86_64");
    emitter.instruction("cmp r9, r10"); // advanced beyond the last valid haystack start offset?
    emitter.instruction("jg __rt_mb_strpos_notfound_linux_x86_64"); // no more candidate start offsets
    emitter.instruction("xor r11d, r11d"); // start the needle comparison from index zero

    emitter.label("__rt_mb_strpos_inner_linux_x86_64");
    emitter.instruction("cmp r11, rcx"); // did every byte in the needle match?
    emitter.instruction("jge __rt_mb_strpos_found_linux_x86_64"); // full needle matched at byte offset r9
    emitter.instruction("movzx eax, BYTE PTR [r8 + r11]"); // load the current haystack byte
    emitter.instruction("movzx esi, BYTE PTR [rdx + r11]"); // load the current needle byte (rsi free after r10)
    emitter.instruction("cmp eax, esi"); // compare the haystack and needle bytes
    emitter.instruction("jne __rt_mb_strpos_next_linux_x86_64"); // abandon this candidate on first mismatch
    emitter.instruction("add r11, 1"); // advance to the next needle byte
    emitter.instruction("jmp __rt_mb_strpos_inner_linux_x86_64"); // continue matching

    emitter.label("__rt_mb_strpos_next_linux_x86_64");
    emitter.instruction("add r8, 1"); // advance the candidate pointer
    emitter.instruction("add r9, 1"); // advance the logical byte offset
    emitter.instruction("jmp __rt_mb_strpos_outer_linux_x86_64"); // retry from the next start offset

    // -- match at byte offset r9: convert to a code-point index --
    emitter.label("__rt_mb_strpos_found_linux_x86_64");
    emitter.instruction("xor eax, eax"); // code-point counter (return register)
    emitter.instruction("xor r11d, r11d"); // byte index
    emitter.label("__rt_mb_strpos_count_linux_x86_64");
    emitter.instruction("cmp r11, r9"); // reached the match byte offset?
    emitter.instruction("jae __rt_mb_strpos_count_done_linux_x86_64"); // yes → rax holds the code-point index
    emitter.instruction("movzx esi, BYTE PTR [rdi + r11]"); // load the current byte (rdi = haystack, untouched)
    emitter.instruction("and esi, 0xC0"); // isolate the top two bits
    emitter.instruction("cmp esi, 0x80"); // continuation byte (10xxxxxx)?
    emitter.instruction("je __rt_mb_strpos_count_skip_linux_x86_64"); // yes → not a code-point start
    emitter.instruction("add rax, 1"); // count a code-point-leading byte
    emitter.label("__rt_mb_strpos_count_skip_linux_x86_64");
    emitter.instruction("add r11, 1"); // advance to the next byte
    emitter.instruction("jmp __rt_mb_strpos_count_linux_x86_64"); // continue counting
    emitter.label("__rt_mb_strpos_count_done_linux_x86_64");
    emitter.instruction("ret"); // return the code-point index

    emitter.label("__rt_mb_strpos_empty_linux_x86_64");
    emitter.instruction("xor eax, eax"); // empty needle matches at code-point zero
    emitter.instruction("ret"); // return to caller

    emitter.label("__rt_mb_strpos_notfound_linux_x86_64");
    emitter.instruction("mov rax, -1"); // not-found sentinel (→ PHP false)
    emitter.instruction("ret"); // return to caller
}
