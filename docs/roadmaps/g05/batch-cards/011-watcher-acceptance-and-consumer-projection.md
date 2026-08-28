# 011 Watcher Acceptance And Consumer Projection

Status: planned
Owner: Tom
Created: 2026-08-28
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: card 010

## Goal

Close the first Contract 059 route with deterministic lifecycle, failure,
consumer activity, and integration guidance.

## Scope

Prove multiple bounded watchers, explicit wait, both stop paths, completion
races, hook rejection, cancellation, deadline, provider failure, summary
redaction, joined cleanup, and unchanged omission. Update shared route and
feature documentation only after the proof passes.

## Acceptance Criteria

- [ ] no successful turn with active or unjoined watchers
- [ ] consumer activity is ordered, bounded, and truthful
- [ ] raw logs, commands, paths, environment, and PIDs stay private
- [ ] failure classification and cleanup remain exact
- [ ] guide and matrix claims match the route fixture

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent swallowtail-host-local swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-claude-agent swallowtail-host-local swallowtail-testkit`
- `effigy qa:docs`
- `git diff --check`

## Auto-Continuation

No. Remains planned until card 010 lands.
