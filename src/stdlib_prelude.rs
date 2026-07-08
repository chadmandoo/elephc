//! Purpose:
//! Injects a small set of PHP standard-library functions (written in elephc-PHP) that
//! need no dedicated codegen or runtime helper: `mb_substr`, `mb_ltrim`, `array_last`,
//! `array_count_values`, `http_build_query`, and a `filter_var` subset. Each builds on
//! primitives the compiler already supports (strlen/ord/substr/mb_strlen, foreach,
//! assoc writes, urlencode, preg_match).
//!
//! Called from:
//! - `crate::pipeline::compile()` and the codegen test harness via `inject_if_used`,
//!   after include resolution (so usage inside includes is detected) and before name
//!   resolution (so user calls resolve to the injected functions).
//!
//! Key details:
//! - Pay-for-use with a deliberately cheap, sound over-approximation: the program AST's
//!   `Debug` rendering is scanned for the target names (function names and string
//!   literals both surface there), so a referencing program always injects. A spurious
//!   match (an unrelated string) only adds small functions that dead-code elimination
//!   drops — the same trade the var_export prelude documents.
//! - A user-declared function of the same name wins: top-level declarations (including
//!   inside namespace blocks) are collected first and shadowed prelude functions are
//!   filtered out of the injected set, so there is no redeclaration conflict.
//! - Documented semantic subsets: `array_count_values` silently skips non-int/string
//!   values (PHP warns); `http_build_query` covers flat scalar data with the default
//!   separator/encoding (nested arrays and the optional parameters are out of scope);
//!   `mb_ltrim`'s DEFAULT character set is the ASCII whitespace subset (PHP 8.4 also
//!   strips Unicode spaces) — explicit `$characters` arguments behave exactly;
//!   `filter_var` supports FILTER_VALIDATE_EMAIL / FILTER_VALIDATE_URL with
//!   documented-approximate patterns (php-src's exact filter behaviour has RFC edge
//!   cases these do not chase) and returns false for other filter ids.
//! - `mb_substr`'s optional length uses a PHP_INT_MAX sentinel default instead of a
//!   null default: `=== null` checks on null-defaulted parameters miscompile (see the
//!   list_id prelude note), and no real caller passes PHP_INT_MAX as a length.
//! - `preg_quote`'s optional delimiter uses an empty-string sentinel instead of PHP's
//!   null default (same miscompile note); empty behaves exactly like null (no extra
//!   character escaped). It escapes PHP's documented special set plus NUL as `\000`.
//! - `__elephc_explode_limit` is only reached through the EIR desugar of 3-argument
//!   `explode()` calls (`lower_static_explode_limit`); the PHP_INT_MAX sentinel keeps
//!   the no-limit case on the builtin's exact behavior. Its `explode` trigger name is
//!   deliberately broad (any program mentioning `explode` injects) — the usual
//!   over-approximation trade.
//! - `__elephc_preg_match_all_texts`/`__elephc_preg_match_all_offsets` compose
//!   `preg_match` over advancing substrings to deliver PREG_PATTERN_ORDER matches
//!   (per-group lists of strings, or of `[text, offset]` pairs under
//!   PREG_OFFSET_CAPTURE). They are two separate builders — not one function with a
//!   flags branch — so every array in play has a uniform element shape: a list whose
//!   element type unions `Str` with an array loses its string contents across an
//!   ownership event (returning it corrupts the boxed cells; see the EC-17 miscompile
//!   family). The EIR desugar picks the builder statically from the flags argument.
//!   Documented approximations: match/group byte offsets come from leftmost `strpos`
//!   of the matched text (exact for context-free patterns; anchors/lookarounds could
//!   disagree), an empty group text reports offset -1 (PHP distinguishes unmatched
//!   groups from empty participating ones), group count comes from a lexical scan of
//!   the pattern (`(` not escaped, not `(?:`-style; parens inside character classes
//!   over-count), and PREG_SET_ORDER is not supported (set-major consumers get
//!   pattern-major data).

use std::collections::HashSet;

use crate::parser::ast::{Program, StmtKind};

