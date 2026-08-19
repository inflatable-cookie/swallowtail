# 323 Antigravity 1.1.15 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../105-antigravity-1-1-15-useful-newer.md`
Depends on: card 322; Research 167

## Goal

Raise the `antigravity-cli.release` qualified ceiling from `1.1.14` to
`1.1.15` on both catalogue and headless claims. Keep `1.1.8`
incompatible. Reuse existing behavior revisions.

## Scope

1. Extend Maintained `1.1.9..=1.1.15` on
   `antigravity.catalogue.release-window-1` and
   `antigravity.headless.release-window-1`. Keep AllowUnverified.
2. Keep `1.1.8` incompatible. Move synthetic later-stable UnverifiedNewer
   to `1.1.16`.
3. Refresh focused tests, matrices, the Antigravity guide, architecture,
   and contracts that name the ceiling.

## Out Of Scope

- mapping `--input-format` or Gemini API-key sign-in
- Gemini CLI requalification
- capturing a live catalogue or print prompt
- install, update, or publication

## Acceptance Criteria

- [x] official `1.1.15` classifies as Qualified Maintained
- [x] `1.1.8` remains incompatible
- [x] `1.1.16` remains permitted UnverifiedNewer
- [x] decoder specimen remains `antigravity-cli-1.1.9`
- [x] `1.1.14` specimens remain
- [x] focused Antigravity proof and package verify pass
- [x] matrices and guides name the new release ceiling

## Validation

- `effigy validate:focused swallowtail-adapter-antigravity`
- `effigy package:verify-affected swallowtail-adapter-antigravity`
- `effigy qa:routes`
- `effigy qa:northstar`
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if card 322 did not name compatible-extension
- stop if live provider work would be required to close the claim
- stop if `1.1.15` is no longer the official stable point

## Auto-Continuation

No. After closeout, Research 159 AllowUnverified families except deferred
Gemini sit on current official stables. Gemini stays deferred.

## Evidence

- Research 167
- `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.15/`
- Decoder specimen remains `antigravity-cli-1.1.9`
- latest qualified = `1.1.15`
- catalogue behavior = `antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1`
- headless behavior = `antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1`
- synthetic later-stable UnverifiedNewer is `1.1.16`
