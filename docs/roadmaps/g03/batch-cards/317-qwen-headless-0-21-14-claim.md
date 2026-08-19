# 317 Qwen Headless 0.21.14 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../102-qwen-headless-0-21-14-useful-newer.md`
Depends on: card 316; Research 164

## Goal

Raise the `qwen-code.package` qualified ceiling from `0.21.13` to official
`0.21.14` on existing catalogue-filter. Keep later stables
AllowUnverified. Ignore preview `0.21.14-preview.0`.

## Scope

1. Extend Maintained `0.21.0..=0.21.14` on
   `qwen-code.headless.v0.21.0-catalogue-filter`. Keep AllowUnverified.
2. Keep Deprecated `0.19.11..=0.20.1`. Keep unpublished `0.20.2`
   incompatible. Synthetic later UnverifiedNewer is `0.21.15`.
3. Refresh focused tests, matrices, Qwen guide, and architecture that
   name the ceiling.

## Out Of Scope

- mapping `goal_state`, initialize `effort_status`, `qwen sessions ps`,
  `/advisor`, or live-session registry
- qualifying preview `0.21.14-preview.0`
- Gemini or other Research 159 families
- provider prompts, live catalogue, live headless sessions, install,
  update, or publication

## Acceptance Criteria

- [x] official `0.21.14` classifies as Qualified Maintained
- [x] exact `0.21.13` remains Qualified
- [x] unpublished `0.20.2` remains incompatible
- [x] `0.21.15` remains permitted UnverifiedNewer
- [x] `0.21.13` specimens remain
- [x] focused Qwen proof and package verify pass
- [x] matrices and guides name the new package ceiling

## Validation

- `effigy validate:focused swallowtail-adapter-qwen`
- `effigy package:verify-affected swallowtail-adapter-qwen`
- `effigy qa:routes`
- `effigy qa:northstar`
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if card 316 did not name compatible-extension
- stop if live provider prompt would be required to close the claim
- stop if `0.21.14` is no longer the official stable point

## Auto-Continuation

No. After closeout, implement Kimi `0.37.2` useful-newer qualification.
Gemini stays deferred.

## Evidence

- Research 164
- `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.21.14/`
- `0.21.13` specimens remain
- latest qualified = `0.21.14`
- behavior = `qwen-code.headless.v0.21.0-catalogue-filter`
- synthetic later-stable UnverifiedNewer is `0.21.15`
