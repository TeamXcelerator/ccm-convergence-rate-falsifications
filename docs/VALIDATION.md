# v2.1 software validation

The harness pins Xcelerator Toolkit v0.17.0 at `382d1c1c341ef39368f112b1875df882b0ce808a`.
The [machine-readable record](validation/v2.1.json) binds this source and
retains preceding qualification under `prior_qualification`.

| Check | Current result |
|---|---|
| Default Rust suite | 9 passed |
| HP/Arb Rust suite | 27 passed |
| Script-contract suite | 16 passed |
| Clippy, all targets, default and Arb | Passed |
| Formatting | Passed |
| Source-bound Arb release executable | Locked source and complete-root capability matched |
| Real-binary admission | 16 configurations passed; no numerical computation |

Root responses reuse directed geometry at each reached guard precision.
The cache retains the same reciprocal endpoints that interval division already
computes, preserves guard order and rounding acceptance, and releases optional
storage under admission pressure. Experiment definitions, mathematical artifact
identities and manuscript claims are unchanged. Build provenance and measurement
receipts record the new dependency revision.

Toolkit qualification includes 122 passing response tests in each of the HP
release and Arb lanes. Twelve existing explicit GPU/benchmark tests per lane
were not invoked. Full response JSON comparisons use the prior scalar oracle;
additional checks cover reuse, refresh, cancellation, guard exhaustion and
reservation fallback. A retained C=90000, N=300, 3386-bit artifact replay checked
2400 root responses across eight events against both the prior evaluator and
the saved decimal strings, with exact equality. This is bounded root-kernel
evidence, not a whole-claim or multi-GPU speed measurement.

Consumer checks use Rust 1.98.0. The Arb binary links FLINT 3.0.1, GMP 6.3.0
and MPFR 4.2.1; the Toolkit's bundled HP build uses MPFR 4.2.2. GPU kernels
are unchanged. See [retesting](RETESTING.md) for run configuration and the
executable's `--build-info` output for exact producer provenance.
