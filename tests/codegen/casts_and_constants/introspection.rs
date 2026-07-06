//! Purpose:
//! Integration or regression tests for end-to-end codegen coverage of casts, constants, and introspection introspection, including gettype integer, gettype float, and gettype string.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - Inline PHP fixtures are compiled to native binaries and assertions compare stdout or expected failures.

use super::*;

/// Tests that `gettype(42)` returns "integer".
#[test]
fn test_gettype_int() {
    let out = compile_and_run("<?php echo gettype(42);");
    assert_eq!(out, "integer");
}

/// Tests that `gettype(3.14)` returns "double" (PHP's float type name).
#[test]
fn test_gettype_float() {
    let out = compile_and_run("<?php echo gettype(3.14);");
    assert_eq!(out, "double");
}

/// Tests that `gettype("hi")` returns "string".
#[test]
fn test_gettype_string() {
    let out = compile_and_run("<?php echo gettype(\"hi\");");
    assert_eq!(out, "string");
}

/// Tests that `gettype(true)` returns "boolean".
#[test]
fn test_gettype_bool() {
    let out = compile_and_run("<?php echo gettype(true);");
    assert_eq!(out, "boolean");
}

/// Tests that `gettype(null)` returns "NULL".
#[test]
fn test_gettype_null() {
    let out = compile_and_run("<?php echo gettype(null);");
    assert_eq!(out, "NULL");
}

/// Tests that `gettype` on a mixed value returns the concrete payload type
/// (integer, string, NULL, array, boolean) rather than "mixed".
#[test]
fn test_gettype_mixed_returns_concrete_payload_type() {
    let out = compile_and_run(
        r#"<?php
$map = [
    "i" => 42,
    "s" => "hi",
    "n" => null,
    "a" => [1, 2],
    "b" => true,
];
echo gettype($map["i"]);
echo "|";
echo gettype($map["s"]);
echo "|";
echo gettype($map["n"]);
echo "|";
echo gettype($map["a"]);
echo "|";
echo gettype($map["b"]);
"#,
    );
    assert_eq!(out, "integer|string|NULL|array|boolean");
}

// --- empty ---

/// Tests that `empty(0)` is true (0 is falsy in PHP).
#[test]
fn test_empty_zero() {
    let out = compile_and_run("<?php echo empty(0);");
    assert_eq!(out, "1");
}

/// Tests that `empty(42)` is false (non-zero int is truthy).
#[test]
fn test_empty_nonzero() {
    let out = compile_and_run("<?php echo empty(42);");
    assert_eq!(out, "");
}

/// Tests that `empty("")` is true (empty string is falsy).
#[test]
fn test_empty_empty_string() {
    let out = compile_and_run("<?php echo empty(\"\");");
    assert_eq!(out, "1");
}

/// Tests that `empty("hi")` is false (non-empty string is truthy).
#[test]
fn test_empty_nonempty_string() {
    let out = compile_and_run("<?php echo empty(\"hi\");");
    assert_eq!(out, "");
}

/// Tests that `empty(null)` is true.
#[test]
fn test_empty_null() {
    let out = compile_and_run("<?php echo empty(null);");
    assert_eq!(out, "1");
}

/// Tests that `empty(false)` is true.
#[test]
fn test_empty_false() {
    let out = compile_and_run("<?php echo empty(false);");
    assert_eq!(out, "1");
}

/// Tests that `empty(true)` is false.
#[test]
fn test_empty_true() {
    let out = compile_and_run("<?php echo empty(true);");
    assert_eq!(out, "");
}

/// Tests that `empty` on a mixed-valued associative array uses boxed payload
/// semantics (zeros/blank/null/empty-array are falsy; non-zeros/non-blank are truthy).
#[test]
fn test_empty_mixed_uses_boxed_payload_semantics() {
    let out = compile_and_run(
        r#"<?php
$map = [
    "zero" => 0,
    "blank" => "",
    "null" => null,
    "arr" => [],
    "one" => 1,
    "text" => "hi",
];
echo empty($map["zero"]) ? "1" : "0";
echo empty($map["blank"]) ? "1" : "0";
echo empty($map["null"]) ? "1" : "0";
echo empty($map["arr"]) ? "1" : "0";
echo empty($map["one"]) ? "1" : "0";
echo empty($map["text"]) ? "1" : "0";
"#,
    );
    assert_eq!(out, "111100");
}

