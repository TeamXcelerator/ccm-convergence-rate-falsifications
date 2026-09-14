# Research evidence and interpretation

[Repository overview](../README.md) · [Reproduction guide](RETESTING.md) ·
[Individual scripts](../scripts/README.md) · [Validation](VALIDATION.md)

The software retains the measurements needed to assess a finite hypothesis,
including negative results, missing indices and unresolved source precision.
The software does not convert these outcomes into an asymptotic theorem.
The manuscript and software have separate version histories. Each numerical
run retains its own source identities, measurements and assessed scope.

## Match the question to the observable

| Family | What the run can assess | Boundary of the evidence |
|---|---|---|
| Weil/prolate approximation | Least-squares and discrete minimax residuals on the stated comparison grid | A sampled maximum is not a certified continuous supremum. This comparison does not test CCM Lemma 7.2, whose individual h0/h4 observables and normalization are not supplied by this experiment. |
| Indexed root errors | Per-index absolute/relative errors and declared window aggregates for the retained finite source | A missing, duplicate, unordered or unconverged root prevents a complete-window verdict. A bounded prefix cannot decide an all-N assertion. |
| Transform comparison | Both complex components at localized real-part crossings, with quadrature refinement | A real-part crossing alone is not a zero of the complex transform. The finite Fourier/secular control applies when a CCM source is present. |
| Supplemental capture | Source-bound sector, conditioning, response, distance and prefix diagnostics | Capture completeness, arithmetic precision and interval certification are separate properties. |

Independent acquisition isolates the positive movable roots of the retained
even point source without reference ordinates. Reference values enter the
subsequent error comparison. Reference-seeded refinement is available as a
separately recorded policy. Neither policy silently supplies a missing root
from the reference list. See [complete spectral windows](RETESTING.md#complete-spectral-windows).

## Read the three result layers

1. **Run integrity:** were the required operations and primary measurements
   valid and saved? An execution or required publication failure makes the
   run incomplete.
2. **Finite hypotheses:** did each named comparison pass or fail? A valid
   experiment may refute its finite hypothesis. `UNASSESSED` retains questions
   the evidence cannot decide.
3. **Capture completeness:** did every applicable requested diagnostic finish?
   An unsupported export has a reason and is distinguished from a failed
   applicable export. `--require-complete-capture` makes applicable
   incompleteness affect the command's exit status.

A zero exit status alone is not a claim that every hypothesis passed or that
all requested supplemental evidence completed. Inspect the categories in
`summary.json`, and `capture.json` when present. The launcher prints this
assessment and saves `assessment.json` for the invocation.

## Retained files

Each invocation directory contains `invocation.log`, an aggregate
`assessment.json` when assessment succeeds, and one or more unique `run-*`
subdirectories. Each run can contain:

| File | Contents |
|---|---|
| `request.json` | Inputs, resolved configuration and requested numerical/root policies |
| `build.json` | Harness version, Toolkit revision, source implementation digest, dependency lock and enabled capabilities |
| `primary.json`, `primary-sources.json` | CCM primary results and source identities, when a CCM source is needed |
| `measurements.json` | Sample pairs or per-index observations, errors/residuals, numerical controls and finite hypothesis checks |
| `measurement-receipt.json` | Managed retention of measurements using `research_capture_receipt` |
| `capture-plan.json` | Requested diagnostics, source identities, grids and explicit exclusions |
| `capture.json` | Supplemental capture outcomes and artifact references |
| `summary.json` | Run integrity, capture status, individual checks, elapsed time and measurement retention |
| `failure.json` | Failure detail when a required operation fails |

Primary measurements are written before supplemental capture and publication.
An interrupted or failed run can therefore retain useful measurements without
being a completed run. Preserve its original status when reassessing:

```bash
python3 scripts/summarize_runs.py /absolute/path/to/claim-invocation
```

The optional `--output /absolute/path/to/new-assessment.json` writes a new
aggregate file and refuses to overwrite an existing one. Reassessment reads
the saved journals; it does not perform new numerical validation or change
original receipts.

## Ultra capture and controls

Individual scripts default to `--research-capture ultra`; the direct executable
defaults to `claim`. Select `claim`, `research`, `gap`, `maximum` or `ultra`
explicitly when comparing capture policies. The plan saved for the run is the
authoritative record of requested and excluded diagnostics.

Ultra requests sector spectra and selected eigenpairs, evenness, root
conditioning, distance/profile/resolution and target-residual analyses,
deviation decomposition, prime-power and u-flow responses, a prefix ladder
with checkpoint eigenstates, and a retained-reduction check where supported.

| Control | Default / effect |
|---|---|
| `--research-sector-eigenpairs` | 8 requested vectors, capped at N by the harness |
| `--capture-prefix-checkpoints` | Additional comma-separated even-sector dimensions; the final N+1 checkpoint is retained |
| `--capture-working-precision-bits` | Prefix and retained-reduction working precision; defaults to source precision and is validated against it |
| `--capture-grid-resolution` | 4000 for supplemental distance quadrature |
| `--capture-profile-steps` | 1000 for supplemental distance profiles |
| `--capture-reduction-max-dimension` | 8193; bounds the retained-reduction request |
| `--require-complete-capture` | Makes incomplete applicable capture fail the invocation |

Capture settings do not change the primary C, N, P or root policy. They can
change supplemental diagnostic identities and runtime. Keep them in the
provenance of comparisons between runs.

Target diagnostics require a readable JSON specification file selected through
`XC_TARGET_SPEC_FILE`. The file's definition digest binds target-derived
artifacts; a different target defines a different comparison. A missing or
invalid target is recorded without erasing saved primary measurements.

A naive-transform-only run has no CCM source, so CCM capture is unsupported;
its complex measurements and quadrature evidence are retained. Historical
adaptive-even sources exclude even-sector-only response and checkpoint
exports. The C=1000, N=800, P=1000 source-resolution exclusion and the
higher-precision comparison procedure are documented in
[Ultra applicability](RETESTING.md#ultra-applicability).

Root interval certificates and sector-gap proof artifacts are separate
assurance tasks. Ultra does not imply they were requested or proved.

## Reuse and historical comparisons

Compatible inputs are reused under their content-bound identities. Missing
inputs are computed locally, and changed semantic identities produce new
artifacts. An incompatible identity by itself is not evidence that stored
bytes were corrupt. Existing measurements and journals remain historical
evidence under their original source, policy, precision and validation scope.

When comparing results, join the C,N,P configuration, source and target
identities, root acquisition policy, index window, numerical profile and
observable definition. In particular, keep least-squares and minimax residuals,
mean and maximum errors, trimmed and full windows, and real-part crossings
and complex residuals distinct. Do not replace an unresolved measurement with
zero error or silently omit it from the requested window.
