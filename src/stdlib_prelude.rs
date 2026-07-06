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

use std::collections::HashSet;

use crate::parser::ast::{Program, StmtKind};

/// The names this prelude can provide, in injection order.
const STDLIB_PRELUDE_NAMES: [&str; 11] = [
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
    if ($start < 0) {
        $start = $cpCount + $start;
        if ($start < 0) {
            $start = 0;
        }
    }
    if ($length === PHP_INT_MAX) {
        $end = $cpCount;
    } elseif ($length < 0) {
        $end = $cpCount + $length;
    } else {
        $end = $start + $length;
    }
    if ($end > $cpCount) {
        $end = $cpCount;
    }
    if ($start >= $cpCount || $end <= $start) {
        return '';
    }
    $startByte = __elephc_mb_byte_index($string, $start, $byteLen);
    $endByte = __elephc_mb_byte_index($string, $end, $byteLen);
    return substr($string, $startByte, $endByte - $startByte);
}
function mb_ltrim(string $string, string $characters = " \f\n\r\t\v\0"): string {
    while ($string !== '') {
        $first = mb_substr($string, 0, 1);
        if ($first === '' || !str_contains($characters, $first)) {
            break;
        }
        $string = mb_substr($string, 1);
    }
    return $string;
}
function mb_rtrim(string $string, string $characters = " \f\n\r\t\v\0"): string {
    while ($string !== '') {
        $last = mb_substr($string, -1);
        if ($last === '' || !str_contains($characters, $last)) {
            break;
        }
        $string = mb_substr($string, 0, mb_strlen($string) - 1);
    }
    return $string;
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
