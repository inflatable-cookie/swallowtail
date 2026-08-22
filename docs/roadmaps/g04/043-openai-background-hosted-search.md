# g04.043 OpenAI Background Hosted Search

Status: stopped after evidence
Owner: Tom
Created: 2026-08-22
Updated: 2026-08-23
Depends on: per-route feature completion programme; g01.023; g03.030
Vision tags: explicit selection, provider truth, provider-owned tools
Contract refs: 009, 010, 011, 014, 021, 029, 037, 041, 044, 052
Research: 015, 049, 050, 067, 191

## Problem

`openai.background` already binds exact `gpt-5.6`, output bounds,
provider-native structured output, temporary retention, bounded stream
reattachment, cancellation, detachment, and reconciliation. It also exposes a
prepared reasoning selection, but exact-model support for `minimal` is an
inherited mismatch recorded as follow-up `g04.043-R1`. It still rejects
`ExternalSearchPolicy::Enabled` and encodes no provider-owned tool.

Current official OpenAI documentation names the Responses `web_search` tool
for `gpt-5.6`, distinguishes it from legacy `web_search_preview`, exposes a
total built-in-tool-call bound, and can return complete search-source evidence.
That is enough to compile an evidence gate, not enough to change the route's
claim. The exact background, streaming, retention, source, activity, model,
and compatibility behavior must be frozen first.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind bounded provider-owned web
search on the existing `openai.background` route without turning provider
search into host networking, consumer tools, Codex search, or a generic
Responses tool surface.

## Goals

- [x] freeze current official Responses web-search, create/retrieve, background,
      streaming, and `gpt-5.6` evidence
- [x] qualify exact `web_search` request fields, positive use bound, source
      inclusion, event shapes, and terminal/retrieved representation
- [x] distinguish provider tool selection, invocation, sources, citations,
      usage, billing, and assistant output
- [x] classify compatibility with background execution, `stream=true`,
      `store=false`, reattachment, reasoning, and structured output
- [x] decide the exact private facade revision and Contract 029 claim delta
- [ ] bind only Research 191 deliver-now rows through typed prepared input,
      immutable plan/evidence, request policy, driver validation, and wire
- [ ] preserve exact request and lifecycle behavior when search is absent
- [ ] project only exact search progress and activity evidence supplied by the
      qualified stream or retrieved response
- [ ] publish route-local guidance and matrix truth without inferring that the
      model searched merely because search was enabled

## Non-Goals

- arbitrary Responses tools, MCP, file search, computer use, image search, or
  consumer function calls
- raw tool arrays, tool choice, search query, user location, context size,
  return-token budget, domain filters, or generic provider options
- Codex `--search`, OpenAI Search API, Chat Completions search models, or
  sibling OpenAI routes
- service tier, Fast mode, model expansion, conversation state, retry, or
  cross-process stream reattachment
- live credentials, account inspection, paid requests, or provider prompts

## Named Scope

The lane is restricted to production route `openai.background`, driver
`swallowtail.openai.background`, exact model route
`openai.public.gpt-5.6.background`, axis
`openai.responses-background-facade`, and the existing public API-key
pay-as-you-go boundary. Research 191 must decide whether a new exact opaque
facade point is required; search must not be backfilled into the current
`openai-responses-background-2026-07-21` claim without frozen evidence.

The only candidate is the provider-owned Responses `web_search` tool selected
through portable `ExternalSearchPolicy::Enabled`. The exact tool type, explicit
provider-network posture, positive maximum total tool calls, source inclusion,
model support, and event mapping remain evidence questions. The model may
decline to search; enablement is not invocation or result evidence.

An empty Research 191 deliver-now set is an honest stop. Domain filtering and
other richer web-search controls remain separate even if the basic tool is
admitted.

## Execution Plan

### Batch 43.1 — Exact Responses And Model Evidence

- [x] Execute card 119.
- [x] freeze official current pages and stable secret-free request, stream,
      retrieve, source, usage, and failure specimens
- [x] promote Research 191 with request/value, model, lifecycle, activity,
      compatibility, and facade-revision dispositions

### Batch 43.2 — Conditional Prepared Binding

- [ ] Execute card 120 only if card 119 admits a deliver-now subset.
- [ ] bind one optional provider-owned-search selection through the existing
      prepared route and low-level driver
