# 245 Cursor Agent 2026.08 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../078-cursor-agent-2026-08-exact-milestones.md`
Depends on: card 244; Research 135

## Goal

Add exact Cursor Agent milestones `2026.08.04-aaa8809` and
`2026.08.11-e8db854` to the three `cursor-agent.release-date` claims
without inferring the calendar gap.

## Scope

1. Keep July exact points. Add host `2026-08-04`/`aaa8809` and official
   `2026-08-11`/`e8db854`. Advance claim ids to `release-window-3`.
2. Keep behaviors `cursor-agent.catalogue.calendar-release-v1`,
   `cursor-agent.acp-v1.interactive-v1`, and
   `cursor-agent.stream-json.structured-v1`. Keep AllowUnverified.
3. Refresh focused tests, matrices, the Cursor guide, and architecture.
4. Leave later dates visible UnverifiedNewer (synthetic `2026-08-12`).

## Out Of Scope

- npm `cursor-agent`
- Gemini or other Research 127 families
- capturing a live catalogue or prompt
- install, update, or publication

## Acceptance Criteria

- [x] the four exact dates classify as Qualified Maintained
- [x] dates between those points remain incompatible
- [x] `2026-08-12` remains permitted UnverifiedNewer
- [x] mismatched builds on qualified dates remain rejected
- [x] focused Cursor proof and package verify pass
- [x] matrices and guides name the new exact points

## Validation

- `effigy validate:focused swallowtail-adapter-cursor`
- `effigy package:verify-affected swallowtail-adapter-cursor`
- `effigy qa:routes`
- `effigy qa:northstar`
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if card 244 did not name exact-milestones
- stop if live provider work would be required to close the claim
- stop if `2026.08.11-e8db854` is no longer the official stable point

## Auto-Continuation

No. After closeout, reassess remaining Research 127 families one at a time
and qualify useful-newer support; do not leave the current host/official
stable unqualified. Gemini stays deferred.

## Evidence

- Research 135
- `CURSOR_AGENT_LATEST_QUALIFIED_VERSION` = `2026-08-11`
- `CURSOR_AGENT_LATEST_QUALIFIED_BUILD_REVISION` = `e8db854`
- host milestone `2026-08-04`/`aaa8809`
- Decoder specimen remains `cursor-agent-2026.07.01-41b2de7`
- synthetic later-stable UnverifiedNewer is `2026-08-12`
