//! Purpose:
//! Integration tests for stream-extension builtins: fgetc, readfile, fpassthru,
//! flock, and tmpfile.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - Each test uses a fresh temporary directory; the helpers in `support` keep
//!   the working directory isolated for parallel test runs.

use super::*;

#[test]
fn test_fgetc_reads_one_byte() {
    let (out, dir) = compile_and_run_in_dir(
        r#"<?php
file_put_contents("c.txt", "abc");
$h = fopen("c.txt", "r");
echo fgetc($h) . fgetc($h) . fgetc($h);
fclose($h);
"#,
    );
    assert_eq!(out, "abc");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_fgetc_returns_empty_at_eof() {
    let (out, dir) = compile_and_run_in_dir(
        r#"<?php
file_put_contents("eof.txt", "x");
$h = fopen("eof.txt", "r");
fgetc($h);
$tail = fgetc($h);
echo strlen($tail);
fclose($h);
"#,
    );
    assert_eq!(out, "0");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_readfile_streams_to_stdout() {
    let (out, dir) = compile_and_run_in_dir(
        r#"<?php
file_put_contents("rf.txt", "hello world");
$bytes = readfile("rf.txt");
echo "|" . $bytes;
"#,
    );
    assert_eq!(out, "hello world|11");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_readfile_missing_returns_zero() {
    let out = compile_and_run(
        r#"<?php
$bytes = readfile("/nonexistent/path/xyz.txt");
echo $bytes;
"#,
    );
    assert_eq!(out, "0");
}

#[test]
fn test_readfile_large_buffer_loop() {
    let (out, dir) = compile_and_run_in_dir(
        r#"<?php
$payload = str_repeat("A", 5000);
file_put_contents("big.txt", $payload);
$bytes = readfile("big.txt");
echo "|" . $bytes;
"#,
    );
    assert!(out.starts_with(&"A".repeat(5000)), "got: {}", out);
    assert!(out.ends_with("|5000"), "got: {}", out);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_fpassthru_streams_remaining_bytes() {
    let (out, dir) = compile_and_run_in_dir(
        r#"<?php
file_put_contents("pt.txt", "abcdefghij");
$h = fopen("pt.txt", "r");
fread($h, 4);
$bytes = fpassthru($h);
echo "|" . $bytes;
fclose($h);
"#,
    );
    assert_eq!(out, "efghij|6");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_flock_exclusive_then_unlock() {
    let (out, dir) = compile_and_run_in_dir(
        r#"<?php
file_put_contents("lock.txt", "data");
$h = fopen("lock.txt", "r+");
$got = flock($h, LOCK_EX);
$released = flock($h, LOCK_UN);
fclose($h);
echo ($got ? "y" : "n") . "|" . ($released ? "y" : "n");
"#,
    );
    assert_eq!(out, "y|y");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_flock_shared() {
    let (out, dir) = compile_and_run_in_dir(
        r#"<?php
file_put_contents("ls.txt", "");
$h = fopen("ls.txt", "r");
echo flock($h, LOCK_SH) ? "y" : "n";
flock($h, LOCK_UN);
fclose($h);
"#,
    );
    assert_eq!(out, "y");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_tmpfile_returns_writable_resource() {
    let out = compile_and_run(
        r#"<?php
$h = tmpfile();
$wrote = fwrite($h, "scratch");
fseek($h, 0);
$content = fread($h, 7);
fclose($h);
echo $wrote . "|" . $content;
"#,
    );
    assert_eq!(out, "7|scratch");
}

#[test]
fn test_tmpfile_returns_resource_type() {
    let out = compile_and_run(
        r#"<?php
$h = tmpfile();
echo gettype($h) . "|";
echo $h === false ? "false" : "resource";
fclose($h);
"#,
    );
    assert_eq!(out, "resource|resource");
}

#[test]
fn test_lock_constants_have_php_values() {
    let out = compile_and_run(
        r#"<?php echo LOCK_SH . "|" . LOCK_EX . "|" . LOCK_UN . "|" . LOCK_NB;"#,
    );
    assert_eq!(out, "1|2|3|4");
}

#[test]
fn test_function_exists_streams_ext() {
    let out = compile_and_run(
        r#"<?php
echo function_exists('fgetc') ? "y" : "n";
echo function_exists('readfile') ? "y" : "n";
echo function_exists('fpassthru') ? "y" : "n";
echo function_exists('flock') ? "y" : "n";
echo function_exists('tmpfile') ? "y" : "n";
"#,
    );
    assert_eq!(out, "yyyyy");
}

#[test]
fn test_streams_ext_case_insensitive_calls() {
    let (out, dir) = compile_and_run_in_dir(
        r#"<?php
file_put_contents("ci.txt", "ok");
$bytes = READFILE("ci.txt");
echo "|" . $bytes;
"#,
    );
    assert_eq!(out, "ok|2");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_streams_ext_namespace_fallback() {
    let (out, dir) = compile_and_run_in_dir(
        r#"<?php
namespace App;
file_put_contents("ns.txt", "hi");
$bytes = readfile("ns.txt");
echo "|" . $bytes;
"#,
    );
    assert_eq!(out, "hi|2");
    let _ = fs::remove_dir_all(&dir);
}
