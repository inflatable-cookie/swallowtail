# 075 OpenAI Background Driver And Conformance

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../023-openai-background-responses-proof.md`

## Objective

Implement the bounded OpenAI Responses background route against Contract 021
and card 074's corpus, prove hosted-direct conformance, and close roadmap 023.

## Scope

- one separately registered public-API background structured-run driver
- exact host-approved endpoint, endpoint audience, API-key lease, configured
  model route, positive output bound, and retention selection
- one provider inference attempt across create, bounded stream reattachment,
  and cancel management requests
- ordered output, terminal status, usage, rate, request correlation, provider
  failure, cancellation, deadline, disconnect, and cleanup projection
- local and remote-authoritative deterministic host fixtures
- no detached local work and endpoint/credential release only after joined
  HTTP/SSE cleanup
- separately gated installed or live probe only with explicit operator access
- roadmap, architecture, front-door, and log closeout

## Acceptance Criteria

- [x] no OpenAI branch enters provider-neutral core or runtime behavior
- [x] the descriptor claims only the background subset proven by card 074
- [x] connection loss does not start another inference attempt or hide replay
- [x] confirmed and unconfirmed provider cancellation remain distinguishable
- [x] local and remote-authoritative hosts pass the same public seam
- [x] default QA uses no live credential or paid inference
- [x] full QA passes or failures are recorded honestly

## Evidence

- `swallowtail.openai.background` registers only structured-run and direct-
  inference roles over `http-sse-background`
- preflight requires the exact public endpoint audience, API-key lease, model
  route, output bound, deadline, background execution, temporary retention,
  and one maximum reattachment before host effects
- one create attempt survives one cursor reattachment; recovery exhaustion
  performs one bounded retrieve instead of retrying inference
- semantic SSE order, cursor continuity, response correlation, output
  agreement, terminal status, usage, rate, and request correlation fail closed
- cancel fixtures preserve confirmed, completion-raced, and unconfirmed
  provider truth; deadline remains a distinct local terminal winner
- local and remote-authoritative fixtures join HTTP/SSE and operation work
  before credential release
- 14 focused OpenAI tests pass without live access; full repository QA passes
  with 314 tests
- warnings-denied focused clippy and `git diff --check` pass
- `effigy doctor` remains at the inherited 19 findings: 12 warnings and 7
  errors

## Validation

- focused adapter and hosted-direct conformance tests
- warnings-denied focused clippy
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Stop Conditions

- card 074 is incomplete
- implementation exposes a missing shared contract
- current provider behavior drifts from the frozen corpus
- cleanup cannot join local work before credential release

## Auto-Continuation

No. Roadmap 023 is complete. Card 076 owns the subsequent evidence checkpoint.
