//! Purpose:
//! Regression tests for call-site parameter type inference of untyped parameters.
//! Ensures a parameter called with heterogeneous argument types is inferred as
//! `Mixed` (boxed), not collapsed to a single type that mis-tags scalar arguments.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - The bug surfaced via PDO `bindValue` with mixed `?`/`:name` placeholders: an
//!   int argument passed to a param that another call site passed a string to was
//!   coerced to a string, so `is_int` returned false and values were corrupted.

use crate::support::*;

/// A free function with an untyped parameter called with both string and int
/// arguments infers the parameter as `Mixed`, so `is_int` on the int argument is
/// true (the int is boxed, not coerced to string).
#[test]
fn test_untyped_param_heterogeneous_calls_infer_mixed() {
    let out = compile_and_run(
        r#"<?php
function tag($a) { return is_int($a) ? "I" : "N"; }
echo tag("x") . tag(1) . tag(2.5) . tag(7);
"#,
    );
    assert_eq!(out, "NINI");
}

/// The same inference applies to instance-method parameters: params called with
/// both int and string are `Mixed`, so `is_int` and the round-tripped argument
/// values are correct regardless of call order.
#[test]
fn test_untyped_method_param_heterogeneous_calls_infer_mixed() {
    let out = compile_and_run(
        r#"<?php
class Box {
    public function put($a, $b, int $c) {
        return (is_int($a) ? "I" : "N") . $a . ":" . $b . ":" . $c;
    }
}
$o = new Box();
echo $o->put(1, "x", 5) . "|" . $o->put("y", 2, 6);
"#,
    );
    assert_eq!(out, "I1:x:5|Ny:2:6");
}

/// A parameter that is genuinely homogeneous (only int call sites) stays a
/// concrete int and is not over-widened to `Mixed`.
#[test]
fn test_untyped_param_homogeneous_int_stays_int() {
    let out = compile_and_run(
        r#"<?php
function only_int($a) { return is_int($a) ? "I" : "N"; }
echo only_int(1) . only_int(2) . only_int(3);
"#,
    );
    assert_eq!(out, "III");
}

/// An integer argument passed to a declared `float` parameter is converted with int→float
/// (`IToF`) before the call, not reinterpreted as a raw 64-bit bit-pattern. A single int argument
/// to a single float parameter previously produced garbage (the int bits read as a double).
#[test]
fn test_int_arg_to_float_param_single() {
    let out = compile_and_run(
        r#"<?php
function f(float $g): float { return $g; }
echo f(2), "|", f(7);
"#,
    );
    assert_eq!(out, "2|7");
}

/// When a float parameter receiving an int argument sits next to another float argument, the
/// int→float conversion must target the correct slot. Without the conversion the unconverted
/// argument slot was overwritten by the neighbouring float argument, so both parameters read the
/// same value regardless of argument order.
#[test]
fn test_int_arg_to_float_param_beside_float_arg() {
    let out = compile_and_run(
        r#"<?php
function f(float $g, float $h): string { return $g . "," . $h; }
echo f(90.5, 2), "|", f(2, 90.5);
"#,
    );
    assert_eq!(out, "90.5,2|2,90.5");
}

/// A parameter that is conditionally reassigned in the body widens to `Mixed`, so
/// the prologue boxes it via `__rt_mixed_from_value`. That call clobbers the ABI
/// argument registers; if a later parameter is still living in a register when the
/// earlier one is boxed, its value is lost. The prologue must spill every incoming
/// parameter register to its slot BEFORE boxing any of them. Repro: `$length` (5)
/// read back as 0 (the value of `$start`) because boxing `$start` zeroed `rsi`
/// before `$length` was spilled. The reassign branch need not even be taken — its
/// mere presence widens `$start` to Mixed and triggers the boxing.
#[test]
fn test_conditional_param_reassign_preserves_sibling_param() {
    let out = compile_and_run(
        r#"<?php
function h(int $start, int $length = 777): string {
    if ($start < 0) { $start = 1 + $start; }
    return "got:$length";
}
echo h(0, 5), "|", h(-2, 9);
"#,
    );
    assert_eq!(out, "got:5|got:9");
}

/// The same corruption with three parameters and a string sibling: boxing the
/// reassigned first parameter must not disturb the later int/string parameters
/// still in argument registers. Exercises both the taken and untaken reassign
/// branch and a string-typed sibling (two-register ABI slot).
#[test]
fn test_conditional_param_reassign_preserves_multiple_siblings() {
    let out = compile_and_run(
        r#"<?php
function a(int $x, int $y, string $z): string {
    if ($x < 0) { $x = -$x; }
    return "$x|$y|$z";
}
echo a(-3, 7, "k"), "/", a(4, 8, "m");
"#,
    );
    assert_eq!(out, "3|7|k/4|8|m");
}
