# 009 Host-Local Watcher Registry

Status: planned
Owner: Tom
Created: 2026-08-28
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: card 008

## Goal

Implement the Contract 059 host-owned registry and joined local lifecycle
behind the portable core.

## Scope

Bind host-authorized start requests, bounded output summaries, status, wait,
model and operator stop, cancellation, deadline, process-tree cleanup, and
join in `swallowtail-host-local`. Reuse existing task, time, and process
authority; do not expose command or PID data publicly.

## Acceptance Criteria

- [ ] registration alone starts nothing
- [ ] rejected starts perform no work
- [ ] every accepted watcher is joined before turn cleanup
- [ ] bounds and backpressure fail safely
- [ ] races never stop foreign work

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy package:verify-affected swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `git diff --check`

## Auto-Continuation

No. Remains planned until card 008 lands.
