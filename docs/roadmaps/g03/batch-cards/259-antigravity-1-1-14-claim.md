# 259 Antigravity 1.1.14 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../085-antigravity-1-1-14-useful-newer.md`
Depends on: card 258; Research 142

## Goal

Raise the `antigravity-cli.release` qualified ceiling from exact `1.1.9`
to `1.1.14` on both catalogue and headless claims. Keep `1.1.8`
incompatible. Reuse existing behavior revisions.

## Scope

1. Replace exact `1.1.9` with Maintained `1.1.9..=1.1.14` on
   `antigravity.catalogue.release-window-1` and
   `antigravity.headless.release-window-1`. Keep AllowUnverified.
2. Keep `1.1.8` incompatible. Move synthetic later-stable UnverifiedNewer
   to `1.1.15`.
3. Refresh focused tests, matrices, the Antigravity guide, architecture,
   and contracts that name the ceiling.

## Out Of Scope

- mapping `--input-format` or Gemini API-key sign-in
- Gemini CLI requalification
- capturing a live catalogue or print prompt
- install, update, or publication

## Acceptance Criteria

- [x] published `1.1.10` through `1.1.14` classify as Qualified
- [x] `1.1.8` remains incompatible
- [x] `1.1.15` remains permitted UnverifiedNewer
- [x] decoder specimen remains `antigravity-cli-1.1.9`
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

- stop if card 258 did not name compatible-extension
- stop if live provider work would be required to close the claim
- stop if `1.1.14` is no longer the official stable point

## Auto-Continuation

No. After closeout, Research 127 AllowUnverified families except deferred
Gemini sit on current official stables. Gemini stays deferred.

## Evidence

- Research 142
- `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.14/`
- Decoder specimen remains `antigravity-cli-1.1.9`
- latest qualified = `1.1.14`
- catalogue behavior = `antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1`
- headless behavior = `antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1`
- synthetic later-stable UnverifiedNewer is `1.1.15`
