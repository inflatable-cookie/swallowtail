# 073 xAI One-Response Structured Run

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../022-structured-run-projection-and-direct-coverage.md`

## Objective

Expose the first xAI Responses WebSocket response as one bounded structured
run without inheriting continuation.

## Scope

1. Register and prepare an independent structured role on the exact WebSocket
   route.
2. Open one approved connection, send one `response.create`, stream to
   terminal state, then close.
3. Send no previous-response id, second turn, retry, reconnect, or recovery.
4. Preserve exact usage, billed-cost, cancellation, deadline, failure, and
   credential-last cleanup evidence.
5. Add both host topologies and full interactive-session regression.

## Acceptance Criteria

- [x] one response only
- [x] no connection or session binding escapes the run
- [x] `store=false` and prohibited retention remain exact
- [x] billed cost and usage remain distinct
- [x] cancellation invalidates and joins the connection
- [x] interactive xAI continuation remains unchanged

## Evidence

- The descriptor registers independent structured and interactive roles on the
  exact Responses WebSocket facade.
- `prepare_responses_run` binds one resource-free request to the selected
  model, endpoint, access evidence, and execution host.
- The structured driver opens one connection, sends one `response.create`
  with `store=false` and no previous-response id, streams one terminal
  response, then closes and joins all connection work.
- The run exposes no provider run or session reference. Usage and billed cost
  remain separate observations correlated to an operation-private response id.
- Cancellation, deadlines, unsupported-input rejection, local and
  remote-authoritative hosts, and credential-last cleanup have deterministic
  fixture coverage.
- The existing two-turn private-continuation session suite passes unchanged.

## Validation Evidence

- `cargo test -p swallowtail-adapter-xai`
- `cargo clippy -p swallowtail-adapter-xai --all-targets -- -D warnings`
- `effigy qa:docs`
- `effigy qa:routes`
- `git diff --check`

## Validation

- focused xAI fixtures and conformance
- strict adapter Clippy
- route and docs checks
- `git diff --check`

## Stop Conditions

- a terminal response cannot be distinguished from connection close
- provider failure would permit unsafe continuation or retry
- fixtures require a live xAI credential

## Auto-Continuation

Yes. Continue to card 074.
