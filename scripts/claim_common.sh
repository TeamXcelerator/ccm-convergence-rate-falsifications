#!/usr/bin/env bash
# Shared zero-configuration launcher for independently runnable Paper 2 claims.

CLAIM_REPO_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
cd "$CLAIM_REPO_ROOT"

NUMERICAL_PROFILE=${NUMERICAL_PROFILE:-paper}

claim_init() {
  while (($# > 0)); do
    case "$1" in
      --numerical-profile)
        if (($# < 2)); then
          echo "--numerical-profile requires paper or current" >&2
          exit 2
        fi
        NUMERICAL_PROFILE=$2
        shift 2
        ;;
      --help|-h)
        echo "Usage: bash ${BASH_SOURCE[1]} [--numerical-profile paper|current]"
        echo "  paper   original adaptive-even/legacy-solver route with the current precision contract (default)"
        echo "  current optimized even-sector/Auto toolkit route"
        exit 0
        ;;
      *)
        echo "Unknown claim-script argument: $1" >&2
        echo "Use --help for supported options." >&2
        exit 2
        ;;
    esac
  done

  case "$NUMERICAL_PROFILE" in
    paper|current)
      ;;
    *)
      echo "NUMERICAL_PROFILE must be paper or current" >&2
      exit 2
      ;;
  esac

  if [[ -z "${BIN+x}" ]]; then
    CLAIM_TARGET_DIR=${CARGO_TARGET_DIR:-"$CLAIM_REPO_ROOT/target"}
    BIN="$CLAIM_TARGET_DIR/release/ccm-falsifications"
    cargo build --quiet --release --features hp --locked \
      --bin ccm-falsifications \
      --target-dir "$CLAIM_TARGET_DIR"
  elif [[ ! -x "$BIN" ]]; then
    echo "Configured reproduction binary is not executable: $BIN" >&2
    exit 1
  fi

  echo "Numerical profile: $NUMERICAL_PROFILE"
}

claim_run() {
  "$BIN" --numerical-profile "$NUMERICAL_PROFILE" "$@"
}
