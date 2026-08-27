# 2026-08-27 g04.080 xAI Responses WebSocket Web Search Closeout

Status: stopped after evidence
Owner: Tom
Milestone: g04.080
Cards: 222 complete; 223-224 blocked
Branch: `t3code/xai-websocket-web-search`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-8f962dec`
Base: `3599b5b4c657e1dc2fb2c48d363af6bd637baabe` (`origin/main` at dispatch)
Planning base ancestor: `27dcc8fc5e520576b5dbceda90f32a84dddba254`
PR: https://github.com/inflatable-cookie/swallowtail/pull/79

## Result

Card 222 completed an exact official-docs, OpenAPI, and production-seam audit
of provider-owned `web_search` on `xai.responses-websocket`. Research 227
admits no deliver-now row. Cards 223 and 224 are blocked and were not
executed. The xAI adapter, prepared facade, request encoder, decoder,
fixtures, guide, matrices, and API baseline are unchanged. No provider
request, credential use, paid search, account inspection, or ambient
configuration mutation was used.

## Evidence Stop

WebSocket `response.create` still uses the Responses create body minus
`stream` and `background`. Official HTTP/Responses examples and OpenAPI admit
`tools: [{"type":"web_search"}]`. Both official WebSocket specimens still send
`tools: []`. There is no composed WebSocket search specimen.

`grok-4.6` is a candidate from the get-started Tools list. `grok-4.5` is
withheld. The Grok 4.6 model-detail capabilities list omits web search; that
discrepancy is recorded rather than flattened.

`max_turns` is the Responses create-body turn bound (`int32`, omitted default
is an unnamed server cap). `max_turns: 1` is the smallest documented positive
candidate. It does not cap intra-turn search queries. `max_tool_calls` is a
response field, not a request bound.

OpenAPI describes `web_search_call` completed-output items and `url_citation`
annotations. It has no WebSocket path and no search streaming-event schemas.
Official Responses streaming examples handle `response.output_text.delta`
only. The current adapter fail-closes on unknown events and requires
`response.completed.output` to be exactly one assistant message.

Omission retains exact `tools: []`. Host networking stays denied.

## Changed Surfaces

- `docs/research/227-xai-responses-websocket-web-search-evidence.md`:
  promoted exact sources, matrices, claim strength, and empty deliver-now
  table
- cards 222-224, g04.080, programme, triage, indexes, this closeout

No production code, public API, shared contract/runtime, guide capability,
matrix, or changelog edit.

## Validation

Passed:

- `effigy validate:focused swallowtail-adapter-xai`
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `git diff --check`

No production code changed. Doctor was not re-run for repair; the inherited
`scan.god-files` 380 findings (334 warnings, 46 errors) and
`scan.generated-in-src` one warning are unchanged by docs-only edits. The
prior stale graph-index note did not recur on this doctor run.

## Continuation

Keep g04 open. Reassess the remaining per-route feature inventory for the next
serial lane unless the operator supplies a different direction. Contract 029
currentness remains standing. Do not compile the next family from this
closeout.
