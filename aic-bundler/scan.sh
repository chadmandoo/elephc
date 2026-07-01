#!/usr/bin/env bash
# Compilability scan: for each Domain root, bundle its transitive closure and
# run elephc --check. Emits a per-root PASS/GAP map + first error for gaps.
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
CLASSMAP=/srv/stacks/aic/vendor/composer/autoload_classmap.php
BIN="$HERE/../elephc/target/release/elephc"
NS_PREFIX="${1:-AIC\\Components\\Domain\\}"

mapfile -t ROOTS < <(php -r '$m=require $argv[1]; $p=$argv[2]; foreach(array_keys($m) as $k){ if(str_starts_with($k,$p)) echo $k,"\n"; }' "$CLASSMAP" "$NS_PREFIX" | sort -u)

pass=0; gap=0
printf '%-26s %-6s %-4s  %s\n' "CLASS" "FILES" "RES" "FIRST ERROR"
printf '%s\n' "----------------------------------------------------------------------------"
for r in "${ROOTS[@]}"; do
    short="${r##*\\}"
    out="$HERE/out/scan/$short"
    rm -rf "$out"
    php "$HERE/bundle.php" --classmap="$CLASSMAP" --out="$out" --root="$r" >/dev/null 2>&1
    files=$(php -r '$j=json_decode(file_get_contents($argv[1]),true); echo $j["files"];' "$out/_manifest.json" 2>/dev/null || echo "?")
    printf '<?php\nrequire_once __DIR__."/_bundle.php";\necho "ok";\n' > "$out/main.php"
    err=$("$BIN" --check "$out/main.php" 2>&1 | grep -m1 "error" || true)
    if [ -z "$err" ]; then
        printf '%-26s %-6s \033[32m%-4s\033[0m  %s\n' "$short" "$files" "OK" ""
        pass=$((pass+1))
    else
        # strip the long path prefix for readability
        short_err=$(echo "$err" | sed -E 's#/srv/[^ ]*/##; s/^error//')
        printf '%-26s %-6s \033[31m%-4s\033[0m  %s\n' "$short" "$files" "GAP" "$short_err"
        gap=$((gap+1))
    fi
done
printf '%s\n' "----------------------------------------------------------------------------"
printf 'Domain roots: %d   compiles-clean: %d   has-gap: %d\n' "${#ROOTS[@]}" "$pass" "$gap"
