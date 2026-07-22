# 097 Gemini Live Portability Conformance And Closeout

Status: planned
Owner: Tom
Updated: 2026-07-22
Milestone: `../033-gemini-live-realtime-portability-proof.md`

## Objective

Prove Gemini Live against the unchanged realtime-media profile plus the bounded
rollover assertions under both execution-host topologies, then close roadmap
033 without live provider access.

## Readiness Gate

Card 096 must implement the exact production driver and pass deterministic
protocol and lifecycle fixtures.

## Scope

- eleventh provider-neutral profile unchanged
- bounded planned-rollover assertion pack against production
- local and remote-authoritative host identities
- two-turn success across rollover, active-response warning, missing handle,
  exhaustion, replacement failure, provider failure, unknown event, format
  drift, unexpected disconnect, cancellation, deadline, and cleanup failure
- exact old/new connection, task, blocking-work, timer, and credential join
  order
- architecture, roadmap, front-door, and log closeout
- one meaningful full repository validation round

## Acceptance Criteria

- [ ] both topologies preserve exact instance, host, audience, access, preview,
      route, model, formats, rollover, and cleanup truth
- [ ] the first turn, handoff, and second turn contain no replay or sequence
      ambiguity
- [ ] provider warning, rollover failure, disconnect, cancellation, deadline,
      and provider failure remain distinct
- [ ] handles remain private and interrupted or uncertain sessions cannot be
      reused
- [ ] OpenAI realtime and all prior production drivers remain passing
- [ ] roadmap 033 closes with one sole next task

## Validation

- focused Gemini and realtime-media tests
- focused warnings-denied clippy
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- the production driver needs provider branches in the shared profile
- rollover assertions imply general reconnect or consumer resume
- cleanup can release credentials before both connection generations join
- conformance requires a credential, paid request, browser, or audio device

## Auto-Continuation

No. Close the proof and return to the provider-coverage checkpoint.