/// The names that trigger prelude injection, in injection order.
///
/// Most entries name a function the prelude provides. Builtins that cannot be
/// redeclared in PHP source (`preg_match_all`, `array_filter`, `strtr`,
/// `base64_decode`, `implode`, `var_dump`, `array_keys`, `array_map`) are
/// triggers only: the prelude ships `__elephc_*` impls and the EIR lowering
/// desugars the matching call shapes into calls of those impls (unused impls
/// are dead-stripped).
const STDLIB_PRELUDE_NAMES: [&str; 31] = [
    "mb_substr",
    "mb_ltrim",
    "mb_rtrim",
    "mb_trim",
    "mb_strtoupper",
    "mb_strtolower",
    "array_last",
    "array_count_values",
    "http_build_query",
    "filter_var",
    "parse_url",
    "preg_match_all",
    "preg_quote",
    "explode",
    "array_filter",
    "strtr",
    "base64_decode",
    "Randomizer",
    "implode",
    "var_dump",
    "array_keys",
    "array_map",
    "array_merge",
    "array_values",
    "array_key_exists",
    "strspn",
    "mb_substr_count",
    "strncmp",
    // `end` is a 3-letter substring that occurs unquoted inside unrelated AST
    // `Debug` output (variant/field fragments), which would spuriously inject
    // the whole prelude — and, via the shared runtime object, surface unrelated
    // link gaps. A genuine `end(...)` call renders its `Name` as `parts: ["end"]`
    // / `text: "end"`, so match the QUOTED form: strictly a subset of the bare
    // match, so it only removes false positives.
    "\"end\"",
    "sort",
    // Matches the Debug rendering of `ExprKind::Spread` — runtime spreads
    // into variadic parameters desugar to `__elephc_variadic_collect_*`.
    "Spread(",
];

