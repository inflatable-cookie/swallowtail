# 005 Readiness Contract Targets

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../002-route-readiness-spec-and-contract-targets.md`
Depends on: card 004

## Goal

Name the later contract and any 047/006/008 amendment bounds without writing
the contract text as executable authority.

## Scope

1. Decide new-contract versus amendment for addable routes, sign-in loop,
   store port, subject observation, and overlay.
2. Keep 047 a selection snapshot.
3. Leave implementation cards planned until g04.003 tags.

## Out Of Scope

- implementing records or adapters
- version or tag mutation
- marking g04.004+ ready

## Acceptance Criteria

- [x] architecture notes the lifecycle placement
- [x] contract ids or amendment bounds are named
- [x] g04.003 remains the implementation gate

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Continue through g04.003 only after this roadmap closes.

## Evidence

System architecture planned connection-lifecycle section. Spec 011 names a new
contract after 056 and seam amendments to 006, 008, 010, 014, 015, 017, 029,
032, 037, and 047.
