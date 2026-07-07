//! Purpose:
//! Integration or regression tests for end-to-end codegen coverage of control flow, assignments compound and values, including pre increment, post increment, and pre decrement.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - Inline PHP fixtures are compiled to native binaries and assertions compare stdout or expected failures.

use super::*;

/// Verifies pre-increment (`++$i`) increments before value capture.
/// Fixture: simple local `$i = 1` then `$k = ++$i`, expects `$i` and `$k` both 2.
#[test]
fn test_pre_increment() {
    let out = compile_and_run("<?php $i = 1; $k = ++$i; echo $i . \" \" . $k;");
    assert_eq!(out, "2 2");
}

/// Verifies post-increment (`$i++`) captures value before increment.
/// Fixture: simple local `$i = 1` then `$k = $i++`, expects `$i` = 2 and `$k` = 1.
#[test]
fn test_post_increment() {
    let out = compile_and_run("<?php $i = 1; $k = $i++; echo $i . \" \" . $k;");
    assert_eq!(out, "2 1");
}

/// Verifies pre-decrement (`--$i`) decrements before value capture.
/// Fixture: simple local `$i = 5` then `$k = --$i`, expects `$i` and `$k` both 4.
#[test]
fn test_pre_decrement() {
    let out = compile_and_run("<?php $i = 5; $k = --$i; echo $i . \" \" . $k;");
    assert_eq!(out, "4 4");
}

/// Verifies post-decrement (`$i--`) captures value before decrement.
/// Fixture: simple local `$i = 5` then `$k = $i--`, expects `$i` = 4 and `$k` = 5.
#[test]
fn test_post_decrement() {
    let out = compile_and_run("<?php $i = 5; $k = $i--; echo $i . \" \" . $k;");
    assert_eq!(out, "4 5");
}

/// Verifies `+=` compound addition on integer locals.
/// Fixture: `$x = 10; $x += 5;` expects output "15".
#[test]
fn test_plus_assign() {
    let out = compile_and_run("<?php $x = 10; $x += 5; echo $x;");
    assert_eq!(out, "15");
}

/// Verifies `-=` compound subtraction on integer locals.
/// Fixture: `$x = 10; $x -= 3;` expects output "7".
#[test]
fn test_minus_assign() {
    let out = compile_and_run("<?php $x = 10; $x -= 3; echo $x;");
    assert_eq!(out, "7");
}

/// Verifies `*=` compound multiplication on integer locals.
/// Fixture: `$x = 6; $x *= 7;` expects output "42".
#[test]
fn test_star_assign() {
    let out = compile_and_run("<?php $x = 6; $x *= 7; echo $x;");
    assert_eq!(out, "42");
}

/// Verifies `/=` compound division on integer locals.
/// Fixture: `$x = 84; $x /= 2;` expects output "42".
#[test]
fn test_slash_assign() {
    let out = compile_and_run("<?php $x = 84; $x /= 2; echo $x;");
    assert_eq!(out, "42");
}

/// Verifies `%=` compound modulo on integer locals.
/// Fixture: `$x = 10; $x %= 3;` expects output "1".
#[test]
fn test_percent_assign() {
    let out = compile_and_run("<?php $x = 10; $x %= 3; echo $x;");
    assert_eq!(out, "1");
}

/// Verifies `.=` compound string concatenation.
/// Fixture: `$s = "hello"; $s .= " world";` expects output "hello world".
#[test]
fn test_dot_assign() {
    let out = compile_and_run("<?php $s = \"hello\"; $s .= \" world\"; echo $s;");
    assert_eq!(out, "hello world");
}

/// Verifies `**=` compound exponentiation on integer locals.
/// Fixture: `$x = 2; $x **= 3;` expects output "8".
#[test]
fn test_pow_assign() {
    let out = compile_and_run("<?php $x = 2; $x **= 3; echo $x;");
    assert_eq!(out, "8");
}

/// Verifies `&=`, `|=`, `^=`, `<<=`, `>>=` bitwise compound assignments on integer locals.
/// Fixture: sequential bitwise ops with comma-separated output "2,5,4,32,8".
#[test]
fn test_bitwise_compound_assignments() {
    let out = compile_and_run(
        r#"<?php
$x = 6;
$x &= 3;
echo $x . ",";
$x = 4;
$x |= 1;
echo $x . ",";
$x = 7;
$x ^= 3;
echo $x . ",";
$x = 1;
$x <<= 5;
echo $x . ",";
$x = 64;
$x >>= 3;
echo $x;
"#,
    );
    assert_eq!(out, "2,5,4,32,8");
}

