# 233 Grok 1.0.x Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-17
Milestone: `../073-grok-1-0-identity.md`
Depends on: Research 127; g03.072

## Goal

Freeze exact Grok CLI `1.0.4` identity against the qualified
`0.2.114..=0.2.117` window, and name whether `1.0.x` is a new axis or a
stop. Do not treat it as `0.2` UnverifiedNewer.

## Scope

1. Record npm `@xai-official/grok@1.0.4`, local `grok 1.0.4`, and the
   current `grok-build.executable` claim.
2. Compare protocol and invocation evidence to the `0.2` corpus enough to
   reject silent flattening.
3. Name the next claim or stop card without changing production claims.

## Out Of Scope

- editing Grok selection claims or public matrices
- treating `1.0.x` as compatible `0.2` UnverifiedNewer
- Codex, exact-pin, Gemini, or other 127 families
- provider prompts, install, update, or publication

## Acceptance Criteria

- [x] exact `1.0.4` package and CLI identity is recorded
- [x] the `0.2` UnverifiedNewer flattening is explicitly rejected
- [x] the next card has an explicit identity decision
- [x] no claim membership changes in this card

## Validation

- fixture or schema comparison named in the card evidence
- `effigy qa:northstar`

## Stop Conditions

- stop if `1.0.4` is no longer the official stable point
- stop if qualification would require a provider prompt
- stop if a new public operation is required before identity is named

## Auto-Continuation

No. Compile the follow-up claim or stop card after the identity shape is
named. Do not start exact-pin families inside this card.

## Evidence

- Research 129
- `crates/swallowtail-adapter-grok/tests/fixtures/grok-1-0-4-identity.json`
- Identity decision (operator-corrected): same-axis milestone to qualify.
  Not UnverifiedNewer. Not fail-closed. Not a new axis. Card 234 owns
  handshake evidence and claim membership.
