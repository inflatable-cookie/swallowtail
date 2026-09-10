# v0.5.0 Annotated Source Tag

Date: 2026-09-10
Task: `../roadmaps/g05/049-v0-5-0-annotated-source-tag.md`
Handoff: `../handoffs/20260910-114004-v0-5-0-annotated-source-tag.md`

## Result

Operator authorization created and pushed annotated tag `v0.5.0` at merged
source SHA `582d01d6b6890eed5195a1fcbee0ae985304a6c7` (tree
`eebb6978be365f79ae41436912240886d0aecf78`). The tag object is
`c772c5839806b6cbf1c9d3b495049b362e4c0c52`; local and remote refs agree and
both peel to the exact candidate, and the annotation body bytes equal the
approved three-paragraph text. Push carried only `refs/tags/v0.5.0` with no
branch, force, or other ref. The source-only release has no crates.io
publication, GitHub Release, binary, sidecar, installer, model artifact,
provider mutation, or consumer-repository mutation.

## Verification

Exact preflight confirmed: worker tree clean at planning commit `5ba7b11c`;
remote URL exact; candidate commit and tree present and an ancestor of
canonical `main`; workspace version `0.5.0`, `publish = false`, MSRV `1.95`;
preparation receipt identity retained; review `5616849978` recorded against
the identical tree; `v0.4.4` immutable at `49c9e3b2` (tag object `41da6c1a`);
local and remote `v0.5.0` absent before mutation.

Qualifying pre-tag gate: exact-SHA push run
https://github.com/inflatable-cookie/swallowtail/actions/runs/34464717829
passed all 11 jobs at the candidate, including Pinned MSRV floor and Pinned
MSRV floor tests. Tag-triggered CI run
https://github.com/inflatable-cookie/swallowtail/actions/runs/34467974791
(event `push`, head branch `v0.5.0`, head SHA the exact candidate) completed
`success` with all 11 jobs green, including Pinned MSRV floor and Pinned MSRV
floor tests. The tag was never moved, deleted, recreated, or force-pushed.

## Reconciled Surfaces

The release note, releases index, Contract 036 tagged identity, g05.049,
roadmap and generation front doors, and this log now describe the tagged
state. No consumer, provider, GitHub Release, registry, binary, or
publication work follows automatically.

## Next

Tom/Chatterbox plans the source-tag consumer and working-application lane and
relays the exact tag to Desktop so blocked g02.051 can resume against the
coordinated source. Nothing else follows automatically.
