# 242 Oh My Pi 17.3.7 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../077-oh-my-pi-17-3-7-useful-newer.md`
Depends on: Research 127; Research 134

## Goal

Freeze exact Oh My Pi npm `17.3.7` and host `omp/17.2.15` against qualified
`17.2.9`, and name the segment shape. Do not edit the production claim in
this card.

## Scope

1. Rank remaining Research 127 AllowUnverified host-drift families; pick
   Oh My Pi RPC.
2. Record npm package identity, host CLI identity, selected `--help`
   flags, and selected RPC commands.
3. Name card 243 as a compatible extension reusing
   `oh-my-pi.rpc-v2-v17.2.9`.

## Out Of Scope

- editing Oh My Pi selection claims, discovery parser, or public matrices
- `pi.package`, Cursor Agent, Gemini, or other 127 families
- provider prompts, install, update, or publication

## Acceptance Criteria

- [x] exact `17.3.7` package identity is recorded
- [x] selected RPC flags and commands are compared to the `17.2.9`
      command
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- `effigy validate:focused swallowtail-adapter-oh-my-pi`
- `effigy qa:northstar`

## Stop Conditions

- stop if identity requires a provider prompt
- stop if a new public operation is required before the pin shape is named

## Auto-Continuation

Continue to card 243 once the segment shape is named.

## Evidence

- Research 134
- `crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-17.3.7/`
- Identity decision: compatible-extension. Reuse
  `oh-my-pi.rpc-v2-v17.2.9`. Raise latest qualified to `17.3.7`. Keep
  baseline `17.2.9`. Card 243 owns the claim change.
