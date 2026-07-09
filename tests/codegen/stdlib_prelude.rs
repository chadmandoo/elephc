//! Purpose:
//! Integration tests for the stdlib prelude (EC-2 #485): mb_substr, mb_ltrim, array_last,
//! array_count_values, http_build_query, and the filter_var validate-email/url subset — all
//! injected pay-per-use as elephc-PHP and byte-parity-verified against PHP 8.5.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - The assertions mirror `php -r` runs of the same fixtures (multibyte slicing, negative
//!   offsets, urlencoding with digits and spaces, duplicate counting).

use crate::support::*;

/// mb_substr: prefix, mid-CJK slice, negative start, open end; mb_ltrim: explicit charset and
/// default whitespace; both byte-identical to PHP 8.5.
#[test]
fn test_stdlib_prelude_mb_substr_and_mb_ltrim() {
    let out = compile_and_run(
        "<?php declare(strict_types=1); echo mb_substr('héllo wörld', 0, 1), ':', mb_substr('日本語', 1, 1), ':', mb_substr('abc', -2), ':', mb_substr('héllo', 2), '|', mb_ltrim('///a/b', '/'), ':', mb_ltrim('  x');",
    );
    assert_eq!(out, "h:本:bc:llo|a/b:x");
}

/// array_last over packed and associative arrays; array_count_values duplicate tallies —
/// byte-identical to PHP 8.5.
#[test]
fn test_stdlib_prelude_array_last_and_count_values() {
    let out = compile_and_run(
        "<?php declare(strict_types=1); function main(): void { echo array_last([1, 2, 3]), ':', array_last(['a' => 'x', 'b' => 'y']), '|'; $counts = array_count_values(['a', 'b', 'a', 'a']); echo $counts['a'], ':', $counts['b']; } main();",
    );
    assert_eq!(out, "3:y|3:1");
}

/// http_build_query over flat scalars (ints, spaces, bools) and the filter_var email/url
/// validators' accept/reject behaviour — byte-identical to PHP 8.5.
#[test]
fn test_stdlib_prelude_http_build_query_and_filter_var() {
    let out = compile_and_run(
        "<?php declare(strict_types=1); function main(): void { echo http_build_query(['page' => 2, 'sort' => 'name asc', 'flag' => true]), '|', filter_var('user@example.com', FILTER_VALIDATE_EMAIL) !== false ? 'e1' : 'e0', ':', filter_var('not-an-email', FILTER_VALIDATE_EMAIL) !== false ? 'e1' : 'e0', ':', filter_var('https://aic.lan/x?y=1', FILTER_VALIDATE_URL) !== false ? 'u1' : 'u0', ':', filter_var('nope', FILTER_VALIDATE_URL) !== false ? 'u1' : 'u0', ':', UPLOAD_ERR_OK; } main();",
    );
    assert_eq!(out, "page=2&sort=name+asc&flag=1|e1:e0:u1:u0:0");
}

/// mb_trim (default + explicit charset), the ASCII-subset mb-case functions, and the
/// parse_url component subset (scheme with and without authority, host behind userinfo,
/// port, path) — byte-identical to PHP 8.5.
#[test]
fn test_stdlib_prelude_mb_trim_case_and_parse_url() {
    let out = compile_and_run(
        "<?php declare(strict_types=1); echo mb_trim('  padded  '), ':', mb_trim('//x//', '/'), '|', mb_strtoupper('abc'), ':', mb_strtolower('XYZ'), '|', parse_url('https://aic.lan:8443/admin?x=1#top', PHP_URL_SCHEME), ':', parse_url('https://aic.lan:8443/admin?x=1#top', PHP_URL_HOST), ':', parse_url('https://aic.lan:8443/admin?x=1#top', PHP_URL_PORT), ':', parse_url('/relative/path', PHP_URL_SCHEME) ?? 'null', '|', parse_url('javascript:alert(1)', PHP_URL_SCHEME), ':', parse_url('https://u:p@h.io:99/a/b?q#f', PHP_URL_HOST), ':', parse_url('https://u:p@h.io:99/a/b?q#f', PHP_URL_PATH);",
    );
    assert_eq!(out, "padded:x|ABC:xyz|https:aic.lan:8443:null|javascript:h.io:/a/b");
}

