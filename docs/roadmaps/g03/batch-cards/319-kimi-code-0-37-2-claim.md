# 319 Kimi Code 0.37.2 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../103-kimi-code-0-37-2-useful-newer.md`
Depends on: card 318; Research 165

## Goal

Raise the three `kimi-code.executable` qualified ceilings from `0.36.1`
to official `0.37.2` on existing behaviors. Keep later stables
AllowUnverified.

## Scope

1. Extend ACP and headless Maintained `0.29.0..=0.37.2`. Keep
   AllowUnverified.
2. Extend local-server heartbeat-ping `0.35.0..=0.37.2`. Keep
   `0.32.0..=0.34.0` optional-meta-flags. Qualify `0.37.0` and `0.37.1`.
   Synthetic later UnverifiedNewer is `0.37.3`.
3. Refresh focused tests, matrices, Kimi guides, architecture, and
   contracts that name the ceiling.

## Out Of Scope

- mapping advertised ACP close/delete, `acp --login`, terminal-auth
  metadata, or watch-fs `runtime_id`
- qualifying experimental v2 headless
- Gemini or other Research 159 families
- provider prompts, install, update, local-server start, or publication

## Acceptance Criteria

- [x] official `0.37.2` classifies as Qualified Maintained on all three
      routes
- [x] exact `0.36.1` remains Qualified
- [x] `0.37.0` and `0.37.1` classify as Qualified
- [x] `0.37.3` remains permitted UnverifiedNewer
- [x] `0.36.1` specimens remain
- [x] focused Kimi proof and package verify pass
- [x] matrices and guides name the new package ceiling

## Validation

- `effigy validate:focused swallowtail-adapter-kimi`
- `effigy package:verify-affected swallowtail-adapter-kimi`
- `effigy qa:routes`
- `effigy qa:northstar`
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if card 318 did not name compatible-extension
- stop if live provider prompt or local-server start would be required
- stop if `0.37.2` is no longer the official stable point

## Auto-Continuation

No. After closeout, implement Oh My Pi `17.3.8` useful-newer
qualification. Gemini stays deferred.

## Evidence

- Research 165
- `crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-0.37.2/`
- `0.36.1` specimens remain
- latest qualified = `0.37.2`
- synthetic later-stable UnverifiedNewer is `0.37.3`
