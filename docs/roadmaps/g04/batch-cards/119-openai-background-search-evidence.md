# 119 OpenAI Background Search Evidence

Status: complete; evidence stop
Owner: Tom
Created: 2026-08-22
Updated: 2026-08-23
Milestone: [g04.043 OpenAI Background Hosted Search](../043-openai-background-hosted-search.md)
Depends on: Research 015, 049, 050, and 067

## Goal

Freeze exact current official evidence for bounded `gpt-5.6` Responses web
search on `openai.background`, then define the smallest route/request subset
that can satisfy Contract 041 without widening tools, network, or lifecycle
authority.

## Method

1. Freeze the current official web-search guide, Responses create/retrieve and
   streaming references, background guide, and `gpt-5.6` model page. Record
   retrieval dates, stable secret-free specimens, and SHA-256 digests.
2. Prove the exact selected tool type. Separate `web_search` from legacy
   `web_search_preview`, Chat Completions search, Codex search, file search,
   MCP, custom functions, and generic Responses tools.
3. Freeze request fields for tool selection, provider-network posture, one
   positive maximum total built-in-tool-call count, tool choice, and source
   inclusion. Classify omission and provider defaults; do not infer a bound.
4. Prove exact `gpt-5.6` tool support and whether account or project policy can
   reject the tool. Keep API-key access, provider billing, tool billing, and
   model support separate.
5. Classify the full route combination: `background=true`, `stream=true`,
   `store=false`, output bound, optional reasoning, optional structured output,
   one stream reattachment, retrieve, cancel, delete, detachment, and restart
   reconciliation. Do not assume independent fields compose.
6. Freeze initial-stream, reattached-stream, and retrieved-response web-search
   items. Distinguish selected, invoked, searching, completed, source/citation,
   usage, assistant-output, incomplete, and failed truth.
7. Decide the exact observable-activity fidelity and portable
   `ExternalSearchProgress` evidence. Do not expose query text, raw sources, or
   provider payloads through diagnostics.
8. Decide whether the current opaque facade point can carry this feature or a
   new exact point/private behavior revision is required under Contract 029.
9. Replace Research 191's reservation with exact dispositions. Do not edit
   shared architecture, matrices, programme, or roadmap front doors.

No credential, account inspection, paid request, provider prompt, or live
OpenAI operation is authorized.

## Acceptance Criteria

- [x] current official sources and stable specimens are frozen with digests
- [x] exact tool, bound, choice, source, model, and access semantics are explicit
- [x] background and every existing lifecycle/control combination is classified
- [x] stream, reattachment, retrieve, activity, usage, and failure truth is explicit
- [x] interface revision and Contract 029 disposition is explicit
- [x] Research 191 is promoted with a route/request deliver-now table
- [x] the existing production claim is unchanged unless exact evidence supports
      a named additive behavior revision
- [x] `effigy validate:focused swallowtail-adapter-openai` passes
- [x] `effigy qa:northstar` and `effigy qa:docs:index:research` pass
- [x] `git diff --check` passes

Auto-continue to card 120 only when Research 191 admits a bounded exact
`web_search` mapping with no contract, model, facade, or lifecycle gap.

## Stop Conditions

- current official evidence contradicts exact `gpt-5.6` or background support
- no positive total search/tool-call bound exists
- selection needs arbitrary tools, prompt injection, consumer callbacks, or
  generic provider options
- source/event truth cannot be parsed without inference or content leakage
- existing reasoning, schema, retention, reattachment, cancel, delete,
  detachment, or reconciliation truth would weaken
- the feature needs a new portable contract or live account/provider proof

## Out Of Scope

- production binding or capability/matrix claims
- search filters, location, context size, return-token budget, image search,
  file search, MCP, functions, service tier, or other routes
- shared research/log/roadmap indexes and shared closeout surfaces

## Closeout

Research 191 is promoted with current official OpenAI source digests and
secret-free request, stream, search-item, retrieve, and failure specimens.
The public docs establish `web_search`, `gpt-5.6` feature support, explicit
provider access posture, `max_tool_calls: 1` as a positive total-call
candidate, `tool_choice: "auto"`, source inclusion, and the individual
background lifecycle controls.

They do not establish the exact composed background request, search-event
continuity through one reattachment, retrieved source/output truth, account
policy, or activity/facade mapping. Independent field documentation is not
promoted to route support. No deliver-now row exists. Cards 120 and 121 are
blocked and were not executed; the existing tool-free search claim is
unchanged. The inherited `minimal` reasoning mismatch is recorded as named
follow-up `g04.043-R1`, not absorbed into search implementation.
