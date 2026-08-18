# 234 Grok 1.0.x Milestone Claim

Status: completed
Owner: Tom
Created: 2026-08-17
Milestone: `../073-grok-1-0-identity.md`
Depends on: card 233; Research 129 (operator-corrected)

## Goal

Qualify official Grok CLI `1.0.4` on `grok-build.executable` as a real
supported mapping: same axis, new ordered segment, honest behavior revision.
Do not fail-close. Do not flatten onto `0.2` UnverifiedNewer.

## Scope

1. Collect ACP handshake evidence for installed `1.0.4` against the frozen
   `0.2.114..=0.2.117` corpus (initialize, `cached_token` auth, empty session
   alloc). No provider prompt.
2. Name segment shape:
   - same adapter-private mapping → new `1.0` segment, reuse or mint revision
     only as Contract 029 requires
   - changed mapping → new milestone behavior revision and driver branch
3. Update the Grok claim so `1.0.4` is Qualified (Maintained), keep
   `0.2.114..=0.2.117` as older segments, leave mid-gap
   `0.2.118..=0.2.121` Incompatible unless an explicit segment is justified.
4. Update discovery pins, focused tests, and public matrices for the `1.0`
   bound.

## Out Of Scope

- treating `1.0.x` as `0.2` UnverifiedNewer
- inventing a new Grok axis or public operation
- qualifying alpha `1.0.5`
- exact-pin, Gemini, Codex, or other 127 families
- provider prompts, install, update, or publication

## Acceptance Criteria

- [x] installed `1.0.4` classifies as Qualified, not UnverifiedNewer or
      Incompatible
- [x] `0.2.114..=0.2.117` remain permitted on their existing segments
- [x] `1.0.4` is not advertised as a later `0.2` stable
- [x] focused Grok proof and package verify pass
- [x] matrices and guides name the `1.0` bound

## Validation

- `effigy validate:focused swallowtail-adapter-grok`
- `effigy package:verify-affected swallowtail-adapter-grok`
- `effigy qa:northstar`
- named research/log indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if handshake requires a provider prompt or interactive login
- stop if a new public operation or contract is required before the segment
  can land
- stop if `1.0.4` is no longer the official stable point

## Auto-Continuation

No. After closeout, reassess remaining Research 127 families one at a time.
Do not start exact-pin families inside this card.

## Evidence

- Research 130
- `crates/swallowtail-adapter-grok/tests/fixtures/grok-1-0-4/compatibility.json`
- Behavior: `grok-build.acp-v1.cached-token-model-4-6-v3`
- Model: `grok-4.6`
- Latest qualified: `1.0.4`

## Multi-Version Shape

Contract 029 and the Codex/Qwen claims already support this:

- one axis
- ordered non-overlapping segments
- per-segment behavior revisions
- gaps between segments are Incompatible
- AllowUnverified only above the claim's latest qualified maximum

`1.0.x` is a major-line reset on the same package → new milestone segment after
corpus evidence, not a refusal and not silent inheritance from `0.2.117`.
