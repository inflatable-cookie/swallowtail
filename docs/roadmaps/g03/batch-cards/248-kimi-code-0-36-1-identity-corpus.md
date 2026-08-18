# 248 Kimi Code 0.36.1 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../080-kimi-code-0-36-1-useful-newer.md`
Depends on: Research 127; Research 137

## Goal

Freeze exact Kimi host `0.34.0` and official npm/GitHub `0.36.1` against
qualified `0.31.1`, and name the segment shape. Do not edit the
production claim in this card.

## Scope

1. Rank remaining Research 127 AllowUnverified host-drift families; pick
   Kimi ACP / headless / local-server.
2. Record npm package identity, host CLI identity, selected help, prompt-free
   ACP initialize, and selected source blobs through `0.36.1`.
3. Name card 249 as a compatible extension that reuses ACP and headless
   behaviors and adds local-server private milestones, including
   heartbeat ping/pong.

## Out Of Scope

- editing Kimi selection claims, discovery parser, or public matrices
- Ollama, Gemini, or other 127 families
- provider prompts, install, update, local-server start, or publication

## Acceptance Criteria

- [x] exact `0.34.0` host and `0.36.1` official identity is recorded
- [x] selected ACP, headless, and local-server source is compared to the
      `0.31.1` corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- `effigy validate:focused swallowtail-adapter-kimi`
- `effigy qa:northstar`

## Stop Conditions

- stop if identity requires a provider prompt or starting the local
  server
- stop if a new public operation is required before the pin shape is
  named

## Auto-Continuation

Continue to card 249 once the segment shape is named.

## Evidence

- Research 137
- `crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-0.36.1/`
- Identity decision: compatible-extension. Reuse ACP declared-effort and
  headless stream-json. Add local-server `0.32.0..=0.34.0`
  optional-meta-flags and `0.35.0..=0.36.1` heartbeat-ping. Raise latest
  qualified to `0.36.1`. Keep baselines. Card 249 owns the claim change.
