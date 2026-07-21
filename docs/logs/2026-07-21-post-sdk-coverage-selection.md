# 2026-07-21 Post-SDK Coverage Selection

## Outcome

Selected OpenAI Responses background mode as the next high-information proof
after the separate Bedrock Runtime and control-plane SDK routes. Card 073 and
roadmap 022 are complete. Roadmap 023 is active; card 074 is ready.

## Evidence

- the provider-owned response can remain queued or in progress after one local
  SSE attachment disconnects
- a background response created with streaming supports reattachment from an
  exact provider `sequence_number` cursor
- the provider exposes retrieve and idempotent cancel operations for the same
  response identity
- `store=false` still requires temporary provider retention for asynchronous
  execution and polling
- current Cursor Cloud Agents add a stronger remote harness topology, but also
  require GitHub integration, provider-owned workspace mutation, artifact, VM,
  archive, and deletion authority
- remaining harness SDKs, ACP replacements, hosted APIs, and attached runtimes
  either repeat realized shapes, require language sidecars, or remain
  experimental, Preview, or unfinished

No provider account, credential, subscription, repository integration, live
endpoint, or paid inference participated.

## Decisions

- the first route is direct model inference through the provider-supported
  OpenAI public API, not Codex harness execution
- public API keys, ChatGPT login, ChatGPT subscription, Codex credentials, and
  community OAuth routes remain separate
- the driver selects no default endpoint, model, credential, provider, or
  consumer route
- one inference attempt may use create, stream reattachment, retrieve, and
  cancel management requests; those requests are not inference retry
- temporary provider retention must be selected explicitly before effects and
  cannot be hidden behind `store=false`
- the first proof has no durable detach, cross-process resume, webhook, Batch
  API, conversation, tool, search, file, retry, or fallback behavior

## Changed

- added Research 015 with the current comparison and exact recommendation
- completed roadmap 022 and card 073
- added roadmap 023 and cards 074-075 inside g01
- made card 074 the sole ready continuation
- updated the roadmap, generation, research, batch-card, long-term, and project
  front doors

## Validation

- `effigy qa:docs` passes
- `effigy qa:northstar` passes
- `git diff --check` passes
- `effigy doctor` remains at the inherited 19 findings: 12 warnings and 7
  errors

## Remaining Risks

- a disconnect before the provider response id is observed can leave remote
  continuation or cancellation unconfirmed
- provider completion can race native cancellation
- cursor reattachment must detect duplicate, gap, mismatch, and replay rather
  than silently normalize them
- temporary provider retention and current ZDR behavior remain provider policy
  that can drift
- live access, rate behavior, usage, billing, and provider request acceptance
  remain separately gated
- Cursor's repository and provider-VM authority needs an operator decision
  before that later cloud-harness route is compiled

## Continuation Record

Card 074 is ready. Promote Contract 021, add only the smallest provider-neutral
background and retention records, and freeze the dated OpenAI Responses
loopback HTTP/SSE corpus. Card 075 remains in bounds for the production driver
and conformance after the shared boundary is stable.
