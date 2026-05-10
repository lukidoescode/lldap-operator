#!/usr/bin/env bash
set -euo pipefail

LATEST="${1:-main-branch}"
OUTPUT="${2:-index.html}"

cat > "$OUTPUT" << HTMLEOF
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta http-equiv="refresh" content="0; url=/${LATEST}/">
  <title>Redirecting to latest docs</title>
</head>
<body>
  <p>Redirecting to <a href="/${LATEST}/">latest documentation (${LATEST})</a></p>
</body>
</html>
HTMLEOF

echo "Generated root redirect -> /${LATEST}/"
