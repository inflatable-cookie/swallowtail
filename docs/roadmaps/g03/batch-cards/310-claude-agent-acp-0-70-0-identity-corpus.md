# 310 Claude Agent ACP 0.70.0 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../099-claude-agent-acp-0-70-0-useful-newer.md`
Depends on: Research 159; Research 161

## Goal

Freeze exact Claude Agent ACP host `0.63.0` and official npm/GitHub
`0.70.0` against qualified `0.69.0`, and name the segment shape. Do not
edit the production claim in this card.

## Scope

1. Rank remaining Research 159 AllowUnverified registry-newer families;
   pick Claude Agent ACP.
2. Record npm package identity, host CLI identity, and selected tarball
   `dist` hashes through `0.70.0`.
3. Name card 311 as a compatible extension that extends v7 through
   `0.70.0` and leaves the Providers API unmapped.

## Out Of Scope

- editing Claude Agent selection claims, discovery parser, or public
  matrices
- Claude Code, Grok, Gemini, or other 159 families
- mapping `providers/list`, `providers/set`, or `providers/disable`
- provider prompts, live ACP initialize, install, update, or publication

## Acceptance Criteria

- [x] exact `0.63.0` host and `0.70.0` official identity is recorded
- [x] selected ACP source is compared to the `0.69.0` corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- `effigy qa:northstar`

## Stop Conditions

- stop if identity requires a provider prompt or live authenticated
  initialize
- stop if a new mapped public operation is required before the pin shape
  is named
- stop if `0.70.0` is no longer the official stable point

## Auto-Continuation

Continue to card 311 once the segment shape is named.

## Evidence

- Research 161
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.70.0/`
- Identity decision: compatible-extension. Extend v7
  `0.66.0..=0.70.0`. Raise latest qualified to `0.70.0`. Keep baseline
  and `0.58.0` exclusion. Synthetic later UnverifiedNewer is `0.70.1`.
  Card 311 owns the claim change.
