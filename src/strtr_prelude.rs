//! Purpose:
//! Injects the `__elephc_strtr_pairs` function that backs the two-argument
//! `strtr($str, $pairs)` (replacement-map) form. The name resolver rewrites a `strtr(...)`
//! call with exactly two arguments to `__elephc_strtr_pairs(...)`; the three-argument
//! single-character-translation form stays the native `strtr` builtin.
//!
//! Called from:
//! - `crate::pipeline::compile` (and the codegen test harness) via `inject_if_used`, before
//!   name resolution so the rewritten call resolves to the injected function.
//!
//! Key details:
//! - A dedicated one-function prelude (not an entry in `stdlib_prelude`) on purpose: the
//!   shared stdlib bundle drags in every bundled function when any is triggered, and
//!   `get_debug_type`'s `get_class($value)` on a `Mixed` value is not yet lowerable — so
//!   folding `strtr` into that bundle would make every 2-arg-`strtr` program fail to compile
//!   natively. Injecting only this self-contained function keeps the fix non-hollow.
//! - Pay-for-use via a cheap Debug-render substring scan for `strtr`. Over-injection (a
//!   3-arg-only or string-literal `strtr`) is harmless: the function is self-contained and
//!   link-GC strips it when the rewrite never fires. A user's own top-level
//!   `__elephc_strtr_pairs` declaration shadows the prelude (it is left un-injected).
//! - Longest-match-wins replacement without sorting the keys (the compiler does not yet
//!   support `usort` over a `Mixed`-typed key array): at each position the longest key that
//!   matches is applied, matched spans are not re-scanned, and an empty key never matches —
//!   PHP's `strtr` semantics.

use crate::parser::ast::{Program, StmtKind};

/// The elephc-PHP source for the injected two-argument `strtr` backing function.
const STRTR_PRELUDE_SRC: &str = r#"<?php
function __elephc_strtr_pairs(string $str, array $pairs): string {
    $result = "";
    $i = 0;
    $n = strlen($str);
    while ($i < $n) {
        $bestKey = "";
        $bestLen = 0;
        foreach ($pairs as $key => $value) {
            $ks = (string) $key;
            $kl = strlen($ks);
            if ($kl > $bestLen && $i + $kl <= $n && substr($str, $i, $kl) === $ks) {
                $bestKey = $ks;
                $bestLen = $kl;
            }
        }
        if ($bestLen > 0) {
            $result = $result . (string) $pairs[$bestKey];
            $i = $i + $bestLen;
        } else {
            $result = $result . $str[$i];
            $i = $i + 1;
        }
    }
    return $result;
}
"#;

/// Prepends `__elephc_strtr_pairs` when the program mentions `strtr`, so the name resolver's
/// two-argument rewrite has a definition to bind to; otherwise returns the program unchanged.
/// A user's own top-level `__elephc_strtr_pairs` declaration is left in place (not shadowed).
/// The prelude is hoisted function declarations only, so prepending does not change top-level
/// execution order. Tokenize/parse failure is a compiler bug and panics rather than degrading.
pub fn inject_if_used(program: Program) -> Program {
    if !format!("{program:?}").contains("strtr") {
        return program;
    }
    let user_declares = program.iter().any(|stmt| {
        matches!(
            &stmt.kind,
            StmtKind::FunctionDecl { name, .. } if name.eq_ignore_ascii_case("__elephc_strtr_pairs")
        )
    });
    if user_declares {
        return program;
    }
    let tokens = crate::lexer::tokenize(STRTR_PRELUDE_SRC).expect("strtr prelude must tokenize");
    let mut combined = crate::parser::parse(&tokens).expect("strtr prelude must parse");
    combined.extend(program);
    combined
}
