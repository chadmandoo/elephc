//! Purpose:
//! Integration or regression tests for end-to-end codegen coverage of type-related builtins strict comparison semantics, including strict equality integer same, strict equality integer different, and strict inequality integer same.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - Inline PHP fixtures are compiled to native binaries and assertions compare stdout or expected failures.

use super::*;

/// Verifies `===` returns true for identical integer values.
#[test]
fn test_strict_eq_int_same() {
    let out = compile_and_run("<?php echo 1 === 1;");
    assert_eq!(out, "1");
}

/// Verifies `===` returns empty (false) when comparing two different integer values.
#[test]
fn test_strict_eq_int_different() {
    let out = compile_and_run("<?php echo 1 === 2;");
    assert_eq!(out, "");
}

/// Verifies `!==` returns empty (false) when comparing two identical integer values.
#[test]
fn test_strict_neq_int_same() {
    let out = compile_and_run("<?php echo 1 !== 1;");
    assert_eq!(out, "");
}

/// Verifies `!==` returns true for two different integer values.
#[test]
fn test_strict_neq_int_different() {
    let out = compile_and_run("<?php echo 1 !== 2;");
    assert_eq!(out, "1");
}

/// Verifies `===` returns false when comparing int `1` to bool `true` (different types).
#[test]
fn test_strict_eq_int_vs_bool() {
    let out = compile_and_run("<?php echo 1 === true;");
    assert_eq!(out, "");
}

/// #642: deep `===` for two populated indexed `Array(Mixed)` values — equal by value,
/// unequal by value, unequal by length. Bare `array` params are `Array(Mixed)`, so this
/// exercises the new `__rt_array_strict_eq` deep comparator (packed operands normalized).
#[test]
fn test_strict_eq_arrays_indexed() {
    let out = compile_and_run(
        r#"<?php
function eq(array $a, array $b): bool { return $a === $b; }
echo eq([1,2,3],[1,2,3])?'T':'F', eq([1,2,3],[1,2,4])?'T':'F', eq([1,2],[1,2,3])?'T':'F';
"#,
    );
    assert_eq!(out, "TFF");
}

/// #642: deep `===` for associative arrays is order-sensitive and key-sensitive.
#[test]
fn test_strict_eq_arrays_assoc_order_and_keys() {
    let out = compile_and_run(
        r#"<?php
function eq(array $a, array $b): bool { return $a === $b; }
echo eq(['a'=>1,'b'=>2],['a'=>1,'b'=>2])?'T':'F',
     eq(['a'=>1],['b'=>1])?'T':'F',
     eq(['a'=>1,'b'=>2],['b'=>2,'a'=>1])?'T':'F';
"#,
    );
    assert_eq!(out, "TFF");
}

/// #642: values compare type-strictly (int `1` !== string `'1'`); string values compare
/// by bytes.
#[test]
fn test_strict_eq_arrays_type_strict_values() {
    let out = compile_and_run(
        r#"<?php
$ia=[1]; $sa=['1']; echo ($ia===$sa)?'T':'F';
$xa=['x'=>'hi']; $xb=['x'=>'hi']; echo ($xa===$xb)?'T':'F';
$ya=['x'=>'hi']; $yb=['x'=>'ho']; echo ($ya===$yb)?'T':'F';
"#,
    );
    assert_eq!(out, "FTF");
}

/// #642: the comparator recurses into nested arrays (indexed and associative).
#[test]
fn test_strict_eq_arrays_nested_recursion() {
    let out = compile_and_run(
        r#"<?php
$a=[[1,2],[3]]; $b=[[1,2],[3]]; echo ($a===$b)?'T':'F';
$c=[[1,2]]; $d=[[1,3]]; echo ($c===$d)?'T':'F';
$e=[['a'=>1]]; $f=[['a'=>1]]; echo ($e===$f)?'T':'F';
"#,
    );
    assert_eq!(out, "TFT");
}

/// #642: empty arrays, heterogeneous scalar elements, and a differing bool element.
#[test]
fn test_strict_eq_arrays_empty_and_mixed_scalars() {
    let out = compile_and_run(
        r#"<?php
function eq(array $a, array $b): bool { return $a === $b; }
echo eq([],[])?'T':'F',
     eq([1,'x',true,null],[1,'x',true,null])?'T':'F',
     eq([1,'x',true],[1,'x',false])?'T':'F';
"#,
    );
    assert_eq!(out, "TTF");
}

/// #642: cross-kind — a packed indexed array vs an associative hash (int keys 0,1 vs
/// string keys) is unequal; `!==` inverts the deep comparison.
#[test]
fn test_strict_eq_arrays_cross_kind_and_neq() {
    let out = compile_and_run(
        r#"<?php
function eq(array $a, array $b): bool { return $a === $b; }
function neq(array $a, array $b): bool { return $a !== $b; }
echo eq([1,2],['a'=>1,'b'=>2])?'T':'F',
     neq([1,2],[1,2])?'T':'F',
     neq([1,2],[1,3])?'T':'F';
"#,
    );
    assert_eq!(out, "FFT");
}

