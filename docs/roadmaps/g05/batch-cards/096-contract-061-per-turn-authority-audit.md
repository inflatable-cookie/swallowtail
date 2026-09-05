# 096 Contract 061 Per-Turn Authority Audit

Status: ready; card 095 merged at `ba8275eb`; one planning-only audit across candidates B, K, and L
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