/// The elephc-PHP stdlib prelude source. `__elephc_mb_byte_index` is the shared
/// codepoint→byte-offset walk backing `mb_substr` (UTF-8 continuation bytes are
/// `0b10xxxxxx`; leading bytes classify sequence length).
const STDLIB_PRELUDE_SRC: &str = r#"<?php
function __elephc_mb_byte_index(string $s, int $cpIndex, int $byteLen): int {
    $cp = 0;
    $i = 0;
    while ($i < $byteLen) {
        if ($cp === $cpIndex) {
            return $i;
        }
        $b = ord($s[$i]);
        if ($b < 128) {
            $i = $i + 1;
        } elseif (($b & 224) === 192) {
            $i = $i + 2;
        } elseif (($b & 240) === 224) {
            $i = $i + 3;
        } else {
            $i = $i + 4;
        }
        $cp = $cp + 1;
    }
    return $byteLen;
}
function mb_substr(string $string, int $start, int $length = PHP_INT_MAX): string {
    $byteLen = strlen($string);
    $cpCount = mb_strlen($string);
    // Compute in fresh locals rather than rebinding the $start/$length parameters. This
    // originally dodged a backend miscompile where a conditionally-reassigned parameter
    // corrupted an adjacent parameter's slot ($length read back as 0); that bug is now
    // fixed (two-pass parameter spill in codegen_ir/frame.rs). Kept as-is for clarity.
    $startIdx = $start;
    if ($startIdx < 0) {
        $startIdx = $cpCount + $startIdx;
        if ($startIdx < 0) {
            $startIdx = 0;
        }
    }
    $len = $length;
    if ($len === PHP_INT_MAX) {
        $end = $cpCount;
    } elseif ($len < 0) {
        $end = $cpCount + $len;
    } else {
        $end = $startIdx + $len;
    }
    if ($end > $cpCount) {
        $end = $cpCount;
    }
    if ($startIdx >= $cpCount || $end <= $startIdx) {
        return '';
    }
    $startByte = __elephc_mb_byte_index($string, $startIdx, $byteLen);
    $endByte = __elephc_mb_byte_index($string, $end, $byteLen);
    return substr($string, $startByte, $endByte - $startByte);
}
function mb_ltrim(string $string, string $characters = " \f\n\r\t\v\0"): string {
    // Trim in a fresh local (a loop accumulator here regardless); see mb_substr for the
    // now-fixed sibling-parameter-slot miscompile that first motivated avoiding rebinds.
    $s = $string;
    while ($s !== '') {
        $first = mb_substr($s, 0, 1);
        if ($first === '' || !str_contains($characters, $first)) {
            break;
        }
        $s = mb_substr($s, 1);
    }
    return $s;
}
function mb_rtrim(string $string, string $characters = " \f\n\r\t\v\0"): string {
    $s = $string;
    while ($s !== '') {
        $last = mb_substr($s, -1);
        if ($last === '' || !str_contains($characters, $last)) {
            break;
        }
        $s = mb_substr($s, 0, mb_strlen($s) - 1);
    }
    return $s;
}
function mb_trim(string $string, string $characters = " \f\n\r\t\v\0"): string {
    return mb_rtrim(mb_ltrim($string, $characters), $characters);
}
function mb_strtoupper(string $string): string {
    return strtoupper($string);
}
function mb_strtolower(string $string): string {
    return strtolower($string);
}
function parse_url(string $url, int $component = -1): mixed {
    $scheme = '';
    $afterScheme = $url;
    $colon = strpos($url, ':');
    $slash = strpos($url, '/');
    if ($colon !== false && ($slash === false || $colon < $slash)) {
        $scheme = substr($url, 0, $colon);
        $afterScheme = substr($url, $colon + 1);
    }
    $fragment = '';
    $beforeFragment = $afterScheme;
    $hashAt = strpos($afterScheme, '#');
    if ($hashAt !== false) {
        $fragment = substr($afterScheme, $hashAt + 1);
        $beforeFragment = substr($afterScheme, 0, $hashAt);
    }
    $query = '';
    $beforeQuery = $beforeFragment;
    $questionAt = strpos($beforeFragment, '?');
    if ($questionAt !== false) {
        $query = substr($beforeFragment, $questionAt + 1);
        $beforeQuery = substr($beforeFragment, 0, $questionAt);
    }
    $host = '';
    $port = '';
    $path = $beforeQuery;
    if (str_starts_with($beforeQuery, '//')) {
        $fullAuthority = substr($beforeQuery, 2);
        $path = '';
        $authority = $fullAuthority;
        $pathAt = strpos($fullAuthority, '/');
        if ($pathAt !== false) {
            $path = substr($fullAuthority, $pathAt);
            $authority = substr($fullAuthority, 0, $pathAt);
        }
        $hostWithPort = $authority;
        $at = strrpos($authority, '@');
        if ($at !== false) {
            $hostWithPort = substr($authority, $at + 1);
        }
        $host = $hostWithPort;
        $portColon = strrpos($hostWithPort, ':');
        if ($portColon !== false) {
            $port = substr($hostWithPort, $portColon + 1);
            $host = substr($hostWithPort, 0, $portColon);
        }
    }
    if ($component === 0) {
        return $scheme !== '' ? $scheme : null;
    }
    if ($component === 1) {
        return $host !== '' ? $host : null;
    }
    if ($component === 2) {
        return $port !== '' ? (int) $port : null;
    }
    if ($component === 5) {
        return $path !== '' ? $path : null;
    }
    if ($component === 6) {
        return $query !== '' ? $query : null;
    }
    if ($component === 7) {
        return $fragment !== '' ? $fragment : null;
    }
    return false;
}
function array_last(mixed $array): mixed {
    $last = null;
    foreach ($array as $value) {
        $last = $value;
    }
    return $last;
}
function end(mixed $array): mixed {
    // Read-only last-element access (the AIC uses are `end($list)` value
    // reads). PHP also advances the array's internal pointer to the last
    // element and returns false for an empty array; the pointer side effect
    // is NOT modeled (elephc arrays carry no per-value internal cursor) and
    // empty returns null rather than false — documented approximations. The
    // returned VALUE is byte-parity for a non-empty read.
    $last = null;
    foreach ($array as $value) {
        $last = $value;
    }
    return $last;
}
function array_count_values(mixed $array): mixed {
    $values = [];
    foreach ($array as $value) {
        if (is_string($value) || is_int($value)) {
            $values[] = (string) $value;
        }
    }
    $counts = [];
    $n = count($values);
    $i = 0;
    while ($i < $n) {
        $v = $values[$i];
        $c = 0;
        $j = 0;
        while ($j < $n) {
            if ($values[$j] === $v) {
                $c = $c + 1;
            }
            $j = $j + 1;
        }
        $counts[$v] = $c;
        $i = $i + 1;
    }
    return $counts;
}
function http_build_query(mixed $data): string {
    $parts = [];
    foreach ($data as $key => $value) {
        if ($value === null) {
            continue;
        }
        $rendered = is_bool($value) ? ($value ? '1' : '0') : (string) $value;
        $parts[] = urlencode((string) $key) . '=' . urlencode($rendered);
    }
    return implode('&', $parts);
}
function filter_var(mixed $value, int $filter = 516, mixed $options = null): mixed {
    $s = (string) $value;
    if ($filter === 274) {
        if (preg_match('/^[^@\s"\'\\\\]+@([A-Za-z0-9]([A-Za-z0-9-]*[A-Za-z0-9])?\.)+[A-Za-z]{2,}$/', $s) === 1) {
            return $s;
        }
        return false;
    }
    if ($filter === 273) {
        if (preg_match('/^[A-Za-z][A-Za-z0-9+.-]*:\/\/[^\s\/?#]+[^\s]*$/', $s) === 1) {
            return $s;
        }
        return false;
    }
    if ($filter === 516) {
        return $s;
    }
    return false;
}
function preg_quote(string $str, string $delimiter = ''): string {
    $specials = '.\\+*?[^]$(){}=!<>|:-#';
    $len = strlen($str);
    $out = '';
    $i = 0;
    while ($i < $len) {
        $c = substr($str, $i, 1);
        if (str_contains($specials, $c)) {
            $out = $out . '\\' . $c;
        } elseif ($delimiter !== '' && $c === $delimiter) {
            $out = $out . '\\' . $c;
        } elseif ($c === chr(0)) {
            $out = $out . '\\000';
        } else {
            $out = $out . $c;
        }
        $i = $i + 1;
    }
    return $out;
}
function __elephc_explode_limit(string $separator, string $string, int $limit): array {
    $parts = explode($separator, $string);
    if ($limit === 9223372036854775807) {
        return $parts;
    }
    if ($limit === 0) {
        $limit = 1;
    }
    $total = count($parts);
    if ($limit > 0) {
        if ($total <= $limit) {
            return $parts;
        }
        $out = [];
        $i = 0;
        while ($i < $limit - 1) {
            $out[] = $parts[$i];
            $i = $i + 1;
        }
        $rest = $parts[$limit - 1];
        $j = $limit;
        while ($j < $total) {
            $rest = $rest . $separator . $parts[$j];
            $j = $j + 1;
        }
        $out[] = $rest;
        return $out;
    }
    $keep = $total + $limit;
    $out = [];
    $i = 0;
    while ($i < $keep) {
        $out[] = $parts[$i];
        $i = $i + 1;
    }
    return $out;
}
function __elephc_preg_match_all_group_count(string $pattern): int {
    $len = strlen($pattern);
    $count = 0;
    $i = 0;
    while ($i < $len) {
        $ch = substr($pattern, $i, 1);
        if ($ch === '\\') {
            $i = $i + 2;
            continue;
        }
        if ($ch === '(') {
            $next = '';
            if ($i + 1 < $len) {
                $next = substr($pattern, $i + 1, 1);
            }
            if ($next !== '?') {
                $count = $count + 1;
            } else {
                $third = '';
                if ($i + 2 < $len) {
                    $third = substr($pattern, $i + 2, 1);
                }
                if ($third === 'P' || $third === "'") {
                    $count = $count + 1;
                }
                if ($third === '<') {
                    $fourth = '';
                    if ($i + 3 < $len) {
                        $fourth = substr($pattern, $i + 3, 1);
                    }
                    if ($fourth !== '=' && $fourth !== '!') {
                        $count = $count + 1;
                    }
                }
            }
        }
        $i = $i + 1;
    }
    return $count;
}
function __elephc_preg_match_all_texts(string $pattern, string $subject): array {
    $groupCount = __elephc_preg_match_all_group_count($pattern);
    $subjectLen = strlen($subject);
    $result = [];
    $g = 0;
    while ($g <= $groupCount) {
        $list = [];
        $cursor = 0;
        while ($cursor <= $subjectLen) {
            $rest = substr($subject, $cursor);
            $m = [];
            if (preg_match($pattern, $rest, $m) !== 1) {
                break;
            }
            $full = $m[0];
            $at = strpos($rest, $full);
            if ($at === false) {
                break;
            }
            $absolute = $cursor + $at;
            $text = '';
            if ($g < count($m)) {
                $text = $m[$g];
            }
            $list[] = $text;
            $advance = strlen($full);
            if ($advance < 1) {
                $advance = 1;
            }
            $cursor = $absolute + $advance;
        }
        $result[] = $list;
        $g = $g + 1;
    }
    return $result;
}
function __elephc_preg_match_all_offsets(string $pattern, string $subject): array {
    $groupCount = __elephc_preg_match_all_group_count($pattern);
    $subjectLen = strlen($subject);
    $result = [];
    $g = 0;
    while ($g <= $groupCount) {
        $list = [];
        $cursor = 0;
        while ($cursor <= $subjectLen) {
            $rest = substr($subject, $cursor);
            $m = [];
            if (preg_match($pattern, $rest, $m) !== 1) {
                break;
            }
            $full = $m[0];
            $at = strpos($rest, $full);
            if ($at === false) {
                break;
            }
            $absolute = $cursor + $at;
            $text = '';
            if ($g < count($m)) {
                $text = $m[$g];
            }
            $pairOff = $absolute;
            if ($g > 0) {
                $pairOff = -1;
                if ($text !== '') {
                    $inner = strpos($full, $text);
                    if ($inner !== false) {
                        $pairOff = $absolute + $inner;
                    }
                }
            }
            $list[] = [$text, $pairOff];
            $advance = strlen($full);
            if ($advance < 1) {
                $advance = 1;
            }
            $cursor = $absolute + $advance;
        }
        $result[] = $list;
        $g = $g + 1;
    }
    return $result;
}

