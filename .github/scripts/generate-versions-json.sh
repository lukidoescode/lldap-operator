#!/usr/bin/env bash
set -euo pipefail

GH_PAGES_DIR="${1:-.}"
OUTPUT="${2:-versions.json}"

VERSIONS=()
for dir in "$GH_PAGES_DIR"/v*/; do
  [ -d "$dir" ] || continue
  VERSIONS+=("$(basename "$dir")")
done

if [[ ${#VERSIONS[@]} -eq 0 ]]; then
  SORTED=()
else
  IFS=$'\n'
  SORTED=($(printf '%s\n' "${VERSIONS[@]}" | sort -rV))
  unset IFS
fi

LATEST=""
if [[ ${#SORTED[@]} -gt 0 ]]; then
  for v in "${SORTED[@]}"; do
    if [[ ! "$v" =~ ^v[0-9]+\.[0-9]+\.[0-9]+-.+ ]]; then
      LATEST="$v"
      break
    fi
  done
fi

HAS_MAIN=false
if [ -d "$GH_PAGES_DIR/main-branch" ]; then
  HAS_MAIN=true
fi

{
  echo '{'
  echo "  \"latest\": \"${LATEST}\","
  echo "  \"has_main\": ${HAS_MAIN},"
  echo '  "versions": ['
  for i in "${!SORTED[@]}"; do
    COMMA=","
    if [ "$i" -eq $(( ${#SORTED[@]} - 1 )) ]; then COMMA=""; fi
    echo "    \"${SORTED[$i]}\"${COMMA}"
  done
  echo '  ]'
  echo '}'
} > "$OUTPUT"

echo "Generated $OUTPUT with ${#SORTED[@]} versions (latest: $LATEST)"
