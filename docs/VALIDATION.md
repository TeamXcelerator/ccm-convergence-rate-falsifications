# v2.1 retest candidate validation

[Repository overview](../README.md) · [Reproduction guide](RETESTING.md) ·
[Research evidence](RESEARCH_EVIDENCE.md)

The harness pins Xcelerator Toolkit v0.17.0 at
`ea546f0d9fa134275c15d47fc461012c4f21dbc6`. The
[machine-readable record](validation/v2.1.json) binds the checked implementation,
lockfile, launchers and tests. It records local software checks, separately
from manuscript reproduction or a full Ultra campaign.

| Check | Result |
|---|---|
| WSL default Rust suite | 9 passed |
| WSL Linux HP/Arb Rust suite | 23 passed |
| Python executable-contract tests | 4 passed |
| Strict Clippy, default and HP/Arb, all targets | Passed |
| Rust formatting and individual shell script syntax | Passed |
| Real-binary admission for 16 claim configurations | Passed |
| Individual script publication routing | 14 scripts and no-publish control passed |
| Small C13/N24/P60 Ultra integration | 50/50 acquisitions, source joins and capture completeness passed |

The integration probe used two sector eigenpairs and reduced distance/profile
sampling in a debug build with CPU backends. It did not publish remotely or
qualify full claim-scale CUDA performance. Applicable acquisition completeness
does not promote qualified mathematical bounds to certificates.

The Mellin regressions compare parallel refinements against a sequential oracle at
multiple precisions and worker counts. Cold and warm measurements also match
the previous implementation exactly.

The tests cover finite experiment logic, input rejection and source-bound
execution. These counts belong to this paper's harness, not the Toolkit's
larger workspace suite. They do not establish an asymptotic convergence law,
certify spectral ordinals or validate every manuscript table.

## Reproduce the software checks

From the repository root, with the [build prerequisites](../README.md#run-one-experiment):

```bash
python3 -m unittest discover -s tests -p 'test_*.py' -v
cargo test --locked
cargo test --locked --features arb
cargo clippy --locked --all-targets -- -D warnings
cargo clippy --locked --all-targets --features arb -- -D warnings
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
