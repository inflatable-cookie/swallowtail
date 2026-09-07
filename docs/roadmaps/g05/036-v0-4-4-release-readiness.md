# g05.036 v0.4.4 Release Readiness

Status: ready; cards 122 and 123 compiled; ready on card 121's merge and the tag
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Depends on: Contract 036 with the Hosted Gate Delegation clause; immutable `v0.4.3` at `cbd4ddc8`; cards 119, 120, 121; the release playbook (card 109)
Vision tags: source release, consumer proof, Claude route

## Purpose

Get the first-turn model rejection evidence (cards 119-121) to Bovine Desktop,
which pins tags, as patch `v0.4.4`. This is the first lane run on the
simplified shape: cheap local gates only, the hosted `workflow_dispatch` run
as the heavy evidence, tag request in the same turn the candidate SHA exists,
consumer smoke on the tag. Its wall clock is the g05.034 acceptance measure.

## Runway

1. Cards 119 (merged `cfb0b106`), 120 (merged `4a27676d`), 121, and 124 merge on exact-head review.
2. Card 122 prepares the `0.4.4` candidate per `docs/guides/release-playbook.md`.
3. Chatterbox presents the exact-SHA tag request the moment the candidate
   SHA has a qualifying green run; the operator authorizes; the coordinator
   tags and pushes.
4. Card 123 runs the source consumer on the tag and relays the capsule to
   Acowtancy for the Desktop repin and one real Send with the observer
   registered; the result attaches as consumer evidence and holds nothing.

## Release Boundary

No card creates or pushes a tag. The operator authorizes the exact SHA. No
publication, GitHub Release, binaries, sidecars, installers, or consumer
mutation.

## Feature Freeze

From card 124's merge until card 122's candidate merges, no other PR merges
to `main`. Anything accepted and green before card 121 merges may go in
first and becomes candidate content.

## Batch Cards

- [122 v0.4.4 Candidate Preparation](batch-cards/122-v0-4-4-candidate-preparation.md) — planned; ready on card 124's merge
- [123 v0.4.4 Consumer Proof And Tag Capsule](batch-cards/123-v0-4-4-consumer-proof-and-tag-capsule.md) — planned; ready on the tag

## Dispatch Manifest

Promoted planning commit: the `main` commit that introduces this file.

### Card 122 Manifest

| Field | Card 122 |
| --- | --- |
| Readiness | ready on card 124's merge; the coordinator dispatches on that notice without a Chatterbox round trip; prepare authorization under the operator's standing grant |
| Prerequisites | cards 121 and 124 merged; clean canonical `main`; freeze in force |
| Completion conditions | `docs/releases/0.4.4.md` and index entry from cards 119-121 plus any merged additive tranches; patch class from the semantic API diff; read-only status inferring `0.4.4`; lock in sync; exactly one `effigy --json release prepare --yes --check-gates --version 0.4.4` on the cheap-gate table; the qualifying hosted `workflow_dispatch` run id recorded in the release note; candidate PR; review and hosted run in parallel; merge on both green; candidate SHA and run id reported to Chatterbox immediately; wall clock from dispatch to merge recorded in the Result |
| Owned mutable paths | as card 106 with `0.4.4` for `0.4.3`; no gate-script version edits (card 110 derives them) |
| Reserved shared closeout surfaces | the usual roadmap, index, generation, log, and release-index surfaces |
| Forbidden paths | every `crates/**` source and test path; every `0.4.3` and earlier baseline; contracts; guides other than the release note; claims |
| Approved concurrent siblings | none; freeze |
| Serial edges | the operator's tag decision follows; card 123 follows the tag |
| Worker capability class | release-preparation worker with the playbook; no credentials; no tag authority |
| Acceptance evidence | status output; prepare JSON; the `0.4.4` baseline files; PR head, merged SHA, qualifying run id; wall clock |
| Review oracle | one exact tree supports every release statement; the qualifying run is `workflow_dispatch` at the SHA to tag or an identical tree |
| Stop conditions | a cheap gate fails on a real defect; status infers anything but `0.4.4`; no qualifying run can be obtained |
| Escalation owner | operator via Chatterbox; coordinator for mechanical blockers |

### Card 123 Manifest

| Field | Card 123 |
| --- | --- |
| Readiness | ready when the operator-authorized `v0.4.4` tag exists |
| Prerequisites | the tag; peeled SHA verified by Chatterbox |
| Completion conditions | `effigy package:source-consumer` passes from a clean detached checkout of the tag; capsule (tag, object, peeled SHA, message, run id) relayed to the Acowtancy coordinator; release note status flipped to tagged; Contract 036 identity updated; the Desktop real-Send result with card 119's observer evidence attached when it arrives |
| Owned mutable paths | this card's `## Result`; release note status line; Contract 036 tagged identity lines; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | the usual roadmap, index, generation, and log surfaces |
| Forbidden paths | every crate; every baseline; the candidate |
| Approved concurrent siblings | held feature PRs resume merging after the tag |
| Serial edges | none |
| Worker capability class | release-evidence worker; the Desktop side runs under the Acowtancy coordinator |
| Acceptance evidence | source-consumer output at the tag; the Desktop capsule |
| Review oracle | both proofs use the exact tagged SHA |
| Stop conditions | the Desktop turn fails on a further route defect (typed code plus observer evidence; next patch, never a retag) |
| Escalation owner | operator via Chatterbox; Acowtancy coordinator for the Desktop run |

## Acceptance

`v0.4.4` tagged by the operator at an exact SHA carrying cards 119-121; the
lane's wall clock recorded against the g05.034 target of thirty minutes from
gates green to tag request.
