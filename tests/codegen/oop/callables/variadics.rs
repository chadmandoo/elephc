use super::*;

#[test]
fn test_first_class_callable_variadic_function_call() {
    let out = compile_and_run(
        r#"<?php
function count_args(...$xs) {
    echo count($xs);
}

$f = count_args(...);
$f(1, 2, 3);
"#,
    );
    assert_eq!(out, "3");
}

#[test]
fn test_closure_variadic_call() {
    let out = compile_and_run(
        r#"<?php
$f = function (...$xs) {
    echo count($xs);
};

$f(1, 2, 3);
"#,
    );
    assert_eq!(out, "3");
}

#[test]
fn test_first_class_callable_variadic_with_regular_param() {
    let out = compile_and_run(
        r#"<?php
function head_and_count($a, ...$rest) {
    echo $a;
    echo ":";
    echo count($rest);
}

$f = head_and_count(...);
$f(7, 8, 9);
"#,
    );
    assert_eq!(out, "7:2");
}

#[test]
fn test_first_class_callable_builtin_count_accepts_string_arrays() {
    let out = compile_and_run(
        r#"<?php
$f = count(...);
$xs = ["a", "b"];
echo $f($xs);
"#,
    );
    assert_eq!(out, "2");
}

#[test]
fn test_first_class_callable_builtin_count_accepts_assoc_arrays() {
    let out = compile_and_run(
        r#"<?php
$f = count(...);
$xs = ["a" => 1, "b" => 2];
echo $f($xs);
"#,
    );
    assert_eq!(out, "2");
}
