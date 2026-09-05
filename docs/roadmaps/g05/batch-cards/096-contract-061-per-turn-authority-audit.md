# 096 Contract 061 Per-Turn Authority Audit

Status: complete; existing vocabulary suffices; B, K, and L promotable; PR 232 exact head `497ed460` merged as `b874df63`
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: Contract 041 consumer tool exchange; Contract 061; the Batch 9.4 checkpoint; cards 022-034 and 068-079 merged

## Goal

Settle, once, the authority and lifecycle proof that the Batch 9.4
reassessment required before candidates B (Alibaba, Anthropic, xAI; 76
rows), K (Mistral Vibe, Muse, Oh My Pi, Qwen; 52 rows), and L (OpenCode,
Pi; 69 rows) can be implemented: how a consumer-mediated per-turn exchange
or attachment row is published as truth without inferring mutation
authority. These 197 rows are the last of the 767.

## Scope

1. Classify every per-turn and attachment row across the three candidates
   by what the consumer actually mediates (Contract 041 tool exchange,
   attachment admission, approval or question request) and what evidence
   each route retains.
2. Decide whether the existing `ConsumerMediatedPerTurn` mutation-authority
   posture and the projection vocabulary already cover each row, or whether
   one additive shared baseline is needed, following the provider-operation
   and compound-acknowledgement precedents.
3. Apply the Batch 9.4 rubric per candidate and name the cleanest to
   promote first (Chatterbox expects L).
4. One triage note; zero Rust; stop for Chatterbox promotion.

## Out Of Scope

Rust, contract, architecture, census, or Batch 9.4 note edits; implementing
any candidate; provider contact.

## Acceptance Criteria

- every per-turn and attachment row across B, K, and L is classified with
  a code-anchored reason
- one ruling: existing vocabulary suffices, or one additive shared baseline
  is drafted verbatim (names, admission, assertions) for Chatterbox promotion
- rubric verdict per candidate and one recommended first promotion
- one triage note; zero Rust

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: no row is marked publishable without naming the retained
evidence that proves the consumer mediated it. Smallest counterexample: a
per-turn row whose authority is inferred from a prepared plan or a successful
local call.

## Auto-Continuation

No. Stop after the note for Chatterbox reconciliation.

## Result

Complete. Planning-only; zero Rust.

All 197 rows across candidates B (76), K (52), and L (69) are classified with
code anchors in
[the per-turn authority audit](../../../triage/20260905-143430-contract-061-per-turn-authority-audit.md).
Every per-turn and attachment row is itemised individually: 8 per-turn rows
and 13 attachment rows, 4 of which overlap, for 17 distinct itemised rows; the
other 180 are classified by band. Lifecycle split is 136 selection-summary, 41
session-start-only, 12 post-open-observation-only, and 8 per-turn.

Ruling: the existing `ConsumerMediatedPerTurn` posture and projection
vocabulary suffice. No additive shared baseline is drafted, because none is
needed. Seven of the eight per-turn rows carry retained plan-borne evidence —
a bounded `Capability::Attachments` requirement, or
`SessionAccessPolicy::ambient_harness_with_consumer_mediated_requests` plus the
exact `opencode/permission` and `opencode/question` extension namespaces — and
`admit_lifecycle_authority`
(`crates/swallowtail-runtime/src/consumer_route_projection/admission.rs:211`)
already rejects the card's counterexample as a composer failure. The eighth,
`opencode.http control.provider-turn-reference`, is matrix-descriptor-only and
is withheld at construction as negative coverage.

Rubric: B, K, and L all pass items 1-6. No evidence stop and no gate. Two
conditions are recorded for the implementing cards, neither requiring a shared
change: `feature.permission-exchange` must use a bounded `Namespaced`
extension rather than `QuestionExchange`, and `feature.attachments` rows are
conditional on the prepared capability profile because no route descriptor
declares `Capability::Attachments`.

Recommended first promotion: **L** (OpenCode, Pi), as Chatterbox expected. It
holds six of the eight per-turn rows, has the widest per-turn spectrum, and is
the smallest validation scope at two adapter packages. Suggested order after L
is B, then K.

Validation: `effigy qa:docs`, `effigy qa:northstar`, `git diff --check`.

Stops for Chatterbox reconciliation. No candidate promoted; no shared
vocabulary, contract, architecture, or census surface changed.
