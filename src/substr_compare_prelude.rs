//! Purpose:
//! Injects a pure elephc-PHP `substr_compare()` implementation. PHP's `substr_compare` is not a
//! native builtin, so any program that calls it fails with "Undefined function". This prelude
//! provides it, reduced to `substr` + `strcmp`/`strcasecmp` (all native): exact for the `=== 0`
//! equality use (content-scanner keyword matching) and sign-correct for ordering.
//!
//! Called from:
//! - `crate::pipeline::compile` (and the codegen test harness) via `inject_if_used`, before name
//!   resolution so the call binds to the injected function.
//!
//! Key details:
//! - A dedicated one-function prelude (not an entry in `stdlib_prelude`) for the same reason as
//!   `strtr_prelude`: the shared stdlib bundle drags in every bundled function (including
//!   `get_debug_type`, whose `get_class($mixed)` is not yet lowerable) when any is triggered, so
//!   folding `substr_compare` into that bundle would make every `substr_compare` program fail to
//!   compile natively. Injecting only this self-contained function keeps the fix non-hollow.
//! - `substr_compare` is not a reserved builtin here (it is reported "Undefined function"), so the
//!   prelude defines the real name directly — no rename + name-resolver rewrite is needed (unlike
//!   `strtr`, which shadows a native builtin).
//! - Pay-for-use via a cheap Debug-render substring scan for `substr_compare`. Over-injection is
//!   harmless: the function is self-contained and link-GC strips it if never called. A user's own
//!   top-level `substr_compare` declaration shadows the prelude (it is left un-injected).
//! - Params evaluate exactly once (unlike a `substr(...)`/`substr(...)` desugar that would
//!   duplicate `$length`), so a side-effecting length argument is handled correctly.

use crate::parser::ast::{Program, StmtKind};

/// The elephc-PHP source for the injected `substr_compare` function. A negative `$offset` counts
/// from the end of `$main_str` (clamped at 0); a null `$length` compares to the end of the
/// haystack against the full needle; case-insensitivity selects `strcasecmp` over `strcmp`.
const SUBSTR_COMPARE_PRELUDE_SRC: &str = r#"<?php
function substr_compare(string $main_str, string $str, int $offset, ?int $length = null, bool $case_insensitive = false): int {
    if ($offset < 0) {
        $offset = strlen($main_str) + $offset;
        if ($offset < 0) {
            $offset = 0;
        }
    }
    if ($length === null) {
        $mainSlice = substr($main_str, $offset);
        $needleSlice = $str;
    } else {
        $mainSlice = substr($main_str, $offset, $length);
        $needleSlice = substr($str, 0, $length);
    }
    if ($case_insensitive) {
        return strcasecmp($mainSlice, $needleSlice);
    }
    return strcmp($mainSlice, $needleSlice);
}
"#;

/// Prepends the `substr_compare` implementation when the program mentions `substr_compare` and
/// does not declare its own; otherwise returns the program unchanged. The prelude is a single
/// hoisted function declaration, so prepending does not change top-level execution order.
/// Tokenize/parse failure is a compiler bug and panics rather than degrading.
pub fn inject_if_used(program: Program) -> Program {
    if !format!("{program:?}").contains("substr_compare") {
        return program;
    }
    let user_declares = program.iter().any(|stmt| {
        matches!(
            &stmt.kind,
            StmtKind::FunctionDecl { name, .. } if name.eq_ignore_ascii_case("substr_compare")
        )
    });
    if user_declares {
        return program;
    }
    let tokens = crate::lexer::tokenize(SUBSTR_COMPARE_PRELUDE_SRC)
        .expect("substr_compare prelude must tokenize");
    let mut combined =
        crate::parser::parse(&tokens).expect("substr_compare prelude must parse");
    combined.extend(program);
    combined
}
