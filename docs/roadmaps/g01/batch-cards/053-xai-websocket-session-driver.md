# 053 xAI WebSocket Session Driver

Status: complete
Owner: Tom
Updated: 2026-07-20
Milestone: `../017-xai-responses-websocket-proof.md`

## Objective

Realize resource-free direct-session records, billed-cost evidence, and the
bounded xAI Responses WebSocket driver against card 052's fixture.

## Governing References

- `../../../contracts/009-async-operation-lifecycle.md`
- `../../../contracts/010-execution-host-services-and-inputs.md`
- `../../../contracts/014-hosted-transport-credential-and-evidence-boundary.md`
- `../../../contracts/016-connection-scoped-direct-sessions-and-billed-cost.md`
- `../017-xai-responses-websocket-proof.md`

## Scope

- provider-neutral optional-resource interactive session records and preflight
- exact provider-billed-cost observation
- one xAI direct-inference interactive driver over approved WebSocket
- serial turns, latest-response continuation, local close cancellation,
  deadlines, connection-limit failure, and joined cleanup

## Out Of Scope

- HTTP model catalogue
- reconnect, replay, retry, storage, resume, tools, search, or live auth

## Ordered Steps

1. Add the smallest Contract 016 core/runtime records and preflight rules.
2. Extend recording fixtures before provider code.
3. Implement the xAI driver only against card 052's frozen manifest.
4. Hold endpoint and credential leases for the session scope.
5. Enforce one active turn and driver-owned continuation.
6. Prove every cancellation and close path joins work before credential release.
7. Prepare card 054 from observed driver behavior.

## Acceptance Criteria

- [x] no fake resource is needed
- [x] shared records contain no xAI identity branch
- [x] exact billed cost is typed, cumulative per attempt, and policy-free
- [x] no provider frame is sent for a concurrent turn
- [x] close, cancellation, deadline, and connection limit leave no detached work

## Validation

- focused core, runtime, testkit, and xAI adapter tests
- focused clippy
- `cargo fmt --all -- --check`
- `git diff --check`

## Evidence Required

- focused test count
- dependency-direction check
- lease order and one-active-turn assertions
- redaction and no-retry proof

## Stop Conditions

- implementation needs implicit endpoint derivation or provider storage
- selected WebSocket dependency cannot satisfy bounded close and join semantics
- public API changes exceed Contract 016

## Auto-Continuation

No. Mark card 054 ready only after focused driver validation passes.

## Evidence

- `SessionAccessPolicy::resource_free` and resource-free open requests encode
  absence directly while preserving the existing resource-bound constructor.
- `BilledCostObservation` carries exact provider-reported USD ticks, cumulative-
  replacement semantics, turn, route, access profile, and attempt identity.
- the production driver holds one approved endpoint grant and API-key lease for
  one serial WebSocket session; response ids remain private driver state
- six production-driver fixtures prove two chained turns, concurrent rejection,
  disconnect, distinct continuation and lifetime failures, cancellation,
  deadline, redaction, lease order, and effect-free resource rejection
- 91 focused core, runtime, testkit, protocol, and xAI tests pass
- all workspace targets compile; focused strict clippy, formatting, and diff
  checks pass
- dependency direction remains provider-neutral; no core, runtime, or testkit
  xAI identity branch exists
- source and frame-count assertions show one provider attempt with no retry,
  reconnect, storage, resume, or fallback path
- doctor remains at the pre-existing 19 findings: 12 warnings and 7 errors

## Continuation

Card 054 is ready. Add the provider-neutral connection-scoped direct-session
profile, run it under local and remote-authoritative hosts, then close roadmap
017 with full QA. Kimi ACP and owned llama.cpp remain in bounds after that
checkpoint.
