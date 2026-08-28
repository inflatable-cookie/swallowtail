# 009 Host-Local Watcher Registry

Status: ready
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: completed card 008

## Goal

Implement the Contract 059 host-owned registry and joined local lifecycle
behind the portable core.

## Scope

First correct the pre-1.0 portable start seam: replace caller-supplied start
`WatcherSummary` with bounded redacted operation data that grants no launch
authority and is interpreted under host policy. Only the host may select
progress or terminal summaries.

Then bind host-authorized start requests, bounded output summaries, status,
wait, model and operator stop, cancellation, deadline, process-tree cleanup,
and join in `swallowtail-host-local`. Reuse and strengthen the existing task,
time, and process authority where needed; do not expose command, path,
argument, environment, raw output, or PID data publicly.

## Acceptance Criteria

- [ ] registration alone starts nothing
- [ ] rejected starts perform no work
- [ ] start operation data is bounded and redacted; it is never executable or
      process authority
- [ ] caller input cannot forge host-selected progress or terminal summaries
- [ ] every accepted watcher and owned descendant is stopped and joined before
      turn cleanup
- [ ] wait resolves only after terminal and joined truth
- [ ] bounds and backpressure fail safely
- [ ] races never stop foreign work

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy package:api`
- `git diff --check`

## Stop Conditions

- the existing local process boundary cannot guarantee owned-descendant stop
  and join on supported local hosts without a new contract decision
- start operation data requires a provider-, route-, or consumer-specific
  public command schema
- correctness requires raw output, executable data, paths, environment, or
  PIDs in portable records
- cancellation, deadline, or force-stop cannot reach joined cleanup truth

## Auto-Continuation

No. Return one reviewable PR. On successful closeout, make card 010 ready but
do not execute it. Keep card 011 planned until card 010 lands and Research 257
stays admitted.
