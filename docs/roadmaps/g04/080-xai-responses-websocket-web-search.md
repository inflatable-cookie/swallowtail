# g04.080 xAI Responses WebSocket Web Search

Status: stopped after evidence
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Depends on: g04.039; g04.079 closeout; per-route feature completion programme
Vision tags: explicit behavior, provider-owned search, route-local controls
Contract refs: 011, 029, 037, 039, 040, 041, 044, 052
Research: 004, 067, 169, 187, 227

## Problem

Production route `xai.responses-websocket` owns one exact Responses WebSocket
connection, structured runs, serial text sessions, selected Grok 4.5/4.6
models, model-qualified reasoning, positive output bounds, usage, billed cost,
and joined cleanup. Every request still emits `tools: []`, so the route does
not expose xAI's provider-owned `web_search` tool.

Current official xAI documentation says WebSocket `response.create` uses the
Responses create body and separately documents `web_search` on `grok-4.6`,
server-side `web_search_call` items, citations, optional source inclusion, and
tool-turn bounds. Those mutable pages do not widen the exact dated facade that
Swallowtail qualified. Exact model, request, response, bound, citation, usage,
billing, and WebSocket composition evidence is required before delivery.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind provider-owned web search
on exact `xai.responses-websocket` profiles through the existing explicit
external-search posture. Keep host networking denied, preserve `tools: []` as
omission, and do not promote X search, code execution, MCP, file search, raw
tool arrays, or a provider-neutral search-options map.

## Goals

- [x] freeze exact WebSocket and Responses-body support for `web_search`
- [x] freeze exact model/profile membership, tool syntax, bounds, filters,
      citations, usage, billing, failure, and response-event truth
- [x] classify structured runs and serial sessions independently
- [x] promote Research 227 with a non-empty exact deliver-now table or honest
      empty set
- [ ] conditionally bind only `ExternalSearchPolicy::Enabled` rows with one
      exact provider tool and positive provider-side use bound
- [x] preserve `ExternalNetworkPolicy::Denied`, endpoint/access authority,
      `store=false`, model/reasoning/output controls, continuation,
      restoration, terminal mapping, billed cost, and joined cleanup
- [x] keep omission byte-equivalent with `tools: []`

## Non-Goals

- X search, image search, code execution, collections/file search, MCP,
  function tools, consumer tool exchange, or raw Responses tools
- caller-defined domains, filters, include lists, source formatting, or tool
  budgets unless Research 227 proves a smaller closed shape is required for
  safe delivery
- automatic search based on prompt content or model choice
- browser control, host web fetch, arbitrary host networking, retry, or
  fallback
- changing `grok-build.acp`, adding an xAI HTTP route, currentness, release,
  merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `xai.responses-websocket`, driver
`swallowtail.xai.websocket`, exact facade axis
`xai.responses-websocket-facade`, selected `grok-4.5` and `grok-4.6` route
revisions, structured-run and serial-session profiles, and their existing
private behavior revisions unless exact evidence requires a route-private
advance.

Card 222 must freeze retrieved official WebSocket, Responses API, web-search,
tool-usage, citation, model, and pricing material with dates and digests. It
must determine whether the exact WebSocket body accepts `web_search`, which
selected models support it, whether structured and continuation turns behave
alike, and whether a positive provider-side use bound can be fixed before
network work.

Research 227 must trace the complete response grammar: search-call lifecycle,
query/action metadata, server-side outputs, assistant content, citations,
usage, billed cost, errors, cancellation, deadline, disconnect, and terminal
ordering. Provider invocation, result delivery, citation delivery, usage, and
billing remain separate evidence. A model declining to search is not adapter
failure.

The only eligible public intent is explicit provider-owned web search through
`ExternalSearchPolicy::Enabled`. The exact xAI tool name, filters, include
shape, and fixed positive bound remain adapter-owned. Omission must retain the
current `tools: []` body. Host external networking remains denied because xAI,
not the host, executes the search.

## Execution Plan

### Batch 80.1 — Exact Web-Search Evidence

- [x] Execute card 222.
- [x] freeze exact model/profile/tool/bound/response/citation/billing truth
- [x] promote Research 227 with an exact table or honest empty set

### Batch 80.2 — Conditional Search Binding

- [ ] Execute card 223 only when Research 227 admits a non-empty exact row.
- [ ] bind only admitted web-search rows through typed prepared evidence,
      capability requirements, policy, request encoding, and decoding

### Batch 80.3 — Route-Local Acceptance

- [ ] Execute card 224 after card 223.
- [ ] prove exact dispatch, omission, rejection, response projection, terminal
      truth, and unchanged WebSocket lifecycle

## Acceptance Criteria

- [x] only Research 227 deliver-now model/profile rows prepare enabled search
- [ ] enabled search dispatches one exact `web_search` tool with a positive
      provider-side use bound and no raw caller options
- [x] omission retains exact current request bytes with `tools: []`
- [x] host networking stays denied and no consumer callback is invented
- [x] search activity, citations, usage, billed cost, provider failure, and
      terminal state remain distinct and bounded
- [x] reasoning, output bounds, `store=false`, continuation, restoration,
      cancellation, deadline, connection invalidation, and cleanup do not widen
- [x] default QA performs no provider request, credential use, paid search, or
      ambient configuration mutation

## Lane Runway

- predecessor: g04.079 Claude Code headless maximum-turn delivery
- this milestone: exact xAI Responses WebSocket web-search evidence and
  conditional delivery
- execution topology: one serial worker lane, cards 222-224
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop after card 222 if exact WebSocket composition, model support, or a
  positive provider-side use bound cannot be proved without live provider work.
- Stop if search depends on unbound organization/account configuration or a
  mutable catalogue fact that preparation cannot reject.
- Stop if the exact stream grammar cannot separate provider-owned search from
  consumer tools or cannot preserve bounded citation and terminal truth.
- Stop if delivery requires raw tool JSON, host-network authority, sibling
  route promotion, or a shared-contract change not already settled by 041.

## Batch Cards

- [222-xai-responses-websocket-web-search-evidence.md](batch-cards/222-xai-responses-websocket-web-search-evidence.md)
- [223-xai-responses-websocket-web-search-binding.md](batch-cards/223-xai-responses-websocket-web-search-binding.md)
- [224-xai-responses-websocket-web-search-acceptance.md](batch-cards/224-xai-responses-websocket-web-search-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 187 xAI Responses Control Evidence](../../research/187-xai-responses-control-evidence.md)
- [Research 227 xAI Responses WebSocket Web Search](../../research/227-xai-responses-websocket-web-search-evidence.md)
- [xAI WebSocket Mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode)
- [xAI Web Search](https://docs.x.ai/developers/tools/web-search)
- [xAI Tool Usage Details](https://docs.x.ai/developers/tools/tool-usage-details)
- [xAI Citations](https://docs.x.ai/developers/tools/citations)
- [Contract 041 Input And Tool Admission](../../contracts/041-input-callback-and-provider-tool-admission.md)
- [Contract 044 Working-State Restoration](../../contracts/044-working-state-restoration.md)
- [Realtime Prepared Integration](../../guides/realtime-prepared-integration.md)
