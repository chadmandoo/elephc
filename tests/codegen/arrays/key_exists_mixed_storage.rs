//! Purpose:
//! Regression tests (#640) for `array_key_exists()` on an array whose runtime storage
//! kind is only known at runtime — a bare `array` parameter/property is typed
//! `Array(Mixed)` (elephc reads no phpdoc generic), yet at runtime it may be a
//! string-keyed hash. The indexed lowering rejected string/Mixed keys with
//! "array_key_exists key PHP type Str", so any `array_key_exists($strKey, $map)` over
//! a bare-`array`-typed map failed to compile natively.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - Fix routes such calls through `__rt_array_key_exists_mixed`, which dispatches on
//!   the array header kind byte: hash storage delegates to `__rt_hash_get`'s found
//!   flag (present even for a null value), indexed storage is a bounds check on the
//!   normalized integer key (a numeric string is coerced; a genuine string is absent).
//! - Inline PHP fixtures are compiled to native binaries; assertions compare stdout.
//! - All fixtures passed `--check` before the fix and failed only a native compile.

use crate::support::*;

/// Regression (#640): a string key against a bare-`array` property that holds a
/// string-keyed hash at runtime resolves through the runtime kind dispatch.
#[test]
fn test_string_key_exists_on_hash_backed_array_property() {
    let out = compile_and_run(
        r#"<?php
declare(strict_types=1);
final class R {
    public array $forms = [];
    public function add(string $id): void { $this->forms[$id] = 1; }
    public function has(string $id): bool { return array_key_exists($id, $this->forms); }
}
$r = new R();
$r->add('alpha');
$r->add('beta');
echo $r->has('alpha') ? 'Y' : 'N', $r->has('beta') ? 'Y' : 'N', $r->has('zeta') ? 'Y' : 'N';
"#,
    );
    assert_eq!(out, "YYN");
}

/// Regression (#640): the exact FormRegistry shape — a foreach-built map guarded by
/// `array_key_exists` before a lookup.
#[test]
fn test_array_key_exists_guard_on_foreach_built_map() {
    let out = compile_and_run(
        r#"<?php
declare(strict_types=1);
final class Reg {
    private array $forms = [];
    public function register(string $id, int $v): void { $this->forms[$id] = $v; }
    public function get(string $id): int {
        if (!array_key_exists($id, $this->forms)) { return -1; }
        return $this->forms[$id];
    }
}
$r = new Reg();
$r->register('login', 7);
$r->register('signup', 9);
echo $r->get('login'), ';', $r->get('missing'), ';', $r->get('signup');
"#,
    );
    assert_eq!(out, "7;-1;9");
}

/// Regression (#640): a numeric string key against a genuine indexed list is coerced
/// to its integer key (PHP semantics), while an out-of-bounds numeric string is absent.
#[test]
fn test_numeric_string_key_on_indexed_list_coerces_to_int() {
    let out = compile_and_run(
        r#"<?php
declare(strict_types=1);
/** @param list<int> $a */
function probe(array $a): string {
    return (array_key_exists('1', $a) ? 'Y' : 'N') . (array_key_exists('5', $a) ? 'Y' : 'N');
}
echo probe([10, 20, 30]);
"#,
    );
    assert_eq!(out, "YN");
}

/// Regression (#640): a genuine (non-numeric) string key is never present in a pure
/// indexed list.
#[test]
fn test_non_numeric_string_key_absent_from_indexed_list() {
    let out = compile_and_run(
        r#"<?php
declare(strict_types=1);
/** @param list<int> $a */
function probe(array $a): string { return array_key_exists('x', $a) ? 'Y' : 'N'; }
echo probe([10, 20, 30]);
"#,
    );
    assert_eq!(out, "N");
}

/// Regression (#640): a Mixed key (a foreach key over a map, boxed at runtime) probes
/// presence across a rebuilt map correctly.
#[test]
fn test_mixed_key_exists_on_rebuilt_map() {
    let out = compile_and_run(
        r#"<?php
declare(strict_types=1);
function rebuild(): string {
    $src = ['p' => 1, 'q' => 2];
    $seen = [];
    foreach ($src as $k => $v) { $seen[$k] = $v; }
    $out = '';
    foreach (['p', 'z', 'q'] as $probe) { $out .= array_key_exists($probe, $seen) ? 'Y' : 'N'; }
    return $out;
}
echo rebuild();
"#,
    );
    assert_eq!(out, "YNY");
}
