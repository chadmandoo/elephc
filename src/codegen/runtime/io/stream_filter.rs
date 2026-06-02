//! Purpose:
//! Emits the `__rt_apply_stream_filter` runtime helper, which applies a
//! built-in stream filter to a buffer in place.
//!
//! Called from:
//! - `crate::codegen::runtime::emitters::emit_runtime()` via `crate::codegen::runtime::io`.
//! - `__rt_fread` (read direction) and the `fwrite` emitter (write direction).
//!
//! Key details:
//! - Filter ids: 1 = `string.toupper`, 2 = `string.tolower`, 3 = `string.rot13`.
//!   All three are 1:1 byte transforms, so the buffer length never changes.
//! - This is a leaf helper: it transforms the buffer in place and preserves the
//!   pointer/length registers so callers can return them unchanged.

use crate::codegen::{emit::Emitter, platform::Arch};

/// apply_stream_filter: transform a buffer in place with a built-in filter.
/// Input:  AArch64 x1 = pointer, x2 = length, x3 = filter id
///         x86_64  rax = pointer, rdx = length, rcx = filter id
/// Output: the buffer is transformed in place; the pointer/length are preserved.
pub fn emit_apply_stream_filter(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        emit_apply_stream_filter_linux_x86_64(emitter);
        return;
    }

    emitter.blank();
    emitter.comment("--- runtime: apply_stream_filter ---");
    emitter.label_global("__rt_apply_stream_filter");

    emitter.instruction("mov x9, #0");                                          // x9 = current byte index
    emitter.label("__rt_asf_loop");
    emitter.instruction("cmp x9, x2");                                          // processed every byte?
    emitter.instruction("b.ge __rt_asf_done");                                  // stop when the buffer is exhausted
    emitter.instruction("ldrb w10, [x1, x9]");                                  // load the current byte
    emitter.instruction("cmp x3, #1");                                          // filter id 1 = string.toupper
    emitter.instruction("b.eq __rt_asf_upper");                                 // dispatch to the uppercase transform
    emitter.instruction("cmp x3, #2");                                          // filter id 2 = string.tolower
    emitter.instruction("b.eq __rt_asf_lower");                                 // dispatch to the lowercase transform
    emitter.instruction("cmp x3, #3");                                          // filter id 3 = string.rot13
    emitter.instruction("b.eq __rt_asf_rot13");                                 // dispatch to the rot13 transform
    emitter.instruction("cmp x3, #4");                                          // filter id 4 = string.strip_tags
    emitter.instruction("b.eq __rt_asf_strip_tags");                            // dispatch to the strip-tags state machine
    emitter.instruction("cmp x3, #7");                                          // filter id 7 = convert.base64-decode
    emitter.instruction("b.eq __rt_asf_b64_decode");                            // dispatch to the base64-decode state machine
    emitter.instruction("cmp x3, #5");                                          // filter id 5 = dechunk
    emitter.instruction("b.eq __rt_asf_dechunk");                               // dispatch to the HTTP/1.1 chunked-encoding parser
    emitter.instruction("cmp x3, #6");                                          // filter id 6 = convert.base64-encode
    emitter.instruction("b.eq __rt_asf_b64_encode");                            // dispatch to the base64-encode helper
    emitter.instruction("cmp x3, #9");                                          // filter id 9 = convert.quoted-printable-decode
    emitter.instruction("b.eq __rt_asf_qp_decode");                             // dispatch to the QP decoder
    emitter.instruction("cmp x3, #8");                                          // filter id 8 = convert.quoted-printable-encode
    emitter.instruction("b.eq __rt_asf_qp_encode");                             // dispatch to the QP encoder
    emitter.instruction("b __rt_asf_next");                                     // unknown id: leave the byte unchanged

    emitter.label("__rt_asf_upper");
    emitter.instruction("cmp w10, #0x61");                                      // below 'a'?
    emitter.instruction("b.lt __rt_asf_next");                                  // non-letter: leave unchanged
    emitter.instruction("cmp w10, #0x7A");                                      // above 'z'?
    emitter.instruction("b.gt __rt_asf_next");                                  // non-letter: leave unchanged
    emitter.instruction("sub w10, w10, #0x20");                                 // lowercase -> uppercase
    emitter.instruction("b __rt_asf_store");                                    // store the transformed byte

    emitter.label("__rt_asf_lower");
    emitter.instruction("cmp w10, #0x41");                                      // below 'A'?
    emitter.instruction("b.lt __rt_asf_next");                                  // non-letter: leave unchanged
    emitter.instruction("cmp w10, #0x5A");                                      // above 'Z'?
    emitter.instruction("b.gt __rt_asf_next");                                  // non-letter: leave unchanged
    emitter.instruction("add w10, w10, #0x20");                                 // uppercase -> lowercase
    emitter.instruction("b __rt_asf_store");                                    // store the transformed byte

    emitter.label("__rt_asf_rot13");
    emitter.instruction("mov w11, #0x61");                                      // assume the lowercase base 'a'
    emitter.instruction("cmp w10, #0x61");                                      // below 'a'?
    emitter.instruction("b.lt __rt_asf_rot13_upper");                           // try the uppercase range instead
    emitter.instruction("cmp w10, #0x7A");                                      // within 'a'..'z'?
    emitter.instruction("b.le __rt_asf_rot13_apply");                           // a lowercase letter: rotate it
    emitter.label("__rt_asf_rot13_upper");
    emitter.instruction("mov w11, #0x41");                                      // switch to the uppercase base 'A'
    emitter.instruction("cmp w10, #0x41");                                      // below 'A'?
    emitter.instruction("b.lt __rt_asf_next");                                  // non-letter: leave unchanged
    emitter.instruction("cmp w10, #0x5A");                                      // above 'Z'?
    emitter.instruction("b.gt __rt_asf_next");                                  // non-letter: leave unchanged
    emitter.label("__rt_asf_rot13_apply");
    emitter.instruction("sub w10, w10, w11");                                   // letter index 0..25
    emitter.instruction("add w10, w10, #13");                                   // rotate by 13
    emitter.instruction("cmp w10, #26");                                        // past the end of the alphabet?
    emitter.instruction("b.lt __rt_asf_rot13_nowrap");                          // no wrap needed
    emitter.instruction("sub w10, w10, #26");                                   // wrap back into 0..25
    emitter.label("__rt_asf_rot13_nowrap");
    emitter.instruction("add w10, w10, w11");                                   // back to an ASCII letter

    emitter.label("__rt_asf_store");
    emitter.instruction("strb w10, [x1, x9]");                                  // write the transformed byte back
    emitter.label("__rt_asf_next");
    emitter.instruction("add x9, x9, #1");                                      // advance to the next byte
    emitter.instruction("b __rt_asf_loop");                                     // continue the transform loop
    emitter.label("__rt_asf_done");
    // x2 already holds the input (and output) length for stateless transforms.
    emitter.instruction("ret");

    // -- string.strip_tags: state-machine compaction. Output ≤ input;
    //    returns the compacted length in x0 so fread/fwrite can use it. --
    emitter.label("__rt_asf_strip_tags");
    emitter.instruction("mov x5, #0");                                          // read index
    emitter.instruction("mov x6, #0");                                          // write index
    emitter.instruction("mov x7, #0");                                          // in_tag flag (0 = outside tag, 1 = inside)
    emitter.label("__rt_asf_strip_loop");
    emitter.instruction("cmp x5, x2");
    emitter.instruction("b.ge __rt_asf_strip_done");
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("cbnz x7, __rt_asf_strip_in_tag");
    // not in tag: '<' enters tag; everything else is written.
    emitter.instruction("cmp w8, #60");                                         // '<'
    emitter.instruction("b.eq __rt_asf_strip_enter");
    emitter.instruction("strb w8, [x1, x6]");                                   // write byte
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("b __rt_asf_strip_advance");
    emitter.label("__rt_asf_strip_enter");
    emitter.instruction("mov x7, #1");
    emitter.instruction("b __rt_asf_strip_advance");
    emitter.label("__rt_asf_strip_in_tag");
    // inside tag: '>' exits; otherwise skip the byte.
    emitter.instruction("cmp w8, #62");                                         // '>'
    emitter.instruction("b.ne __rt_asf_strip_advance");
    emitter.instruction("mov x7, #0");
    emitter.label("__rt_asf_strip_advance");
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("b __rt_asf_strip_loop");
    emitter.label("__rt_asf_strip_done");
    emitter.instruction("mov x2, x6");                                          // return compacted length via the same register fread/fwrite use for length
    emitter.instruction("ret");

    // -- convert.base64-decode: walk 4-char groups, emit 3 bytes each.
    //    Non-base64 bytes (whitespace, '=' padding, others) are skipped.
    //    Output ≤ input, so in-place compaction is safe. --
    emitter.label("__rt_asf_b64_decode");
    emitter.instruction("mov x5, #0");                                          // read index
    emitter.instruction("mov x6, #0");                                          // write index
    emitter.instruction("mov x7, #0");                                          // 24-bit group accumulator
    emitter.instruction("mov x4, #0");                                          // chars in current group (0..3)
    emitter.label("__rt_asf_b64_loop");
    emitter.instruction("cmp x5, x2");
    emitter.instruction("b.ge __rt_asf_b64_done");
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("add x5, x5, #1");
    // Classify byte → 6-bit value or skip.
    emitter.instruction("cmp w8, #65");                                         // 'A'
    emitter.instruction("b.lt __rt_asf_b64_try_digit");
    emitter.instruction("cmp w8, #90");                                         // 'Z'
    emitter.instruction("b.gt __rt_asf_b64_try_lower");
    emitter.instruction("sub w8, w8, #65");                                     // A..Z → 0..25
    emitter.instruction("b __rt_asf_b64_add");
    emitter.label("__rt_asf_b64_try_lower");
    emitter.instruction("cmp w8, #97");                                         // 'a'
    emitter.instruction("b.lt __rt_asf_b64_try_plus");
    emitter.instruction("cmp w8, #122");                                        // 'z'
    emitter.instruction("b.gt __rt_asf_b64_try_plus");
    emitter.instruction("sub w8, w8, #71");                                     // a..z → 26..51 (97-26)
    emitter.instruction("b __rt_asf_b64_add");
    emitter.label("__rt_asf_b64_try_digit");
    emitter.instruction("cmp w8, #48");                                         // '0'
    emitter.instruction("b.lt __rt_asf_b64_try_plus");
    emitter.instruction("cmp w8, #57");                                         // '9'
    emitter.instruction("b.gt __rt_asf_b64_try_plus");
    emitter.instruction("add w8, w8, #4");                                      // 0..9 → 52..61
    emitter.instruction("b __rt_asf_b64_add");
    emitter.label("__rt_asf_b64_try_plus");
    emitter.instruction("cmp w8, #43");                                         // '+'
    emitter.instruction("b.eq __rt_asf_b64_plus");
    emitter.instruction("cmp w8, #47");                                         // '/'
    emitter.instruction("b.eq __rt_asf_b64_slash");
    emitter.instruction("b __rt_asf_b64_loop");                                 // skip everything else (ws, '=', etc.)
    emitter.label("__rt_asf_b64_plus");
    emitter.instruction("mov w8, #62");
    emitter.instruction("b __rt_asf_b64_add");
    emitter.label("__rt_asf_b64_slash");
    emitter.instruction("mov w8, #63");
    emitter.label("__rt_asf_b64_add");
    emitter.instruction("lsl x7, x7, #6");
    emitter.instruction("orr x7, x7, x8");
    emitter.instruction("add x4, x4, #1");
    emitter.instruction("cmp x4, #4");
    emitter.instruction("b.lt __rt_asf_b64_loop");
    // 24 bits accumulated → emit 3 bytes.
    emitter.instruction("ubfx w9, w7, #16, #8");
    emitter.instruction("strb w9, [x1, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("ubfx w9, w7, #8, #8");
    emitter.instruction("strb w9, [x1, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("ubfx w9, w7, #0, #8");
    emitter.instruction("strb w9, [x1, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("mov x4, #0");
    emitter.instruction("mov x7, #0");
    emitter.instruction("b __rt_asf_b64_loop");
    emitter.label("__rt_asf_b64_done");
    // Handle partial group (2 or 3 chars).
    emitter.instruction("cmp x4, #2");
    emitter.instruction("b.lt __rt_asf_b64_finish");
    emitter.instruction("cmp x4, #3");
    emitter.instruction("b.eq __rt_asf_b64_three");
    // 2 chars: pad with 12 zero bits, emit 1 byte.
    emitter.instruction("lsl x7, x7, #12");
    emitter.instruction("ubfx w9, w7, #16, #8");
    emitter.instruction("strb w9, [x1, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("b __rt_asf_b64_finish");
    emitter.label("__rt_asf_b64_three");
    // 3 chars: pad with 6 zero bits, emit 2 bytes.
    emitter.instruction("lsl x7, x7, #6");
    emitter.instruction("ubfx w9, w7, #16, #8");
    emitter.instruction("strb w9, [x1, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("ubfx w9, w7, #8, #8");
    emitter.instruction("strb w9, [x1, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.label("__rt_asf_b64_finish");
    emitter.instruction("mov x2, x6");                                          // return decoded length
    emitter.instruction("ret");

    // -- dechunk: parse HTTP/1.1 chunked transfer-encoding inline.
    //    Format: <hex_size>\r\n<bytes>\r\n<hex_size>\r\n<bytes>\r\n...0\r\n\r\n
    //    Output is the concatenation of all <bytes> chunks, with the
    //    size-lines and CRLFs removed. In-place compaction (output ≤
    //    input) using read/write cursors. --
    emitter.label("__rt_asf_dechunk");
    emitter.instruction("mov x5, #0");                                          // read index
    emitter.instruction("mov x6, #0");                                          // write index
    emitter.label("__rt_asf_dechunk_size_loop");
    // Parse a hex chunk-size line: accumulate hex digits in x7 until \r\n.
    emitter.instruction("mov x7, #0");                                          // chunk size accumulator
    emitter.label("__rt_asf_dechunk_size_read");
    emitter.instruction("cmp x5, x2");
    emitter.instruction("b.ge __rt_asf_dechunk_done");
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("cmp w8, #13");                                         // '\r'
    emitter.instruction("b.eq __rt_asf_dechunk_size_eol");                      // end of size line
    emitter.instruction("cmp w8, #59");                                         // ';' (chunk extensions)
    emitter.instruction("b.eq __rt_asf_dechunk_skip_to_eol");                   // ignore extensions
    // Hex digit?
    emitter.instruction("cmp w8, #48");                                         // '0'
    emitter.instruction("b.lt __rt_asf_dechunk_size_read");                     // skip non-digit
    emitter.instruction("cmp w8, #57");                                         // '9'
    emitter.instruction("b.le __rt_asf_dechunk_size_digit");
    // letter? case-fold via |0x20.
    emitter.instruction("orr w8, w8, #0x20");
    emitter.instruction("cmp w8, #97");                                         // 'a'
    emitter.instruction("b.lt __rt_asf_dechunk_size_read");
    emitter.instruction("cmp w8, #102");                                        // 'f'
    emitter.instruction("b.gt __rt_asf_dechunk_size_read");
    emitter.instruction("sub w8, w8, #87");                                     // a..f → 10..15 (97-87)
    emitter.instruction("b __rt_asf_dechunk_size_acc");
    emitter.label("__rt_asf_dechunk_size_digit");
    emitter.instruction("sub w8, w8, #48");                                     // 0..9 → 0..9
    emitter.label("__rt_asf_dechunk_size_acc");
    emitter.instruction("lsl x7, x7, #4");
    emitter.instruction("orr x7, x7, x8");
    emitter.instruction("b __rt_asf_dechunk_size_read");
    emitter.label("__rt_asf_dechunk_skip_to_eol");
    // Skip everything until \r.
    emitter.instruction("cmp x5, x2");
    emitter.instruction("b.ge __rt_asf_dechunk_done");
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("cmp w8, #13");
    emitter.instruction("b.ne __rt_asf_dechunk_skip_to_eol");
    emitter.label("__rt_asf_dechunk_size_eol");
    // Expect '\n' (LF) after the \r. Skip it if present.
    emitter.instruction("cmp x5, x2");
    emitter.instruction("b.ge __rt_asf_dechunk_done");
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("cmp w8, #10");
    emitter.instruction("b.ne __rt_asf_dechunk_skip_lf");
    emitter.instruction("add x5, x5, #1");
    emitter.label("__rt_asf_dechunk_skip_lf");
    // chunk size 0 → end.
    emitter.instruction("cbz x7, __rt_asf_dechunk_done");
    // Copy x7 bytes from [x1+x5] to [x1+x6].
    emitter.instruction("mov x9, #0");
    emitter.label("__rt_asf_dechunk_copy_loop");
    emitter.instruction("cmp x9, x7");
    emitter.instruction("b.ge __rt_asf_dechunk_copy_done");
    emitter.instruction("cmp x5, x2");
    emitter.instruction("b.ge __rt_asf_dechunk_done");
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("strb w8, [x1, x6]");
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("add x9, x9, #1");
    emitter.instruction("b __rt_asf_dechunk_copy_loop");
    emitter.label("__rt_asf_dechunk_copy_done");
    // Skip trailing \r\n after chunk data.
    emitter.instruction("cmp x5, x2");
    emitter.instruction("b.ge __rt_asf_dechunk_size_loop");
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("cmp w8, #13");
    emitter.instruction("b.ne __rt_asf_dechunk_size_loop");
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("cmp x5, x2");
    emitter.instruction("b.ge __rt_asf_dechunk_size_loop");
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("cmp w8, #10");
    emitter.instruction("b.ne __rt_asf_dechunk_size_loop");
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("b __rt_asf_dechunk_size_loop");
    emitter.label("__rt_asf_dechunk_done");
    emitter.instruction("mov x2, x6");
    emitter.instruction("ret");

    // -- convert.quoted-printable-decode: parses '=XX' hex escapes and
    //    soft line breaks ('=\\r?\\n'). Non-escape bytes pass through.
    //    Output ≤ input; in-place compaction. Hex classification is
    //    inlined (no helper-call to keep x30 intact for the outer ret). --
    emitter.label("__rt_asf_qp_decode");
    emitter.instruction("mov x5, #0");                                          // read index
    emitter.instruction("mov x6, #0");                                          // write index
    emitter.label("__rt_asf_qp_loop");
    emitter.instruction("cmp x5, x2");
    emitter.instruction("b.ge __rt_asf_qp_done");
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("cmp w8, #61");                                         // '='
    emitter.instruction("b.eq __rt_asf_qp_escape");
    emitter.instruction("strb w8, [x1, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("b __rt_asf_qp_loop");

    emitter.label("__rt_asf_qp_escape");
    // peek next byte; if \r or \n it's a soft line break.
    emitter.instruction("cmp x5, x2");
    emitter.instruction("b.ge __rt_asf_qp_done");
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("cmp w8, #13");                                         // '\r'
    emitter.instruction("b.eq __rt_asf_qp_soft_break");
    emitter.instruction("cmp w8, #10");                                         // '\n'
    emitter.instruction("b.eq __rt_asf_qp_soft_break_lf");
    // hex hi nibble (inlined classification, w9 = val or -1).
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("mov w9, #-1");
    emitter.instruction("cmp w8, #48");                                         // '0'
    emitter.instruction("b.lt __rt_asf_qp_hi_alpha");
    emitter.instruction("cmp w8, #57");                                         // '9'
    emitter.instruction("b.gt __rt_asf_qp_hi_alpha");
    emitter.instruction("sub w9, w8, #48");
    emitter.instruction("b __rt_asf_qp_hi_done");
    emitter.label("__rt_asf_qp_hi_alpha");
    emitter.instruction("orr w8, w8, #0x20");                                   // lowercase
    emitter.instruction("cmp w8, #97");                                         // 'a'
    emitter.instruction("b.lt __rt_asf_qp_hi_done");
    emitter.instruction("cmp w8, #102");                                        // 'f'
    emitter.instruction("b.gt __rt_asf_qp_hi_done");
    emitter.instruction("sub w9, w8, #87");
    emitter.label("__rt_asf_qp_hi_done");
    emitter.instruction("cmp w9, #0");
    emitter.instruction("b.lt __rt_asf_qp_loop");                               // invalid hi → skip
    emitter.instruction("mov w10, w9");                                         // hi nibble saved
    // hex lo nibble.
    emitter.instruction("cmp x5, x2");
    emitter.instruction("b.ge __rt_asf_qp_done");
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("mov w9, #-1");
    emitter.instruction("cmp w8, #48");
    emitter.instruction("b.lt __rt_asf_qp_lo_alpha");
    emitter.instruction("cmp w8, #57");
    emitter.instruction("b.gt __rt_asf_qp_lo_alpha");
    emitter.instruction("sub w9, w8, #48");
    emitter.instruction("b __rt_asf_qp_lo_done");
    emitter.label("__rt_asf_qp_lo_alpha");
    emitter.instruction("orr w8, w8, #0x20");
    emitter.instruction("cmp w8, #97");
    emitter.instruction("b.lt __rt_asf_qp_lo_done");
    emitter.instruction("cmp w8, #102");
    emitter.instruction("b.gt __rt_asf_qp_lo_done");
    emitter.instruction("sub w9, w8, #87");
    emitter.label("__rt_asf_qp_lo_done");
    emitter.instruction("cmp w9, #0");
    emitter.instruction("b.lt __rt_asf_qp_loop");                               // invalid lo → skip
    emitter.instruction("lsl w10, w10, #4");
    emitter.instruction("orr w10, w10, w9");
    emitter.instruction("strb w10, [x1, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("b __rt_asf_qp_loop");

    emitter.label("__rt_asf_qp_soft_break");
    emitter.instruction("add x5, x5, #1");                                      // skip \r
    emitter.instruction("cmp x5, x2");
    emitter.instruction("b.ge __rt_asf_qp_loop");
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("cmp w8, #10");
    emitter.instruction("b.ne __rt_asf_qp_loop");
    emitter.instruction("add x5, x5, #1");                                      // and \n if present
    emitter.instruction("b __rt_asf_qp_loop");
    emitter.label("__rt_asf_qp_soft_break_lf");
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("b __rt_asf_qp_loop");

    emitter.label("__rt_asf_qp_done");
    emitter.instruction("mov x2, x6");
    emitter.instruction("ret");

    // -- convert.base64-encode: encode 3-byte groups to 4 base64 chars + '='
    //    padding. Output is 4/3 of input; we encode into _stream_grow_scratch
    //    and memcpy back. Caps input at 49152 bytes to keep the 65536-byte
    //    output inside the scratch. --
    emitter.label("__rt_asf_b64_encode");
    // Cap input length so the encoded output fits the 64KB scratch.
    emitter.instruction("mov x4, #49152");                                      // 49152 = 64KB * 3/4
    emitter.instruction("cmp x2, x4");
    emitter.instruction("csel x2, x4, x2, gt");                                 // x2 = MIN(x2, 49152)
    crate::codegen::abi::emit_symbol_address(emitter, "x4", "_stream_grow_scratch");
    crate::codegen::abi::emit_symbol_address(emitter, "x15", "_b64_encode_tbl");
    emitter.instruction("mov x5, #0");                                          // read index
    emitter.instruction("mov x6, #0");                                          // write index
    emitter.label("__rt_asf_b64e_loop");
    emitter.instruction("sub x7, x2, x5");                                      // bytes remaining
    emitter.instruction("cmp x7, #3");
    emitter.instruction("b.lt __rt_asf_b64e_rem");
    // Read 3 bytes.
    emitter.instruction("ldrb w8, [x1, x5]");                                   // byte 0
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("ldrb w9, [x1, x5]");                                   // byte 1
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("ldrb w10, [x1, x5]");                                  // byte 2
    emitter.instruction("add x5, x5, #1");
    // char 0: byte0 >> 2
    emitter.instruction("lsr w11, w8, #2");
    emitter.instruction("ldrb w11, [x15, x11]");
    emitter.instruction("strb w11, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    // char 1: ((byte0 & 3) << 4) | (byte1 >> 4)
    emitter.instruction("and w11, w8, #3");
    emitter.instruction("lsl w11, w11, #4");
    emitter.instruction("lsr w12, w9, #4");
    emitter.instruction("orr w11, w11, w12");
    emitter.instruction("ldrb w11, [x15, x11]");
    emitter.instruction("strb w11, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    // char 2: ((byte1 & 15) << 2) | (byte2 >> 6)
    emitter.instruction("and w11, w9, #15");
    emitter.instruction("lsl w11, w11, #2");
    emitter.instruction("lsr w12, w10, #6");
    emitter.instruction("orr w11, w11, w12");
    emitter.instruction("ldrb w11, [x15, x11]");
    emitter.instruction("strb w11, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    // char 3: byte2 & 0x3f
    emitter.instruction("and w11, w10, #0x3f");
    emitter.instruction("ldrb w11, [x15, x11]");
    emitter.instruction("strb w11, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("b __rt_asf_b64e_loop");
    emitter.label("__rt_asf_b64e_rem");
    emitter.instruction("cbz x7, __rt_asf_b64e_copyback");
    emitter.instruction("cmp x7, #1");
    emitter.instruction("b.eq __rt_asf_b64e_rem1");
    // 2-byte remainder: 3 chars + 1 padding.
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("ldrb w9, [x1, x5]");
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("lsr w11, w8, #2");
    emitter.instruction("ldrb w11, [x15, x11]");
    emitter.instruction("strb w11, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("and w11, w8, #3");
    emitter.instruction("lsl w11, w11, #4");
    emitter.instruction("lsr w12, w9, #4");
    emitter.instruction("orr w11, w11, w12");
    emitter.instruction("ldrb w11, [x15, x11]");
    emitter.instruction("strb w11, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("and w11, w9, #15");
    emitter.instruction("lsl w11, w11, #2");
    emitter.instruction("ldrb w11, [x15, x11]");
    emitter.instruction("strb w11, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("mov w11, #61");                                        // '='
    emitter.instruction("strb w11, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("b __rt_asf_b64e_copyback");
    emitter.label("__rt_asf_b64e_rem1");
    // 1-byte remainder: 2 chars + 2 padding.
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("lsr w11, w8, #2");
    emitter.instruction("ldrb w11, [x15, x11]");
    emitter.instruction("strb w11, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("and w11, w8, #3");
    emitter.instruction("lsl w11, w11, #4");
    emitter.instruction("ldrb w11, [x15, x11]");
    emitter.instruction("strb w11, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("mov w11, #61");                                        // '='
    emitter.instruction("strb w11, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("strb w11, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.label("__rt_asf_b64e_copyback");
    // memcpy scratch[0..x6] back into x1.
    emitter.instruction("mov x5, #0");
    emitter.label("__rt_asf_b64e_cb_loop");
    emitter.instruction("cmp x5, x6");
    emitter.instruction("b.ge __rt_asf_b64e_done");
    emitter.instruction("ldrb w11, [x4, x5]");
    emitter.instruction("strb w11, [x1, x5]");
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("b __rt_asf_b64e_cb_loop");
    emitter.label("__rt_asf_b64e_done");
    emitter.instruction("mov x2, x6");                                          // return encoded length
    emitter.instruction("ret");

    // -- convert.quoted-printable-encode: bytes outside 33..126 (and '=' itself)
    //    become '=XX' hex escapes. Encodes into _stream_grow_scratch and memcpy
    //    back. Caps input at 21845 bytes (worst case = 3x growth) so output
    //    fits 65536. --
    emitter.label("__rt_asf_qp_encode");
    emitter.instruction("mov x4, #21845");                                      // ~64KB/3 worst-case cap
    emitter.instruction("cmp x2, x4");
    emitter.instruction("csel x2, x4, x2, gt");                                 // x2 = MIN(x2, 21845)
    crate::codegen::abi::emit_symbol_address(emitter, "x4", "_stream_grow_scratch");
    crate::codegen::abi::emit_symbol_address(emitter, "x15", "_b64_encode_tbl");
    // hex table is just '0'..'9','A'..'F' so build inline instead.
    emitter.instruction("mov x5, #0");                                          // read index
    emitter.instruction("mov x6, #0");                                          // write index
    emitter.label("__rt_asf_qpe_loop");
    emitter.instruction("cmp x5, x2");
    emitter.instruction("b.ge __rt_asf_qpe_copyback");
    emitter.instruction("ldrb w8, [x1, x5]");
    emitter.instruction("add x5, x5, #1");
    // Pass-through printable ASCII (33..60, 62..126) directly.
    emitter.instruction("cmp w8, #33");
    emitter.instruction("b.lt __rt_asf_qpe_escape");
    emitter.instruction("cmp w8, #126");
    emitter.instruction("b.gt __rt_asf_qpe_escape");
    emitter.instruction("cmp w8, #61");                                         // '=' must be escaped
    emitter.instruction("b.eq __rt_asf_qpe_escape");
    emitter.instruction("strb w8, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("b __rt_asf_qpe_loop");
    emitter.label("__rt_asf_qpe_escape");
    // Emit '=' then two hex digits.
    emitter.instruction("mov w9, #61");                                         // '='
    emitter.instruction("strb w9, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    // High nibble.
    emitter.instruction("lsr w9, w8, #4");
    emitter.instruction("and w9, w9, #0xF");
    emitter.instruction("cmp w9, #10");
    emitter.instruction("b.lt __rt_asf_qpe_hi_dig");
    emitter.instruction("add w9, w9, #55");                                     // 10 → 'A' (10+55=65)
    emitter.instruction("b __rt_asf_qpe_hi_write");
    emitter.label("__rt_asf_qpe_hi_dig");
    emitter.instruction("add w9, w9, #48");                                     // 0 → '0'
    emitter.label("__rt_asf_qpe_hi_write");
    emitter.instruction("strb w9, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    // Low nibble.
    emitter.instruction("and w9, w8, #0xF");
    emitter.instruction("cmp w9, #10");
    emitter.instruction("b.lt __rt_asf_qpe_lo_dig");
    emitter.instruction("add w9, w9, #55");
    emitter.instruction("b __rt_asf_qpe_lo_write");
    emitter.label("__rt_asf_qpe_lo_dig");
    emitter.instruction("add w9, w9, #48");
    emitter.label("__rt_asf_qpe_lo_write");
    emitter.instruction("strb w9, [x4, x6]");
    emitter.instruction("add x6, x6, #1");
    emitter.instruction("b __rt_asf_qpe_loop");
    emitter.label("__rt_asf_qpe_copyback");
    emitter.instruction("mov x5, #0");
    emitter.label("__rt_asf_qpe_cb_loop");
    emitter.instruction("cmp x5, x6");
    emitter.instruction("b.ge __rt_asf_qpe_done");
    emitter.instruction("ldrb w11, [x4, x5]");
    emitter.instruction("strb w11, [x1, x5]");
    emitter.instruction("add x5, x5, #1");
    emitter.instruction("b __rt_asf_qpe_cb_loop");
    emitter.label("__rt_asf_qpe_done");
    emitter.instruction("mov x2, x6");                                          // return encoded length
    emitter.instruction("ret");
}

fn emit_apply_stream_filter_linux_x86_64(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: apply_stream_filter ---");
    emitter.label_global("__rt_apply_stream_filter");

    emitter.instruction("xor r9, r9");                                          // r9 = current byte index
    emitter.label("__rt_asf_loop_x86");
    emitter.instruction("cmp r9, rdx");                                         // processed every byte?
    emitter.instruction("jge __rt_asf_done_x86");                               // stop when the buffer is exhausted
    emitter.instruction("movzx r10d, BYTE PTR [rax + r9]");                     // load the current byte
    emitter.instruction("cmp rcx, 1");                                          // filter id 1 = string.toupper
    emitter.instruction("je __rt_asf_upper_x86");                               // dispatch to the uppercase transform
    emitter.instruction("cmp rcx, 2");                                          // filter id 2 = string.tolower
    emitter.instruction("je __rt_asf_lower_x86");                               // dispatch to the lowercase transform
    emitter.instruction("cmp rcx, 3");                                          // filter id 3 = string.rot13
    emitter.instruction("je __rt_asf_rot13_x86");                               // dispatch to the rot13 transform
    emitter.instruction("cmp rcx, 4");                                          // filter id 4 = string.strip_tags
    emitter.instruction("je __rt_asf_strip_tags_x86");                          // dispatch to the strip-tags state machine
    emitter.instruction("cmp rcx, 7");                                          // filter id 7 = convert.base64-decode
    emitter.instruction("je __rt_asf_b64_decode_x86");                          // dispatch to the base64-decode state machine
    emitter.instruction("cmp rcx, 5");                                          // filter id 5 = dechunk
    emitter.instruction("je __rt_asf_dechunk_x86");                             // dispatch to the HTTP/1.1 chunked-encoding parser
    emitter.instruction("cmp rcx, 6");                                          // filter id 6 = convert.base64-encode
    emitter.instruction("je __rt_asf_b64_encode_x86");                          // dispatch to the base64-encode helper
    emitter.instruction("cmp rcx, 9");                                          // filter id 9 = convert.quoted-printable-decode
    emitter.instruction("je __rt_asf_qp_decode_x86");                           // dispatch to the QP decoder
    emitter.instruction("cmp rcx, 8");                                          // filter id 8 = convert.quoted-printable-encode
    emitter.instruction("je __rt_asf_qp_encode_x86");                           // dispatch to the QP encoder
    emitter.instruction("jmp __rt_asf_next_x86");                               // unknown id: leave the byte unchanged

    emitter.label("__rt_asf_upper_x86");
    emitter.instruction("cmp r10b, 0x61");                                      // below 'a'?
    emitter.instruction("jl __rt_asf_next_x86");                                // non-letter: leave unchanged
    emitter.instruction("cmp r10b, 0x7A");                                      // above 'z'?
    emitter.instruction("jg __rt_asf_next_x86");                                // non-letter: leave unchanged
    emitter.instruction("sub r10b, 0x20");                                      // lowercase -> uppercase
    emitter.instruction("jmp __rt_asf_store_x86");                              // store the transformed byte

    emitter.label("__rt_asf_lower_x86");
    emitter.instruction("cmp r10b, 0x41");                                      // below 'A'?
    emitter.instruction("jl __rt_asf_next_x86");                                // non-letter: leave unchanged
    emitter.instruction("cmp r10b, 0x5A");                                      // above 'Z'?
    emitter.instruction("jg __rt_asf_next_x86");                                // non-letter: leave unchanged
    emitter.instruction("add r10b, 0x20");                                      // uppercase -> lowercase
    emitter.instruction("jmp __rt_asf_store_x86");                              // store the transformed byte

    emitter.label("__rt_asf_rot13_x86");
    emitter.instruction("mov r11b, 0x61");                                      // assume the lowercase base 'a'
    emitter.instruction("cmp r10b, 0x61");                                      // below 'a'?
    emitter.instruction("jl __rt_asf_rot13_upper_x86");                         // try the uppercase range instead
    emitter.instruction("cmp r10b, 0x7A");                                      // within 'a'..'z'?
    emitter.instruction("jle __rt_asf_rot13_apply_x86");                        // a lowercase letter: rotate it
    emitter.label("__rt_asf_rot13_upper_x86");
    emitter.instruction("mov r11b, 0x41");                                      // switch to the uppercase base 'A'
    emitter.instruction("cmp r10b, 0x41");                                      // below 'A'?
    emitter.instruction("jl __rt_asf_next_x86");                                // non-letter: leave unchanged
    emitter.instruction("cmp r10b, 0x5A");                                      // above 'Z'?
    emitter.instruction("jg __rt_asf_next_x86");                                // non-letter: leave unchanged
    emitter.label("__rt_asf_rot13_apply_x86");
    emitter.instruction("sub r10b, r11b");                                      // letter index 0..25
    emitter.instruction("add r10b, 13");                                        // rotate by 13
    emitter.instruction("cmp r10b, 26");                                        // past the end of the alphabet?
    emitter.instruction("jl __rt_asf_rot13_nowrap_x86");                        // no wrap needed
    emitter.instruction("sub r10b, 26");                                        // wrap back into 0..25
    emitter.label("__rt_asf_rot13_nowrap_x86");
    emitter.instruction("add r10b, r11b");                                      // back to an ASCII letter

    emitter.label("__rt_asf_store_x86");
    emitter.instruction("mov BYTE PTR [rax + r9], r10b");                       // write the transformed byte back
    emitter.label("__rt_asf_next_x86");
    emitter.instruction("inc r9");                                              // advance to the next byte
    emitter.instruction("jmp __rt_asf_loop_x86");                               // continue the transform loop
    emitter.label("__rt_asf_done_x86");
    // rdx already holds the input (and output) length for stateless transforms.
    emitter.instruction("ret");

    // -- string.strip_tags: state-machine compaction. --
    emitter.label("__rt_asf_strip_tags_x86");
    emitter.instruction("xor r9, r9");                                          // read index
    emitter.instruction("xor r10, r10");                                        // write index
    emitter.instruction("xor r11, r11");                                        // in_tag flag
    emitter.label("__rt_asf_strip_loop_x86");
    emitter.instruction("cmp r9, rdx");
    emitter.instruction("jge __rt_asf_strip_done_x86");
    emitter.instruction("movzx r8d, BYTE PTR [rax + r9]");
    emitter.instruction("test r11, r11");
    emitter.instruction("jnz __rt_asf_strip_in_tag_x86");
    emitter.instruction("cmp r8b, 60");                                         // '<'
    emitter.instruction("je __rt_asf_strip_enter_x86");
    emitter.instruction("mov BYTE PTR [rax + r10], r8b");
    emitter.instruction("inc r10");
    emitter.instruction("jmp __rt_asf_strip_advance_x86");
    emitter.label("__rt_asf_strip_enter_x86");
    emitter.instruction("mov r11, 1");
    emitter.instruction("jmp __rt_asf_strip_advance_x86");
    emitter.label("__rt_asf_strip_in_tag_x86");
    emitter.instruction("cmp r8b, 62");                                         // '>'
    emitter.instruction("jne __rt_asf_strip_advance_x86");
    emitter.instruction("xor r11, r11");
    emitter.label("__rt_asf_strip_advance_x86");
    emitter.instruction("inc r9");
    emitter.instruction("jmp __rt_asf_strip_loop_x86");
    emitter.label("__rt_asf_strip_done_x86");
    emitter.instruction("mov rdx, r10");                                        // return compacted length via the same register fread/fwrite use for length
    emitter.instruction("ret");

    // -- convert.base64-decode (x86_64) --
    emitter.label("__rt_asf_b64_decode_x86");
    emitter.instruction("xor r9, r9");                                          // read index
    emitter.instruction("xor r10, r10");                                        // write index
    emitter.instruction("xor r11, r11");                                        // 24-bit accumulator
    emitter.instruction("xor r12, r12");                                        // chars in group
    emitter.label("__rt_asf_b64_loop_x86");
    emitter.instruction("cmp r9, rdx");
    emitter.instruction("jge __rt_asf_b64_done_x86");
    emitter.instruction("movzx r8d, BYTE PTR [rax + r9]");
    emitter.instruction("inc r9");
    emitter.instruction("cmp r8b, 65");                                         // 'A'
    emitter.instruction("jl __rt_asf_b64_try_digit_x86");
    emitter.instruction("cmp r8b, 90");                                         // 'Z'
    emitter.instruction("jg __rt_asf_b64_try_lower_x86");
    emitter.instruction("sub r8b, 65");
    emitter.instruction("jmp __rt_asf_b64_add_x86");
    emitter.label("__rt_asf_b64_try_lower_x86");
    emitter.instruction("cmp r8b, 97");
    emitter.instruction("jl __rt_asf_b64_try_plus_x86");
    emitter.instruction("cmp r8b, 122");
    emitter.instruction("jg __rt_asf_b64_try_plus_x86");
    emitter.instruction("sub r8b, 71");
    emitter.instruction("jmp __rt_asf_b64_add_x86");
    emitter.label("__rt_asf_b64_try_digit_x86");
    emitter.instruction("cmp r8b, 48");
    emitter.instruction("jl __rt_asf_b64_try_plus_x86");
    emitter.instruction("cmp r8b, 57");
    emitter.instruction("jg __rt_asf_b64_try_plus_x86");
    emitter.instruction("add r8b, 4");
    emitter.instruction("jmp __rt_asf_b64_add_x86");
    emitter.label("__rt_asf_b64_try_plus_x86");
    emitter.instruction("cmp r8b, 43");
    emitter.instruction("je __rt_asf_b64_plus_x86");
    emitter.instruction("cmp r8b, 47");
    emitter.instruction("je __rt_asf_b64_slash_x86");
    emitter.instruction("jmp __rt_asf_b64_loop_x86");                           // skip non-base64
    emitter.label("__rt_asf_b64_plus_x86");
    emitter.instruction("mov r8b, 62");
    emitter.instruction("jmp __rt_asf_b64_add_x86");
    emitter.label("__rt_asf_b64_slash_x86");
    emitter.instruction("mov r8b, 63");
    emitter.label("__rt_asf_b64_add_x86");
    emitter.instruction("shl r11, 6");
    emitter.instruction("movzx r8, r8b");
    emitter.instruction("or r11, r8");
    emitter.instruction("inc r12");
    emitter.instruction("cmp r12, 4");
    emitter.instruction("jl __rt_asf_b64_loop_x86");
    // Emit 3 bytes.
    emitter.instruction("mov r13, r11");
    emitter.instruction("shr r13, 16");
    emitter.instruction("mov BYTE PTR [rax + r10], r13b");
    emitter.instruction("inc r10");
    emitter.instruction("mov r13, r11");
    emitter.instruction("shr r13, 8");
    emitter.instruction("mov BYTE PTR [rax + r10], r13b");
    emitter.instruction("inc r10");
    emitter.instruction("mov BYTE PTR [rax + r10], r11b");
    emitter.instruction("inc r10");
    emitter.instruction("xor r11, r11");
    emitter.instruction("xor r12, r12");
    emitter.instruction("jmp __rt_asf_b64_loop_x86");
    emitter.label("__rt_asf_b64_done_x86");
    // Partial group handling.
    emitter.instruction("cmp r12, 2");
    emitter.instruction("jl __rt_asf_b64_finish_x86");
    emitter.instruction("cmp r12, 3");
    emitter.instruction("je __rt_asf_b64_three_x86");
    // 2 chars.
    emitter.instruction("shl r11, 12");
    emitter.instruction("mov r13, r11");
    emitter.instruction("shr r13, 16");
    emitter.instruction("mov BYTE PTR [rax + r10], r13b");
    emitter.instruction("inc r10");
    emitter.instruction("jmp __rt_asf_b64_finish_x86");
    emitter.label("__rt_asf_b64_three_x86");
    emitter.instruction("shl r11, 6");
    emitter.instruction("mov r13, r11");
    emitter.instruction("shr r13, 16");
    emitter.instruction("mov BYTE PTR [rax + r10], r13b");
    emitter.instruction("inc r10");
    emitter.instruction("mov r13, r11");
    emitter.instruction("shr r13, 8");
    emitter.instruction("mov BYTE PTR [rax + r10], r13b");
    emitter.instruction("inc r10");
    emitter.label("__rt_asf_b64_finish_x86");
    emitter.instruction("mov rdx, r10");
    emitter.instruction("ret");

    // -- dechunk (x86_64) — HTTP/1.1 chunked transfer-encoding parser --
    emitter.label("__rt_asf_dechunk_x86");
    emitter.instruction("xor r9, r9");                                          // read index
    emitter.instruction("xor r10, r10");                                        // write index
    emitter.label("__rt_asf_dc_size_loop_x86");
    emitter.instruction("xor r11, r11");                                        // chunk size accumulator
    emitter.label("__rt_asf_dc_size_read_x86");
    emitter.instruction("cmp r9, rdx");
    emitter.instruction("jge __rt_asf_dc_done_x86");
    emitter.instruction("movzx r8d, BYTE PTR [rax + r9]");
    emitter.instruction("inc r9");
    emitter.instruction("cmp r8b, 13");
    emitter.instruction("je __rt_asf_dc_size_eol_x86");
    emitter.instruction("cmp r8b, 59");                                         // ';' ext
    emitter.instruction("je __rt_asf_dc_skip_eol_x86");
    emitter.instruction("cmp r8b, 48");                                         // '0'
    emitter.instruction("jl __rt_asf_dc_size_read_x86");
    emitter.instruction("cmp r8b, 57");                                         // '9'
    emitter.instruction("jle __rt_asf_dc_size_digit_x86");
    emitter.instruction("or r8b, 0x20");                                        // case-fold to lower
    emitter.instruction("cmp r8b, 97");                                         // 'a'
    emitter.instruction("jl __rt_asf_dc_size_read_x86");
    emitter.instruction("cmp r8b, 102");                                        // 'f'
    emitter.instruction("jg __rt_asf_dc_size_read_x86");
    emitter.instruction("sub r8b, 87");                                         // a..f → 10..15
    emitter.instruction("jmp __rt_asf_dc_size_acc_x86");
    emitter.label("__rt_asf_dc_size_digit_x86");
    emitter.instruction("sub r8b, 48");
    emitter.label("__rt_asf_dc_size_acc_x86");
    emitter.instruction("shl r11, 4");
    emitter.instruction("movzx r8, r8b");
    emitter.instruction("or r11, r8");
    emitter.instruction("jmp __rt_asf_dc_size_read_x86");
    emitter.label("__rt_asf_dc_skip_eol_x86");
    emitter.instruction("cmp r9, rdx");
    emitter.instruction("jge __rt_asf_dc_done_x86");
    emitter.instruction("movzx r8d, BYTE PTR [rax + r9]");
    emitter.instruction("inc r9");
    emitter.instruction("cmp r8b, 13");
    emitter.instruction("jne __rt_asf_dc_skip_eol_x86");
    emitter.label("__rt_asf_dc_size_eol_x86");
    emitter.instruction("cmp r9, rdx");
    emitter.instruction("jge __rt_asf_dc_done_x86");
    emitter.instruction("movzx r8d, BYTE PTR [rax + r9]");
    emitter.instruction("cmp r8b, 10");
    emitter.instruction("jne __rt_asf_dc_skip_lf_x86");
    emitter.instruction("inc r9");
    emitter.label("__rt_asf_dc_skip_lf_x86");
    emitter.instruction("test r11, r11");
    emitter.instruction("jz __rt_asf_dc_done_x86");
    // Copy r11 bytes.
    emitter.instruction("xor r12, r12");
    emitter.label("__rt_asf_dc_copy_loop_x86");
    emitter.instruction("cmp r12, r11");
    emitter.instruction("jge __rt_asf_dc_copy_done_x86");
    emitter.instruction("cmp r9, rdx");
    emitter.instruction("jge __rt_asf_dc_done_x86");
    emitter.instruction("movzx r8d, BYTE PTR [rax + r9]");
    emitter.instruction("mov BYTE PTR [rax + r10], r8b");
    emitter.instruction("inc r9");
    emitter.instruction("inc r10");
    emitter.instruction("inc r12");
    emitter.instruction("jmp __rt_asf_dc_copy_loop_x86");
    emitter.label("__rt_asf_dc_copy_done_x86");
    // Skip trailing \r\n.
    emitter.instruction("cmp r9, rdx");
    emitter.instruction("jge __rt_asf_dc_size_loop_x86");
    emitter.instruction("movzx r8d, BYTE PTR [rax + r9]");
    emitter.instruction("cmp r8b, 13");
    emitter.instruction("jne __rt_asf_dc_size_loop_x86");
    emitter.instruction("inc r9");
    emitter.instruction("cmp r9, rdx");
    emitter.instruction("jge __rt_asf_dc_size_loop_x86");
    emitter.instruction("movzx r8d, BYTE PTR [rax + r9]");
    emitter.instruction("cmp r8b, 10");
    emitter.instruction("jne __rt_asf_dc_size_loop_x86");
    emitter.instruction("inc r9");
    emitter.instruction("jmp __rt_asf_dc_size_loop_x86");
    emitter.label("__rt_asf_dc_done_x86");
    emitter.instruction("mov rdx, r10");
    emitter.instruction("ret");

    // -- convert.quoted-printable-decode (x86_64) --
    emitter.label("__rt_asf_qp_decode_x86");
    emitter.instruction("xor r9, r9");                                          // read index
    emitter.instruction("xor r10, r10");                                        // write index
    emitter.label("__rt_asf_qp_loop_x86");
    emitter.instruction("cmp r9, rdx");
    emitter.instruction("jge __rt_asf_qp_done_x86");
    emitter.instruction("movzx r8d, BYTE PTR [rax + r9]");
    emitter.instruction("inc r9");
    emitter.instruction("cmp r8b, 61");                                         // '='
    emitter.instruction("je __rt_asf_qp_escape_x86");
    emitter.instruction("mov BYTE PTR [rax + r10], r8b");
    emitter.instruction("inc r10");
    emitter.instruction("jmp __rt_asf_qp_loop_x86");
    emitter.label("__rt_asf_qp_escape_x86");
    emitter.instruction("cmp r9, rdx");
    emitter.instruction("jge __rt_asf_qp_done_x86");
    emitter.instruction("movzx r8d, BYTE PTR [rax + r9]");
    emitter.instruction("cmp r8b, 13");
    emitter.instruction("je __rt_asf_qp_soft_x86");
    emitter.instruction("cmp r8b, 10");
    emitter.instruction("je __rt_asf_qp_soft_lf_x86");
    // hi nibble inlined
    emitter.instruction("inc r9");
    emitter.instruction("mov r11d, -1");
    emitter.instruction("cmp r8b, 48");
    emitter.instruction("jl __rt_asf_qp_hi_alpha_x86");
    emitter.instruction("cmp r8b, 57");
    emitter.instruction("jg __rt_asf_qp_hi_alpha_x86");
    emitter.instruction("movzx r11, r8b");
    emitter.instruction("sub r11, 48");
    emitter.instruction("jmp __rt_asf_qp_hi_done_x86");
    emitter.label("__rt_asf_qp_hi_alpha_x86");
    emitter.instruction("or r8b, 0x20");
    emitter.instruction("cmp r8b, 97");
    emitter.instruction("jl __rt_asf_qp_hi_done_x86");
    emitter.instruction("cmp r8b, 102");
    emitter.instruction("jg __rt_asf_qp_hi_done_x86");
    emitter.instruction("movzx r11, r8b");
    emitter.instruction("sub r11, 87");
    emitter.label("__rt_asf_qp_hi_done_x86");
    emitter.instruction("cmp r11d, 0");
    emitter.instruction("jl __rt_asf_qp_loop_x86");
    emitter.instruction("mov r12, r11");                                        // hi nibble
    // lo nibble
    emitter.instruction("cmp r9, rdx");
    emitter.instruction("jge __rt_asf_qp_done_x86");
    emitter.instruction("movzx r8d, BYTE PTR [rax + r9]");
    emitter.instruction("inc r9");
    emitter.instruction("mov r11d, -1");
    emitter.instruction("cmp r8b, 48");
    emitter.instruction("jl __rt_asf_qp_lo_alpha_x86");
    emitter.instruction("cmp r8b, 57");
    emitter.instruction("jg __rt_asf_qp_lo_alpha_x86");
    emitter.instruction("movzx r11, r8b");
    emitter.instruction("sub r11, 48");
    emitter.instruction("jmp __rt_asf_qp_lo_done_x86");
    emitter.label("__rt_asf_qp_lo_alpha_x86");
    emitter.instruction("or r8b, 0x20");
    emitter.instruction("cmp r8b, 97");
    emitter.instruction("jl __rt_asf_qp_lo_done_x86");
    emitter.instruction("cmp r8b, 102");
    emitter.instruction("jg __rt_asf_qp_lo_done_x86");
    emitter.instruction("movzx r11, r8b");
    emitter.instruction("sub r11, 87");
    emitter.label("__rt_asf_qp_lo_done_x86");
    emitter.instruction("cmp r11d, 0");
    emitter.instruction("jl __rt_asf_qp_loop_x86");
    emitter.instruction("shl r12, 4");
    emitter.instruction("or r12, r11");
    emitter.instruction("mov BYTE PTR [rax + r10], r12b");
    emitter.instruction("inc r10");
    emitter.instruction("jmp __rt_asf_qp_loop_x86");
    emitter.label("__rt_asf_qp_soft_x86");
    emitter.instruction("inc r9");                                              // skip \r
    emitter.instruction("cmp r9, rdx");
    emitter.instruction("jge __rt_asf_qp_loop_x86");
    emitter.instruction("movzx r8d, BYTE PTR [rax + r9]");
    emitter.instruction("cmp r8b, 10");
    emitter.instruction("jne __rt_asf_qp_loop_x86");
    emitter.instruction("inc r9");
    emitter.instruction("jmp __rt_asf_qp_loop_x86");
    emitter.label("__rt_asf_qp_soft_lf_x86");
    emitter.instruction("inc r9");
    emitter.instruction("jmp __rt_asf_qp_loop_x86");
    emitter.label("__rt_asf_qp_done_x86");
    emitter.instruction("mov rdx, r10");
    emitter.instruction("ret");

    // -- convert.base64-encode (x86_64) --
    emitter.label("__rt_asf_b64_encode_x86");
    // Cap input at 49152 bytes so the 4/3 expansion fits the scratch buffer.
    emitter.instruction("mov r11, 49152");
    emitter.instruction("cmp rdx, r11");
    emitter.instruction("cmovg rdx, r11");                                      // rdx = MIN(rdx, 49152)
    emitter.instruction("lea r11, [rip + _stream_grow_scratch]");               // r11 = scratch base
    emitter.instruction("lea r12, [rip + _b64_encode_tbl]");                    // r12 = alphabet table
    emitter.instruction("xor r9, r9");                                          // read idx
    emitter.instruction("xor r10, r10");                                        // write idx
    emitter.label("__rt_asf_b64e_loop_x86");
    emitter.instruction("mov rcx, rdx");
    emitter.instruction("sub rcx, r9");                                         // remaining bytes
    emitter.instruction("cmp rcx, 3");
    emitter.instruction("jl __rt_asf_b64e_rem_x86");
    // Read 3 bytes.
    emitter.instruction("movzx r13d, BYTE PTR [rax + r9]");                     // byte 0
    emitter.instruction("inc r9");
    emitter.instruction("movzx r14d, BYTE PTR [rax + r9]");                     // byte 1
    emitter.instruction("inc r9");
    emitter.instruction("movzx r15d, BYTE PTR [rax + r9]");                     // byte 2
    emitter.instruction("inc r9");
    // char 0: b0 >> 2
    emitter.instruction("mov rcx, r13");
    emitter.instruction("shr rcx, 2");
    emitter.instruction("movzx ecx, BYTE PTR [r12 + rcx]");
    emitter.instruction("mov BYTE PTR [r11 + r10], cl");
    emitter.instruction("inc r10");
    // char 1: ((b0 & 3) << 4) | (b1 >> 4)
    emitter.instruction("mov rcx, r13");
    emitter.instruction("and rcx, 3");
    emitter.instruction("shl rcx, 4");
    emitter.instruction("mov r8, r14");
    emitter.instruction("shr r8, 4");
    emitter.instruction("or rcx, r8");
    emitter.instruction("movzx ecx, BYTE PTR [r12 + rcx]");
    emitter.instruction("mov BYTE PTR [r11 + r10], cl");
    emitter.instruction("inc r10");
    // char 2: ((b1 & 15) << 2) | (b2 >> 6)
    emitter.instruction("mov rcx, r14");
    emitter.instruction("and rcx, 15");
    emitter.instruction("shl rcx, 2");
    emitter.instruction("mov r8, r15");
    emitter.instruction("shr r8, 6");
    emitter.instruction("or rcx, r8");
    emitter.instruction("movzx ecx, BYTE PTR [r12 + rcx]");
    emitter.instruction("mov BYTE PTR [r11 + r10], cl");
    emitter.instruction("inc r10");
    // char 3: b2 & 0x3f
    emitter.instruction("mov rcx, r15");
    emitter.instruction("and rcx, 63");
    emitter.instruction("movzx ecx, BYTE PTR [r12 + rcx]");
    emitter.instruction("mov BYTE PTR [r11 + r10], cl");
    emitter.instruction("inc r10");
    emitter.instruction("jmp __rt_asf_b64e_loop_x86");
    emitter.label("__rt_asf_b64e_rem_x86");
    emitter.instruction("test rcx, rcx");
    emitter.instruction("jz __rt_asf_b64e_copyback_x86");
    emitter.instruction("cmp rcx, 1");
    emitter.instruction("je __rt_asf_b64e_rem1_x86");
    // 2-byte tail: 3 chars + '='
    emitter.instruction("movzx r13d, BYTE PTR [rax + r9]");
    emitter.instruction("inc r9");
    emitter.instruction("movzx r14d, BYTE PTR [rax + r9]");
    emitter.instruction("inc r9");
    emitter.instruction("mov rcx, r13");
    emitter.instruction("shr rcx, 2");
    emitter.instruction("movzx ecx, BYTE PTR [r12 + rcx]");
    emitter.instruction("mov BYTE PTR [r11 + r10], cl");
    emitter.instruction("inc r10");
    emitter.instruction("mov rcx, r13");
    emitter.instruction("and rcx, 3");
    emitter.instruction("shl rcx, 4");
    emitter.instruction("mov r8, r14");
    emitter.instruction("shr r8, 4");
    emitter.instruction("or rcx, r8");
    emitter.instruction("movzx ecx, BYTE PTR [r12 + rcx]");
    emitter.instruction("mov BYTE PTR [r11 + r10], cl");
    emitter.instruction("inc r10");
    emitter.instruction("mov rcx, r14");
    emitter.instruction("and rcx, 15");
    emitter.instruction("shl rcx, 2");
    emitter.instruction("movzx ecx, BYTE PTR [r12 + rcx]");
    emitter.instruction("mov BYTE PTR [r11 + r10], cl");
    emitter.instruction("inc r10");
    emitter.instruction("mov BYTE PTR [r11 + r10], 61");                        // '='
    emitter.instruction("inc r10");
    emitter.instruction("jmp __rt_asf_b64e_copyback_x86");
    emitter.label("__rt_asf_b64e_rem1_x86");
    // 1-byte tail: 2 chars + '=='
    emitter.instruction("movzx r13d, BYTE PTR [rax + r9]");
    emitter.instruction("inc r9");
    emitter.instruction("mov rcx, r13");
    emitter.instruction("shr rcx, 2");
    emitter.instruction("movzx ecx, BYTE PTR [r12 + rcx]");
    emitter.instruction("mov BYTE PTR [r11 + r10], cl");
    emitter.instruction("inc r10");
    emitter.instruction("mov rcx, r13");
    emitter.instruction("and rcx, 3");
    emitter.instruction("shl rcx, 4");
    emitter.instruction("movzx ecx, BYTE PTR [r12 + rcx]");
    emitter.instruction("mov BYTE PTR [r11 + r10], cl");
    emitter.instruction("inc r10");
    emitter.instruction("mov BYTE PTR [r11 + r10], 61");                        // '='
    emitter.instruction("inc r10");
    emitter.instruction("mov BYTE PTR [r11 + r10], 61");                        // '='
    emitter.instruction("inc r10");
    emitter.label("__rt_asf_b64e_copyback_x86");
    emitter.instruction("xor r9, r9");
    emitter.label("__rt_asf_b64e_cb_loop_x86");
    emitter.instruction("cmp r9, r10");
    emitter.instruction("jge __rt_asf_b64e_done_x86");
    emitter.instruction("movzx ecx, BYTE PTR [r11 + r9]");
    emitter.instruction("mov BYTE PTR [rax + r9], cl");
    emitter.instruction("inc r9");
    emitter.instruction("jmp __rt_asf_b64e_cb_loop_x86");
    emitter.label("__rt_asf_b64e_done_x86");
    emitter.instruction("mov rdx, r10");
    emitter.instruction("ret");

    // -- convert.quoted-printable-encode (x86_64) --
    emitter.label("__rt_asf_qp_encode_x86");
    emitter.instruction("mov r11, 21845");
    emitter.instruction("cmp rdx, r11");
    emitter.instruction("cmovg rdx, r11");                                      // rdx = MIN(rdx, 21845)
    emitter.instruction("lea r11, [rip + _stream_grow_scratch]");
    emitter.instruction("xor r9, r9");
    emitter.instruction("xor r10, r10");
    emitter.label("__rt_asf_qpe_loop_x86");
    emitter.instruction("cmp r9, rdx");
    emitter.instruction("jge __rt_asf_qpe_copyback_x86");
    emitter.instruction("movzx r8d, BYTE PTR [rax + r9]");
    emitter.instruction("inc r9");
    emitter.instruction("cmp r8b, 33");
    emitter.instruction("jl __rt_asf_qpe_escape_x86");
    emitter.instruction("cmp r8b, 126");
    emitter.instruction("jg __rt_asf_qpe_escape_x86");
    emitter.instruction("cmp r8b, 61");
    emitter.instruction("je __rt_asf_qpe_escape_x86");
    emitter.instruction("mov BYTE PTR [r11 + r10], r8b");
    emitter.instruction("inc r10");
    emitter.instruction("jmp __rt_asf_qpe_loop_x86");
    emitter.label("__rt_asf_qpe_escape_x86");
    emitter.instruction("mov BYTE PTR [r11 + r10], 61");                        // '='
    emitter.instruction("inc r10");
    // hi nibble
    emitter.instruction("mov rcx, r8");
    emitter.instruction("shr rcx, 4");
    emitter.instruction("and rcx, 15");
    emitter.instruction("cmp rcx, 10");
    emitter.instruction("jl __rt_asf_qpe_hi_dig_x86");
    emitter.instruction("add rcx, 55");
    emitter.instruction("jmp __rt_asf_qpe_hi_write_x86");
    emitter.label("__rt_asf_qpe_hi_dig_x86");
    emitter.instruction("add rcx, 48");
    emitter.label("__rt_asf_qpe_hi_write_x86");
    emitter.instruction("mov BYTE PTR [r11 + r10], cl");
    emitter.instruction("inc r10");
    // lo nibble
    emitter.instruction("mov rcx, r8");
    emitter.instruction("and rcx, 15");
    emitter.instruction("cmp rcx, 10");
    emitter.instruction("jl __rt_asf_qpe_lo_dig_x86");
    emitter.instruction("add rcx, 55");
    emitter.instruction("jmp __rt_asf_qpe_lo_write_x86");
    emitter.label("__rt_asf_qpe_lo_dig_x86");
    emitter.instruction("add rcx, 48");
    emitter.label("__rt_asf_qpe_lo_write_x86");
    emitter.instruction("mov BYTE PTR [r11 + r10], cl");
    emitter.instruction("inc r10");
    emitter.instruction("jmp __rt_asf_qpe_loop_x86");
    emitter.label("__rt_asf_qpe_copyback_x86");
    emitter.instruction("xor r9, r9");
    emitter.label("__rt_asf_qpe_cb_loop_x86");
    emitter.instruction("cmp r9, r10");
    emitter.instruction("jge __rt_asf_qpe_done_x86");
    emitter.instruction("movzx ecx, BYTE PTR [r11 + r9]");
    emitter.instruction("mov BYTE PTR [rax + r9], cl");
    emitter.instruction("inc r9");
    emitter.instruction("jmp __rt_asf_qpe_cb_loop_x86");
    emitter.label("__rt_asf_qpe_done_x86");
    emitter.instruction("mov rdx, r10");
    emitter.instruction("ret");
}