/// #642: object elements compare by identity (`===`), matching PHP — the same instance is
/// equal, two distinct-but-equal instances are not.
#[test]
fn test_strict_eq_arrays_object_identity() {
    let out = compile_and_run(
        r#"<?php
final class Box { public function __construct(public int $n) {} }
function eq(array $a, array $b): bool { return $a === $b; }
$o = new Box(1);
echo eq([$o],[$o])?'T':'F', eq([new Box(1)],[new Box(1)])?'T':'F';
"#,
    );
    assert_eq!(out, "TF");
}

/// #642: a nested array boxed inside a heterogeneous `Array(Mixed)` element deep-compares
/// by value through the mutual recursion between `__rt_array_strict_eq` and
/// `__rt_mixed_strict_eq`, not by box-pointer identity.
#[test]
fn test_strict_eq_arrays_nested_in_mixed() {
    let out = compile_and_run(
        r#"<?php
$a=[1,[2,3]]; $b=[1,[2,3]]; echo ($a===$b)?'T':'F';
$c=[1,[2,3]]; $d=[1,[2,4]]; echo ($c===$d)?'T':'F';
$e=['k'=>1,'m'=>['x'=>2]]; $f=['k'=>1,'m'=>['x'=>2]]; echo ($e===$f)?'T':'F';
"#,
    );
    assert_eq!(out, "TFT");
}

/// Verifies `!==` returns true when comparing int `1` to bool `true` (different types).
#[test]
fn test_strict_neq_int_vs_bool() {
    let out = compile_and_run("<?php echo 1 !== true;");
    assert_eq!(out, "1");
}

/// Verifies `===` returns false when comparing int `1` to string `"1"` (different types).
#[test]
fn test_strict_eq_int_vs_string() {
    let out = compile_and_run("<?php echo 1 === \"1\";");
    assert_eq!(out, "");
}

/// Verifies `===` returns true for two identical string values.
#[test]
fn test_strict_eq_string_same() {
    let out = compile_and_run("<?php echo \"hello\" === \"hello\";");
    assert_eq!(out, "1");
}

/// Verifies `===` returns empty (false) when comparing two different string values.
#[test]
fn test_strict_eq_string_different() {
    let out = compile_and_run("<?php echo \"hello\" === \"world\";");
    assert_eq!(out, "");
}

/// Verifies `!==` returns true for two different string values.
#[test]
fn test_strict_neq_string() {
    let out = compile_and_run("<?php echo \"abc\" !== \"def\";");
    assert_eq!(out, "1");
}

/// Verifies `===` returns true when both operands are boolean `true`.
#[test]
fn test_strict_eq_bool_true() {
    let out = compile_and_run("<?php echo true === true;");
    assert_eq!(out, "1");
}

/// Verifies `===` returns true when both operands are boolean `false`.
#[test]
fn test_strict_eq_bool_false() {
    let out = compile_and_run("<?php echo false === false;");
    assert_eq!(out, "1");
}

/// Verifies `===` returns empty (false) when comparing `true` to `false`.
#[test]
fn test_strict_eq_bool_mixed() {
    let out = compile_and_run("<?php echo true === false;");
    assert_eq!(out, "");
}

/// Verifies `===` returns true when both operands are `null`.
#[test]
fn test_strict_eq_null() {
    let out = compile_and_run("<?php echo null === null;");
    assert_eq!(out, "1");
}

/// Verifies `===` returns false when comparing `null` to integer `0` (different types).
#[test]
fn test_strict_eq_null_vs_int() {
    let out = compile_and_run("<?php echo null === 0;");
    assert_eq!(out, "");
}

/// Verifies `===` returns false when comparing `null` to bool `false` (different types).
#[test]
fn test_strict_eq_null_vs_false() {
    let out = compile_and_run("<?php echo null === false;");
    assert_eq!(out, "");
}

/// Verifies `===` returns true for two identical float values.
#[test]
fn test_strict_eq_float_same() {
    let out = compile_and_run("<?php echo 3.14 === 3.14;");
    assert_eq!(out, "1");
}

/// Verifies `===` returns empty (false) when comparing two different float values.
#[test]
fn test_strict_eq_float_different() {
    let out = compile_and_run("<?php echo 3.14 === 2.71;");
    assert_eq!(out, "");
}

/// Verifies `===` returns false when comparing float `1.0` to int `1` (different types).
#[test]
fn test_strict_eq_float_vs_int() {
    let out = compile_and_run("<?php echo 1.0 === 1;");
    assert_eq!(out, "");
}

