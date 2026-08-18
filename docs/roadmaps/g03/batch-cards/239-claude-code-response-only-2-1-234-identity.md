# 239 Claude Code Response-Only 2.1.234 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../076-claude-code-response-only-2-1-234-provisional.md`
Depends on: Research 127; Research 133; g03.075

## Goal

Freeze Claude Code `2.1.234` on `claude-code.response-only-stream-json`
against qualified `2.1.227..=2.1.228`, and name keep-provisional versus
ceiling-raise. Do not edit the production claim in this card.

## Scope

1. Rank remaining AllowUnverified host-drift families; pick response-only.
2. Record selected `--help` flags for the response-only command on host
   `2.1.234`.
3. Name card 240 as keep-provisional: no qualified-bound move, no deny-list
   entry, no axis mix.

## Out Of Scope

- editing response-only or headless selection claims
- a live protocol transcript
- Oh My Pi, Gemini, or other 127 families
- install, update, or publication

## Acceptance Criteria

- [x] exact `2.1.234` response-only identity is recorded
- [x] selected flags are compared to the `2.1.227`/`2.1.228` command
- [x] the next card has an explicit keep-provisional decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy qa:northstar`

## Stop Conditions

- stop if identity requires a provider prompt
- stop if a new public operation is required before the shape is named

## Auto-Continuation

Continue to card 240 once the segment shape is named.

## Evidence

- Research 133
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.234/response-only.json`
- First identity decision: keep-provisional. Operator rejected that.
  Card 241 owns the compatible-extension claim through `2.1.234`.
