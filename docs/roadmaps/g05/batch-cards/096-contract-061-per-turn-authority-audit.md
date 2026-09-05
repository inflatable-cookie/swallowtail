# 096 Contract 061 Per-Turn Authority Audit

Status: planned; gated behind card 095's merge; then ready as one planning-only audit across candidates B, K, and L
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

## Readiness

Becomes ready when card 095 merges, so faster CI precedes it. Chatterbox
publishes its manifest then.
