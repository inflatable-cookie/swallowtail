# 258 Antigravity 1.1.14 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../085-antigravity-1-1-14-useful-newer.md`
Depends on: Research 127; Research 142

## Goal

Freeze exact Antigravity host `1.1.9` and official GitHub `1.1.14` against
qualified `1.1.9`, and name the segment shape. Do not edit the production
claim in this card.

## Scope

1. Rank remaining Research 127 AllowUnverified registry-newer families;
   pick Antigravity.
2. Record GitHub release identity, host CLI identity, changelog, and
   selected help through `1.1.14`.
3. Name card 259 as a compatible extension that reuses catalogue and
   stream-json revisions and keeps `1.1.8` incompatible.

## Out Of Scope

- editing Antigravity selection claims or public matrices
- Gemini or other 127 families
- provider prompts, live catalogue, live print runs, install, update, or
  publication

## Acceptance Criteria

- [x] exact `1.1.9` host and `1.1.14` official identity is recorded
- [x] selected help is compared to the `1.1.9` corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- `effigy validate:focused swallowtail-adapter-antigravity`
- `effigy qa:northstar`

## Stop Conditions

- stop if identity requires a provider prompt or live session
- stop if a new mapped public operation is required before the pin shape
  is named

## Auto-Continuation

Continue to card 259 once the segment shape is named.

## Evidence

- Research 142
- `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.14/`
- Identity decision: compatible-extension. Reuse catalogue and
  stream-json 1.1.9 revisions. Qualify published `1.1.10` through
  `1.1.14`. Keep `1.1.8` incompatible. Raise latest qualified to `1.1.14`.
  Keep baseline. Card 259 owns the claim change.
