//! Purpose:
//! Integration or regression tests for end-to-end codegen coverage of control flow branches and loops, including if true, if false, and if else.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - Inline PHP fixtures are compiled to native binaries and assertions compare stdout or expected failures.

use super::*;

/// Verifies that if (true) executes the branch.
#[test]
fn test_if_true() {
    let out = compile_and_run("<?php if (1 == 1) { echo \"yes\"; }");
    assert_eq!(out, "yes");
}

/// Verifies that if (false) skips the branch, producing no output.
#[test]
fn test_if_false() {
    let out = compile_and_run("<?php if (1 == 2) { echo \"yes\"; }");
    assert_eq!(out, "");
}

/// Verifies if/else selects the else branch when the condition is false.
#[test]
fn test_if_else() {
    let out = compile_and_run("<?php if (1 == 2) { echo \"a\"; } else { echo \"b\"; }");
    assert_eq!(out, "b");
}

/// Verifies elseif chain takes the second branch when first condition is false.
#[test]
fn test_if_elseif_else() {
    let out = compile_and_run(
        "<?php $x = 2; if ($x == 1) { echo \"one\"; } elseif ($x == 2) { echo \"two\"; } else { echo \"other\"; }",
    );
    assert_eq!(out, "two");
}

/// Verifies elseif/else falls through to the else branch when all conditions are false.
#[test]
fn test_if_else_falls_through() {
    let out = compile_and_run(
        "<?php $x = 99; if ($x == 1) { echo \"a\"; } elseif ($x == 2) { echo \"b\"; } else { echo \"c\"; }",
    );
    assert_eq!(out, "c");
}

// --- while ---

/// Verifies a basic while loop iterates 5 times and prints 01234.
#[test]
fn test_while_loop() {
    let out = compile_and_run("<?php $i = 0; while ($i < 5) { echo $i; $i = $i + 1; }");
    assert_eq!(out, "01234");
}

/// Verifies while (false) body never executes.
#[test]
fn test_while_zero_iterations() {
    let out = compile_and_run("<?php while (0) { echo \"no\"; }");
    assert_eq!(out, "");
}

/// Verifies break exits the while loop when $i reaches 3, producing 012.
#[test]
fn test_while_break() {
    let out = compile_and_run(
        "<?php $i = 0; while ($i < 10) { if ($i == 3) { break; } echo $i; $i = $i + 1; }",
    );
    assert_eq!(out, "012");
}

/// Verifies continue skips to the next iteration, skipping the echo at $i==3.
#[test]
fn test_while_continue() {
    let out = compile_and_run(
        "<?php $i = 0; while ($i < 5) { $i = $i + 1; if ($i == 3) { continue; } echo $i; }",
    );
    assert_eq!(out, "1245");
}

/// Verifies break 2 exits both the inner and outer loop from within the innermost loop.
#[test]
fn test_multilevel_break_exits_nested_loops() {
    let out = compile_and_run(
        r#"<?php
for ($i = 0; $i < 3; $i++) {
    echo "i" . $i . ":";
    for ($j = 0; $j < 3; $j++) {
        if ($i == 1) { break 2; }
        echo $j;
    }
}
echo "end";
"#,
    );
    assert_eq!(out, "i0:012i1:end");
}

/// Verifies continue 2 jumps to the outer loop's update expression, skipping the inner loop body and outer echo.
#[test]
fn test_multilevel_continue_targets_outer_loop_update() {
    let out = compile_and_run(
        r#"<?php
for ($i = 0; $i < 3; $i++) {
    echo "i" . $i . ":";
    for ($j = 0; $j < 3; $j++) {
        if ($j == 1) { continue 2; }
        echo $j;
    }
    echo "x";
}
echo "end";
"#,
    );
    assert_eq!(out, "i0:0i1:0i2:0end");
}

/// Verifies continue 2 from within a switch targets the outer for loop, not the switch itself.
#[test]
fn test_multilevel_continue_from_switch_targets_outer_loop() {
    let out = compile_and_run(
        r#"<?php
for ($i = 0; $i < 3; $i++) {
    echo "a";
    switch ($i) {
        case 1:
            echo "b";
            continue 2;
        default:
            echo "c";
    }
    echo "d";
}
"#,
    );
    assert_eq!(out, "acdabacd");
}

/// Verifies break 2 through a try/finally runs the finally block exactly once before exiting.
#[test]
fn test_multilevel_break_through_finally_runs_finally_once() {
    let out = compile_and_run(
        r#"<?php
for ($i = 0; $i < 2; $i++) {
    for ($j = 0; $j < 2; $j++) {
        try {
            echo "t";
            break 2;
        } finally {
            echo "f";
        }
    }
    echo "x";
}
echo "e";
"#,
    );
    assert_eq!(out, "tfe");
}

// --- for ---

/// Verifies a basic for loop with manual increment in body iterates 5 times and prints 01234.
#[test]
fn test_for_loop() {
    let out = compile_and_run("<?php for ($i = 0; $i < 5; $i = $i + 1) { echo $i; }");
    assert_eq!(out, "01234");
}

/// Verifies break exits the for loop when $i reaches 3, producing 012.
#[test]
fn test_for_break() {
    let out = compile_and_run(
        "<?php for ($i = 0; $i < 10; $i = $i + 1) { if ($i == 3) { break; } echo $i; }",
    );
    assert_eq!(out, "012");
}

// --- FizzBuzz ---

