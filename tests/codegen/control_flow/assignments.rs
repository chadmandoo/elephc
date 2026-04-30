use super::*;

#[test]
fn test_pre_increment() {
    let out = compile_and_run("<?php $i = 1; $k = ++$i; echo $i . \" \" . $k;");
    assert_eq!(out, "2 2");
}

#[test]
fn test_post_increment() {
    let out = compile_and_run("<?php $i = 1; $k = $i++; echo $i . \" \" . $k;");
    assert_eq!(out, "2 1");
}

#[test]
fn test_pre_decrement() {
    let out = compile_and_run("<?php $i = 5; $k = --$i; echo $i . \" \" . $k;");
    assert_eq!(out, "4 4");
}

#[test]
fn test_post_decrement() {
    let out = compile_and_run("<?php $i = 5; $k = $i--; echo $i . \" \" . $k;");
    assert_eq!(out, "4 5");
}

#[test]
fn test_plus_assign() {
    let out = compile_and_run("<?php $x = 10; $x += 5; echo $x;");
    assert_eq!(out, "15");
}

#[test]
fn test_minus_assign() {
    let out = compile_and_run("<?php $x = 10; $x -= 3; echo $x;");
    assert_eq!(out, "7");
}

#[test]
fn test_star_assign() {
    let out = compile_and_run("<?php $x = 6; $x *= 7; echo $x;");
    assert_eq!(out, "42");
}

#[test]
fn test_slash_assign() {
    let out = compile_and_run("<?php $x = 84; $x /= 2; echo $x;");
    assert_eq!(out, "42");
}

#[test]
fn test_percent_assign() {
    let out = compile_and_run("<?php $x = 10; $x %= 3; echo $x;");
    assert_eq!(out, "1");
}

#[test]
fn test_dot_assign() {
    let out = compile_and_run("<?php $s = \"hello\"; $s .= \" world\"; echo $s;");
    assert_eq!(out, "hello world");
}

#[test]
fn test_pow_assign() {
    let out = compile_and_run("<?php $x = 2; $x **= 3; echo $x;");
    assert_eq!(out, "8");
}

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

#[test]
fn test_assignment_expression_returns_assigned_value() {
    let out = compile_and_run("<?php echo ($x = 5); echo ':'; echo $x;");
    assert_eq!(out, "5:5");
}

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

#[test]
fn test_compound_assignment_expression_returns_new_value() {
    let out = compile_and_run("<?php $x = 4; echo ($x += 3); echo ':'; echo $x;");
    assert_eq!(out, "7:7");
}

#[test]
fn test_assignment_expression_right_associative_codegen() {
    let out = compile_and_run("<?php $x = $y = 4; echo $x; echo ':'; echo $y;");
    assert_eq!(out, "4:4");
}

#[test]
fn test_assignment_expression_target_is_not_constant_propagated() {
    let out = compile_and_run("<?php $x = 1; echo ($x = 2); echo ':'; echo $x;");
    assert_eq!(out, "2:2");
}

#[test]
fn test_assignment_expression_clears_stale_constants_inside_same_expr() {
    let out = compile_and_run("<?php $x = 1; echo (($x = 2) + $x);");
    assert_eq!(out, "4");
}

#[test]
fn test_assignment_expression_in_array_assignment_operands() {
    let out = compile_and_run(
        r#"<?php
$items = [0];
$items[$i = 0] = ($x = 7);
echo $i . ":" . $x . ":" . $items[0];
"#,
    );
    assert_eq!(out, "0:7:7");
}

#[test]
fn test_assignment_expression_inside_closure_gets_closure_local_slot() {
    let out = compile_and_run(
        r#"<?php
$f = function() {
    echo ($x = 9);
};
$f();
"#,
    );
    assert_eq!(out, "9");
}

#[test]
fn test_ternary_in_assignment() {
    let out = compile_and_run("<?php $a = 10; $b = 20; $max = $a > $b ? $a : $b; echo $max;");
    assert_eq!(out, "20");
}
