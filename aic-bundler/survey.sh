#!/usr/bin/env bash
# AIC compilability survey: scan every AIC class whose FQCN CONTAINS a segment
# (default "\Domain\") — bundle its transitive closure + elephc --check — and
# emit (1) a per-class OK/GAP map, (2) per-ward tallies, (3) a gap catalog
# (distinct compiler-error kinds + counts + example classes).
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
CLASSMAP=/srv/stacks/aic/vendor/composer/autoload_classmap.php
BIN="/srv/stacks/elephc/target/release/elephc"
SEG="${1:-\\Domain\\}"          # substring the FQCN must contain
OUT="$HERE/out/survey"
rm -rf "$OUT"; mkdir -p "$OUT"
: > "$OUT/gaps.tsv"; : > "$OUT/map.txt"

mapfile -t ROOTS < <(php -r '$m=require $argv[1]; $s=$argv[2]; foreach(array_keys($m) as $k){ if(str_starts_with($k,"AIC\\") && str_contains($k,$s) && !str_contains($k,"\\Tests\\") && !str_contains($k,"\\PhpStan\\") && !str_contains($k,"GeneratorStream") && !str_contains($k,"IntegerXmlAttribute")) echo $k,"\n"; }' "$CLASSMAP" "$SEG" | sort -u)

pass=0; gap=0
declare -A WARD_OK WARD_GAP
for r in "${ROOTS[@]}"; do
    ward="$(echo "$r" | sed -E 's#^AIC\\([^\\]+)\\.*#\1#')"
    d="$OUT/$(echo "$r" | tr '\\' '_')"
    php "$HERE/bundle.php" --classmap="$CLASSMAP" --out="$d" --root="$r" >/dev/null 2>&1
    printf '<?php\nrequire_once __DIR__."/_bundle.php";\necho "ok";\n' > "$d/main.php"
    err=$("$BIN" --check "$d/main.php" 2>&1 | grep -m1 -iE "error|not supported|unknown|unsupported|cannot|expected" || true)
    if [ -z "$err" ]; then
        pass=$((pass+1)); WARD_OK[$ward]=$(( ${WARD_OK[$ward]:-0} + 1 ))
        printf '%-52s OK\n' "$r" >> "$OUT/map.txt"
    else
        gap=$((gap+1)); WARD_GAP[$ward]=$(( ${WARD_GAP[$ward]:-0} + 1 ))
        clean=$(echo "$err" | sed -E 's#/srv/[^ ]*/##g; s/^\[[0-9:]+\]:? *//')
        printf '%-52s GAP  %s\n' "$r" "$clean" >> "$OUT/map.txt"
        # gap signature: strip file paths, line/col, and concrete FQCNs for grouping
        sig=$(echo "$clean" | sed -E 's/AIC\\[A-Za-z0-9_\\]+/<FQCN>/g; s/"[^"]*"/<x>/g; s/[0-9]+/N/g' | tr -s ' ')
        printf '%s\t%s\n' "$sig" "$r" >> "$OUT/gaps.tsv"
    fi
done

echo "================ AIC compilability survey — segment '$SEG' ================"
printf 'roots: %d   OK: %d   GAP: %d   (%.0f%% compile-clean)\n' \
    "${#ROOTS[@]}" "$pass" "$gap" "$(awk "BEGIN{print ${#ROOTS[@]}?100*$pass/${#ROOTS[@]}:0}")"
echo
echo "=== per-ward (ward: OK/total) ==="
for w in $(printf '%s\n' "${!WARD_OK[@]}" "${!WARD_GAP[@]}" | sort -u); do
    ok=${WARD_OK[$w]:-0}; gp=${WARD_GAP[$w]:-0}
    printf '  %-20s %d/%d\n' "$w" "$ok" "$((ok+gp))"
done
echo
echo "=== GAP CATALOG (distinct compiler-error kind × count) ==="
cut -f1 "$OUT/gaps.tsv" | sort | uniq -c | sort -rn
echo
echo "full per-class map: $OUT/map.txt ; gaps: $OUT/gaps.tsv"