function __elephc_array_filter_hash(array $h, callable $cb, int $mode): array {
    // Associative array_filter impl (ARRAY_FILTER_USE_VALUE/KEY/BOTH): the
    // EIR lowering routes AssocArray receivers here. Keys survive verbatim;
    // insertion order is preserved by the keyed writes.
    $out = [];
    foreach ($h as $k => $v) {
        $keep = false;
        if ($mode === 2) {
            $keep = $cb($k);
        } elseif ($mode === 1) {
            $keep = $cb($v, $k);
        } else {
            $keep = $cb($v);
        }
        if ($keep) {
            $out[$k] = $v;
        }
    }
    return $out;
}

function __elephc_array_filter_any(mixed $h, callable $cb, int $mode): mixed {
    // array_filter over a statically-Mixed receiver (json_decode results,
    // adaptive locals). The `mixed` parameter keeps the adaptive foreach —
    // an `array` param receiving a raw box fatals. Keys survive verbatim
    // for BOTH shapes: PHP array_filter preserves keys even on packed input.
    $out = [];
    foreach ($h as $k => $v) {
        $keep = false;
        if ($mode === 2) {
            $keep = $cb($k);
        } elseif ($mode === 1) {
            $keep = $cb($v, $k);
        } else {
            $keep = $cb($v);
        }
        if ($keep) {
            $out[$k] = $v;
        }
    }
    return $out;
}

