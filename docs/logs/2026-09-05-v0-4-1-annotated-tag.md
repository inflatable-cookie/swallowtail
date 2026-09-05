# v0.4.1 Annotated Tag And Post-Release Runway

Date: 2026-09-05
Roadmap: `../roadmaps/g05/030-v0-4-1-release-readiness.md`
Cards: `../roadmaps/g05/batch-cards/091-v0-4-1-candidate-preparation-and-exact-sha-ci.md`, `../roadmaps/g05/batch-cards/092-v0-4-1-consumer-proof-and-operator-tag-gate.md`

## Result

Operator authorization created and pushed annotated tag `v0.4.1` at merged
source SHA `c3cce7504ffd5eae138a0190f1cd81332db68c3c`. The tag object is
`c888b2dc1a968d8dda66a99da1bb5fd51067df58`; local and remote peels agree.
The source-only release has no crates.io publication, GitHub Release, binary,
sidecar, installer, provider mutation, or consumer-repository mutation.

No application had driven the candidate before the tag.

## Verification

Merged-SHA workflow-dispatch run
https://github.com/inflatable-cookie/swallowtail/actions/runs/33969131592
passed all six jobs. `effigy package:source-consumer` passed from a clean
detached checkout at the merged SHA.

## Reconciled Surfaces

The release note, releases index, root README, Contract 036 tagged identity,
g05.030 and cards 091-092, roadmap indexes, and this log now describe the
tagged state. The feature freeze is lifted. PR 227 remains open as deferred
post-tag fixture-sweep work and was not part of this release.

## Next

Ask the operator for the next runway decision.
