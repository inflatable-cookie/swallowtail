# 246 OpenCode HTTP 1.18.18 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../079-opencode-http-1-18-18-useful-newer.md`
Depends on: Research 127; Research 136

## Goal

Freeze exact OpenCode npm/host `1.18.18` against qualified `1.18.10`, and
name the segment shape. Do not edit the production claim in this card.

## Scope

1. Rank remaining Research 127 AllowUnverified host-drift families; pick
   OpenCode HTTP.
2. Record npm package identity, host CLI identity, selected help, and
   selected OpenAPI closures through `1.18.18`.
3. Name card 247 as a compatible extension that adds private `surface-19`.

## Out Of Scope

- editing OpenCode selection claims, discovery parser, or public matrices
- Kimi, Gemini, or other 127 families
- provider prompts, install, update, server start, or publication

## Acceptance Criteria

- [x] exact `1.18.18` package and host identity is recorded
- [x] selected HTTP/SSE operations are compared to the `1.18.10` corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- `effigy validate:focused swallowtail-adapter-opencode`
- `effigy qa:northstar`

## Stop Conditions

- stop if identity requires a provider prompt or starting the server
- stop if a new public operation is required before the pin shape is named

## Auto-Continuation

Continue to card 247 once the segment shape is named.

## Evidence

- Research 136
- `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.18/`
- Identity decision: compatible-extension. Keep surfaces `01` through
  `18`. Add private `surface-19` for `1.18.11..=1.18.18`. Raise latest
  qualified to `1.18.18`. Keep baseline `1.14.48` and unpublished gaps.
  Card 247 owns the claim change.
