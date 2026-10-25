#!/usr/bin/env bash
# Launcher regression checks with synthetic journals. No numerical work or publication.
set -euo pipefail
repo=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
temp=$(mktemp -d)
trap 'rm -rf -- "$temp"' EXIT
export TEST_BUILD_INFO="$temp/build.json" TEST_CALLS="$temp/calls"
export BIN="$temp/executable" CLAIM_RUN_ROOT="$temp/run parent"
unset PREC RESEARCH_CAPTURE ROOT_ACQUISITION NUMERICAL_PROFILE
export XC_CLAIM_MEMORY_GIB=32
export XC_PUBLISH_TARGET=both XC_PUBLISH_EXECUTE=true
python3 - "$repo" "$TEST_BUILD_INFO" <<'PY'
import json, re, sys
from pathlib import Path
repo = Path(sys.argv[1])
sys.path.insert(0, str(repo / 'scripts'))
from check_binary import expected_digest
revision = re.search(r'rev\s*=\s*"([0-9a-f]{40})"', (repo/'Cargo.toml').read_text()).group(1)
Path(sys.argv[2]).write_text(json.dumps(dict(schema_version=1, complete_positive_roots=True,
    toolkit_revision=revision, implementation_digest=expected_digest(repo))))
PY
cat > "$BIN" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
case "${1-}" in
  --version) echo 'ccm-falsifications 2.1.0'; exit ;;
  --build-info) cat "$TEST_BUILD_INFO"; exit ;;
esac
printf '%s|%s\n' "$XC_PUBLISH_TARGET" "$XC_PUBLISH_EXECUTE" >> "$TEST_CALLS"
validate=false
output=
while (($#)); do
  case "$1" in
    --validate-only) validate=true ;;
    --capture-output) output=$2; shift ;;
    --capture-output=*) output=${1#*=} ;;
  esac
  shift
done
if [[ $validate == true ]]; then
  echo '{"configuration_admitted":true}'
  exit "${TEST_VALIDATE_EXIT:-0}"
fi
[[ -n $output ]]
if [[ ${TEST_NO_JOURNAL:-0} != 1 ]]; then
  mkdir -p "$output/run-synthetic"
  printf '{}\n' | tee "$output/run-synthetic/request.json" > "$output/run-synthetic/build.json"
  cat > "$output/run-synthetic/summary.json" <<'JSON'
{"schema_version":1,"measurements_saved":true,"run_status":"PASS","capture_status":"PASS","checks":[{"name":"synthetic finite comparison","category":"hypothesis","status":"FAIL","reason":"intentional negative finite result"}]}
JSON
  cat > "$output/run-synthetic/measurements.json" <<'JSON'
{"observable":"synthetic launcher fixture","values":{},"checks":[]}
JSON
fi
exit "${TEST_RUN_EXIT:-0}"
SH
chmod +x "$BIN"
checks=0
pass() { checks=$((checks+1)); printf '[PASS] %s\n' "$1"; }
run() { bash "$repo/scripts/$1" "${@:2}" > "$temp/output" 2>&1; }
reject() {
  if run "$@"; then
    cat "$temp/output" >&2
    echo 'Unexpected launcher success' >&2
    exit 1
  fi
}

# Help must bypass executable and dependency checks, wherever it occurs.
saved_bin=$BIN
export BIN=/nonexistent/ccm-falsifications
run claim1a_lambda13_grid4001.sh --precision-digits 1000 --help
[[ ! -e $TEST_CALLS && ! -e $CLAIM_RUN_ROOT ]]
pass 'help after arguments needs no executable or journal'
export BIN=$saved_bin

run claim1a_lambda13_grid4001.sh --validate-only
[[ ! -e $CLAIM_RUN_ROOT && $(tail -n 1 "$TEST_CALLS") == 'none|false' ]]
pass 'validation exits successfully without a journal and disables inherited publication'

export TEST_VALIDATE_EXIT=7
reject claim1a_lambda13_grid4001.sh --validate-only
[[ ! -e $CLAIM_RUN_ROOT ]]
unset TEST_VALIDATE_EXIT
pass 'configuration refusal propagates without starting a journal'

run claim1a_lambda13_grid4001.sh --validate-only --publish
[[ $(tail -n 1 "$TEST_CALLS") == 'both|true' ]]
run claim1b_lambda100_grid4001.sh --validate-only --publish
[[ $(tail -n 1 "$TEST_CALLS") == 'private|true' ]]
pass 'explicit publication selects the intended policy (stub only)'

reject claim1a_lambda13_grid4001.sh --capture-output
reject claim1a_lambda13_grid4001.sh --capture-output=
[[ ! -e $CLAIM_RUN_ROOT ]]
pass 'missing output directory arguments fail before computation'

run claim1a_lambda13_grid4001.sh
run claim1a_lambda13_grid4001.sh
mapfile -t assessments < <(find "$CLAIM_RUN_ROOT" -name assessment.json)
[[ ${#assessments[@]} == 2 ]]
pass 'negative finite hypotheses retain successful, distinct journals under a path with spaces'

export TEST_RUN_EXIT=9
reject claim1a_lambda13_grid4001.sh
unset TEST_RUN_EXIT
pass 'computation failure cannot be hidden by a successful fixture summary'

export TEST_NO_JOURNAL=1
reject claim1a_lambda13_grid4001.sh
pass 'missing computation journals cannot report success'
printf 'Launcher regression checks: %s passed. No numerical computation or publication.\n' "$checks"
