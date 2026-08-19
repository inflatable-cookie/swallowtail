# 314 Grok 1.0.5 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../101-grok-1-0-5-useful-newer.md`
Depends on: Research 159; Research 163

## Goal

Freeze exact Grok host and npm `1.0.5` against qualified exact `1.0.4`,
and name the segment shape. Do not edit the production claim in this card.

## Scope

1. Rank remaining Research 159 AllowUnverified registry-newer families;
   pick Grok `1.0.5`. Ignore alpha `1.0.6`.
2. Record npm package identity, host CLI identity, selected `agent stdio`
   help, and ACP handshake facts through `1.0.5`.
3. Name card 315 as a compatible extension of
   `grok-build.acp-v1.cached-token-model-4-6-v3`.

## Out Of Scope

- editing Grok selection claims, discovery parser, or public matrices
- Qwen, Gemini, or other 159 families
- qualifying alpha `1.0.6`
- provider prompts, interactive login, install, update, or publication

## Acceptance Criteria

- [x] exact `1.0.5` package and CLI identity is recorded
- [x] selected ACP handshake is compared to the `1.0.4` corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- claim-card validation covers focused Grok proof

## Stop Conditions

- stop if identity requires a provider prompt
- stop if a new public operation is required before the pin shape is named
- stop if `1.0.5` is no longer npm `latest`

## Auto-Continuation

Continue to card 315 once the segment shape is named.

## Evidence

- Research 163
- `crates/swallowtail-adapter-grok/tests/fixtures/grok-1-0-5-identity.json`
- `crates/swallowtail-adapter-grok/tests/fixtures/grok-1-0-5/`
- Identity decision: compatible-extension. Reuse
  `grok-build.acp-v1.cached-token-model-4-6-v3`. Raise latest qualified to
  `1.0.5`. Keep exact `1.0.4` as the segment floor. Card 315 owns the
  claim change.
