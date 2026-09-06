# g05.033 v0.4.3 Release Readiness

Status: ready; card 105 in flight; cards 106 and 107 ready on its merge
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Depends on: Contract 036; immutable `v0.4.2` at `f94dd16f`; card 105; the g05.032 precedent and its compressions; the 2026-09-06 Desktop real-Send evidence
Vision tags: source release, consumer proof, Claude route

## Purpose

Ship card 105 (the Claude SDK termination cause reaching the consumer) as
patch `v0.4.3`, on the g05.032 lane with its lessons applied: gates green
means the exact-SHA tag request goes to the operator at once, and the
consumer smoke runs on the tag as post-release evidence rather than as a
pre-tag gate. Bovine Desktop has proved a visible window and a real Send on
`v0.4.2`; the one remaining failure is the cause this patch surfaces.

## Runway

1. Card 105 merges on exact-head review with fake-SDK proofs.
2. Card 106 prepares the `0.4.3` candidate: read-only status, one Effigy
   prepare transaction with per-gate logs, release note authored from card
   105's result and any merged additive tranches (cards 083, 085, 088, 104
   if merged and green), scripts repointed to `0.4.3`, candidate PR with
   review and exact-SHA CI in parallel, merge. No frozen-tree rerun.
3. Chatterbox compiles the exact-SHA tag request the moment card 106's
   candidate merges with green CI; the operator authorizes; the coordinator
   tags and pushes.
4. Card 107 runs the source consumer on the tag and relays the tag capsule
   to Acowtancy for the Desktop repin and one real Send; the Desktop result
   attaches as consumer evidence and does not hold the tag.

## Release Boundary

No card creates or pushes a tag. The operator authorizes the exact SHA. No
publication, GitHub Release, binaries, sidecars, installers, or consumer
mutation.

## Feature Freeze

From card 105's merge until card 106's candidate merges, no other PR merges
to `main`. Open feature PRs (cards 083, 085, 086, 088, 104) hold at review
and rebase after the candidate; if one is accepted and green before card
105 merges, it may go in first and becomes candidate content.

## Batch Cards

- [106 v0.4.3 Candidate Preparation](batch-cards/106-v0-4-3-candidate-preparation.md) — planned; ready on card 105's merge
- [107 v0.4.3 Consumer Proof And Tag Capsule](batch-cards/107-v0-4-3-consumer-proof-and-tag-capsule.md) — planned; ready on the tag

## Dispatch Manifest

Promoted planning commit: the `main` commit that introduces this file.
Card 105's manifest lives in g05.029.

### Card 106 Manifest

| Field | Card 106 |
| --- | --- |
| Readiness | ready on card 105's merge; the coordinator dispatches on that notice without a further Chatterbox round trip; operator prepare authorization on the `v0.4.1` standing-grant pattern |
| Prerequisites | card 105 merged; clean canonical base; freeze in force |
| Completion conditions | `docs/releases/0.4.3.md` and index entry from card 105's result plus merged tranches; patch class from the semantic API diff (additive only); read-only status inferring `0.4.3`; lock in sync; exactly one `effigy --json release prepare --yes --check-gates --version 0.4.3` with per-gate logs; no frozen-tree rerun; distinct `0.4.3` baseline, route inventory, dependency graph; `0.4.2` files untouched; gate scripts and consumer front-door repointed to `0.4.3`; candidate PR; review and workflow-dispatch CI in parallel; merge on both green; candidate SHA reported to Chatterbox immediately |
| Owned mutable paths | as card 101's manifest with `0.4.3` for `0.4.2`: workspace `Cargo.toml` versions via prepare; `Cargo.lock`; `CHANGELOG.md` promotion; `docs/releases/0.4.3.md`; `docs/releases/README.md` current entry; `release-baselines/public-api-0.4.3/**`, `production-routes-0.4.3.txt`, `internal-dependencies-0.4.3.tsv`; `.release-prepared.json`; the four gate scripts, `scripts/check-consumer-front-door.py`, `scripts/README.md`; root `README.md` posture lines; this card's `## Result`; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, this roadmap, `docs/roadmaps/g05/batch-cards/README.md`, `docs/roadmaps/generation-index.md`, `docs/logs/README.md`, `docs/releases/README.md` historical lines |
| Forbidden paths | every `crates/**` source and test path; every `0.4.2` and earlier baseline; contracts; guides; matrices; claims |
| Approved concurrent siblings | none; freeze |
| Serial edges | the operator's tag decision follows the merged candidate; card 107 follows the tag |
| Worker capability class | release-preparation worker with Effigy release discipline; no credentials; no tag authority |
| Acceptance evidence | status output; prepare JSON with all gates green and per-gate logs; `0.4.3` baseline files; PR head, merged SHA, workflow run id |
| Review oracle | one exact tree supports every release statement |
| Stop conditions | a gate fails on a real defect; status infers anything but `0.4.3`; a transient is renewed once under the standing grant |
| Escalation owner | operator via Chatterbox; coordinator for mechanical blockers |

### Card 107 Manifest

| Field | Card 107 |
| --- | --- |
| Readiness | ready when the operator-authorized `v0.4.3` tag exists |
| Prerequisites | the tag; peeled SHA verified by Chatterbox |
| Completion conditions | `effigy package:source-consumer` passes from a clean detached checkout of the tag; tag capsule (tag, object, peeled SHA, message) relayed to the Acowtancy coordinator; release note status flipped to tagged; Contract 036 identity updated; the Desktop real-Send result attached when it arrives, pass or typed cause |
| Owned mutable paths | this card's `## Result`; release note status line; Contract 036 tagged identity lines; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | the usual roadmap, index, generation, and log surfaces |
| Forbidden paths | every crate; every baseline; the candidate |
| Approved concurrent siblings | the held feature PRs resume merging after the tag |
| Serial edges | none |
| Worker capability class | release-evidence worker; the Desktop side runs under the Acowtancy coordinator |
| Acceptance evidence | source-consumer output at the tag; the Desktop capsule |
| Review oracle | both proofs use the exact tagged SHA |
| Stop conditions | the Desktop turn fails on a further route defect (typed code; next patch, never a retag) |
| Escalation owner | operator via Chatterbox; Acowtancy coordinator for the Desktop run |

## Acceptance

`v0.4.3` tagged by the operator at an exact SHA whose tree carries card 105;
the Desktop's next real Send either replies or fails with a named cause.