// --- unset ---

/// Tests that `unset` marks a variable as undefined so `is_null` returns true.
#[test]
fn test_unset_variable() {
    let out = compile_and_run(
        r#"<?php
$x = 42;
unset($x);
echo is_null($x);
"#,
    );
    assert_eq!(out, "1");
}

// --- settype ---

/// Tests that `settype($x, "string")` converts an integer to a string.
#[test]
fn test_settype_to_string() {
    let out = compile_and_run(
        r#"<?php
$x = 42;
settype($x, "string");
echo $x;
"#,
    );
    assert_eq!(out, "42");
}

/// Tests that `settype($x, "integer")` truncates a float to an integer.
#[test]
fn test_settype_to_int() {
    let out = compile_and_run(
        r#"<?php
$x = 3.7;
settype($x, "integer");
echo $x;
"#,
    );
    assert_eq!(out, "3");
}

// --- Missing type function tests ---

/// EC-21 (#504): `get_debug_type()` — statically-typed arguments fold to constant
/// names at compile time; Mixed arguments dispatch on the boxed cell's kind tag at
/// runtime (objects resolve their FQCN through the class table). Byte-parity vs PHP 8.5.
#[test]
fn test_get_debug_type_static_and_mixed() {
    let out = compile_and_run(
        r#"<?php
class Widget {}
function describe(mixed $level): string {
    return get_debug_type($level);
}
function main(): void {
    $i = 7;
    $s = 'x';
    $f = 1.5;
    $b = true;
    $a = [1, 2];
    $o = new Widget();
    echo get_debug_type($i), ':', get_debug_type($s), ':', get_debug_type($f), ':', get_debug_type($b), ':', get_debug_type($a), ':', get_debug_type($o), '|';
    echo describe(7), ':', describe('x'), ':', describe(1.5), ':', describe(false), ':', describe([1]), ':', describe(new Widget()), ':', describe(null), '|';
    echo describe(json_decode('42'));
}
main();
"#,
    );
    assert_eq!(
        out,
        "int:string:float:bool:array:Widget|int:string:float:bool:array:Widget:null|int"
    );
}

/// EC-21 follow-on (#504): `instanceof self` narrows to the ENCLOSING class, not to a
/// literal class named "self" — the LogLevel::fromMixed guard shape (return the
/// narrowed receiver from a `: self`-typed factory). Byte-parity vs PHP 8.5.
#[test]
fn test_instanceof_self_guard_narrowing() {
    let out = compile_and_run(
        r#"<?php
class Level {
    public function __construct(public string $name) {}
    public static function fromMixed(mixed $level): self {
        if ($level instanceof self) {
            return $level;
        }
        return new self('default');
    }
}
function main(): void {
    $direct = Level::fromMixed(new Level('warn'));
    $fallback = Level::fromMixed('warn');
    echo $direct->name, ':', $fallback->name;
}
main();
"#,
    );
    assert_eq!(out, "warn:default");
}

/// EC-22 (#505): `set_error_handler`/`restore_error_handler` are accepted no-ops —
/// the closure argument compiles and is evaluated, set returns null ("no previous
/// handler"), restore returns true, and the guarded call's stdout behavior matches
/// PHP (natives emit no PHP-level warnings for a handler to swallow).
#[test]
fn test_error_handler_noops() {
    let out = compile_and_run(
        r#"<?php
function main(): void {
    $prev = set_error_handler(static fn (): bool => true);
    echo $prev === null ? 'null' : 'set', ';';
    $h = fopen('/etc/hostname', 'r');
    echo $h === false ? 'false' : 'handle', ';';
    echo restore_error_handler() ? 'true' : 'false';
}
main();
"#,
    );
    assert_eq!(out, "null;handle;true");
}