function __elephc_array_keys_any(mixed $h): mixed {
    // array_keys over a statically-Mixed receiver: adaptive foreach collects
    // keys (int or string) into a fresh packed list, PHP order preserved.
    $out = [];
    foreach ($h as $k => $v) {
        $out[] = $k;
    }
    return $out;
}

function __elephc_array_map_any(callable $fn, mixed $h): mixed {
    // Single-array array_map over a statically-Mixed receiver. PHP's
    // one-array form preserves keys (string AND int), so the result is
    // built with keyed writes.
    $out = [];
    foreach ($h as $k => $v) {
        $out[$k] = $fn($v);
    }
    return $out;
}

function __elephc_array_merge_any(mixed $a, mixed $b): mixed {
    // Pairwise array_merge for associative/Mixed operands (the native
    // lowering handles packed pairs only). PHP semantics: string keys
    // overwrite in place (first-occurrence position), int keys are
    // renumbered sequentially. Every write funnels through the ONE
    // mixed-key store below — mixing statically-int keyed writes with
    // mixed-key writes on the same local would pick the packed set op
    // on promoted-hash storage.
    $out = [];
    $n = 0;
    foreach ($a as $k => $v) {
        $key = $k;
        if (!is_string($k)) {
            $key = $n;
            $n = $n + 1;
        }
        $out[$key] = $v;
    }
    foreach ($b as $k => $v) {
        $key = $k;
        if (!is_string($k)) {
            $key = $n;
            $n = $n + 1;
        }
        $out[$key] = $v;
    }
    return $out;
}

