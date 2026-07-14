//! Purpose:
//! Regression tests for empty-array (`[]`, typed `Array(Never)`) interactions with
//! call-site parameter specialization. A bare `array` parameter recovers its element
//! type from call arguments (elephc reads no phpdoc generics), and an empty-array
//! literal is the bottom array — it must neither pin the parameter below every concrete
//! array nor be rejected by an already-specialized array parameter.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - Inline PHP fixtures are compiled to native binaries; assertions compare stdout.
//! - Both fixtures passed `--check` before the fix and failed only a native compile.

use crate::support::*;

/// Regression (#645): an empty-array argument seen BEFORE a concrete one must not pin a
/// bare `array` parameter to `Array(Never)`.
///
/// `specialize_generic_array_hint` adopted the argument's type for a generic `array`
/// hint, so the first call `c([])` narrowed the parameter to `Array(Never)` — below every
/// concrete array. The second call `c([1, 2, 3])` then failed with
/// "parameter $x expects Array(Never), got Array(Int)". The reverse order already worked
/// (`Array(Never)` is assignable to `Array(Int)`), which is what made this order-dependent.
///
/// Fix: an empty-array placeholder carries no element-type information, so the generic
/// hint keeps its declared `Array(Mixed)`; a later non-empty call still specializes it.
///
/// NOTE: `--check` passed this all along — only a native compile ever saw it.
#[test]
fn test_empty_array_arg_before_concrete_does_not_pin_parameter() {
    let out = compile_and_run(
        r#"<?php
declare(strict_types=1);
function c(array $x): int { return count($x); }
echo c([]), ';', c([1, 2, 3]);
"#,
    );
    assert_eq!(out, "0;3");
}

/// Regression (#645): the same parameter, with a body that actually CONSUMES its elements,
/// must recompile for the concrete element type rather than staying pinned to the empty
/// placeholder — an empty-first call followed by an int array iterates and sums correctly.
#[test]
fn test_empty_array_first_then_iterated_concrete_array() {
    let out = compile_and_run(
        r#"<?php
declare(strict_types=1);
function sum(array $x): int { $t = 0; foreach ($x as $v) { $t += $v; } return $t; }
echo sum([]), ';', sum([10, 20, 30]);
"#,
    );
    assert_eq!(out, "0;60");
}

/// Regression (empty-array map compat): an empty-array argument seen AFTER a concrete
/// map must be accepted by the now map-typed (`AssocArray{Str, Str}`) parameter.
///
/// A parameter first called with `["class" => "btn"]` specializes to
/// `AssocArray{Str, Str}`. The `types_compatible` map arm only accepted an array
/// argument when both key and value were `Mixed`, so a later `[]` call failed with
/// "expects AssocArray{Str, Str}, got Array(Never)". An empty array is a valid empty map:
/// it has no entries that could violate the declared key/value types. This is the exact
/// `ComponentFactory::input([])` shape (`array $attrs = []`).
///
/// NOTE: `--check` passed this all along — only a native compile ever saw it.
#[test]
fn test_empty_array_arg_accepted_by_specialized_map_parameter() {
    let out = compile_and_run(
        r#"<?php
declare(strict_types=1);
function attrs(array $a): int { return count($a); }
echo attrs(['class' => 'btn', 'id' => 'x']), ';', attrs([]), ';', attrs(['k' => 'v']);
"#,
    );
    assert_eq!(out, "2;0;1");
}