/// Verifies nested if/elseif/else chain correctly maps 1–15 to Fizz/Buzz/FizzBuzz/decimal output.
#[test]
fn test_fizzbuzz() {
    let source = r#"<?php
$i = 1;
while ($i <= 15) {
    if ($i % 15 == 0) {
        echo "FizzBuzz\n";
    } elseif ($i % 3 == 0) {
        echo "Fizz\n";
    } elseif ($i % 5 == 0) {
        echo "Buzz\n";
    } else {
        echo $i;
        echo "\n";
    }
    $i = $i + 1;
}
"#;
    let out = compile_and_run(source);
    assert_eq!(
        out,
        "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz\n"
    );
}

// --- Increment/Decrement ---

/// Verifies for loop with ++$i post-increment in update expression prints 01234.
#[test]
fn test_for_with_increment() {
    let out = compile_and_run("<?php for ($i = 0; $i < 5; $i++) { echo $i; }");
    assert_eq!(out, "01234");
}

/// Verifies pre-increment (++$i) updates before the loop body echo, producing 123.
#[test]
fn test_while_with_pre_increment() {
    let out = compile_and_run("<?php $i = 0; while ($i < 3) { ++$i; echo $i; }");
    assert_eq!(out, "123");
}

// --- Functions ---

/// Verifies null is falsy in an if condition, selecting the else branch.
#[test]
fn test_if_null_is_falsy() {
    let out = compile_and_run(
        r#"<?php
$x = null;
if ($x) {
    echo "true";
} else {
    echo "false";
}
"#,
    );
    assert_eq!(out, "false");
}

/// Verifies null as a while condition prevents loop entry and prints ok.
#[test]
fn test_while_null_no_loop() {
    let out = compile_and_run("<?php $x = null; while ($x) { echo \"bad\"; } echo \"ok\";");
    assert_eq!(out, "ok");
}

// --- Ternary operator ---

/// `match (true)` guard arms narrow their result expression like an `if`
/// then-branch: `$r instanceof RouteRule => $r->pattern` sees $r as RouteRule
/// even though the declared type is a marker interface. Interface-typed
/// receivers that survive un-narrowed route through the boxed Mixed property
/// path instead of a class-slot lookup.
#[test]
fn test_match_true_instanceof_arm_narrowing() {
    let out = compile_and_run(
        r#"<?php
interface Rule {}
final class RouteRule implements Rule { public function __construct(public string $pattern) {} }
final class TypeRule implements Rule { public function __construct(public string $type) {} }
function describe(Rule $r): string {
    return match (true) {
        $r instanceof RouteRule => "route:" . $r->pattern,
        $r instanceof TypeRule => "type:" . $r->type,
        default => "other",
    };
}
echo describe(new RouteRule("/x")), "|", describe(new TypeRule("page"));
"#,
    );
    assert_eq!(out, "route:/x|type:page");
}

/// EC-39 (#532): generator bodies never require a tail return — calling one
/// produces the Generator itself and falling off the end exhausts iteration
/// (PgsqlResult::iterate / FileResult::yieldRows pattern).
#[test]
fn test_generator_method_requires_no_tail_return() {
    let out = compile_and_run(
        r#"<?php
final class R {
    /** @param list<int> $rows */
    public function __construct(private array $rows) {}
    public function iterate(): Generator {
        foreach ($this->rows as $row) {
            yield $row * 2;
        }
    }
}
$r = new R([1, 2, 3]);
foreach ($r->iterate() as $v) {
    echo $v, ";";
}
"#,
    );
    assert_eq!(out, "2;4;6;");
}

/// EC-39 (#532): return facts are collected under the FLOW env at each
/// statement, not the post-body env — check_stmt persists flow complements
/// past terminal branches (`if ($x !== null) { return $x; }` leaves $x: Void
/// afterwards), so post-body collection reported the guarded `return $x` as
/// Void and failed the declared-type compat check. Covers the ?int guard
/// (RowComparator::compareForOrderBy), the Mixed session read
/// (SynchronizerTokenStrategy::issue), and the foreach-guard-throw tail
/// (VersionResolver::findOwnVersion).
#[test]
fn test_guarded_returns_report_narrowed_types() {
    let out = compile_and_run(
        r#"<?php
function compareForPredicate(mixed $l, mixed $r): ?int {
    if (is_int($l) && is_int($r)) {
        return $l <=> $r;
    }
    return null;
}
function compareForOrderBy(mixed $l, mixed $r): int {
    $cmp = compareForPredicate($l, $r);
    if ($cmp !== null) {
        return $cmp;
    }
    return strlen((string) $l) <=> strlen((string) $r);
}
function pickFirst(array $values): string {
    foreach ($values as $value) {
        if (is_string($value)) {
            return $value;
        }
    }
    throw new RuntimeException("no string entry");
}
final class Sess {
    /** @var array<string, string> */
    private array $data = [];
    public function get(string $key): mixed {
        return $this->data[$key] ?? null;
    }
    public function set(string $key, string $v): void {
        $this->data[$key] = $v;
    }
}
final class Strat {
    public function issue(Sess $session, string $formId): string {
        $key = "tok." . $formId;
        $existing = $session->get($key);
        if ($existing !== null) {
            return $existing;
        }
        $token = "T" . $formId;
        $session->set($key, $token);
        return $token;
    }
}
echo compareForOrderBy(2, 1), "|", compareForOrderBy("ab", "c"), "|";
echo pickFirst([1, "hit", "later"]), "|";
$s = new Strat();
$sess = new Sess();
echo $s->issue($sess, "f"), "=", $s->issue($sess, "f");
"#,
    );
    assert_eq!(out, "1|1|hit|Tf=Tf");
}
