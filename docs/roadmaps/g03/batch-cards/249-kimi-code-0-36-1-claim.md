# 249 Kimi Code 0.36.1 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../080-kimi-code-0-36-1-useful-newer.md`
Depends on: card 248; Research 137

## Goal

Raise the three `kimi-code.executable` qualified ceilings from `0.31.1`
to `0.36.1`. Reuse ACP and headless behaviors. Add local-server private
milestones, including application ping/pong from `0.35.0`.

## Scope

1. Raise ACP and headless `0.29.0..=0.36.1` on existing behaviors. Keep
   ACP exact `0.28.1`. Keep AllowUnverified.
2. Keep local-server historical exact points through `0.31.1`. Add
   `0.32.0..=0.34.0` optional-meta-flags and `0.35.0..=0.36.1`
   heartbeat-ping. Answer `ping` with `pong`.
3. Move synthetic later-stable UnverifiedNewer to `0.37.0`. Unverified
   newer still does not inherit import, reconciliation, or detachment.
4. Refresh focused tests, matrices, Kimi guides, architecture, and
   contracts that name the ceiling.

## Out Of Scope

- Python `kimi-cli`
- mapping advertised ACP close/delete
- experimental v2 headless
- Gemini or other Research 127 families
- capturing a live prompt or starting the local server
- install, update, or publication

## Acceptance Criteria

- [x] published `0.32.0` through `0.36.1` classify as Qualified
      Maintained on all three routes
- [x] `0.37.0` remains permitted UnverifiedNewer
- [x] decoder specimens remain the existing corpora
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

- stop if card 248 did not name compatible-extension
- stop if live provider work or local-server start would be required to
  close the claim
- stop if `0.36.1` is no longer the official stable point

## Auto-Continuation

No. After closeout, reassess remaining Research 127 families one at a
time and qualify useful-newer support; do not leave the current
host/official stable unqualified. Gemini stays deferred.

## Evidence

- Research 137
- `crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-0.36.1/`
- Decoder specimens remain the existing ACP, headless, and local-server
  corpora
- latest qualified = `0.36.1`
- local-server behaviors:
  `kimi.local-server.rest-ws-v2-optional-meta-flags` and
  `kimi.local-server.rest-ws-v2-heartbeat-ping`
- synthetic later-stable UnverifiedNewer is `0.37.0`
