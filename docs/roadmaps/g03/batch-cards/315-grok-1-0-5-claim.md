# 315 Grok 1.0.5 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../101-grok-1-0-5-useful-newer.md`
Depends on: card 314; Research 163

## Goal

Raise the `grok-build.executable` qualified ceiling from exact `1.0.4` to
official `1.0.5` on existing cached-token-model-4-6-v3. Keep later stables
AllowUnverified. Ignore alpha `1.0.6` as official latest.

## Scope

1. Extend Maintained `1.0.4..=1.0.5` on
   `grok-build.acp-v1.cached-token-model-4-6-v3`. Keep AllowUnverified.
2. Keep deprecated `0.2.114..=0.2.117`. Keep gaps `0.2.118..=0.2.121` and
   `1.0.0..=1.0.3`. Pin discovery source revision `5115b46bc909` for
   `1.0.5`.
3. Refresh focused tests, matrices, Grok guide, and architecture that
   name the ceiling.

## Out Of Scope

- qualifying alpha `1.0.6`
- mapping `--leader-socket`, vendor `_x.ai/*` notifications, or session
  list/resume/close
- Gemini or other Research 159 families
- provider prompts, interactive login, install, update, or publication

## Acceptance Criteria

- [x] official `1.0.5` classifies as Qualified Maintained
- [x] exact `1.0.4` remains Qualified
- [x] `1.0.0..=1.0.3` and `0.2.118..=0.2.121` remain incompatible
- [x] `1.0.6` remains permitted UnverifiedNewer
- [x] `1.0.4` specimens remain
- [x] focused Grok proof and package verify pass
- [x] matrices and guides name the new package ceiling

## Validation

- `effigy validate:focused swallowtail-adapter-grok`
- `effigy package:verify-affected swallowtail-adapter-grok`
- `effigy qa:routes`
- `effigy qa:northstar`
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if card 314 did not name compatible-extension
- stop if live provider prompt would be required to close the claim
- stop if `1.0.5` is no longer the official stable point

## Auto-Continuation

No. After closeout, implement Qwen `0.21.14` useful-newer qualification.
Gemini stays deferred.

## Evidence

- Research 163
- `crates/swallowtail-adapter-grok/tests/fixtures/grok-1-0-5/`
- `1.0.4` specimens remain
- latest qualified = `1.0.5`
- v3 behavior = `grok-build.acp-v1.cached-token-model-4-6-v3`
- published alpha UnverifiedNewer remains `1.0.6`
