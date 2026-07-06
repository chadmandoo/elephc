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
