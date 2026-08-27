# 222 xAI Responses WebSocket Web Search Evidence

Status: ready
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Milestone: [g04.080 xAI Responses WebSocket Web Search](../080-xai-responses-websocket-web-search.md)
Depends on: g04.039; g04.079 closeout

## Goal

Freeze exact xAI Responses WebSocket web-search model, profile, request,
bound, response, citation, usage, billing, and lifecycle truth, then promote
Research 227 with a non-empty exact deliver-now table or an honest empty set.

## Work

1. Reuse and verify the exact route, driver, facade, model-route revisions,
   request specimens, control rows, and lifecycle from Research 187. Current
   documentation is a lead only.
2. Retrieve and freeze official WebSocket, Responses API, web-search,
   tool-usage, citation, model, and pricing material with dates, final URLs,
   complete-body digests, and decisive excerpts or schemas.
3. Prove whether WebSocket `response.create` accepts the exact Responses
   `tools: [{"type":"web_search", ...}]` shape. Settle `stream`, `background`,
   `store=false`, `previous_response_id`, `include`, and repeated-turn rules.
4. Classify `grok-4.5` and `grok-4.6` independently for structured run, first
   session turn, continuation turn, and fresh restoration. Do not infer support
   from catalogue presence or a model-family prefix.
5. Freeze the smallest positive provider-side query/use/turn bound: field,
   numeric domain, defaults, overflow, omission, interaction with reasoning
   and output bounds, and behavior at the bound. Do not invent a host retry or
   client truncation.
6. Freeze optional web-search filters, image-search switches, source inclusion,
   and citation behavior. Withhold every caller option not required by the
   smallest safe row.
7. Trace all exact response items and events: `web_search_call`, action/query,
   status, server-side output visibility, assistant text, annotations or inline
   citations, usage, billed cost, errors, response completion, and socket close.
8. Distinguish dispatch, provider acceptance, invocation, result delivery,
   citation delivery, usage, billing, and model choice. A model may decline to
   invoke an admitted tool.
9. Audit prepared run/session inputs and evidence, capability constraints,
   operation/session policies, plan agreement, request encoder, decoder,
   activity projection, terminal mapping, fixtures, examples, guide, matrices,
   changelog, and API baseline.
10. Prove omission retains exact `tools: []` bytes and all current reasoning,
    output-bound, `store=false`, continuation, restoration, cancellation,
    deadline, connection, terminal, cost, and cleanup behavior.
11. Keep host external networking denied. Provider-owned search must not become
    browser control, host fetch, consumer tool exchange, or an endpoint grant.
12. Promote Research 227 with exact sources, matrices, and a non-empty
    deliver-now table or explicit empty set. Update milestone/card state and
    close out honestly.

## Acceptance Criteria

- [ ] exact facade/model/profile/request and WebSocket composition are frozen
- [ ] one positive provider-side use bound has an exact disposition
- [ ] response-call, citation, usage, billed-cost, failure, and terminal truth
      have bounded independent dispositions
- [ ] production preparation, encoder, decoder, activity, fixtures, docs, and
      public seams are audited
- [ ] Research 227 contains a non-empty exact table or honest empty set
- [ ] no production code, public API, shared contract/runtime, currentness,
      release, merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-xai
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
git diff --check
```

Auto-continue to card 223 only when Research 227 admits a non-empty exact row
with closed model/profile membership, WebSocket composition, a positive
provider-side bound, and bounded response/citation truth.

## Stop Conditions

- exact WebSocket support, model membership, bound, response grammar,
  citations, usage, billing, or terminal truth remains ambiguous
- provider search depends on unbound account, organization, catalogue,
  endpoint, or ambient configuration facts
- deterministic proof needs a provider request, paid search, credential use,
  account inspection, or shared-contract change

## Out Of Scope

- production binding, X/image/code/file/MCP/function tools, caller filters,
  sibling routes, live provider work, currentness, release, merge, rollover,
  or g04 closure
