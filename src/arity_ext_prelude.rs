//! Purpose:
//! Injects small elephc-PHP helper functions that back builtin forms the backend does not
//! provide: the extended-arity `array_search($needle, $haystack, $strict)` and
//! `strpos($haystack, $needle, $offset)` (whose extra argument the runtime does not thread),
//! the otherwise-missing `strcspn($subject, $characters)`, and the two-argument
//! `base64_decode($string, $strict)` (the native builtin is 1-arg only). The name resolver
//! rewrites the specific call forms to the injected functions; the shorter native forms are
//! untouched.
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
function __elephc_base64_decode_strict(string $string, bool $strict): string|false {
    if ($strict) {
        $alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
        $n = strlen($string);
        $i = 0;
        while ($i < $n) {
            if (strpos($alphabet, $string[$i]) === false) {
                return false;
            }
            $i = $i + 1;
        }
    }
    return base64_decode($string);
}
function __elephc_mkdir(string $directory, int $mode = 0o777, bool $recursive = false): bool {
    if (!$recursive) {
        return mkdir($directory);
    }
    if (is_dir($directory)) {
        return true;
    }
    $parts = explode('/', $directory);
    $path = '';
    foreach ($parts as $part) {
        if ($part === '') {
            if ($path === '') {
                $path = '/';
            }
            continue;
        }
        if ($path === '' || $path === '/') {
            $path = $path . $part;
        } else {
            $path = $path . '/' . $part;
        }
        if (!is_dir($path) && !mkdir($path)) {
            return false;
        }
    }
    return true;
}
function __elephc_preg_quote(string $str, ?string $delimiter = null): string {
    $special = ".\\+*?[^]$(){}=!<>|:-#";
    if ($delimiter !== null && $delimiter !== "") {
        $special = $special . $delimiter;
    }
    $result = "";
    $n = strlen($str);
    $i = 0;
    while ($i < $n) {
        $ch = $str[$i];
        if ($ch === "\0") {
            $result = $result . "\\000";
        } elseif (strpos($special, $ch) !== false) {
            $result = $result . "\\" . $ch;
        } else {
            $result = $result . $ch;
        }
        $i = $i + 1;
    }
    return $result;
}
function __elephc_strncmp(string $a, string $b, int $length): int {
    if ($length < 0) {
        $length = 0;
    }
    return strcmp(substr($a, 0, $length), substr($b, 0, $length));
}
function __elephc_strncasecmp(string $a, string $b, int $length): int {
    if ($length < 0) {
        $length = 0;
    }
    return strcasecmp(substr($a, 0, $length), substr($b, 0, $length));
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
        && !rendered.contains("base64_decode")
        && !rendered.contains("mkdir")
        && !rendered.contains("preg_quote")
        && !rendered.contains("strncmp")
        && !rendered.contains("strncasecmp")
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
                    || name.eq_ignore_ascii_case("__elephc_base64_decode_strict")
                    || name.eq_ignore_ascii_case("__elephc_mkdir")
                    || name.eq_ignore_ascii_case("__elephc_preg_quote")
                    || name.eq_ignore_ascii_case("__elephc_strncmp")
                    || name.eq_ignore_ascii_case("__elephc_strncasecmp")
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