/// Verifies `===` works correctly inside an `if` condition with an integer variable.
#[test]
fn test_strict_eq_in_if() {
    let out = compile_and_run(
        r#"<?php
$x = 5;
if ($x === 5) {
    echo "yes";
} else {
    echo "no";
}
"#,
    );
    assert_eq!(out, "yes");
}

/// Verifies `!==` works correctly inside an `if` condition with string variables.
#[test]
fn test_strict_neq_in_if() {
    let out = compile_and_run(
        r#"<?php
$x = "hello";
if ($x !== "world") {
    echo "different";
} else {
    echo "same";
}
"#,
    );
    assert_eq!(out, "different");
}

/// Verifies `===` returns true when two distinct variables hold the same string value.
#[test]
fn test_strict_eq_string_variables() {
    let out = compile_and_run(
        r#"<?php
$a = "test";
$b = "test";
echo $a === $b;
"#,
    );
    assert_eq!(out, "1");
}

/// Verifies `!==` returns true when two distinct variables hold different string values.
#[test]
fn test_strict_neq_string_variables() {
    let out = compile_and_run(
        r#"<?php
$a = "foo";
$b = "bar";
echo $a !== $b;
"#,
    );
    assert_eq!(out, "1");
}

/// Verifies both operands of `===` are evaluated even when types differ (no short-circuit on type mismatch).
#[test]
fn test_strict_eq_side_effects_preserved() {
    let out = compile_and_run(
        r#"<?php
function effect() { echo "X"; return 1; }
$r = 1.0 === effect();
echo $r;
"#,
    );
    assert_eq!(out, "X");
}

/// Verifies the boolean result of `===` can be assigned to a variable.
#[test]
fn test_strict_eq_assign_result() {
    let out = compile_and_run(
        r#"<?php
$x = 1 === 1;
echo $x;
"#,
    );
    assert_eq!(out, "1");
}

/// Verifies the boolean result of `!==` can be assigned to a variable.
#[test]
fn test_strict_neq_assign_result() {
    let out = compile_and_run(
        r#"<?php
$x = 1 !== 2;
echo $x;
"#,
    );
    assert_eq!(out, "1");
}

/// Verifies strict comparison uses both type and value from a map with int, string, and bool entries.
#[test]
fn test_strict_compare_mixed_uses_payload_type_and_value() {
    let out = compile_and_run(
        r#"<?php
$map = [
    "int_a" => 42,
    "int_b" => 42,
    "int_c" => 7,
    "str_a" => "42",
    "str_b" => "42",
    "bool_t" => true,
];
echo $map["int_a"] === $map["int_b"] ? "1" : "0";
echo $map["int_a"] === $map["int_c"] ? "1" : "0";
echo $map["int_a"] === $map["str_a"] ? "1" : "0";
echo $map["str_a"] === $map["str_b"] ? "1" : "0";
echo $map["int_a"] !== $map["str_a"] ? "1" : "0";
echo $map["bool_t"] === true ? "1" : "0";
"#,
    );
    assert_eq!(out, "100111");
}

// --- Array identity against the empty-array literal `[]` ---

/// Regression (#642): `$array === []` compares by emptiness, not pointer identity.
///
/// A bare `array` value is typed `Array(Mixed)` and `[]` is `Array(Never)`; the
/// generic strict-eq path saw the differing static types and emitted an unconditional
/// `false`, silently miscompiling the idiomatic `$errors === []` emptiness check
/// (a `--check`-invisible #635-class bug). An array equals `[]` exactly when it holds
/// zero entries, so this is a length==0 test.
#[test]
fn test_array_strict_eq_empty_literal_is_emptiness() {
    let out = compile_and_run(
        r#"<?php
declare(strict_types=1);
function classify(array $a): string {
    $seen = [];
    foreach ($a as $k => $v) { $seen[$k] = $v; }
    return ($seen === []) ? 'empty' : 'nonempty';
}
echo classify([]), ';', classify(['x' => 1]), ';';
echo ([] === []) ? 'Y' : 'N', ';';
echo ([] === ['a']) ? 'Y' : 'N';
"#,
    );
    assert_eq!(out, "empty;nonempty;Y;N");
}

/// Regression (#642): the reversed operand order and the `!==` form.
#[test]
fn test_array_strict_eq_empty_literal_reversed_and_negated() {
    let out = compile_and_run(
        r#"<?php
declare(strict_types=1);
/** @param list<int> $a */
function has_none(array $a): string { return ([] === $a) ? 'E' : 'N'; }
/** @param list<int> $a */
function has_any(array $a): string { return ($a !== []) ? 'has' : 'none'; }
echo has_none([]), ';', has_none([1, 2]), ';', has_any([]), ';', has_any([9]);
"#,
    );
    assert_eq!(out, "E;N;none;has");
}

// --- Include / Require ---
