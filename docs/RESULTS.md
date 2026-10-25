# Results and evidence scope

[Overview](../README.md) · [Scripts](../scripts/README.md) · [Reproduction](RETESTING.md)

The November 3, 2026 manuscript uses corrected finite measurements and
independent checks described below. The machine-readable
[table data](manuscript-results.json) retain their source bindings and scope.
This guide summarizes the evidence; it is not a new numerical run or a fresh
audit of every remotely stored artifact.

## Completed experiments

All 13 named experiments and the three additional standard configurations have
retained completion evidence for required measurements, applicable diagnostic
acquisition, and configured publication. Some completed through supplemental
recovery; the original incomplete journals retain their original status.
The [script catalog](../scripts/README.md) gives every configuration and the
precision needed to reproduce each standard row.

The CCM runs used Ultra capture with sector-gap certificates off.
Cross-configuration comparison and dimension/precision-budget diagnostics were
deferred. The naive-only Mellin experiment has no CCM source, so its CCM
captures are inapplicable. Claim 1a has public and private artifact coverage;
the other experiments have private coverage. This public repository retains
the manuscript data and reproduction software, rather than every bulk artifact.

## What the results establish

**Weil/prolate approximation.** The four profile comparisons concern a finite
prolate discretization and sampled residuals. They show close finite agreement
and do not falsify CCM Lemma 7.2. An approximation on a finite grid is not a
uniform or asymptotic approximation theorem.

**Spectral error.** All seven lambda=N spectra have independent directed
interval checks of their 1,700 positive roots. Degree counting establishes
completeness for the retained polynomial, and the rank-one determinant identity
connects those roots to the saved finite operator spectrum used by the
conjecture. Full maxima, logarithmically scaled maxima, and maxima after
excluding the highest 20% of roots grow across the tested rows. This is evidence
against uniform inverse-logarithmic maximum-error decay in that range.
Coefficient-derived lower bounds on full-window maximum error rise from 525
to 8,860; independent interval calculations reconcile the spectral moments
with the root brackets.

These checks concern the saved finite sources. Analytic matrix-assembly and
ground-state uncertainty have not been propagated into the error comparison,
and the finite trend does not determine an asymptotic limit. The earlier
mean-error argument is withdrawn. The corrected standard C=13 and C=100
full-window means have independent finite-source checks; the precise cause of
the historical discrepancies has not been fully reconstructed.

**Auxiliary Mellin transforms.** The 32 observed real-part crossings in Test 3a
and 37 in Test 3b fail the complex-zero residual test. Those are valid negative
finite results. The manuscript separately proves an eventual absence of
critical-line zeros for the tested fixed hard cutoffs. The proof concerns
these auxiliary transforms, not CCM's different transform or a cutoff that
grows with height.

## Read three kinds of status

| Status | Meaning |
|---|---|
| PASS | The named finite check or required operation passed. |
| FAIL | The named hypothesis failed its stated comparison; read the category and measured reason. |
| INCOMPLETE | Required evidence or an applicable diagnostic is missing, invalid, or unresolved. |
| UNASSESSED | The evidence does not decide the stated question. |
| UNSUPPORTED | The question or diagnostic lies outside the applicable source or experiment domain. |

**Run integrity** reports whether required measurements were produced and
validated. **Hypothesis outcomes** report what those measurements say about a
specified finite comparison. **Capture completeness** reports acquisition of
applicable supplemental diagnostics. These are separate assessments.

A completed capture can contain conditional bounds, finite-interpolant scope,
or partially unresolved numerical rows. Inspect those qualifications within
the artifact. Use `--require-complete-capture` to require all applicable
acquisitions for a successful invocation; this does not turn each acquired
artifact into an unconditional mathematical certificate.

## Retained files

Each computation creates a `run-*` directory inside a unique invocation folder.
Files depend on which stages completed:

| File | Purpose |
|---|---|
| `request.json`, `build.json` | Numerical configuration and executable identity |
| `primary.json`, `primary-sources.json` | Primary measurements and source identities |
| `measurements.json`, `measurement-receipt.json` | Experiment values and acquisition evidence |
| `capture-plan.json`, `capture.json` | Requested diagnostics and their outcomes |
| `input-preparation-status.json` | External input preparation and source-join status, when applicable |
| `summary.json` | Run integrity, capture completeness, and finite checks |
| `failure.json` | A retained required-computation failure, when applicable |

The wrapper also retains `invocation.log` and `assessment.json`. A supplemental
recovery preserves the original journal and identifies the retained sources it
uses. Publication coverage verifies the eligible artifact destinations; it
does not imply that every console log was uploaded.

## Compare reproductions carefully

Match C, N, precision, grid, sample count, requested root window, trimming,
numerical profile, source identities, and Toolkit revision. Keep mean and
maximum errors distinct, and report the window over which each is computed.
A different finite source, root selection, or normalization can change the
observable even when the displayed parameter names look similar.
