//! Purpose:
//! Injects small elephc-PHP helper functions that back builtin forms the backend does not
//! provide: the extended-arity `array_search($needle, $haystack, $strict)` and
//! `strpos($haystack, $needle, $offset)` (whose extra argument the runtime does not thread),
//! and the otherwise-missing `strcspn($subject, $characters)`. The name resolver rewrites the
//! specific call forms to the injected functions; the shorter native forms are untouched.
//!
//! Called from:
//! - `crate::pipeline::compile` (and the codegen test harness) via `inject_if_used`, before
//!   name resolution so the rewritten calls resolve to the injected functions.
//!
//! Key details:
//! - Dedicated helper prelude (not folded into `stdlib_prelude`): that shared bundle injects
//!   all of its functions together and its `get_debug_type` polyfill's `get_class($value)` on
//!   a Mixed value is not yet natively lowerable, so folding these in would break every
//!   triggering program at native compile. Each helper here is verified to compile natively
//!   under generic Mixed arguments.
//! - Pay-for-use: injected when the program mentions `array_search` or `strpos` (a Debug-render
//!   substring scan). Over-injection is harmless — the helpers are self-contained and link-GC
//!   strips whichever the arity rewrite never binds.
//! - Both helpers stay within natively-lowerable primitives: `array_search`'s strict/loose
//!   compare is a plain `foreach` with `===`/`==`; `strpos`'s offset is a `substr` slice plus
//!   the native 2-arg `strpos`, re-based onto the original string (negative offsets count from
//!   the end, matching PHP). `array_merge`'s variadic form is deliberately NOT here: both a
//!   chained `array_merge` and a manual string-keyed merge hit backend Mixed-array limits.

use crate::parser::ast::{Program, StmtKind};

/// The elephc-PHP source for the injected extended-arity builtin helpers.
const ARITY_EXT_PRELUDE_SRC: &str = r#"<?php
function __elephc_array_search_strict(mixed $needle, array $haystack, bool $strict): int|string|false {
    foreach ($haystack as $key => $value) {
        if ($strict) {
            if ($value === $needle) {
                return $key;
            }
        } else {
            if ($value == $needle) {
                return $key;
            }
        }
    }
    return false;
}
function __elephc_strpos_offset(string $haystack, string $needle, int $offset): int|false {
    $len = strlen($haystack);
    $start = $offset < 0 ? $len + $offset : $offset;
    if ($start < 0) {
        $start = 0;
    }
    $sub = substr($haystack, $start);
    $pos = strpos($sub, $needle);
    if ($pos === false) {
        return false;
    }
    return $pos + $start;
}
function __elephc_strcspn(string $subject, string $characters): int {
    $n = strlen($subject);
    $i = 0;
    while ($i < $n) {
        if (strpos($characters, $subject[$i]) !== false) {
            break;
        }
        $i = $i + 1;
    }
    return $i;
}
"#;

/// Prepends the extended-arity builtin helpers when the program mentions `array_search` or
/// `strpos`, so the name resolver's arity rewrites have definitions to bind to; otherwise
/// returns the program unchanged. A user's own top-level declaration of either helper is left
/// in place. The prelude is hoisted function declarations only, so prepending does not change
/// top-level execution order. Tokenize/parse failure is a compiler bug and panics.
pub fn inject_if_used(program: Program) -> Program {
    let rendered = format!("{program:?}");
    if !rendered.contains("array_search")
        && !rendered.contains("strpos")
        && !rendered.contains("strcspn")
    {
        return program;
    }
    let user_declares = program.iter().any(|stmt| {
        matches!(
            &stmt.kind,
            StmtKind::FunctionDecl { name, .. }
                if name.eq_ignore_ascii_case("__elephc_array_search_strict")
                    || name.eq_ignore_ascii_case("__elephc_strpos_offset")
                    || name.eq_ignore_ascii_case("__elephc_strcspn")
        )
    });
    if user_declares {
        return program;
    }
    let tokens =
        crate::lexer::tokenize(ARITY_EXT_PRELUDE_SRC).expect("arity-ext prelude must tokenize");
    let mut combined = crate::parser::parse(&tokens).expect("arity-ext prelude must parse");
    combined.extend(program);
    combined
}
