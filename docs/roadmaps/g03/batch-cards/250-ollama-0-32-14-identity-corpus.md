# 250 Ollama 0.32.14 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../081-ollama-0-32-14-useful-newer.md`
Depends on: Research 127; Research 138

## Goal

Freeze exact Ollama host `0.32.9` and official GitHub `v0.32.14` against
qualified `0.32.1`, and name the segment shape. Do not edit the production
claim in this card.

## Scope

1. Rank remaining Research 127 AllowUnverified host-drift families; pick
   Ollama attached.
2. Record host CLI identity, GitHub latest identity, and selected native
   API structs/routes through `0.32.14`.
3. Name card 251 as a compatible extension that reuses
   `ollama.native-text-v1`, keeps `0.32.2` excluded, and adds `0.32.10`.

## Out Of Scope

- editing Ollama selection claims, discovery parser, or public matrices
- Claude Agent ACP, Gemini, or other 127 families
- provider prompts, install, update, server start, or publication

## Acceptance Criteria

- [x] exact `0.32.9` host and `0.32.14` official identity is recorded
- [x] selected native API source is compared to the `0.32.1` corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- `effigy validate:focused swallowtail-adapter-ollama`
- `effigy qa:northstar`

## Stop Conditions

- stop if identity requires a provider prompt or starting the attached
  server
- stop if a new public operation is required before the pin shape is named

## Auto-Continuation

Continue to card 251 once the segment shape is named.

## Evidence

- Research 138
- `crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.32.14/`
- Identity decision: compatible-extension. Reuse `ollama.native-text-v1`.
  Raise latest qualified to `0.32.14`. Keep baseline `0.14.0` and
  exclusion `0.32.2`. Add exclusion `0.32.10`. Card 251 owns the claim
  change.
