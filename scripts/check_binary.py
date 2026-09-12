#!/usr/bin/env python3
"""Reject stale or incomplete prebuilt binaries before starting a claim."""
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

SOURCES = ("src/main.rs", "src/journal.rs", "src/source.rs", "src/experiments.rs",
           "src/transforms.rs", "src/numerics.rs", "Cargo.toml", "Cargo.lock")

def expected_digest(repo):
    texts = [(repo / name).read_text(encoding="utf-8").replace("\r\n", "\n") for name in SOURCES]
    return hashlib.sha256(json.dumps(texts, ensure_ascii=False, separators=(",", ":")).encode()).hexdigest()

def validate(repo, info):
    revisions = set(re.findall(r'rev\s*=\s*"([0-9a-f]{40})"', (repo / "Cargo.toml").read_text(encoding="utf-8")))
    if len(revisions) != 1 or info.get("toolkit_revision") not in revisions:
        raise ValueError("the executable uses a different toolkit revision")
    if info.get("schema_version") != 1 or not info.get("complete_positive_roots"):
        raise ValueError("the executable lacks complete-range Arb root discovery")
    if info.get("implementation_digest") != expected_digest(repo):
        raise ValueError("the executable does not match these sources and Cargo.lock")

def main():
    repo, binary = Path(sys.argv[1]), sys.argv[2]
    try:
        result = subprocess.run([binary, "--build-info"], check=True, text=True, capture_output=True)
        validate(repo, json.loads(result.stdout))
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        print("[INCOMPLETE] Binary verification failed: " + str(error), file=sys.stderr)
        print("Unset BIN and rerun the individual script to build the current locked source with Arb.", file=sys.stderr)
        return 2
    print("[PASS] executable matches the locked source and complete-root capability")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
