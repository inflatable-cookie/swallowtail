# 257 Qwen Headless 0.21.13 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../084-qwen-headless-0-21-13-useful-newer.md`
Depends on: card 256; Research 141

## Goal

Raise the `qwen-code.package` qualified ceiling from `0.21.2` to
`0.21.13`. Keep unpublished stable `0.20.2` incompatible. Reuse
catalogue-filter.

## Scope

1. Extend Maintained `0.21.0..=0.21.13` on
   `qwen-code.headless.v0.21.0-catalogue-filter`. Keep AllowUnverified.
2. Keep unpublished stable `0.20.2` incompatible. Move synthetic
   later-stable UnverifiedNewer to `0.21.14`.
3. Refresh focused tests, matrices, the Qwen guide, architecture, and
   contracts that name the ceiling.

## Out Of Scope

- mapping `goal_state`, initialize `effort_status`, `--continue`, or ACP
  session restore
- Gemini or other Research 127 families
- capturing a live catalogue or headless prompt
- install, update, or publication

## Acceptance Criteria

- [x] published `0.21.3` through `0.21.13` classify as Qualified
- [x] unpublished stable `0.20.2` remains incompatible
- [x] `0.21.14` remains permitted UnverifiedNewer
- [x] decoder specimen remains `qwen-code-v0.19.11`
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

- stop if card 256 did not name compatible-extension
- stop if live provider work would be required to close the claim
- stop if `0.21.13` is no longer the official stable point

## Auto-Continuation

No. After closeout, reassess remaining Research 127 families one at a
time and qualify useful-newer support; do not leave the current
host/official stable unqualified. Gemini stays deferred.

## Evidence

- Research 141
- `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.21.13/`
- Decoder specimen remains `qwen-code-v0.19.11`
- Frozen corpus remains `qwen-code-v0.19.11-v0.21.2`
- latest qualified = `0.21.13`
- behavior = `qwen-code.headless.v0.21.0-catalogue-filter`
- synthetic later-stable UnverifiedNewer is `0.21.14`
