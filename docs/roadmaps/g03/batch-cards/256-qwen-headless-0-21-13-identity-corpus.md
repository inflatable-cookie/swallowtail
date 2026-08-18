# 256 Qwen Headless 0.21.13 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../084-qwen-headless-0-21-13-useful-newer.md`
Depends on: Research 127; Research 141

## Goal

Freeze exact Qwen host `0.21.2` and official npm/GitHub `0.21.13` against
qualified `0.21.2`, and name the segment shape. Do not edit the production
claim in this card.

## Scope

1. Rank remaining Research 127 AllowUnverified registry-newer families;
   pick Qwen headless.
2. Record npm package identity, host CLI identity, and selected git blobs
   through `0.21.13`.
3. Name card 257 as a compatible extension that reuses catalogue-filter
   and keeps unpublished stable `0.20.2` incompatible.

## Out Of Scope

- editing Qwen selection claims, discovery parser, or public matrices
- Antigravity, Gemini, or other 127 families
- provider prompts, live catalogue, live headless sessions, install,
  update, or publication

## Acceptance Criteria

- [x] exact `0.21.2` host and `0.21.13` official identity is recorded
- [x] selected headless source is compared to the `0.21.2` corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- `effigy validate:focused swallowtail-adapter-qwen`
- `effigy qa:northstar`

## Stop Conditions

- stop if identity requires a provider prompt or live session
- stop if a new mapped public operation is required before the pin shape
  is named

## Auto-Continuation

Continue to card 257 once the segment shape is named.

## Evidence

- Research 141
- `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.21.13/`
- Identity decision: compatible-extension. Reuse
  `qwen-code.headless.v0.21.0-catalogue-filter`. Qualify published
  `0.21.3` through `0.21.13`. Keep unpublished `0.20.2` incompatible.
  Raise latest qualified to `0.21.13`. Keep baseline. Card 257 owns the
  claim change.
