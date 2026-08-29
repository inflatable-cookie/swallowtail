# 009 Host-Local Watcher Registry

Status: complete
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-29
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
wait, model and operator stop, cancellation, deadline, and join in
`swallowtail-host-local`. Keep lifecycle coordination separate from execution
containment. A process-backed start requires an explicitly supplied Contract
059 containment backend before work; the default local process service and its
process groups do not qualify. Do not expose command, path, argument,
environment, raw output, or PID data publicly.

Repair and restack PR 117 against Research 259 and the 2026-08-29 Contract 059
decision. Preserve its reviewed registry, wait, retirement, rollback, and
wake-up work where still valid. Remove the default process-backed capability
claim and any process-table observation presented as containment evidence.

## Acceptance Criteria

- [x] registration alone starts nothing
- [x] rejected starts perform no work
- [x] start operation data is bounded and redacted; it is never executable or
      process authority
- [x] caller input cannot forge host-selected progress or terminal summaries
- [x] process-backed start is absent or rejected before work when no exact
      containment backend is supplied
- [x] every accepted process-backed watcher binds its containment lease before
      the watcher id returns
- [x] stop, cancellation, deadline, failure, and close prove the containment
      scope empty and supervision joined before clean turn cleanup
- [x] a root handle, process group, observed parent chain, output pipe, or
      `/bin/ps` poll is never treated as containment evidence
- [x] task-backed work cannot create unmanaged descendants or binds them to the
      same containment lease
- [x] wait resolves only after terminal and joined truth
- [x] bounds and backpressure fail safely
- [x] races never stop foreign work
- [x] default `LocalHostServices` composition makes no process-backed watcher
      claim on macOS

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy package:api`
- `effigy qa:northstar`
- `git diff --check`

## Stop Conditions

- the implementation needs to infer containment from an operating system,
  process group, polling observer, PID, or executable behavior
- the execution seam cannot represent an injected host containment lease
  without provider-, route-, or consumer-specific portable vocabulary
- start operation data requires a provider-, route-, or consumer-specific
  public command schema
- correctness requires raw output, executable data, paths, environment, or
  PIDs in portable records
- cancellation, deadline, or force-stop cannot reach joined cleanup truth

## Auto-Continuation

No. Return one reviewable, restacked PR 117. On successful closeout, keep card
010 gated until an exact containment-capable host composition is selected and
proved. Keep card 011 planned behind card 010.
