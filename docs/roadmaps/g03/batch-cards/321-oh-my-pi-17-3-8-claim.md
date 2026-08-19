# 321 Oh My Pi 17.3.8 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../104-oh-my-pi-17-3-8-useful-newer.md`
Depends on: card 320; Research 166

## Goal

Raise the `oh-my-pi.package` qualified ceiling from exact `17.3.7` to
exact `17.3.8` as a compatible extension of `oh-my-pi.rpc-v2-v17.2.9`.

## Scope

1. Extend the existing Maintained segment through `17.3.8`. Keep baseline
   `17.2.9` and AllowUnverified.
2. Keep behavior `oh-my-pi.rpc-v2-v17.2.9`. Do not add a milestone
   revision.
3. Refresh focused tests, matrices, the Oh My Pi guide, architecture, and
   Contract 029 ceiling text.
4. Leave later stables visible UnverifiedNewer (synthetic `17.3.9`).

## Out Of Scope

- `pi.package`
- Gemini or Antigravity
- capturing a live RPC prompt
- install, update, or publication

## Acceptance Criteria

- [x] `17.2.9..=17.3.8` classifies as Qualified Maintained
- [x] `17.2.8` remains incompatible
- [x] `17.3.9` remains permitted UnverifiedNewer
- [x] decoder specimen remains `oh-my-pi-rpc-17.2.9`
- [x] `17.3.7` specimens remain
- [x] focused Oh My Pi proof and package verify pass
- [x] matrices and guides name the new package ceiling

## Validation

- `effigy validate:focused swallowtail-adapter-oh-my-pi`
- `effigy package:verify-affected swallowtail-adapter-oh-my-pi`
- `effigy qa:routes`
- `effigy qa:northstar`
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if card 320 did not name compatible-extension
- stop if live provider work would be required to close the claim
- stop if `17.3.8` is no longer the official stable point

## Auto-Continuation

No. After closeout, implement Antigravity `1.1.15` useful-newer
qualification. Gemini stays deferred.

## Evidence

- Research 166
- `crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-17.3.8/`
- Decoder specimen remains `oh-my-pi-rpc-17.2.9`
- `OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION` = `17.3.8`
- Behavior: `oh-my-pi.rpc-v2-v17.2.9`
- synthetic later-stable UnverifiedNewer is `17.3.9`
