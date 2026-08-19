# 318 Kimi Code 0.37.2 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../103-kimi-code-0-37-2-useful-newer.md`
Depends on: Research 159; Research 165

## Goal

Freeze exact Kimi host `0.34.0` and official npm/GitHub `0.37.2` against
qualified `0.36.1`, and name the segment shape. Do not edit the production
claim in this card.

## Scope

1. Rank remaining Research 159 AllowUnverified registry-newer families;
   pick Kimi ACP / headless / local-server.
2. Record npm package identity, host CLI identity, selected help,
   prompt-free ACP initialize, and selected source blobs through
   `0.37.2`.
3. Name card 319 as a compatible extension of existing ACP, headless, and
   local-server heartbeat-ping behaviors.

## Out Of Scope

- editing Kimi selection claims, discovery parser, or public matrices
- Oh My Pi, Gemini, or other 159 families
- provider prompts, install, update, local-server start, or publication

## Acceptance Criteria

- [x] exact `0.37.2` package and host identity is recorded
- [x] selected ACP, headless, and local-server source is compared to the
      `0.36.1` corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- claim-card validation covers focused Kimi proof

## Stop Conditions

- stop if identity requires a provider prompt or starting the local
  server
- stop if a new public operation is required before the pin shape is
  named
- stop if `0.37.2` is no longer npm `latest`

## Auto-Continuation

Continue to card 319 once the segment shape is named.

## Evidence

- Research 165
- `crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-0.37.2/`
- Identity decision: compatible-extension. Reuse ACP declared-effort and
  headless stream-json. Extend local-server `0.35.0..=0.37.2`
  heartbeat-ping. Raise latest qualified to `0.37.2`. Keep baselines.
  Card 319 owns the claim change.
