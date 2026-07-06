//! Purpose:
//! Groups the strings integration test submodules into the parent suite.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - Submodules group focused fixtures for search, transform, encoding, formatting, interpolation and hashes, and related suites.

use crate::support::*;

#[path = "strings/search.rs"]
mod search;
#[path = "strings/transform.rs"]
mod transform;
#[path = "strings/encoding.rs"]
mod encoding;
#[path = "strings/formatting.rs"]
mod formatting;
#[path = "strings/interpolation_and_hashes.rs"]
mod interpolation_and_hashes;
#[path = "strings/misc.rs"]
mod misc;

/// EC-3 (#486): string spaceship follows PHP 8's rule — NUMERIC strings order numerically
/// ("10" <=> "9" is 1, not lexicographic -1), everything else byte-for-byte — and drives the
/// usort-comparator staple. Byte-parity vs PHP 8.5.
#[test]
fn test_string_spaceship_php8_semantics() {
    let out = compile_and_run(
        "<?php declare(strict_types=1); final class Item { public function __construct(public string $name) {} } function sortedNames(array $items): string { usort($items, static fn (Item $l, Item $r): int => $l->name <=> $r->name); $out = []; foreach ($items as $i) { $out[] = $i->name; } return implode(',', $out); } function main(): void { echo ('ab' <=> 'ac'), ':', ('b' <=> 'a'), ':', ('x' <=> 'x'), ':', ('10' <=> '9'), ':', ('2' <=> '10'), ':', sortedNames([new Item('zeta'), new Item('alpha'), new Item('mid')]); } main();",
    );
    assert_eq!(out, "-1:1:0:1:-1:alpha,mid,zeta");
}

/// EC-16 (#499): array spaceship follows PHP's rule — count first (fewer elements sorts
/// smaller regardless of content), then pairwise elements (strings via the PHP 8 string
/// spaceship, so numeric strings order numerically). Drives the multi-key usort tuple
/// comparator `[$l->a, $l->b] <=> [$r->a, $r->b]`. Byte-parity vs PHP 8.5.
#[test]
fn test_array_spaceship_count_then_pairwise() {
    let out = compile_and_run(
        "<?php declare(strict_types=1); final class R { public function __construct(public string $m, public string $p) {} } function main(): void { echo (['GET','/a'] <=> ['GET','/b']), ':', (['GET','/b'] <=> ['GET','/a']), ':', (['GET','/a'] <=> ['GET','/a']), ':', (['a'] <=> ['a','b']), ':', (['a','b'] <=> ['a']), ':', ([2,1] <=> [2,3]), ':', ([10,5] <=> [2,300]), ':', (['10','x'] <=> ['9','x']), ':'; $rs = [new R('POST','/b'), new R('GET','/z'), new R('GET','/a'), new R('POST','/a')]; usort($rs, static fn (R $l, R $r): int => [$l->m, $l->p] <=> [$r->m, $r->p]); foreach ($rs as $r) { echo $r->m, ' ', $r->p, ';'; } } main();",
    );
    assert_eq!(out, "-1:1:0:-1:1:-1:1:1:GET /a;GET /z;POST /a;POST /b;");
}

/// urlencode/rawurlencode pass decimal digits through unencoded: the classification checked
/// letters FIRST, and its below-'A' shortcut routed digits (which sit below 'A') straight to
/// the punctuation set, percent-encoding them ("2" became "%32"). Byte-parity vs PHP 8.5.
#[test]
fn test_urlencode_passes_digits_through()  {
    let out = compile_and_run(
        "<?php echo urlencode('a2b9z'), '|', urlencode('name asc'), '|', rawurlencode('a2 b9~'), '|', urlencode('x-1_2.3');",
    );
    assert_eq!(out, "a2b9z|name+asc|a2%20b9~|x-1_2.3");
}
