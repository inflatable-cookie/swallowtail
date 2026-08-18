# 247 OpenCode HTTP 1.18.18 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../079-opencode-http-1-18-18-useful-newer.md`
Depends on: card 246; Research 136

## Goal

Raise the `opencode.server` qualified ceiling from published segments
through `1.18.10` to published segments through `1.18.18`, adding private
`surface-19` for the selected Model interleaved-field closure change.

## Scope

1. Keep `1.18.0..=1.18.10` on `surface-18`. Add `1.18.11..=1.18.18` on
   `surface-19`. Keep baseline `1.14.48` and AllowUnverified.
2. Extend delete, import, continuity, callback, runtime, and
   reconciliation corpora through `1.18.18`. Do not infer unpublished
   gaps.
3. Refresh focused tests, matrices, the OpenCode guide, architecture, and
   contracts that name the ceiling.
4. Leave later stables visible UnverifiedNewer (synthetic `1.18.19`).

## Out Of Scope

- OpenCode ACP
- Gemini or other Research 127 families
- capturing a live prompt or starting the attached server
- install, update, or publication

## Acceptance Criteria

- [x] published `1.18.11..=1.18.18` classify as Qualified Maintained
- [x] `1.15.8` and other unpublished gaps remain incompatible
- [x] `1.18.19` remains permitted UnverifiedNewer
- [x] decoder specimen remains `opencode-1.14.48`
- [x] focused OpenCode proof and package verify pass
- [x] matrices and guides name the new package ceiling

## Validation

- `effigy validate:focused swallowtail-adapter-opencode`
- `effigy package:verify-affected swallowtail-adapter-opencode swallowtail-testkit`
- `effigy qa:routes`
- `effigy qa:northstar`
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if card 246 did not name compatible-extension
- stop if live provider work or server start would be required to close
  the claim
- stop if `1.18.18` is no longer the official stable point

## Auto-Continuation

No. After closeout, reassess remaining Research 127 families one at a time
and qualify useful-newer support; do not leave the current host/official
stable unqualified. Gemini stays deferred.

## Evidence

- Research 136
- `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.18/`
- Decoder specimen remains `opencode-1.14.48`
- `OPENCODE_LATEST_QUALIFIED_VERSION` = `1.18.18`
- Behavior: `opencode.http-sse.surface-19` for `1.18.11..=1.18.18`
- synthetic later-stable UnverifiedNewer is `1.18.19`
