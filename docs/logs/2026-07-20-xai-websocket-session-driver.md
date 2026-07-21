# xAI WebSocket Session Driver

Date: 2026-07-20
Roadmap: 017
Card: 053

## Changed

- added resource-free interactive-session policy and open-request records
  without weakening existing resource-bound sessions
- added typed exact provider-billed-cost evidence with currency scale,
  cumulative-replacement semantics, turn, route, access, and attempt scope
- implemented the xAI Responses WebSocket interactive-session driver over
  host-approved blocking network and credential services
- implemented one session-owned connection, serial turns, private latest-
  response continuation, ordered output, usage, billed cost, cancellation,
  deadlines, provider failures, disconnect invalidation, and joined cleanup
- split preflight validation, billed-cost tests, and turn pumping along semantic
  boundaries to avoid new oversized-file debt

## Boundaries

- exact `/v1/responses` endpoint grant and public API-key lease only
- resource-free direct inference; no process or working-resource service
- one active turn and one provider attempt; no retry, reconnect, replay,
  storage, resume, model catalogue, tools, search, or fallback
- response ids remain driver-private and cannot become resume bindings
- cancellation and timeout close the connection and invalidate continuation
- provider messages, bearer material, prompts, response ids, and raw payloads
  do not enter stable diagnostics

## Evidence

- six production-driver fixtures prove the exact bearer upgrade, two chained
  turns, cost scope, concurrent rejection, cancellation, timeout, disconnect,
  continuation loss, connection lifetime failure, redaction, and lease order
- 91 focused core, runtime, testkit, protocol, and xAI tests pass
- all workspace targets compile
- focused strict clippy, formatting, and diff checks pass
- task-join evidence precedes credential release; concurrent and invalid-
  continuation assertions observe no second provider frame
- doctor remains at the pre-existing 19 findings: 12 warnings and 7 errors
- no provider credential, external inference request, or live network test was
  used

## Risks

- the WebSocket evidence remains tied to the dated official-guide snapshot,
  not a versioned xAI protocol release
- no default model catalogue exists; callers must bind an exact route from
  separately governed configuration
- connection-limit and lost-continuation errors invalidate the session; there
  is deliberately no recovery policy
- live authentication, service-side cancellation timing, rate limits, and
  provider connection lifetime remain unverified by deterministic fixtures

## Continuation

Card 054 is ready. Add the provider-neutral connection-scoped direct-session
profile, prove local and remote-authoritative topology against this driver, run
full QA, and close roadmap 017. Kimi ACP and owned llama.cpp remain the bounded
post-roadmap choices.
