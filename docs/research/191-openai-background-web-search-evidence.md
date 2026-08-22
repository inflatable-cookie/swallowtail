# 191 OpenAI Background Web Search Evidence

Status: reserved for g04.043 card 119
Owner: Tom
Created: 2026-08-22
Updated: 2026-08-22
Card: g04.043 / 119

## Question

Can exact `gpt-5.6` on the qualified `openai.background` Responses route bind
one bounded provider-owned `web_search` tool through portable
`ExternalSearchPolicy::Enabled` while preserving background, streaming,
retention, reattachment, retrieval, generation-control, and activity truth?

## Required Evidence

- current official web-search, Responses create/retrieve, background,
  streaming, and `gpt-5.6` model surfaces
- exact `web_search` versus legacy `web_search_preview` request semantics
- positive `max_tool_calls` bounds and provider tool-choice behavior
- source/citation inclusion, stream events, retrieved output, usage, billing,
  and failure representation
- exact compatibility with `background=true`, `stream=true`, `store=false`,
  reasoning, structured output, reattachment, cancellation, and deletion
- model, access, account-policy, and interface-facade qualification
- provider-tool activity and portable event mapping without query/result leaks

## Candidate Disposition Table

| Candidate | Disposition | Evidence needed |
| --- | --- | --- |
| `gpt-5.6` Responses `web_search` with a positive total-call maximum | evidence-gated | exact request, model, lifecycle, event, source, and facade proof |
| `web_search_preview` | withheld | legacy tool is not the selected new-integration surface |
| arbitrary Responses tools or raw tool configuration | withheld | forbidden generic tool surface |
| Codex, Chat Completions search, or sibling-route search | not applicable | different route and transport |

## Promotion Rule

Card 119 replaces this reservation with exact frozen sources, digests,
specimens, compatibility findings, facade-revision disposition, and a
deliver-now/evidence-gated/withheld table. No capability claim follows from
this reservation.
