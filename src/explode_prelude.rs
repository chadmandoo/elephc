//! Purpose:
//! Injects the `__elephc_explode_limit` function that backs the three-argument
//! `explode($separator, $string, $limit)` form. The name resolver rewrites an `explode(...)`
//! call with exactly three arguments to `__elephc_explode_limit(...)`; the two-argument form
//! stays the native `explode` builtin.
//!
//! Called from:
//! - `crate::pipeline::compile` (and the codegen test harness) via `inject_if_used`, before
//!   name resolution so the rewritten call resolves to the injected function.
//!
//! Key details:
//! - A dedicated one-function prelude (not an entry in `stdlib_prelude`): the shared stdlib
//!   bundle injects every bundled function together, and its `get_debug_type` polyfill's
//!   `get_class($value)` on a `Mixed` value is not yet lowerable, so folding this in would
//!   make every 3-arg-`explode` program fail to compile natively (hollow). Injecting only
//!   this self-contained function keeps the fix non-hollow.
//! - Pay-for-use via a cheap Debug-render substring scan for `explode`. Over-injection (a
//!   2-arg-only or string-literal `explode`) is harmless: the function is self-contained and
//!   link-GC strips it when the rewrite never fires.
//! - Implements PHP's `$limit` semantics on top of the native 2-arg `explode`: `limit > 0`
//!   caps the result at `limit` elements with the final element holding the un-split
//!   remainder; `limit === 0` behaves as `1`; `limit < 0` drops the last `|limit|` elements
//!   (empty array if that removes everything). It re-joins with the separator via manual
//!   loops rather than `array_slice`/`implode` to stay within natively-lowerable primitives.

use crate::parser::ast::{Program, StmtKind};

/// The elephc-PHP source for the injected three-argument `explode` backing function.
const EXPLODE_PRELUDE_SRC: &str = r#"<?php
function __elephc_explode_limit(string $separator, string $string, int $limit): array {
    $all = explode($separator, $string);
    $n = count($all);
    $lim = $limit === 0 ? 1 : $limit;
    if ($lim > 0) {
        if ($lim >= $n) {
            return $all;
        }
        $result = [];
        $j = 0;
        while ($j < $lim - 1) {
            $result[] = $all[$j];
            $j = $j + 1;
        }
        $rest = $all[$lim - 1];
        $j = $lim;
        while ($j < $n) {
            $rest = $rest . $separator . $all[$j];
            $j = $j + 1;
        }
        $result[] = $rest;
        return $result;
    }
    $keep = $n + $lim;
    if ($keep <= 0) {
        return [];
    }
    $result = [];
    $j = 0;
    while ($j < $keep) {
        $result[] = $all[$j];
        $j = $j + 1;
    }
    return $result;
}
"#;

/// Prepends `__elephc_explode_limit` when the program mentions `explode`, so the name
/// resolver's three-argument rewrite has a definition to bind to; otherwise returns the
/// program unchanged. A user's own top-level `__elephc_explode_limit` declaration is left in
/// place. The prelude is a hoisted function declaration only, so prepending does not change
/// top-level execution order. Tokenize/parse failure is a compiler bug and panics.
pub fn inject_if_used(program: Program) -> Program {
    if !format!("{program:?}").contains("explode") {
        return program;
    }
    let user_declares = program.iter().any(|stmt| {
        matches!(
            &stmt.kind,
            StmtKind::FunctionDecl { name, .. } if name.eq_ignore_ascii_case("__elephc_explode_limit")
        )
    });
    if user_declares {
        return program;
    }
    let tokens = crate::lexer::tokenize(EXPLODE_PRELUDE_SRC).expect("explode prelude must tokenize");
    let mut combined = crate::parser::parse(&tokens).expect("explode prelude must parse");
    combined.extend(program);
    combined
}
