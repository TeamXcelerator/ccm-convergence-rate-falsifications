#!/usr/bin/env bash
# Shared launcher. Each computation keeps a fresh journal and assessment.
CLAIM_REPO_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
cd "$CLAIM_REPO_ROOT"

claim_init() {
  CLAIM_ARGS=()
  CLAIM_HELP=false
  CLAIM_VALIDATE_ONLY=false
  local publish=false arg
  for arg in "$@"; do
    case "$arg" in
      --publish) publish=true ;;
      --help|-h) CLAIM_HELP=true; CLAIM_ARGS+=("$arg") ;;
      --validate-only) CLAIM_VALIDATE_ONLY=true; CLAIM_ARGS+=("$arg") ;;
      *) CLAIM_ARGS+=("$arg") ;;
    esac
  done
  if [[ $CLAIM_HELP == true ]]; then
    echo "Usage: bash ${BASH_SOURCE[1]} [binary options for this experiment]"
    echo 'Defaults: current numerical profile, independent roots, Ultra capture.'
    echo 'Options override script defaults. BIN selects a prebuilt executable.'
    echo 'CLAIM_RUN_ROOT selects the journal parent. Existing runs are preserved.'
    echo '--validate-only checks configuration without starting a computation or journal.'
    echo '--publish enables configured artifact publication; it is off by default.'
    echo 'See scripts/README.md and docs/RETESTING.md for inputs and exact configurations.'
    return 0
  fi
  command -v python3 >/dev/null || {
    echo 'Python 3 is required for executable checks and journal summaries.' >&2; exit 2;
  }
  local memory=${XC_CLAIM_MEMORY_GIB:-32}
  [[ $memory =~ ^[1-9][0-9]{0,9}$ ]] && ((memory <= 8589934591)) || {
    echo 'XC_CLAIM_MEMORY_GIB must be an integer in 1..8589934591' >&2; exit 2;
  }
  export XC_RESEARCH_WORKING_BYTES=$((memory << 30))
  export XC_PUBLISH_TARGET=none XC_PUBLISH_EXECUTE=false
  if [[ $publish == true ]]; then
    export XC_RUN_PROFILE=author XC_PUBLISH_EXECUTE=true XC_PUBLISH_REPLACE=false
    case "${BASH_SOURCE[1]##*/}" in
      claim1a_lambda13_grid4001.sh) export XC_PUBLISH_TARGET=both ;;
      *) export XC_PUBLISH_TARGET=private ;;
    esac
  fi
  echo "Claim working budget: $memory GiB; publication: $XC_PUBLISH_TARGET"
  if [[ -z ${BIN+x} ]]; then
    command -v cargo >/dev/null || {
      echo 'Cargo is unavailable. Load your Rust environment or set BIN to a matching executable.' >&2; exit 2;
    }
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
  if [[ $CLAIM_HELP == true ]]; then
    return 0
  fi
  local -a command=("$BIN" --numerical-profile "${NUMERICAL_PROFILE:-current}"
    --root-acquisition "${ROOT_ACQUISITION:-independent}"
    --capture-maximum-stream-scalar-bytes "${XC_CLAIM_STREAM_SCALAR_BYTES:-274877906944}"
    --research-capture "${RESEARCH_CAPTURE:-ultra}" "$@" "${CLAIM_ARGS[@]}")
  if [[ $CLAIM_VALIDATE_ONLY == true ]]; then
    "${command[@]}"
    return
  fi
  local parent=${CLAIM_RUN_ROOT:-"$CLAIM_REPO_ROOT/claim-runs"}
  local output arg next=0
  # Explicit capture-output remains supported; unique invocation folder prevents mixing.
  for arg in "${CLAIM_ARGS[@]}"; do
    if ((next)); then
      [[ -n $arg && $arg != -* ]] || { echo '--capture-output requires a directory path.' >&2; return 2; }
      parent=$arg; next=0
    fi
    case "$arg" in --capture-output) next=1 ;; --capture-output=*) parent=${arg#*=} ;; esac
  done
  [[ $next == 0 && -n $parent ]] || { echo '--capture-output requires a directory path.' >&2; return 2; }
  mkdir -p -- "$parent"
  output=$(mktemp -d -- "$parent/claim-$(date -u +%Y%m%dT%H%M%SZ)-XXXXXX")
  local -a statuses
  set +e
  "${command[@]}" --capture-output "$output" 2>&1 | tee "$output/invocation.log"
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
