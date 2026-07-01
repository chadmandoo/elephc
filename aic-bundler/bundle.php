<?php

declare(strict_types=1);

/**
 * elephc AIC bundler (P4 prototype).
 *
 * Given one or more root FQCNs, walk their transitive dependency closure using
 * AIC's Composer classmap (READ-ONLY) and emit a single _bundle.php that
 * require_once's every closure file in dependency (topological) order.
 *
 * Dependency discovery is a greedy tokenizer pass: for every name-token in a
 * file, resolve it two ways — as fully-qualified, and as same-namespace — and
 * keep whichever lands on a classmap key. In strict one-class-per-file PSR-4
 * code that captures every dep (use lines, qualified refs, bare same-namespace
 * refs, trait-use, attributes) without needing a full name resolver.
 *
 * Names that resolve to NOTHING in the classmap are "external" — PHP stdlib,
 * language builtins, or (interesting) things elephc must support. We report
 * them so a bigger slice surfaces new compiler gaps.
 *
 * Usage:
 *   php bundle.php --classmap=/path/autoload_classmap.php \
 *                  --out=/path/out \
 *                  --root='AIC\Components\Domain\ComponentDescriptor' [--root=...]
 */

/** @return array{classmap:string,out:string,roots:array<int,string>} */
function parse_args(array $argv): array
{
    $classmap = '';
    $out = '';
    $roots = [];
    foreach (array_slice($argv, 1) as $arg) {
        if (str_starts_with($arg, '--classmap=')) {
            $classmap = substr($arg, 11);
        } elseif (str_starts_with($arg, '--out=')) {
            $out = substr($arg, 6);
        } elseif (str_starts_with($arg, '--root=')) {
            $roots[] = ltrim(substr($arg, 7), '\\');
        }
    }
    if ($classmap === '' || $out === '' || $roots === []) {
        fwrite(STDERR, "usage: php bundle.php --classmap=FILE --out=DIR --root=FQCN [--root=FQCN ...]\n");
        exit(2);
    }
    return ['classmap' => $classmap, 'out' => $out, 'roots' => $roots];
}

/**
 * Extract dependency FQCNs referenced by a source file.
 *
 * @param array<string,string> $classmap FQCN => absolute file path
 * @return array{deps:array<int,string>,external:array<int,string>}
 */
function scan_file(string $path, array $classmap): array
{
    $src = file_get_contents($path);
    if ($src === false) {
        throw new RuntimeException("cannot read {$path}");
    }
    $tokens = token_get_all($src);

    $namespace = '';
    $deps = [];
    $external = [];
    $prevSig = null; // previous significant (non-ws, non-comment) token id/char

    $count = count($tokens);
    for ($i = 0; $i < $count; $i++) {
        $tok = $tokens[$i];
        if (!is_array($tok)) {
            $prevSig = $tok; // a char token like '::' arrives as a plain string
            continue;
        }
        [$id, $text] = $tok;

        if (in_array($id, [T_WHITESPACE, T_COMMENT, T_DOC_COMMENT], true)) {
            continue; // don't disturb prevSig
        }

        // Track the file's namespace declaration, and consume its name token so
        // it isn't re-scanned as a (bogus) class reference.
        if ($id === T_NAMESPACE) {
            $ns = '';
            for ($j = $i + 1; $j < $count; $j++) {
                $t = $tokens[$j];
                if (is_array($t) && in_array($t[0], [T_STRING, T_NAME_QUALIFIED, T_NS_SEPARATOR], true)) {
                    $ns .= $t[1];
                    $i = $j; // consume
                } elseif (is_array($t) && $t[0] === T_WHITESPACE) {
                    if ($ns !== '') {
                        break;
                    }
                } else {
                    break;
                }
            }
            $namespace = trim($ns, '\\');
            $prevSig = T_NAMESPACE;
            continue;
        }

        // Only name-ish tokens can be class references.
        if (!in_array($id, [T_STRING, T_NAME_QUALIFIED, T_NAME_FULLY_QUALIFIED], true)) {
            $prevSig = $id;
            continue;
        }

        // Skip member/const/case/method-name positions: `Foo::Case`, `$x->prop`,
        // `function name`, `->method`, `const NAME`. These are not class refs.
        if (in_array($prevSig, ['::', '->', T_DOUBLE_COLON, T_OBJECT_OPERATOR, T_FUNCTION, T_CONST, T_CASE, T_NULLSAFE_OBJECT_OPERATOR], true)) {
            $prevSig = $id;
            continue;
        }
        $prevSig = $id;

        $name = ltrim($text, '\\');
        if ($name === '') {
            continue;
        }

        // Candidate 1: treat as fully-qualified.
        // Candidate 2: treat as same-namespace unqualified (only for bare names).
        $candidates = [$name];
        if ($namespace !== '' && !str_contains($name, '\\')) {
            $candidates[] = $namespace . '\\' . $name;
        }

        $hit = null;
        foreach ($candidates as $c) {
            if (isset($classmap[$c])) {
                $hit = $c;
                break;
            }
        }
        if ($hit !== null) {
            $deps[$hit] = true;
        } else {
            // Report qualified/namespaced externals (skip lowercase language
            // builtins like string/int/self — those are single bare words).
            if (str_contains($name, '\\') || ctype_upper($name[0])) {
                $external[$name] = true;
            }
        }
    }

    return ['deps' => array_keys($deps), 'external' => array_keys($external)];
}

