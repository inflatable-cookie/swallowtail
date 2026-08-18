# 255 Pi RPC 0.84.2 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../083-pi-rpc-0-84-2-useful-newer.md`
Depends on: card 254; Research 140

## Goal

Raise the `pi.package` qualified ceiling from exact `0.83.0` to `0.84.2`.
Keep unpublished `0.83.1` incompatible. Add private `0.84.0`
message-update-delta.

## Scope

1. Mark exact `0.83.0` Deprecated on
   `pi.rpc.strict-lf-v0.83.0-bash-extension-hook`.
2. Add Maintained `0.84.0..=0.84.2` on
   `pi.rpc.strict-lf-v0.84.0-message-update-delta`. Keep AllowUnverified.
3. Keep unpublished `0.80.11`, `0.81.2`, `0.82.2`, and `0.83.1`
   incompatible. Move synthetic later-stable UnverifiedNewer to `0.84.3`.
4. Refresh focused tests, matrices, the Pi guide, architecture, and
   contracts that name the ceiling.

## Out Of Scope

- Oh My Pi axis
- mapping bash, session switching, load/resume, or streaming usage
- Gemini or other Research 127 families
- capturing a live RPC prompt
- install, update, or publication

## Acceptance Criteria

- [x] published `0.84.0` through `0.84.2` classify as Qualified
- [x] unpublished `0.83.1` remains incompatible
- [x] `0.84.3` remains permitted UnverifiedNewer
- [x] decoder specimen remains `pi-rpc-0.80.10`
- [x] focused Pi proof and package verify pass
- [x] matrices and guides name the new package ceiling

## Validation

- `effigy validate:focused swallowtail-adapter-pi`
- `effigy package:verify-affected swallowtail-adapter-pi`
- `effigy qa:routes`
- `effigy qa:northstar`
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if card 254 did not name compatible-extension
- stop if live provider work would be required to close the claim
- stop if `0.84.2` is no longer the official stable point

## Auto-Continuation

No. After closeout, reassess remaining Research 127 families one at a
time and qualify useful-newer support; do not leave the current
host/official stable unqualified. Gemini stays deferred.

## Evidence

- Research 140
- `crates/swallowtail-adapter-pi/tests/fixtures/pi-rpc-0.84.2/`
- Decoder specimen remains `pi-rpc-0.80.10`
- Frozen corpus remains `pi-rpc-0.80.10-0.83.0`
- latest qualified = `0.84.2`
- v0.84.0 behavior = `pi.rpc.strict-lf-v0.84.0-message-update-delta`
- synthetic later-stable UnverifiedNewer is `0.84.3`
