# 019 Codex Operation-Local Child Activity Ownership

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../007-codex-operation-local-child-activity-ownership.md`
Depends on: card 009

## Goal

Repair the consumer-proven Codex child-envelope rejection with bounded,
operation-local ownership derived only from trusted spawn topology.

## Scope

1. Retain the root provider thread as the ordinary activity owner.
2. Admit exact child thread ids from successfully projected `spawnAgent`
   topology, under one fixed per-operation bound.
3. Attribute activity whose envelope carries an admitted child id to that
   child.
4. Keep root output, turn, terminal, callback, provider-request, and session
   checks unchanged.
5. Clear admitted children when the operation terminates.
6. Add root, admitted child, foreign, cross-operation, cleanup, and envelope
   corpus coverage.

## Acceptance Criteria

- [x] root ordinary activity still projects
- [x] child activity is admitted only after exact spawn evidence
- [x] child activity carries `ActivityActor::Subagent`
- [x] missing parent evidence remains `SubagentParent::Unknown`
- [x] unknown or another operation's child id fails closed
- [x] terminal cleanup empties admission state
- [x] no authenticated, provider, or consumer effect runs
- [x] Cursor card 012 returns as the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-adapter-codex`
- `effigy package:verify-affected swallowtail-adapter-codex`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy format:check`
- `git diff --check`
- no broad workspace or authenticated provider suite

## Auto-Continuation

No. Return to Cursor card 012 after deterministic closeout.

## Evidence

- one completed successful spawn admits its exact receiver ids
- the operation-local admission set is bounded at 256 and cleared by `finish`
- child envelopes are attributed without becoming root output
- root terminal and provider-request ownership remain unchanged
- the `0.146.0` corpus contains a top-level child-owned envelope
- 139 focused Codex tests passed
- the extracted Codex package compiled
- no live provider, authentication, installation, or consumer effect ran
