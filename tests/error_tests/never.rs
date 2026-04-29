use super::*;

#[test]
fn test_error_never_function_cannot_return_value() {
    expect_error(
        "<?php function fail(): never { return 42; }",
        "Function 'fail' return type expects Never, got",
    );
}

#[test]
fn test_error_never_function_cannot_return_void() {
    expect_error(
        "<?php function fail(): never { return; }",
        "Function 'fail' return type expects Never, got",
    );
}

#[test]
fn test_error_never_rejected_as_parameter_type() {
    expect_error(
        "<?php function take(never $x) {} take(1);",
        "cannot use type never",
    );
}

#[test]
fn test_error_never_rejected_as_property_type() {
    expect_error(
        "<?php class Box { public never $value; }",
        "cannot use type never",
    );
}

#[test]
fn test_error_never_rejected_as_typed_local() {
    expect_error(
        "<?php never $x = 1;",
        "cannot use type never",
    );
}

#[test]
fn test_error_never_override_widening_to_void_rejected() {
    expect_error(
        "<?php class Parent_ { public function f(): never { throw new \\Exception(); } } class Child_ extends Parent_ { public function f(): void {} }",
        "incompatible return type",
    );
}

#[test]
fn test_error_never_interface_implementation_widening_rejected() {
    expect_error(
        "<?php interface Failer { public function fail(): never; } class Bad implements Failer { public function fail(): int { return 1; } }",
        "incompatible return type",
    );
}