function __elephc_variadic_collect_int(mixed $src): array {
    // Runtime spread into an `int ...$n` variadic: adaptive walk over the
    // spread source (packed, hash, or Mixed box), casting each element so
    // the collection holds raw int slots matching the callee's typed reads.
    $out = [];
    foreach ($src as $v) {
        $out[] = (int) $v;
    }
    return $out;
}

function __elephc_variadic_collect_float(mixed $src): array {
    $out = [];
    foreach ($src as $v) {
        $out[] = (float) $v;
    }
    return $out;
}

function __elephc_variadic_collect_string(mixed $src): array {
    $out = [];
    foreach ($src as $v) {
        $out[] = (string) $v;
    }
    return $out;
}

function __elephc_variadic_collect_bool(mixed $src): array {
    $out = [];
    foreach ($src as $v) {
        $out[] = (bool) $v;
    }
    return $out;
}

function __elephc_variadic_collect_mixed(mixed $src): array {
    // Mixed-element variadics keep boxed cells; the callee's adaptive
    // reads handle per-slot tags.
    $out = [];
    foreach ($src as $v) {
        $out[] = $v;
    }
    return $out;
}

function strspn(string $string, string $characters, int $offset = 0, int $length = PHP_INT_MAX): int {
    // Length of the initial run of $string (from $offset) made entirely of
    // bytes from $characters. Negative offset/length follow PHP's substr
    // conventions; the PHP_INT_MAX sentinel means "to the end" (see the
    // mb_substr note on null-default miscompiles).
    $len = strlen($string);
    $start = $offset;
    if ($start < 0) {
        $start = $len + $start;
        if ($start < 0) {
            $start = 0;
        }
    }
    if ($start > $len) {
        return 0;
    }
    $end = $len;
    if ($length !== PHP_INT_MAX) {
        if ($length < 0) {
            $end = $len + $length;
        } else {
            $end = $start + $length;
        }
        if ($end > $len) {
            $end = $len;
        }
    }
    $n = 0;
    $i = $start;
    while ($i < $end) {
        if (!str_contains($characters, $string[$i])) {
            break;
        }
        $n = $n + 1;
        $i = $i + 1;
    }
    return $n;
}

function mb_substr_count(string $haystack, string $needle): int {
    // Non-overlapping needle count. Byte-wise counting is exact for
    // well-formed UTF-8: a codepoint sequence cannot straddle another
    // codepoint's bytes, so multibyte needles match at the same positions
    // mb_substr_count reports.
    if ($needle === '') {
        return 0;
    }
    $count = 0;
    $pos = 0;
    $step = strlen($needle);
    while (true) {
        $rest = substr($haystack, $pos);
        $found = strpos($rest, $needle);
        if ($found === false) {
            break;
        }
        $count = $count + 1;
        $pos = $pos + $found + $step;
    }
    return $count;
}

function strncmp(string $string1, string $string2, int $length): int {
    // PHP documents "< 0, > 0, or 0" — sign only — so the prefix spaceship
    // satisfies the contract (binary-safe byte comparison).
    return substr($string1, 0, $length) <=> substr($string2, 0, $length);
}

function __elephc_array_values_any(mixed $h): mixed {
    // array_values over adaptive/Mixed receivers: rebuild as a fresh packed
    // list (keys dropped, order preserved).
    $out = [];
    foreach ($h as $v) {
        $out[] = $v;
    }
    return $out;
}

