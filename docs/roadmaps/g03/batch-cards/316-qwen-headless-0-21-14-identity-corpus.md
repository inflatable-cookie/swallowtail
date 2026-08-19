# 316 Qwen Headless 0.21.14 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../102-qwen-headless-0-21-14-useful-newer.md`
Depends on: Research 159; Research 164

## Goal

Freeze exact Qwen host `0.21.2` and official npm/GitHub `0.21.14` against
qualified `0.21.13`, and name the segment shape. Do not edit the production
claim in this card.

## Scope

1. Rank remaining Research 159 AllowUnverified registry-newer families;
   pick Qwen `0.21.14`. Ignore preview `0.21.14-preview.0`.
2. Record npm package identity, host CLI identity, and selected git blobs
   through `0.21.14`.
3. Name card 317 as a compatible extension of
   `qwen-code.headless.v0.21.0-catalogue-filter`.

## Out Of Scope

- editing Qwen selection claims, discovery parser, or public matrices
- Kimi, Gemini, or other 159 families
- qualifying preview `0.21.14-preview.0`
- provider prompts, live catalogue, live headless sessions, install,
  update, or publication

## Acceptance Criteria

- [x] exact `0.21.14` package and host identity is recorded
- [x] selected headless source is compared to the `0.21.13` corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- claim-card validation covers focused Qwen proof

## Stop Conditions

- stop if identity requires a provider prompt or live session
- stop if a new mapped public operation is required before the pin shape
  is named
- stop if `0.21.14` is no longer npm `latest`

## Auto-Continuation

Continue to card 317 once the segment shape is named.

## Evidence

- Research 164
- `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.21.14/`
- Identity decision: compatible-extension. Reuse
  `qwen-code.headless.v0.21.0-catalogue-filter`. Raise latest qualified to
  `0.21.14`. Keep `0.21.13` inside the window. Card 317 owns the claim
  change.
