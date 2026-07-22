# 2026-07-22 Interface Compatibility Window Correction

The operator clarified the deployment requirement after card 099: a consuming
application release may remain installed while client devices carry harness
versions spanning six months or more. Swallowtail therefore cannot model
support as only the upstream version current when the application ships.

Contract 029 now distinguishes two records. The configured instance and
immutable plan bind the exact installed interface version. The driver declares
a maintained support window for that axis: oldest supported baseline,
latest-qualified boundary, ordered behavior milestones, maintained or
deprecated status per segment, and exact exclusions. Preflight classifies the
exact point and rejects versions outside the window without fallback.

Core realizes the same shape. Semantic, integer, and calendar-date axes support
ordered windows. Opaque axes remain exact-only. A match returns an adapter-
private behavior revision and visible support status, allowing one public
operation surface to retain older protocol behavior without accumulating
provider shims in shared core. Driver descriptors may carry independent claims
for multiple version axes.

The provider-neutral harness RPC fixture proves a deprecated baseline segment,
a maintained milestone segment, an excluded release, and versions below and
above the support window. Pi `0.80.10` remains the initial one-point production
claim until later releases have frozen corpus and conformance evidence.

Focused core, runtime, testkit, and Pi validation passes 129 tests. Focused
warnings-denied clippy, workspace all-target check, docs QA, explicit index
links, formatting, and diff checks pass. Doctor returns the inherited 19
oversized-file findings, with no correction-added finding.

Moving a baseline is now an explicit Swallowtail release change. Deprecation
does not itself remove support. Card 100 remains the next task.