- [ ] preserve the absent path and reject policy, plan, evidence, driver, or
      wire drift before provider work

Card 120 is blocked. Research 191 admits no deliver-now row.

### Batch 43.3 — Route-Local Acceptance

- [ ] Execute card 121 only after card 120.
- [ ] prove request, stream, reattachment, retrieve, activity, failure, and
      cleanup behavior without live provider work
- [ ] update route-local guidance and report the deferred shared closeout delta

Card 121 is blocked. There is no admitted dispatch to prove.

## Acceptance Criteria

- [ ] only Research 191 deliver-now rows prepare
- [ ] `ExternalSearch`, `ProviderExternalNetwork`, request policy, prepared
      evidence, driver, and exact wire agree
- [ ] the request has one exact provider-owned tool and a positive bounded
      total call maximum; no generic tool surface enters the public API
- [ ] absence retains the current tool-free request byte shape and behavior
- [ ] search remains independent from reasoning and structured output; only
      evidence-qualified combinations prepare
- [ ] initial stream, one reattachment, retrieve, cancellation, deletion,
      detachment, and reconciliation retain their existing truth
- [ ] search enablement, invocation, source/citation delivery, usage, billing,
      and model output remain distinct
- [ ] default QA uses no credential, account, external request, or paid work
- [ ] docs and matrices claim only the exact evidence supplied by the route

## Lane Runway

- predecessor: g04.042 Cline thinking-controls evidence stop
- this milestone: OpenAI background provider-owned web-search evidence and
  conditional binding
- execution topology: one serial worker lane, cards 119-121
- next route family: selected by the orchestrator after evidence and merge
  closeout; no later family is precompiled here

## Decision Gates

- Stop if current official evidence does not support `web_search` on exact
  `gpt-5.6` Responses background operations.
- Stop if the tool cannot be given a positive total-call bound.
- Stop if search requires an untyped tool array, prompt convention, consumer
  callback, arbitrary network authority, or new portable contract.
- Stop if background, stream, store, reattachment, retrieve, reasoning, or
  structured-output combinations cannot be classified exactly.
- Stop if the selected stream/retrieve surface cannot distinguish search
  progress from assistant output or fails closed on malformed tool events.
- Stop if compatibility requires silently rewriting the July facade point or
  weakening current retention, cancellation, deletion, or reconciliation truth.

## Batch Cards

- [119-openai-background-search-evidence.md](batch-cards/119-openai-background-search-evidence.md) — complete; evidence stop
- [120-openai-background-search-binding.md](batch-cards/120-openai-background-search-binding.md) — blocked
- [121-openai-background-search-acceptance.md](batch-cards/121-openai-background-search-acceptance.md) — blocked

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Contract 021 Provider-Owned Background Run](../../contracts/021-provider-owned-background-run-and-temporary-retention-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 041 Input Callback And Provider Tool Admission](../../contracts/041-input-callback-and-provider-tool-admission.md)
- [Contract 044 Observable Agent Activity](../../contracts/044-observable-agent-activity-and-disclosure.md)
- [OpenAI Background Prepared Integration](../../guides/openai-background-prepared-integration.md)
- [OpenAI Web Search](https://developers.openai.com/api/docs/guides/tools-web-search)
- [OpenAI Responses Create](https://developers.openai.com/api/reference/cli/resources/responses/methods/create)
- [OpenAI Models](https://developers.openai.com/api/docs/models)

## Evidence Stop

Research 191 is promoted, but its deliver-now set is empty. Official OpenAI
documentation proves the candidate tool and its individual request and
lifecycle fields, not the exact composed `openai.background` search route.
The current adapter also has no search event/item parser or source-aware
terminal representation. Cards 120 and 121 are therefore blocked. The
existing `openai-responses-background-2026-07-21` facade point, tool-free
request bytes, and search claim remain unchanged. The inherited exact-model
reasoning mismatch is recorded as named follow-up `g04.043-R1` below.

## Named Follow-Up

`g04.043-R1 — reconcile OpenAI background reasoning vocabulary` is outside
the search binding lane. The exact GPT-5.6 model page lists
`none|low|medium|high|xhigh|max`, while the current OpenAI background guide and
preparation validator also admit `minimal`. The follow-up must reconcile the
guide, preparation code, route-local tests, and orchestrator-owned production
claim before reasoning support is described as exact. This lane records the
mismatch and does not change those surfaces.
