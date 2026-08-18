# 241 Claude Code Response-Only 2.1.234 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../076-claude-code-response-only-2-1-234-provisional.md`
Depends on: card 239; Research 133 (operator-corrected)

## Goal

Raise `claude-code.response-only-stream-json` from qualified
`2.1.227..=2.1.228` to `2.1.227..=2.1.234` as a compatible extension of
`claude-code.response-only.stream-json.v1`.

## Scope

1. Extend the existing Maintained segment through `2.1.234`. Keep baseline
   `2.1.227`, AllowUnverified, empty deny-list, and fail-closed protocol
   validation.
2. Move the synthetic UnverifiedNewer point to `2.1.235`.
3. Refresh focused tests, matrices, guide, architecture, and Contract 039
   ceiling text.
4. Keep headless `2.1.220..=2.1.234` on its own axis.

## Out Of Scope

- mixing axes
- a live response-only prompt
- Oh My Pi, Gemini, or other 127 families
- install, update, or publication

## Acceptance Criteria

- [x] `2.1.227..=2.1.234` classifies as Qualified Maintained
- [x] `2.1.226` remains incompatible
- [x] `2.1.235` remains permitted UnverifiedNewer
- [x] deny-list stays empty
- [x] headless claim stays `2.1.220..=2.1.234`
- [x] focused Claude Code proof and package verify pass
- [x] matrices and guides name the new response-only ceiling

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy qa:routes`
- `effigy qa:northstar`
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if live provider work would be required to close the claim
- stop if `2.1.234` is no longer the official stable point

## Auto-Continuation

No. After closeout, reassess remaining Research 127 families one at a time
and qualify useful-newer support; do not leave the current host/official
stable unqualified. Gemini stays deferred.

## Evidence

- Research 133 (operator-corrected)
- `CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION` = `2.1.234`
- Behavior: `claude-code.response-only.stream-json.v1`
- Decoder specimens remain `2.1.227` / `2.1.228`
- synthetic later-stable UnverifiedNewer is `2.1.235`