/**
 * PHP built-in interfaces/classes that Composer classmaps may back with a
 * `symfony/polyfill` stub. On a native target elephc provides these itself, so
 * they must NOT enter the compile closure (their polyfill files are guarded by
 * `PHP_VERSION_ID < N` conditionals that reference host-only constants).
 *
 * @var array<string,true>
 */
const BUILTIN_TYPES = [
    'Stringable' => true,
    'UnitEnum' => true,
    'BackedEnum' => true,
    'JsonSerializable' => true,
    'Countable' => true,
    'Traversable' => true,
    'Iterator' => true,
    'IteratorAggregate' => true,
    'ArrayAccess' => true,
    'Throwable' => true,
    'Attribute' => true,
];

/**
 * Should a resolved FQCN be treated as a compiler builtin (excluded from the
 * closure) rather than an AIC source file?
 *
 * @param string $fqcn resolved fully-qualified class name
 * @param string $path classmap file path
 */
function is_builtin_or_polyfill(string $fqcn, string $path): bool
{
    if (isset(BUILTIN_TYPES[ltrim($fqcn, '\\')])) {
        return true;
    }
    return str_contains($path, '/vendor/symfony/polyfill');
}

$opts = parse_args($argv);

/** @var array<string,string> $classmap */
$classmap = require $opts['classmap'];
if (!is_array($classmap)) {
    fwrite(STDERR, "classmap did not return an array\n");
    exit(2);
}

// BFS/DFS the closure.
$fileOf = [];          // FQCN => path
$depsOf = [];          // FQCN => list<FQCN>
$externalAll = [];     // name => true
$queue = $opts['roots'];
$seen = [];

foreach ($opts['roots'] as $r) {
    if (!isset($classmap[$r])) {
        fwrite(STDERR, "root not in classmap: {$r}\n");
        exit(2);
    }
}

while ($queue !== []) {
    $fqcn = array_shift($queue);
    if (isset($seen[$fqcn])) {
        continue;
    }
    $seen[$fqcn] = true;

    $path = $classmap[$fqcn] ?? null;
    if ($path === null) {
        continue; // external — not an AIC file
    }
    if (is_builtin_or_polyfill($fqcn, $path)) {
        $externalAll[$fqcn . ' (builtin)'] = true;
        continue; // elephc provides this natively — keep it out of the closure
    }
    $fileOf[$fqcn] = $path;

    $scan = scan_file($path, $classmap);
    $depsOf[$fqcn] = $scan['deps'];
    foreach ($scan['external'] as $e) {
        $externalAll[$e] = true;
    }
    foreach ($scan['deps'] as $d) {
        if (!isset($seen[$d])) {
            $queue[] = $d;
        }
    }
}

// Topological order (deps before dependents); break cycles arbitrarily.
$ordered = [];
$mark = []; // 0=unvisited implicit, 1=on-stack, 2=done
$visit = function (string $n) use (&$visit, &$mark, &$ordered, $depsOf): void {
    if (($mark[$n] ?? 0) !== 0) {
        return; // done or on-stack (cycle) — skip
    }
    $mark[$n] = 1;
    foreach ($depsOf[$n] ?? [] as $d) {
        if (isset($depsOf[$d])) { // only AIC files we captured
            $visit($d);
        }
    }
    $mark[$n] = 2;
    $ordered[] = $n;
};
foreach (array_keys($fileOf) as $n) {
    $visit($n);
}

// Emit.
@mkdir($opts['out'], 0777, true);
$outDir = rtrim($opts['out'], '/');

$lines = ["<?php", "", "declare(strict_types=1);", "", "// Generated by bundle.php — transitive closure of: " . implode(', ', $opts['roots']), ""];
foreach ($ordered as $fqcn) {
    $lines[] = "require_once " . var_export($fileOf[$fqcn], true) . "; // {$fqcn}";
}
$lines[] = "";
file_put_contents($outDir . '/_bundle.php', implode("\n", $lines) . "\n");

// Manifest / report.
ksort($externalAll);
$report = [
    'roots' => $opts['roots'],
    'files' => count($ordered),
    'order' => $ordered,
    'external_referenced' => array_keys($externalAll),
];
file_put_contents($outDir . '/_manifest.json', json_encode($report, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");

fprintf(STDOUT, "closure: %d files\n", count($ordered));
fprintf(STDOUT, "bundle : %s/_bundle.php\n", $outDir);
fprintf(STDOUT, "external names (%d):\n", count($externalAll));
foreach (array_keys($externalAll) as $e) {
    fprintf(STDOUT, "  - %s\n", $e);
}
