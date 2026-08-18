# 237 Claude Code Headless 2.1.234 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../075-claude-code-headless-2-1-234-useful-newer.md`
Depends on: Research 127; Research 132

## Goal

Freeze exact Claude Code host and npm `2.1.234` against qualified headless
`2.1.220`, and name the segment shape. Do not edit the production claim in
this card.

## Scope

1. Rank remaining Research 127 AllowUnverified host-drift families; pick
   Claude Code headless.
2. Record local `2.1.234 (Claude Code)` identity, npm package identity, and
   selected `--help` flags.
3. Name card 238 as a compatible extension reusing
   `claude-code.headless.stream-json.v1`.

## Out Of Scope

- editing Claude Code selection claims, discovery parser, or public
  matrices
- Claude Code response-only, Gemini, or other 127 families
- provider prompts, install, update, or publication

## Acceptance Criteria

- [x] exact `2.1.234` package and CLI identity is recorded
- [x] selected headless flags are compared to the `2.1.220` command
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy qa:northstar`

## Stop Conditions

- stop if identity requires a provider prompt
- stop if a new public operation is required before the pin shape is named

## Auto-Continuation

Continue to card 238 once the segment shape is named.

## Evidence

- Research 132
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.234/`
- Identity decision: compatible-extension. Reuse
  `claude-code.headless.stream-json.v1`. Raise latest qualified to
  `2.1.234`. Keep baseline `2.1.220`. Card 238 owns the claim change.
