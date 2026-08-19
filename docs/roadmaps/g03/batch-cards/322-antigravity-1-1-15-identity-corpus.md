# 322 Antigravity 1.1.15 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../105-antigravity-1-1-15-useful-newer.md`
Depends on: Research 159; Research 167

## Goal

Freeze exact Antigravity host `1.1.9` and official GitHub `1.1.15` against
qualified `1.1.14`, and name the segment shape. Do not edit the production
claim in this card.

## Scope

1. Rank remaining Research 159 AllowUnverified registry-newer families;
   pick Antigravity.
2. Record GitHub release identity, host CLI identity, changelog, and
   selected help through `1.1.15`.
3. Name card 323 as a compatible extension reusing the existing catalogue
   and stream-json behavior revisions.

## Out Of Scope

- editing Antigravity selection claims, discovery parser, or public
  matrices
- Gemini CLI requalification
- provider prompts, live catalogue, live print, install, update, or
  publication

## Acceptance Criteria

- [x] exact `1.1.9` host and `1.1.15` official identity is recorded
- [x] selected help is compared to the `1.1.14` corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- claim-card validation covers focused Antigravity proof

## Stop Conditions

- stop if identity requires a provider prompt or live session
- stop if a new public operation is required before the pin shape is named
- stop if `1.1.15` is no longer GitHub latest

## Auto-Continuation

Continue to card 323 once the segment shape is named.

## Evidence

- Research 167
- `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.15/`
- Identity decision: compatible-extension. Reuse catalogue and stream-json
  `cli-1.1.8-artifact-1.1.9-v1`. Raise latest qualified to `1.1.15`. Keep
  `1.1.8` incompatible. Card 323 owns the claim change.
