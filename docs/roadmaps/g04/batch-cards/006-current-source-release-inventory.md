# 006 Current Source Release Inventory

Status: ready
Owner: Tom
Created: 2026-08-19
Milestone: `../003-current-source-tag-before-readiness.md`

## Goal

Freeze the unreleased package, route, and public-API delta from `v0.3.2` and
classify `0.3.3` versus `0.4.0` under Contract 036.

## Scope

1. Inventory current-source packages and production routes.
2. Compare semantic API baselines for existing packages.
3. Keep OpenHands as a package without a production route.
4. Record the selected coordinated version before changelog mutation.

## Out Of Scope

- readiness facade types
- tag or remote mutation
- publication
- changing provider claims

## Acceptance Criteria

- [ ] package and route counts match architecture and Contract 036
- [ ] patch versus minor is explicit
- [ ] no facade implementation is in the candidate set

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy qa:routes`
- `git diff --check`

## Auto-Continuation

Yes, into card 007 after the version class is recorded.