/// array_search accepts the optional strict (3rd) argument — the per-type comparison already
/// matches strict membership for homogeneously-typed arrays (ward-theme ParentChain::depthOf).
#[test]
fn test_array_search_accepts_strict_argument() {
    let out = compile_and_run(
        "<?php declare(strict_types=1); function main(): void { $themes = ['base', 'main', 'admin']; echo array_search('main', $themes, true), ':', array_search('missing', $themes, true) === false ? 'F' : '?'; } main();",
    );
    assert_eq!(out, "1:F");
}

/// EC-40 (#533): strspn covers the 2-arg form and the 3-arg offset form
/// (CssTokenScanner advances a cursor with `strspn($css, WS, $offset)`;
/// FileConfigSyncStorage validates leading/body character runs).
#[test]
fn test_strspn_two_and_three_arg_forms() {
    let out = compile_and_run(
        r#"<?php
$css = "   a  b";
$offset = 0;
$offset += strspn($css, " \t\n", $offset);
echo $offset, "|";
echo strspn("abc123", "abcdefghijklmnopqrstuvwxyz"), "|";
echo strspn("42abc", "0123456789"), "|";
echo strspn("", "abc"), "|";
echo strspn("xxaayy", "a", 2);
"#,
    );
    assert_eq!(out, "3|3|2|0|2");
}

/// EC-40 (#533): mb_substr_count counts non-overlapping needles (byte-wise
/// counting is exact for well-formed UTF-8 — needles cannot straddle
/// codepoints), and strncmp honors PHP's sign-only contract
/// (PipelineSpec line counting; WalCommitWriter's 4096-byte prefix compare).
#[test]
fn test_mb_substr_count_and_strncmp() {
    let out = compile_and_run(
        r#"<?php
$body = "line1\nline2\nline3\n";
echo mb_substr_count(mb_rtrim($body, "\n"), "\n") + 1, "|";
echo mb_substr_count("ababab", "ab"), "|";
echo mb_substr_count("aaa", "aa"), "|";
echo mb_substr_count("héhé", "é"), "|";
echo strncmp("abcdef", "abcxyz", 3), "|";
$c = strncmp("abcdef", "abcxyz", 4);
echo $c < 0 ? "neg" : "posz", "|";
$d = strncmp("b", "a", 4096);
echo $d > 0 ? "pos" : "negz", "|";
echo strncmp("ab", "ab", 10);
"#,
    );
    assert_eq!(out, "3|3|1|2|0|neg|pos|0");
}

/// EC-50 (#543): end() returns the last element of an array (read-only value
/// access — ComponentDescriptorFactory's `(string) preg_replace(..., end($segments))`).
/// The internal-pointer side effect PHP's end() performs is not modeled (elephc
/// arrays carry no per-value cursor); the returned value is byte-parity for a
/// non-empty read.
#[test]
fn test_end_returns_last_element() {
    let out = compile_and_run(
        r#"<?php
$segments = explode(" ", "camel case words");
echo (string) end($segments), "|";
$one = ["only"];
echo end($one), "|";
$nums = [10, 20, 30];
echo end($nums);
"#,
    );
    assert_eq!(out, "words|only|30");
}

/// PHP_VERSION / PHP_BINARY (interpreter-identity string constants) and FILEINFO_MIME_TYPE
/// (ext/fileinfo int flag) resolve to their compile-time PHP 8.5 target values.
#[test]
fn test_php_version_binary_fileinfo_constants() {
    let out = compile_and_run(
        r#"<?php
echo PHP_VERSION, "|", PHP_BINARY, "|", FILEINFO_MIME_TYPE, "|";
echo explode('.', PHP_VERSION)[0];
"#,
    );
    assert_eq!(out, "8.5.7|/usr/bin/php8.5|16|8");
}

/// EC-54 builtins batch: substr_count (2-arg occurrence count), addcslashes (literal-set
/// backslash escaping), pack('H*') (hex→binary), and array_fill_keys over a Mixed keys
/// argument (adaptive prelude) — each byte-identical to PHP 8.5.
#[test]
fn test_ec54_builtins_batch() {
    let out = compile_and_run(
        r#"<?php
echo substr_count("banana split banana", "banana"), "|";
echo addcslashes("50%_off", '%_'), "|";
echo bin2hex(pack('H*', 'deadbeef')), "|";
$filled = array_fill_keys(json_decode('["a","b"]', true), 7);
echo $filled['a'], $filled['b'];
"#,
    );
    assert_eq!(out, "2|50\\%\\_off|deadbeef|77");
}

