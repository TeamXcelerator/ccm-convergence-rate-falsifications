# Manuscript corrections awaiting author review

The v2.1.0 source upgrade leaves `paper.tex` and `paper.pdf` unchanged. Their
historical claims are not silently adopted as the upgraded harness's verdicts.

1. **CCM attribution.** The Weil/prolate comparison does not test the
   prolate-to-Hermite estimate in CCM Lemma 7.2. Proposed correction:
   “Our numerical study concerns the proposed approximation of the Weil
   minimizer by the E-transform of prolate functions. It does not test the
   prolate-to-Hermite estimate proved in CCM Lemma 7.2, and the reported
   computations do not constitute a counterexample to that lemma.”
   Attribution of a specific conjectured rate requires an exact source.
2. **Fit and norm.** Separate the historical least-squares scalar's sampled
   maximum error from the actual discrete minimax scalar. Neither is a
   certified continuous supremum norm. Resolve grid and source-precision
   dependence before revising the table or inferring asymptotic behavior.
3. **Spectral error.** Retain mean error, but add maximum error and its
   logarithmic products for the maximum-error conjecture. Disclose the root
   acquisition and counting scope. Incomplete windows cannot supply full-N
   aggregate evidence. Trimming is a separate prefix statistic, not a repair
   of missing roots or a change to the conjecture's observable.
4. **Mellin comparison.** Historical sign changes locate zeros of the real
   part. A complex zero also requires vanishing imaginary part. Revise the
   comparison only after retaining both components and refinement evidence;
   the exact Fourier/secular identity supplies an independent finite-state
   control for the CCM-side transform.
5. **Conclusions and title.** Decide after the controlled retests which
   numerical findings survive. The current experiments cannot establish an
   asymptotic law or contradict a lemma they do not test. Preserve useful
   finite evidence and make its scope explicit.

Primary mathematical sources:
[CCM, §7](https://arxiv.org/html/2511.22755v1#S7),
[Śliwiński, Conjecture 4.1](https://arxiv.org/html/2601.12133v1#S4).
The proposed wording and substantive manuscript changes require author review.
