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

/// Regression guard (#639): string builtins must accept a boxed-Mixed subject.
///
/// Array KEYS are boxed Mixed cells by design — a `foreach` key is an `Op::IterCurrentKey` cell
/// and `array_keys()` yields the same. String builtins used to reject that operand outright
/// ("mb_ereg_match subject for PHP type Mixed" and friends), which alone accounted for a large
/// share of the native-compilation gap. This locks in the operand unbox across the family and
/// through both key sources, so the rejection cannot come back unnoticed.
#[test]
fn test_string_builtins_accept_boxed_mixed_key_subject() {
    let out = compile_and_run(
        r#"<?php
$src = ["data-foo" => 1, "Bar" => 2];
$viaKeys = "";
foreach (array_keys($src) as $k) {
    $viaKeys .= strlen($k) . strtoupper($k) . strtolower($k) . substr($k, 0, 2)
        . (str_contains($k, "a") ? "y" : "n") . trim($k) . str_replace("-", "_", $k) . ";";
}
$viaForeach = "";
foreach (["aa" => 1, "bbb" => 2] as $k => $v) { $viaForeach .= strlen($k) . strtoupper($k) . ";"; }
$re = "";
foreach (array_keys(["ok-1" => 1, "Bad Key" => 2]) as $k) {
    $re .= mb_ereg_match('^[a-z][a-z0-9-]*\z', $k) ? "y" : "n";
}
echo $viaKeys, "|", $viaForeach, "|", $re;
"#,
    );
    assert_eq!(
        out,
        "8DATA-FOOdata-foodaydata-foodata_foo;3BARbarBayBarBar;|2AA;3BBB;|yn"
    );
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
