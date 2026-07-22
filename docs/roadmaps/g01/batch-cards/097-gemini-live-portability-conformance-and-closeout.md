# 097 Gemini Live Portability Conformance And Closeout

Status: completed
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

- [x] both topologies preserve exact instance, host, audience, access, preview,
      route, model, formats, rollover, and cleanup truth
- [x] the first turn, handoff, and second turn contain no replay or sequence
      ambiguity
- [x] provider warning, rollover failure, disconnect, cancellation, deadline,
      and provider failure remain distinct
- [x] handles remain private and interrupted or uncertain sessions cannot be
      reused
- [x] OpenAI realtime and all prior production drivers remain passing
- [x] roadmap 033 closes with one sole next task

## Completion Evidence

- the unchanged eleventh realtime profile passes with no Gemini branch and the
  bounded-rollover assertions remain a separate provider-neutral pack
- production loopback passes exact two-generation lifecycle under local and
  remote-authoritative execution-host identities
- the first response accepts multiple handle observations, retains only the
  latest, completes after `GoAway`, and hands off without replay
- missing handle, replacement failure, second warning, provider failure,
  unknown event, format drift, disconnect, cancellation, deadline, nonreuse,
  and cleanup failure remain distinct
- normal and failed cleanup join response tasks, timers, and both blocking
  connection generations before the sole credential release
- `effigy qa` passes all 443 runnable repository tests, formatting, workspace
  checks, warnings-denied clippy, and docs QA; three gated probes remain ignored
- doctor remains at the inherited 19 findings with no new oversized file

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
