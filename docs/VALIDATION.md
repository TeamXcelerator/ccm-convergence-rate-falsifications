# v2.1 software validation

[Repository overview](../README.md) · [Reproduction guide](RETESTING.md) ·
[Research evidence](RESEARCH_EVIDENCE.md)

The harness pins Xcelerator Toolkit v0.15.0 at
`5d50b5b862b075a43e7c2c9ccedd29b568a32808`. The
[machine-readable record](validation/v2.1.json) binds the checked implementation,
lockfile, launchers and tests. It records local software checks, separately
from manuscript reproduction or a full Ultra campaign.

| Check | Result |
|---|---|
| Windows default Rust suite | 8 passed |
| WSL Linux release HP/Arb Rust suite | 17 passed |
| Python executable-contract tests | 4 passed |
| Strict Clippy, default and release HP/Arb, all targets | Passed |
| Rust formatting and individual shell script syntax | Passed |
| Script help and documentation links | Passed |

The tests cover finite experiment logic, input rejection and source-bound
execution. These counts belong to this paper's harness, not the Toolkit's
larger workspace suite. They do not establish an asymptotic convergence law,
certify spectral ordinals or validate every manuscript table.

## Reproduce the software checks

From the repository root, with the [build prerequisites](../README.md#run-one-experiment):

```bash
python3 -m unittest discover -s tests -p 'test_*.py' -v
cargo test --locked
cargo test --release --locked --features arb
cargo clippy --locked --all-targets -- -D warnings
cargo clippy --release --locked --all-targets --features arb -- -D warnings
cargo fmt --all -- --check
for script in scripts/*.sh; do bash -n "$script"; done
```

The default tier works on Windows. Run the HP/Arb tier under Linux/WSL with
FLINT 3 or newer. The checks run locally and require no hosted CI execution.
A rebuilt `arb` executable can also be checked before a numerical run:

```bash
python3 scripts/check_binary.py . ./target/release/ccm-falsifications
```

## Verify the recorded source

The implementation digest uses the same LF-normalized UTF-8 source list and
JSON serialization as [check_binary.py](../scripts/check_binary.py).
Additional per-file hashes cover the launcher and test sources. To verify
those hashes without running an experiment:

```bash
python3 - <<'PY'
import hashlib, json
from pathlib import Path
record = json.loads(Path('docs/validation/v2.1.json').read_text(encoding='utf-8'))
for name, expected in record['source_sha256_lf'].items():
    content = Path(name).read_bytes().replace(b'\r\n', b'\n')
    assert hashlib.sha256(content).hexdigest() == expected, name
print('[PASS] recorded source files match')
PY
```

The record does not hash itself or assert that the unchanged manuscript was
scientifically revised. Read saved numerical journals using the
[evidence guide](RESEARCH_EVIDENCE.md); retain their original status and source
identities when comparing software releases.