function __elephc_array_key_exists_any(mixed $key, mixed $h): bool {
    // array_key_exists over adaptive/Mixed receivers: adaptive key walk with
    // strict comparison per key type (PHP normalizes canonical numeric-string
    // keys to ints at INSERT time, so stored keys never alias across types).
    foreach ($h as $k => $v) {
        if ($k === $key) {
            return true;
        }
    }
    return false;
}

function __elephc_mixed_gt(mixed $a, mixed $b): bool {
    // Type-aware "greater than" for boxed sort elements. Homogeneous
    // string/int/float pairs compare exactly like PHP; anything else falls
    // back to string comparison of the casts (documented approximation —
    // PHP's full cross-type comparison table is out of scope). Spaceship is
    // used throughout: relational `<`/`>` on strings is not lowered, but
    // `<=>` has the dedicated string runtime.
    if (is_int($a) && is_int($b)) {
        return ((int) $a) > ((int) $b);
    }
    if (is_float($a) && is_float($b)) {
        return ((float) $a) > ((float) $b);
    }
    return (((string) $a) <=> ((string) $b)) > 0;
}

function __elephc_sort_mixed_copy(mixed $h): array {
    // sort() over Mixed-slotted storage (boxed cells): rebuild as a fresh
    // packed list and insertion-sort it with type-aware comparison. The EIR
    // desugar assigns the result back over the by-ref argument.
    $out = [];
    foreach ($h as $v) {
        $out[] = $v;
    }
    $n = count($out);
    $i = 1;
    while ($i < $n) {
        $v = $out[$i];
        $j = $i - 1;
        while ($j >= 0 && __elephc_mixed_gt($out[$j], $v)) {
            $out[$j + 1] = $out[$j];
            $j = $j - 1;
        }
        $out[$j + 1] = $v;
        $i = $i + 1;
    }
    return $out;
}

function __elephc_strtr_pairs(string $s, array $pairs): string {
    // 2-arg strtr: longest-match, non-overlapping, single pass (unlike
    // str_replace's sequential passes). Empty 'from' keys never match; equal
    // lengths keep the first array-order winner. The EIR lowering routes
    // 2-argument strtr() calls here.
    $out = "";
    $i = 0;
    $len = strlen($s);
    while ($i < $len) {
        $bestLen = 0;
        $bestRepl = "";
        foreach ($pairs as $from => $to) {
            $f = (string) $from;
            $fl = strlen($f);
            if ($fl > $bestLen && $fl <= $len - $i && substr($s, $i, $fl) === $f) {
                $bestLen = $fl;
                $bestRepl = (string) $to;
            }
        }
        if ($bestLen > 0) {
            $out = $out . $bestRepl;
            $i = $i + $bestLen;
        } else {
            $out = $out . $s[$i];
            $i = $i + 1;
        }
    }
    return $out;
}
function __elephc_base64_decode_ex(string $s, bool $strict): mixed {
    // 2-arg base64_decode: whitespace is skipped in both modes (php-src
    // behavior); strict returns false on any other non-alphabet byte, a
    // lone-character tail, or inconsistent '=' padding. Cleaning also fixes
    // the lenient divergences of the raw builtin (embedded whitespace,
    // unpadded tails). Exotic mid-stream '=' placements are documented-
    // approximate. Delegates the actual decode to the 1-arg builtin.
    $clean = "";
    $len = strlen($s);
    $i = 0;
    $padCount = 0;
    $fail = false;
    while ($i < $len) {
        $c = $s[$i];
        $o = ord($c);
        $isAlpha = ($o >= 65 && $o <= 90) || ($o >= 97 && $o <= 122) || ($o >= 48 && $o <= 57) || $c === "+" || $c === "/";
        $isWs = $c === " " || $c === "\t" || $c === "\n" || $c === "\r" || $c === "\v" || $c === "\f";
        if ($c === "=") {
            $padCount = $padCount + 1;
            if ($padCount > 2) {
                $fail = true;
            }
        } elseif ($isAlpha) {
            if ($padCount > 0) {
                $fail = true;
            }
            $clean = $clean . $c;
        } elseif (!$isWs) {
            $fail = true;
        }
        $i = $i + 1;
    }
    $rem = strlen($clean) % 4;
    if ($strict) {
        if ($fail || $rem === 1) {
            return false;
        }
        if ($padCount > 0 && ($rem + $padCount) % 4 !== 0) {
            return false;
        }
    }
    if ($rem === 1) {
        $clean = substr($clean, 0, strlen($clean) - 1);
        $rem = 0;
    }
    if ($rem === 2) {
        $clean = $clean . "==";
    } elseif ($rem === 3) {
        $clean = $clean . "=";
    }
    return __elephc_base64_decode_raw($clean);
}
namespace Random {
    class Randomizer {
        public function __construct() {
        }
        public function getBytes(int $length): string {
            return \random_bytes($length);
        }
        public function getInt(int $min, int $max): int {
            return \random_int($min, $max);
        }
        public function nextInt(): int {
            return \random_int(0, 9223372036854775807);
        }
        public function getBytesFromString(string $string, int $length): string {
            $out = "";
            $max = (int) (\strlen($string) - 1);
            $i = 0;
            while ($i < $length) {
                $out = $out . $string[\random_int(0, $max)];
                $i = $i + 1;
            }
            return $out;
        }
    }
}