/// Verifies simple assignment `$x = 5` is an expression returning the assigned value.
/// Fixture: `echo ($x = 5); echo ':'; echo $x;` expects "5:5".
#[test]
fn test_assignment_expression_returns_assigned_value() {
    let out = compile_and_run("<?php echo ($x = 5); echo ':'; echo $x;");
    assert_eq!(out, "5:5");
}

/// Verifies string assignment `$s = "hi"` is an expression returning the assigned value.
/// Fixture: `echo ($s = "hi"); echo ":"; echo $s;` expects "hi:hi".
#[test]
fn test_string_assignment_expression_returns_assigned_value() {
    let out = compile_and_run(r#"<?php echo ($s = "hi"); echo ":"; echo $s;"#);
    assert_eq!(out, "hi:hi");
}

/// Verifies assignment with `and` respects PHP operator precedence (lowest precedence).
/// Fixture: `$x = true and false;` parses as `$x = (true and false)`, so `$x` is `false`; echo "T"/"F" expects "T".
#[test]
fn test_assignment_expression_word_and_uses_php_precedence() {
    let out = compile_and_run(
        r#"<?php
$x = true and false;
echo $x ? "T" : "F";
"#,
    );
    assert_eq!(out, "T");
}

/// Verifies assignment expression inside `if` condition updates the local variable.
/// Fixture: `if ($x = 3) { echo $x; }` expects output "3".
#[test]
fn test_assignment_expression_in_condition_updates_local() {
    let out = compile_and_run(
        r#"<?php
if ($x = 3) {
    echo $x;
}
"#,
    );
    assert_eq!(out, "3");
}

/// Verifies compound assignment `+=` returns the new value (not the original).
/// Fixture: `$x = 4; echo ($x += 3); echo ':'; echo $x;` expects "7:7".
#[test]
fn test_compound_assignment_expression_returns_new_value() {
    let out = compile_and_run("<?php $x = 4; echo ($x += 3); echo ':'; echo $x;");
    assert_eq!(out, "7:7");
}

/// Verifies `??=` returns the existing non-null value when the variable already has one.
/// Fixture: `maybe(true)` returns 7 (non-null), assigned to `$x`; `($x ??= 5)` returns 7 without assigning.
#[test]
fn test_null_coalesce_assignment_expression_returns_existing_mixed_value() {
    let out = compile_and_run(
        r#"<?php
function maybe(bool $flag): mixed {
    return $flag ? 7 : null;
}
$x = maybe(true);
echo ($x ??= 5);
echo ":";
echo $x;
"#,
    );
    assert_eq!(out, "7:7");
}

/// Verifies `??=` returns and assigns the default when the variable is null.
/// Fixture: `maybe(false)` returns null, `$x` is set to that null; `($x ??= 5)` assigns 5 and returns 5.
#[test]
fn test_null_coalesce_assignment_expression_returns_default_for_mixed_null() {
    let out = compile_and_run(
        r#"<?php
function maybe(bool $flag): mixed {
    return $flag ? 7 : null;
}
$x = maybe(false);
echo ($x ??= 5);
echo ":";
echo $x;
"#,
    );
    assert_eq!(out, "5:5");
}

/// Verifies array-element assignment is an expression returning the assigned value.
/// Fixture: `$items = [1, 2]; echo ($items[1] = 9);` expects both echo and `$items[1]` to be 9.
#[test]
fn test_array_assignment_expression_returns_assigned_value() {
    let out = compile_and_run("<?php $items = [1, 2]; echo ($items[1] = 9); echo ':' . $items[1];");
    assert_eq!(out, "9:9");
}

/// Verifies array assignment evaluates the RHS container snapshot before writing to the destination.
/// Fixture: `$items = []; $result = ($items[0] = $items);` — both result and `$items[0]` receive the empty array (snapshot taken before write).
#[test]
fn test_array_assignment_expression_snapshots_rhs_container_before_write() {
    let out = compile_and_run(
        r#"<?php
$items = [];
$result = ($items[0] = $items);
echo count($result) . ":" . count($items[0]);
"#,
    );
    assert_eq!(out, "0:0");
}

/// Verifies array-element assignment with a variable index returns the assigned value.
/// Fixture: `$items = [1, 2]; $i = 1; echo ($items[$i] = 9);` expects echo and `$items[1]` both 9.
#[test]
fn test_array_assignment_expression_variable_index_returns_assigned_value() {
    let out = compile_and_run("<?php $items = [1, 2]; $i = 1; echo ($items[$i] = 9); echo ':' . $items[1];");
    assert_eq!(out, "9:9");
}

/// Verifies compound assignment on an array element returns the new value.
/// Fixture: `$items = [3]; echo ($items[0] += 4); echo ':' . $items[0];` expects "7:7".
#[test]
fn test_array_compound_assignment_expression_returns_new_value() {
    let out = compile_and_run("<?php $items = [3]; echo ($items[0] += 4); echo ':' . $items[0];");
    assert_eq!(out, "7:7");
}

/// Verifies associative array element assignment with compound `+=` returns the new value.
/// Fixture: `$items = ["count" => 2]; echo ($items["count"] += 5);` expects echo "7" and `$items["count"]` = 7.
#[test]
fn test_assoc_array_assignment_expression_returns_assigned_value() {
    let out = compile_and_run(
        r#"<?php
$items = ["count" => 2];
echo ($items["count"] += 5);
echo ":" . $items["count"];
"#,
    );
    assert_eq!(out, "7:7");
}

/// Verifies `??=` on a populated array element returns the existing value without reassigning.
/// Fixture: `$items = [5, 8];` — `($items[0] ??= 5)` returns 5, `($items[1] ??= 6)` returns 8, final state is unchanged.
#[test]
fn test_array_null_coalesce_assignment_expression_returns_slot_value() {
    let out = compile_and_run(
        r#"<?php
$items = [5, 8];
echo ($items[0] ??= 5);
echo ":";
echo ($items[1] ??= 6);
echo ":" . $items[0] . ":" . $items[1];
"#,
    );
    assert_eq!(out, "5:8:5:8");
}

/// Verifies `??=` on an empty array element evaluates the RHS snapshot before write when the slot is absent.
/// Fixture: `$items = []; $result = ($items[0] ??= $items);` — both receive the empty array snapshot.
#[test]
fn test_array_null_coalesce_assignment_expression_snapshots_rhs_container_before_write() {
    let out = compile_and_run(
        r#"<?php
$items = [];
$result = ($items[0] ??= $items);
echo count($result) . ":" . count($items[0]);
"#,
    );
    assert_eq!(out, "0:0");
}

/// Verifies object property assignment with compound `+=` returns the new value.
/// Fixture: `Box` with `$value = 1`; `$box->value += 4` returns and stores 5.
#[test]
fn test_property_assignment_expression_returns_assigned_value() {
    let out = compile_and_run(
        r#"<?php
class Box {
    public $value = 1;
}
$box = new Box();
echo ($box->value += 4);
echo ":" . $box->value;
"#,
    );
    assert_eq!(out, "5:5");
}

/// Verifies compound assignment on an object property array element returns the new value.
/// Fixture: `Box` with `$items = [2, 4]`; `$box->items[1] *= 3` returns and stores 12.
#[test]
fn test_property_array_assignment_expression_returns_assigned_value() {
    let out = compile_and_run(
        r#"<?php
class Box {
    public $items = [2, 4];
}
$box = new Box();
echo ($box->items[1] *= 3);
echo ":" . $box->items[1];
"#,
    );
    assert_eq!(out, "12:12");
}

/// Verifies object property array element assignment snapshots the RHS container before writing.
/// Fixture: `$box->items = []; $result = ($box->items[0] = $box->items);` — both get the empty array snapshot.
#[test]
fn test_property_array_assignment_expression_snapshots_rhs_container_before_write() {
    let out = compile_and_run(
        r#"<?php
class Box {
    public $items = [];
}
$box = new Box();
$result = ($box->items[0] = $box->items);
echo count($result) . ":" . count($box->items[0]);
"#,
    );
    assert_eq!(out, "0:0");
}

/// Verifies static property assignment with compound `+=` returns the new value.
/// Fixture: `Registry::$count = 10;` then `Registry::$count += 5` returns and stores 15.
#[test]
fn test_static_property_assignment_expression_returns_assigned_value() {
    let out = compile_and_run(
        r#"<?php
class Registry {
    public static $count = 10;
}
echo (Registry::$count += 5);
echo ":" . Registry::$count;
"#,
    );
    assert_eq!(out, "15:15");
}

/// Verifies compound assignment on a static property array element returns the new value.
/// Fixture: `Registry::$items = [3, 5];` then `Registry::$items[0] += 3` returns and stores 6.
#[test]
fn test_static_property_array_assignment_expression_returns_assigned_value() {
    let out = compile_and_run(
        r#"<?php
class Registry {
    public static $items = [3, 5];
}
echo (Registry::$items[0] += 3);
echo ":" . Registry::$items[0];
"#,
    );
    assert_eq!(out, "6:6");
}

/// Verifies static property array element assignment snapshots the RHS container before writing.
/// Fixture: `Registry::$items = []; $result = (Registry::$items[0] = Registry::$items);` — both get empty array snapshot.
#[test]
fn test_static_property_array_assignment_expression_snapshots_rhs_container_before_write() {
    let out = compile_and_run(
        r#"<?php
class Registry {
    public static $items = [];
}
$result = (Registry::$items[0] = Registry::$items);
echo count($result) . ":" . count(Registry::$items[0]);
"#,
    );
    assert_eq!(out, "0:0");
}

/// Verifies `??=` on a null static property returns and assigns the default value.
/// Fixture: `Registry::$value = null;` then `Registry::$value ??= 6` assigns and returns 6.
#[test]
fn test_static_property_null_coalesce_assignment_expression_returns_value() {
    let out = compile_and_run(
        r#"<?php
class Registry {
    public static ?int $value = null;
}
echo (Registry::$value ??= 6);
echo ":" . Registry::$value;
"#,
    );
    assert_eq!(out, "6:6");
}

/// Verifies three-level chained local assignment `$a = $b = $c = 5` assigns to all three variables.
/// Fixture: `$a = $b = $c = 5; echo $a + $b + $c;` expects "15".
#[test]
fn test_chained_three_level_local_assignment() {
    let out = compile_and_run("<?php $a = $b = $c = 5; echo $a + $b + $c;");
    assert_eq!(out, "15");
}

/// Verifies three-level chained local assignment with strings.
/// Fixture: `$a = $b = "hi"; echo $a . $b;` expects "hihi".
#[test]
fn test_chained_string_local_assignment() {
    let out = compile_and_run(r#"<?php $a = $b = "hi"; echo $a . $b;"#);
    assert_eq!(out, "hihi");
}

/// Verifies chained assignment to a static property and a local variable in a single expression.
/// Regression test: static property assignment must not consume the right-hand-side local incorrectly.
/// Fixture: `self::$x = $local = 42` inside a static method, result is `self::$x + $local` = 42 + 42 = 84.
#[test]
fn test_chained_static_property_and_local_assignment() {
    let out = compile_and_run(
        r#"<?php
class C {
    public static int $x = 0;
    public static function init(): int {
        self::$x = $local = 42;
        return self::$x + $local;
    }
}
echo C::init();
"#,
    );
    assert_eq!(out, "84");
}

/// Regression for #370: `+=` on an undefined variable treats it as null/0
/// and produces the correct value, matching PHP's behavior.
#[test]
fn test_undefined_var_plus_equals() {
    let out = compile_and_run(r#"<?php $x += 1; echo $x;"#);
    assert_eq!(out, "1");
}

/// Regression for #370: `+=` on an undefined variable also initializes to
/// null/0 when the compound assignment is used as an expression.
#[test]
fn test_undefined_var_plus_equals_expression() {
    let out = compile_and_run(r#"<?php echo ($x += 1); echo ":"; echo $x;"#);
    assert_eq!(out, "1:1");
}

/// Regression for #370: `-=` on an undefined variable treats it as null/0.
#[test]
fn test_undefined_var_minus_equals() {
    let out = compile_and_run(r#"<?php $x -= 1; echo $x;"#);
    assert_eq!(out, "-1");
}

/// Regression for #370: `.=` on an undefined variable treats it as null/"".
#[test]
fn test_undefined_var_dot_equals() {
    let out = compile_and_run(r#"<?php $y .= "world"; echo $y;"#);
    assert_eq!(out, "world");
}

/// Regression for #370: `??=` on an undefined variable does NOT emit a warning
/// and assigns the default value, matching PHP's null-coalesce semantics.
#[test]
fn test_undefined_var_null_coalesce_equals() {
    let out = compile_and_run(r#"<?php $z ??= 42; echo $z;"#);
    assert_eq!(out, "42");
}

/// Regression for #370: `??=` on an undefined variable assigns and returns the
/// default value when used as an expression.
#[test]
fn test_undefined_var_null_coalesce_equals_expression() {
    let out = compile_and_run(r#"<?php echo ($z ??= 42); echo ":"; echo $z;"#);
    assert_eq!(out, "42:42");
}

/// Regression for #370: `*=` on an undefined variable treats it as 0.
#[test]
fn test_undefined_var_star_equals() {
    let out = compile_and_run(r#"<?php $w *= 5; echo $w;"#);
    assert_eq!(out, "0");
}

/// `??=` on a missing string key of a hash-typed local assigns the default,
/// returns it as the expression result, and keeps the stored value on a second
/// application (the probe must be missing-tolerant: a plain hash read's miss
/// result is element-typed and `is_null` never took the assign branch).
#[test]
fn test_hash_key_null_coalesce_assign() {
    let out = compile_and_run(
        r#"<?php
$a = [];
echo $a["k"] ??= "first";
echo "|", $a["k"] ??= "second";
echo "|", $a["k"];
"#,
    );
    assert_eq!(out, "first|first|first");
}

/// Plain `??` on a hash-typed receiver with a missing key takes the default —
/// the same missing-tolerant probe as the `??=` form (regression: the hash arm
/// ignored the silent-read request and emitted a plain warning read).
#[test]
fn test_hash_key_null_coalesce_read() {
    let out = compile_and_run(
        r#"<?php
$h = ["a" => "x"];
echo $h["missing"] ?? "dflt";
echo "|", $h["a"] ?? "wrong";
"#,
    );
    assert_eq!(out, "dflt|x");
}

/// `??=` autovivifies through two levels of a local array, an int-keyed local,
/// an object array property, and a single-level static property — each assigns
/// once, keeps the stored value on re-application, and reads back correctly.
#[test]
fn test_null_coalesce_assign_nested_and_receivers() {
    let out = compile_and_run(
        r#"<?php
final class Bag { /** @var array<string, int> */ public array $m = []; }
final class S {
    /** @var array<string, string> */
    public static array $m = [];
}
$loc = [];
$loc["k"]["n"] ??= "deep";
$loc["k"]["n"] ??= "other";
echo $loc["k"]["n"];
$ints = [];
echo "|", $ints[7] ??= "seven";
echo "|", $ints[7] ??= "nope";
$b = new Bag();
echo "|", $b->m["c"] ??= 42;
echo "|", $b->m["c"] ??= 99;
echo "|", S::$m["k"] ??= "sv";
echo "|", S::$m["k"] ??= "no";
echo "|", S::$m["k"];
"#,
    );
    assert_eq!(out, "deep|seven|seven|42|42|sv|sv|sv");
}

/// Expression-form `??=` on a nested static-property target inside a method:
/// the merge must TRANSFER the hidden temp's ownership (load + clear) rather
/// than load-and-release — the release took the slot's own reference and the
/// epilogue cleanup then double-released, freeing the boxed value the written
/// container entry still pointed at. The corruption only surfaced after the
/// function returned (main's epilogue runs after top-level reads), so the
/// read-back happens from OUTSIDE the writing method.
#[test]
fn test_nested_static_prop_null_coalesce_assign_expression_in_method() {
    let out = compile_and_run(
        r#"<?php
final class Reg {
    /** @var array<string, array<string, string>> */
    public static array $i = [];
    public static function put(string $k, string $n, string $v): string {
        return self::$i[$k][$n] ??= $v;
    }
}
echo Reg::put("a", "b", "one");
echo "|", Reg::put("a", "b", "two");
echo "|", Reg::$i["a"]["b"] ?? "MISS";
echo "|", Reg::$i["x"]["y"] ?? "MISS";
$outer = Reg::$i["a"] ?? [];
foreach ($outer as $kk => $vv) { echo "|", $kk, "=", $vv; }
"#,
    );
    assert_eq!(out, "one|one|one|MISS|b=one");
}
