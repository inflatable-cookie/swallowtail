# 093 Realtime Media Conformance And Closeout

Status: planned
Owner: Tom
Updated: 2026-07-22
Milestone: `../031-openai-realtime-media-direct-session-proof.md`

## Objective

Prove the OpenAI Realtime driver under both execution-host topologies and close
roadmap 031 without live provider access.

## Readiness Gate

Card 092 must implement the exact production driver and pass its deterministic
protocol and lifecycle fixtures.

## Scope

- eleventh provider-neutral profile against the production driver
- local and remote-authoritative host identities
- two-turn success, bounds, parallel rejection, provider failure, unknown
  event, format drift, disconnect, cancellation, deadline, and cleanup failure
- exact connection, task, blocking-work, timer, and credential join order
- full architecture, roadmap, front-door, and log closeout
- one meaningful full repository validation round

## Acceptance Criteria

- [ ] both topologies preserve exact instance, host, audience, access, route,
      model, media format, lifecycle, and cleanup truth
- [ ] two serial turns and one active response match the frozen transcript
- [ ] cancellation acknowledgement, deadline, disconnect, and provider failure
      remain distinct
- [ ] interrupted sessions cannot be reused
- [ ] all prior profiles and production drivers remain unchanged
- [ ] roadmap 031 closes with one sole next task

## Validation

- focused realtime-media and OpenAI tests
- focused warnings-denied clippy
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- topology fixtures require provider-specific branches in the shared profile
- cleanup can release credentials before duplex work joins
- conformance implies device, playback, truncation, reconnect, or resume truth

## Auto-Continuation

No. Close the proof and return to the provider-coverage checkpoint.