function __elephc_implode_values(string $glue, mixed $h): string {
    // implode over associative/runtime-hash values (PHP implodes VALUES for
    // hashes). The EIR lowering routes AssocArray- and Mixed-typed array
    // arguments here; foreach and the string cast are representation-adaptive
    // so packed arrays reached through Mixed keep byte-parity too.
    $out = "";
    $first = true;
    foreach ($h as $v) {
        if ($first) {
            $first = false;
        } else {
            $out = $out . $glue;
        }
        $out = $out . (string) $v;
    }
    return $out;
}

function __elephc_var_dump_value(mixed $v, string $pad): void {
    // Recursive var_dump renderer for container values (the EIR lowering
    // routes container/Mixed-typed var_dump arguments here). Only the ARRAY
    // structure is rendered locally; every non-array leaf delegates to the
    // raw builtin walker so scalar/float/object formatting stays exact.
    if (is_array($v)) {
        echo "array(", count($v), ") {\n";
        foreach ($v as $k => $val) {
            echo $pad, "  [";
            if (is_string($k)) {
                echo "\"", $k, "\"";
            } else {
                echo $k;
            }
            echo "]=>\n", $pad, "  ";
            __elephc_var_dump_value($val, $pad . "  ");
        }
        echo $pad, "}\n";
    } else {
        __elephc_var_dump_raw($v);
    }
}
"#;

/// Injects the stdlib prelude when the program references any of its names and filters
/// out functions the program declares itself.
pub fn inject_if_used(program: Program) -> Program {
    let rendered = format!("{:?}", program);
    if !STDLIB_PRELUDE_NAMES
        .iter()
        .any(|name| rendered.contains(name))
    {
        return program;
    }
    let declared = declared_function_names(&program);
    let tokens =
        crate::lexer::tokenize(STDLIB_PRELUDE_SRC).expect("stdlib prelude must tokenize");
    let prelude = crate::parser::parse(&tokens).expect("stdlib prelude must parse");
    let mut combined: Program = prelude
        .into_iter()
        .filter(|stmt| match &stmt.kind {
            StmtKind::FunctionDecl { name, .. } => {
                !declared.contains(&name.to_ascii_lowercase())
            }
            _ => true,
        })
        .collect();
    combined.extend(program);
    combined
}

/// Collects the lowercased names of functions the program declares at the top level or
/// directly inside namespace declarations, so user definitions shadow the prelude.
fn declared_function_names(program: &Program) -> HashSet<String> {
    let mut names = HashSet::new();
    collect_declared(program, &mut names);
    names
}

fn collect_declared(stmts: &[crate::parser::ast::Stmt], names: &mut HashSet<String>) {
    for stmt in stmts {
        match &stmt.kind {
            StmtKind::FunctionDecl { name, .. } => {
                names.insert(name.to_ascii_lowercase());
            }
            StmtKind::NamespaceBlock { body, .. } | StmtKind::Synthetic(body) => {
                collect_declared(body, names);
            }
            _ => {}
        }
    }
}
