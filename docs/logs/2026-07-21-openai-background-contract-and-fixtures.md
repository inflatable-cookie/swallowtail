# 2026-07-21 OpenAI Background Contract And Fixtures

## Outcome

Completed card 074. Contract 021 and the first OpenAI adapter corpus now make
provider-managed background execution explicit without widening ordinary
harness or direct-inference behavior. Card 075 is ready.

## Changed

- added opt-in provider-background, temporary-retention, and stream-
  reattachment capabilities
- added explicit attached/background, retention, and bounded reattachment
  operation policy; defaults remain attached, retention-prohibited, and no
  reattachment
- added a provider cancellation outcome beside terminal status for confirmed,
  completion-raced, and unconfirmed remote stop
- made Codex, Anthropic, Bedrock, and llama.cpp structured-run drivers reject
  the new posture instead of ignoring it
- added Contract 021 for operation identity, provider cursor, reattachment,
  cancellation, access, retention, and joined cleanup
- created the fixture-first `swallowtail-adapter-openai` crate
- froze OpenAI Responses create, retrieve, reattach, cancel, queued, in-
  progress, completed, incomplete, failed, cancelled, usage, rate, request-
  correlation, access, failure, and redaction evidence dated 2026-07-21
- added fail-closed duplicate, gap, response-mismatch, unknown-event, malformed-
  disconnect, and unsafe response-id cases
- added a loopback HTTP/SSE seam proving one create attempt remains separate
  from retrieve, reattach, and cancel management requests

## Authority And Bounds

- provider-supported OpenAI public Responses API only
- exact `api.openai.com` endpoint audience, public API key, API billing, and
  provider support authority
- `background=true`, `stream=true`, `store=false`, one exact model, one
  positive output bound, required temporary retention, and one maximum
  reattachment
- no ChatGPT, Codex, harness, subscription OAuth, community OAuth, tools,
  search, files, conversations, webhooks, Batch API, retry, or fallback
- no credential, account, external request, or paid inference in default QA

## Validation

- focused core, runtime, existing structured-run adapter, OpenAI protocol, and
  loopback tests pass
- focused core/runtime/OpenAI warnings-denied clippy passes
- full repository QA passes with 300 tests; three installed/live probes remain
  gated and ignored by default
- `git diff --check` passes
- `effigy doctor` remains at the inherited 19 oversized-file findings: 12
  warnings and 7 errors; the new runtime outcome tests were split before
  closeout so this batch adds no finding

## Remaining Risks

- production HTTP/SSE transport, endpoint and credential lifetime, runtime
  projection, cancellation races, deadline, and joined cleanup remain card 075
- a disconnect before `response.created` can leave remote state unconfirmed;
  the production driver must report that without claiming provider stop
- temporary provider retention remains mandatory even with `store=false`
- provider schema, event, rate-header, status, or retention drift requires a
  new evidence date and fixture delta
- live access remains separately gated

## Continuation Record

Card 075 is the sole ready task. Implement the exact bounded background route,
prove local and remote-authoritative hosted conformance, then close roadmap
023. No shared contract or access-policy decision remains open.
