# 252 Claude Agent ACP 0.69.0 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../082-claude-agent-acp-0-69-0-useful-newer.md`
Depends on: Research 127; Research 139

## Goal

Freeze exact Claude Agent ACP host `0.63.0` and official npm/GitHub
`0.69.0` against qualified `0.64.0`, and name the segment shape. Do not
edit the production claim in this card.

## Scope

1. Rank remaining Research 127 AllowUnverified registry-newer families;
   pick Claude Agent ACP.
2. Record npm package identity, host CLI identity, and selected tarball
   `dist` hashes through `0.69.0`.
3. Name card 253 as a compatible extension that extends v6 through
   `0.65.0` and adds private v7 from `0.66.0`.

## Out Of Scope

- editing Claude Agent selection claims, discovery parser, or public
  matrices
- Claude Code, Pi, Gemini, or other 127 families
- provider prompts, live ACP initialize, install, update, or publication

## Acceptance Criteria

- [x] exact `0.63.0` host and `0.69.0` official identity is recorded
- [x] selected ACP source is compared to the `0.64.0` corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy qa:northstar`

## Stop Conditions

- stop if identity requires a provider prompt or live authenticated
  initialize
- stop if a new mapped public operation is required before the pin shape
  is named

## Auto-Continuation

Continue to card 253 once the segment shape is named.

## Evidence

- Research 139
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.69.0/`
- Identity decision: compatible-extension. Extend v6
  `0.64.0..=0.65.0`. Add v7 `0.66.0..=0.69.0` initialize-meta-extensions.
  Raise latest qualified to `0.69.0`. Keep baseline and `0.58.0`
  exclusion. Card 253 owns the claim change.
