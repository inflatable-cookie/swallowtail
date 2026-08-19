# 309 Codex 0.148.0 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../098-codex-0-148-useful-newer.md`
Depends on: card 308; Research 160

## Goal

Raise the `codex.cli` qualified ceiling from `0.147.0` to official
`0.148.0` on exec, app-server, lifecycle, and thread-catalogue claims.
Keep later stables AllowUnverified. Reuse existing behavior revisions.

## Scope

1. Raise `CODEX_LATEST_QUALIFIED_VERSION` to `0.148.0`. Keep AllowUnverified
   and existing gaps.
2. Move synthetic later-stable UnverifiedNewer to `0.148.1`.
3. Refresh focused tests, matrices, the Codex guide, Contract 048's moving
   thread-catalogue ceiling, and CHANGELOG Unreleased.

## Out Of Scope

- mapping `exec fork`, top-level `fork`, `thread/fork`, or Bedrock
- replacing the host Codex install
- Claude Agent or other 159 families
- provider prompts, live sessions, install, update, or publication

## Acceptance Criteria

- [x] official `0.148.0` classifies as Qualified Maintained
- [x] host `0.147.0` remains Qualified
- [x] `0.148.1` remains permitted UnverifiedNewer
- [x] existing gaps stay incompatible
- [x] decoder specimens stay
- [x] focused Codex proof and package verify pass
- [x] matrices and guides name the new release ceiling

## Validation

- `effigy validate:focused swallowtail-adapter-codex`
- `effigy package:verify-affected swallowtail-adapter-codex`
- `effigy qa:routes`
- `effigy qa:northstar`
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if card 308 did not name compatible-extension
- stop if live provider work would be required to close the claim
- stop if `0.148.0` is no longer the official stable point

## Auto-Continuation

No. After closeout, implement Claude Agent ACP `0.70.0` useful-newer
qualification. Gemini stays deferred.

## Evidence

- Research 160
- `CODEX_LATEST_QUALIFIED_VERSION` = `0.148.0`
- same behavior revisions as `0.147.0`
- synthetic later-stable UnverifiedNewer is now `0.148.1`
