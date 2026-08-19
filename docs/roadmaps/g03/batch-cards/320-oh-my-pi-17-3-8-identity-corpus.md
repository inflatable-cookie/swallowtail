# 320 Oh My Pi 17.3.8 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../104-oh-my-pi-17-3-8-useful-newer.md`
Depends on: Research 159; Research 166

## Goal

Freeze exact Oh My Pi npm `17.3.8` and host `omp/17.2.15` against
qualified `17.3.7`, and name the segment shape. Do not edit the production
claim in this card.

## Scope

1. Rank remaining Research 159 AllowUnverified registry-newer families;
   pick Oh My Pi RPC.
2. Record npm package identity, host CLI identity, selected `--help`
   flags, and selected RPC commands.
3. Name card 321 as a compatible extension reusing
   `oh-my-pi.rpc-v2-v17.2.9`.

## Out Of Scope

- editing Oh My Pi selection claims, discovery parser, or public matrices
- `pi.package`, Antigravity, Gemini, or other 159 families
- provider prompts, install, update, or publication

## Acceptance Criteria

- [x] exact `17.3.8` package identity is recorded
- [x] selected RPC flags and commands are compared to the `17.3.7`
      corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- claim-card validation covers focused Oh My Pi proof

## Stop Conditions

- stop if identity requires a provider prompt
- stop if a new public operation is required before the pin shape is named
- stop if `17.3.8` is no longer npm `latest`

## Auto-Continuation

Continue to card 321 once the segment shape is named.

## Evidence

- Research 166
- `crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-17.3.8/`
- Identity decision: compatible-extension. Reuse
  `oh-my-pi.rpc-v2-v17.2.9`. Raise latest qualified to `17.3.8`. Keep
  baseline `17.2.9`. Card 321 owns the claim change.
