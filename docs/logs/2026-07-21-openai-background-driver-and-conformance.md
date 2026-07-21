# 2026-07-21 OpenAI Background Driver And Conformance

## Outcome

Completed card 075 and roadmap 023. The OpenAI public Responses background
route is now a production Swallowtail driver with deterministic hosted-direct
conformance. Card 076 is the sole ready continuation.

## Changed

- added the separately registered `swallowtail.openai.background` structured-
  run and direct-inference driver over `http-sse-background`
- bound one exact host, `api.openai.com` endpoint audience, public API-key
  lease, provider/model route, positive output bound, explicit deadline,
  background execution, temporary retention, and one maximum reattachment
- retained endpoint and credential authority across create, stream,
  reattachment, retrieve, and cancel management without ambient reacquisition
- required `response.created` before returning a handle, keeping the provider
  run reference distinct from local runtime, task, attachment, and request ids
- implemented one create attempt, one cursor reattachment, one bounded terminal
  retrieve after recovery exhaustion, and no inference retry
- projected ordered output, terminal status, cumulative usage, rate state,
  request correlation, provider failures, cancellation, deadline, and cleanup
- preserved confirmed, completion-raced, and unconfirmed provider cancellation
  separately from the local terminal winner
- rejected cursor duplicate or gap, response mismatch, phase regression,
  unknown semantic event, malformed event, output mismatch, and unsafe payload
  before claiming success
- added local and remote-authoritative production fixtures with joined network,
  task, and credential cleanup
- opened roadmap 024 and card 076 as an evidence-only post-background
  checkpoint

## Authority And Bounds

- provider-supported OpenAI public Responses API only
- no ChatGPT, Codex, harness, subscription OAuth, or community OAuth reuse
- no provider, endpoint, model, credential, billing, retention, retry,
  topology, or support-authority fallback
- no durable detach or resume, tools, search, files, structured output,
  conversations, webhooks, Batch API, or default live access
- `store=false` does not remove required temporary provider retention
- default validation uses no credential, account, external request, or paid
  inference

## Validation

- 14 focused OpenAI protocol, loopback, production-driver, and hosted-direct
  conformance tests pass
- focused OpenAI warnings-denied clippy passes
- full repository QA passes with 314 tests; three installed/live probes remain
  gated and ignored by default
- `git diff --check` passes
- `effigy doctor` remains at the inherited 19 oversized-file findings: 12
  warnings and 7 errors

## Remaining Risks

- a disconnect before `response.created` leaves remote state unconfirmed; the
  driver fails start without claiming provider cancellation
- cancellation can remain unconfirmed after all local work is cleanly joined
- temporary retention duration and provider event, status, header, or schema
  drift require a new evidence date and fixture delta
- the first subset requires a deadline and permits only one reattachment; it
  does not claim durable background-job management
- live authentication remains separately gated

## Continuation Record

Card 076 is the sole ready task. Re-rank remaining cloud-harness, direct API,
shared-protocol, SDK, catalogue, and attached-runtime gaps from current
authoritative evidence. Keep harness isolation optional. Return to the operator
before implementation if the leading route would establish repository,
remote-mutation, durable-deletion, credential, routing, or deployment policy.
