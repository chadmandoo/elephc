//! Purpose:
//! Integration or regression tests for end-to-end codegen coverage of strings misc, including escaped dollar.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - Inline PHP fixtures are compiled to native binaries and assertions compare stdout or expected failures.

use super::*;

/// Tests that a backslash-escaped dollar (`\$`) inside a double-quoted string is
/// treated as a literal dollar sign, matching PHP's escape sequence behavior.
#[test]
fn test_string_escaped_dollar() {
    let out = compile_and_run(r#"<?php echo "price is \$5";"#);
    assert_eq!(out, "price is $5");
}

/// Tests that a multibyte UTF-8 string literal preceding ASCII digits round-trips
/// correctly through the compiler with no byte-level corruption or digit mishandling.
#[test]
fn test_multibyte_string_literal_before_ascii_digits_round_trips() {
    let out = compile_and_run("<?php echo '日本語123';");
    assert_eq!(out, "日本語123");
}

/// Verifies compiled PHP output for string control escape sequences.
#[test]
fn test_string_control_escape_sequences() {
    // \r, \v, \e, \f process to their ASCII control bytes (regression for
    // the lexer previously emitting a literal backslash for these escapes).
    let out = compile_and_run(
        r#"<?php echo strlen("a\r\nb") . "|" . ord("\r") . "|" . ord("\v") . "|" . ord("\e") . "|" . ord("\f");"#,
    );
    assert_eq!(out, "4|13|11|27|12");
}

// --- md5 / sha1 ---

/// Verifies the ext-fileinfo `finfo` prelude class sniffs the MIME types AIC's
/// FilesystemManagedFileStore accepts (image/jpeg, image/png, image/gif, image/webp,
/// application/pdf) by magic bytes, byte-parity with PHP 8.5 for real headers. Unrecognized
/// content returns `application/octet-stream` (a documented approximation of libmagic's
/// text/plain — both match no allow-list entry, so AIC rejects such uploads identically).
/// Forge EC-78 / #572.
#[test]
fn test_finfo_buffer_mime_detection() {
    let out = compile_and_run(
        r#"<?php
$fi = new finfo(FILEINFO_MIME_TYPE);
echo $fi->buffer("\xFF\xD8\xFF\xE0\x00\x10JFIF") . "|";
echo $fi->buffer("\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR") . "|";
echo $fi->buffer("GIF89a\x01\x00") . "|";
echo $fi->buffer("RIFF\x24\x00\x00\x00WEBPVP8 ") . "|";
echo $fi->buffer("%PDF-1.7\n") . "|";
echo $fi->buffer("just plain text");
"#,
    );
    assert_eq!(
        out,
        "image/jpeg|image/png|image/gif|image/webp|application/pdf|application/octet-stream"
    );
}
