#!/usr/bin/env bash
# Generate the canonical reference zero file: data/zeta_zeros.json
#
# - 1000 zeros at 1000-digit precision (5x the maximum κ tested,
#   2x the maximum precision used in computation)
# - Decimal strings (no f64 truncation in the canonical store)
# - Cross-validated against the universally-tabulated first 10 zeros
#   to 16 digits (matches IEEE double-precision tables, LMFDB,
#   Odlyzko's tabulated values, and OEIS A058303)
# - SHA-256 hashed for reviewer verification
#
# Reproducibility: identical PARI/GP version produces identical output
# byte-for-byte. The expected hash is recorded in README.md.
#
# Author: Ronnie Andrews, Jr. (Team Xcelerator Inc.(R))
set -euo pipefail

N_ZEROS=${1:-1000}
DIGITS=${2:-1000}
OUTPUT=${3:-data/zeta_zeros.json}

# --- Tool check ---------------------------------------------------------
if ! command -v gp > /dev/null; then
  echo "ERROR: PARI/GP not found."
  echo "Install:  sudo apt install pari-gp"
  exit 1
fi
if ! command -v python3 > /dev/null; then
  echo "ERROR: python3 not found."
  exit 1
fi
if ! command -v sha256sum > /dev/null; then
  echo "ERROR: sha256sum not found."
  exit 1
fi

PARI_VERSION=$(gp --version-short 2>/dev/null || gp --version 2>&1 | grep -m1 'Version' | head -1)
echo "==> Tool versions"
echo "    PARI/GP:    $PARI_VERSION"
echo "    Python:     $(python3 --version)"
echo "    sha256sum:  $(sha256sum --version | head -1)"
echo

# --- Generate zeros via PARI's lfunzeros --------------------------------
# `lfunzeros(L, H)` returns zeros up to height H. To get the first N
# zeros we estimate H from N via Riemann's asymptotic counting formula:
#   N(T) ≈ (T/2π) · log(T/(2πe))
# Inverting gives the height of the N-th zero. We use 1.4× margin to
# safely overshoot, then truncate.
echo "==> Generating $N_ZEROS zeros at $DIGITS-digit precision"
mkdir -p "$(dirname "$OUTPUT")"

TMP_RAW=$(mktemp)
trap 'rm -f $TMP_RAW' EXIT

gp -q << EOF > "$TMP_RAW"
default(realprecision, $DIGITS);
default(parisize, "1G");

N = $N_ZEROS;
denom = log(max(N / (2*Pi*exp(1)), 1.5));
H = ceil(1.4 * 2*Pi*N / denom);
if(H < 50, H = 50);

zs = lfunzeros(lfuncreate(1), H);
if(length(zs) < N, error(Strprintf("only %d zeros below height %d; need %d", length(zs), H, N)));
for(i=1, N, print(zs[i]));
quit;
EOF

# --- Convert to JSON array of strings -----------------------------------
echo "==> Converting to JSON array of decimal strings"
python3 - "$TMP_RAW" "$OUTPUT" "$DIGITS" "$N_ZEROS" << 'PY'
import json, sys
raw_path, json_path, expected_digits, expected_n = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4])

with open(raw_path) as f:
    zeros = [line.strip() for line in f if line.strip()]

if len(zeros) != expected_n:
    print(f"ERROR: expected {expected_n} zeros, got {len(zeros)}", file=sys.stderr); sys.exit(1)
for i, z in enumerate(zeros):
    if not z[0].isdigit() or '.' not in z:
        print(f"ERROR: zero {i+1} malformed: {z[:80]!r}", file=sys.stderr); sys.exit(1)

median_len = sorted(len(z) for z in zeros)[len(zeros)//2]
if median_len < expected_digits - 5:
    print(f"ERROR: median string length {median_len} < expected {expected_digits}",
          file=sys.stderr); sys.exit(1)

with open(json_path, 'w') as f:
    json.dump(zeros, f, indent=0, ensure_ascii=True)

print(f"    captured {len(zeros)} zeros, median length {median_len} chars")
PY

# --- Cross-validate first 10 against universally-tabulated values -------
# These 16-digit values match IEEE double-precision tables, LMFDB,
# Odlyzko's published tables, and OEIS A058303. They are reproduced in
# many independent references and serve as a sanity check that we are
# computing the right zeros in the right order.
echo "==> Cross-validating first 10 zeros vs universally-tabulated values"
python3 - "$OUTPUT" << 'PY'
import json, sys
# Universally-tabulated first 10 zero imaginary parts (16 sig figs).
# These match Odlyzko's published tables, LMFDB, OEIS A058303, and
# any standard reference. Compared numerically (within 1e-12) so that
# decimal truncation vs rounding in the source data does not cause
# spurious mismatches.
expected = [
    14.134725141734693,
    21.022039638771554,
    25.010857580145687,
    30.424876125859513,
    32.935061587739190,
    37.586178158825675,
    40.918719012147495,
    43.327073280914999,
    48.005150881167160,
    49.773832477672300,
]
TOL = 1e-12
with open(sys.argv[1]) as f:
    zeros = json.load(f)
mismatches = 0
for i, ref in enumerate(expected):
    got = float(zeros[i][:25])  # 25 chars is well within f64 range
    diff = abs(got - ref)
    rel = diff / ref
    if rel > TOL:
        print(f"MISMATCH zero {i+1}:")
        print(f"  expected:    {ref:.15f}")
        print(f"  got:         {got:.15f}")
        print(f"  abs diff:    {diff:.3e}")
        print(f"  rel diff:    {rel:.3e}  (tolerance {TOL:.0e})")
        mismatches += 1
if mismatches:
    print(f"\nFAILED: {mismatches} of 10 zeros disagree with tabulated values.")
    sys.exit(1)
print("    all 10 first zeros agree with tabulated values to relative tolerance 1e-12")
PY

# --- SHA-256 hash -------------------------------------------------------
HASH=$(sha256sum "$OUTPUT" | awk '{print $1}')
SIZE=$(stat -c%s "$OUTPUT" 2>/dev/null || stat -f%z "$OUTPUT")

echo
echo "==> Done"
echo "    File:          $OUTPUT"
echo "    Size:          $SIZE bytes"
echo "    SHA-256:       $HASH"
echo "    PARI version:  $PARI_VERSION"
echo
echo "Record the SHA-256 in README.md so reviewers can verify reproducibility."
