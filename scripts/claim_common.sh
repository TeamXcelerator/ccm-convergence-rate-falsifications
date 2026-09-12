#!/usr/bin/env bash
# Shared launcher. Each invocation keeps an independent journal and net summary.
CLAIM_REPO_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
cd "$CLAIM_REPO_ROOT"

claim_init() {
  CLAIM_ARGS=("$@")
  if [[ ${1:-} == --help || ${1:-} == -h ]]; then
    echo "Usage: bash ${BASH_SOURCE[1]} [binary options for this experiment]"
    echo 'Defaults: current numerical profile, independent roots, Ultra capture.'
    echo 'Options override script defaults. BIN selects a prebuilt executable.'
    echo 'CLAIM_RUN_ROOT selects the journal parent. Existing runs are preserved.'
    return 0
  fi
  if [[ -z ${BIN+x} ]]; then
    if ! command -v pkg-config >/dev/null || ! pkg-config --atleast-version=3 flint; then
      echo '[INCOMPLETE] Complete root discovery requires FLINT 3 or newer.' >&2
      echo 'On Ubuntu 24.04: apt-get install libflint-dev pkg-config' >&2
      exit 2
    fi
    CLAIM_TARGET_DIR=${CARGO_TARGET_DIR:-"$CLAIM_REPO_ROOT/target"}
    BIN="$CLAIM_TARGET_DIR/release/ccm-falsifications"
    cargo build --quiet --release --features arb --locked --bin ccm-falsifications --target-dir "$CLAIM_TARGET_DIR"
  fi
  [[ -x $BIN ]] || { echo "Binary is not executable: $BIN" >&2; exit 2; }
  [[ $("$BIN" --version) == 'ccm-falsifications 2.1.0' ]] || { echo 'Wrong harness version; rebuild the locked source.' >&2; exit 2; }
  python3 "$CLAIM_REPO_ROOT/scripts/check_binary.py" "$CLAIM_REPO_ROOT" "$BIN"
}

claim_run() {
  if [[ ${CLAIM_ARGS[0]:-} == --help || ${CLAIM_ARGS[0]:-} == -h ]]; then
    if [[ -n ${BIN:-} && -x $BIN ]]; then "$BIN" "$1" --help; fi
    return 0
  fi
  local parent=${CLAIM_RUN_ROOT:-"$CLAIM_REPO_ROOT/claim-runs"}
  local output arg next=0
  # Explicit capture-output remains supported; unique invocation folder prevents mixing.
  for arg in "${CLAIM_ARGS[@]}"; do
    if ((next)); then parent=$arg; next=0; fi
    case "$arg" in --capture-output) next=1 ;; --capture-output=*) parent=${arg#*=} ;; esac
  done
  mkdir -p "$parent"
  output=$(mktemp -d "$parent/claim-$(date -u +%Y%m%dT%H%M%SZ)-XXXXXX")
  local -a statuses
  set +e
  "$BIN" --numerical-profile "${NUMERICAL_PROFILE:-current}" \
    --root-acquisition "${ROOT_ACQUISITION:-independent}" \
    --research-capture "${RESEARCH_CAPTURE:-ultra}" "$@" "${CLAIM_ARGS[@]}" \
    --capture-output "$output" 2>&1 | tee "$output/invocation.log"
  statuses=("${PIPESTATUS[@]}")
  set -e
  local summary_rc=0
  python3 "$CLAIM_REPO_ROOT/scripts/summarize_runs.py" "$output" --output "$output/assessment.json" || summary_rc=$?
  echo "Measurements and summary: $output"
  if ((statuses[0] || statuses[1] || summary_rc)); then
    echo '[INCOMPLETE] invocation, logging or evidence validation failed; all available output retained.' >&2
    return 1
  fi
}
