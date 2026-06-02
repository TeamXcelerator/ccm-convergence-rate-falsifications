#!/usr/bin/env bash
# Verify that data/zeta_zeros.json matches the expected SHA-256 hash.
# Run this after cloning to confirm the reference data wasn't corrupted
# in transit. Hash is recorded in README.md and updated whenever the
# generator script is rerun.
#
# Author: Ronnie Andrews, Jr. (Team Xcelerator Inc.(R))
set -euo pipefail

EXPECTED="${1:-}"
FILE=${2:-data/zeta_zeros.json}

if [ -z "$EXPECTED" ]; then
  echo "Usage: $0 <expected_sha256> [path]"
  echo "       Get expected hash from README.md."
  exit 2
fi

if [ ! -f "$FILE" ]; then
  echo "ERROR: $FILE not found. Run scripts/generate_zeros.sh first."
  exit 1
fi

ACTUAL=$(sha256sum "$FILE" | awk '{print $1}')
if [ "$ACTUAL" = "$EXPECTED" ]; then
  echo "OK  $FILE  $ACTUAL"
  exit 0
else
  echo "FAIL"
  echo "  expected: $EXPECTED"
  echo "  actual:   $ACTUAL"
  exit 1
fi
