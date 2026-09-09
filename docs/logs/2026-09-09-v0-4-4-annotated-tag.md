# v0.4.4 Annotated Tag And Post-Tag Runway

Date: 2026-09-09
Roadmap: `../roadmaps/g05/036-v0-4-4-release-readiness.md`
Cards: Card 154 (final candidate preparation), Desktop Card 323 (exact-tree acceptance)

## Result

Operator authorization created and pushed annotated tag `v0.4.4` at merged
source SHA `49c9e3b291609c9ebf5b35a284c08302f3b8d5e3` (tree
`1a9db12742b68e839dfebe42f0633f5d1aa0265b`). The tag object is
`41da6c1afe60380d248275bd0780c2e8f79e5ab9`; local and remote refs agree and
both peel to the exact candidate, and the annotation bytes equal the approved
text. The source-only release has no crates.io publication, GitHub Release,
binary, sidecar, installer, provider mutation, or consumer-repository
mutation.

## Verification

Exact-SHA push run
https://github.com/inflatable-cookie/swallowtail/actions/runs/34350208617
passed all 11 jobs, including Pinned MSRV floor and Pinned MSRV floor tests.
Tag-triggered CI run
https://github.com/inflatable-cookie/swallowtail/actions/runs/34372646295
completed `success` with all 11 jobs green, including Pinned MSRV floor and
Pinned MSRV floor tests.

## Reconciled Surfaces

The release note, releases index, Contract 036 tagged identity, g05.036,
roadmap and generation front doors, and this log now describe the tagged
state. The feature freeze is lifted. No consumer, provider, GitHub Release,
registry, binary, or publication work follows automatically.

## Next

g05.034 records the lane wall clock. A successor to cancelled Card 123 runs
post-tag source-consumer and Desktop repin evidence on the tag.
