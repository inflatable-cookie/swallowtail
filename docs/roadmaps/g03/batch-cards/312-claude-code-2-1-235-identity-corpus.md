# 312 Claude Code 2.1.235 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../100-claude-code-2-1-235-useful-newer.md`
Depends on: Research 159; Research 162

## Goal

Freeze exact Claude Code host and npm `2.1.235` against qualified headless
`2.1.220..=2.1.234` and response-only `2.1.227..=2.1.234`, and name the
segment shape. Do not edit the production claim in this card.

## Scope

1. Rank remaining Research 159 AllowUnverified registry-newer families;
   pick Claude Code as one family.
2. Record local `2.1.235 (Claude Code)` identity, npm package identity, and
   selected `--help` flags for both axes.
3. Name card 313 as a compatible extension reusing both existing stream-JSON
   behaviors.

## Out Of Scope

- editing Claude Code selection claims, discovery parser, or public
  matrices
- Claude Agent ACP, Grok, Gemini, or other 159 families
- provider prompts, install, update, or publication

## Acceptance Criteria

- [x] exact `2.1.235` package and CLI identity is recorded
- [x] selected headless and response-only flags are compared to the
      `2.1.234` corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- claim-card validation covers focused Claude Agent proof

## Stop Conditions

- stop if identity requires a provider prompt
- stop if a new public operation is required before the pin shape is named
- stop if one axis would need to stay behind the other

## Auto-Continuation

Continue to card 313 once the segment shape is named.

## Evidence

- Research 162
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.235/`
- Identity decision: compatible-extension. Reuse
  `claude-code.headless.stream-json.v1` and
  `claude-code.response-only.stream-json.v1`. Raise latest qualified to
  `2.1.235` on both axes. Keep baselines `2.1.220` and `2.1.227`. Card 313
  owns the claim change.
