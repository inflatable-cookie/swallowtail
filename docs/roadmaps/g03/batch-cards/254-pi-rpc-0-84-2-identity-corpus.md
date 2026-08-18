# 254 Pi RPC 0.84.2 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../083-pi-rpc-0-84-2-useful-newer.md`
Depends on: Research 127; Research 140

## Goal

Freeze exact Pi host `0.83.0` and official npm/GitHub `0.84.2` against
qualified `0.83.0`, and name the segment shape. Do not edit the production
claim in this card.

## Scope

1. Rank remaining Research 127 AllowUnverified registry-newer families;
   pick Pi RPC.
2. Record npm package identity, host CLI identity, and selected git blobs
   through `0.84.2`.
3. Name card 255 as a compatible extension that keeps unpublished `0.83.1`
   incompatible and adds private `0.84.0` message-update-delta.

## Out Of Scope

- editing Pi selection claims, discovery parser, or public matrices
- Oh My Pi, Qwen, Gemini, or other 127 families
- provider prompts, live RPC sessions, install, update, or publication

## Acceptance Criteria

- [x] exact `0.83.0` host and `0.84.2` official identity is recorded
- [x] selected RPC source is compared to the `0.83.0` corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- `effigy validate:focused swallowtail-adapter-pi`
- `effigy qa:northstar`

## Stop Conditions

- stop if identity requires a provider prompt or live RPC session
- stop if a new mapped public operation is required before the pin shape
  is named

## Auto-Continuation

Continue to card 255 once the segment shape is named.

## Evidence

- Research 140
- `crates/swallowtail-adapter-pi/tests/fixtures/pi-rpc-0.84.2/`
- Identity decision: compatible-extension. Keep unpublished `0.83.1`
  incompatible. Add `0.84.0..=0.84.2` message-update-delta. Raise latest
  qualified to `0.84.2`. Keep baseline. Card 255 owns the claim change.
