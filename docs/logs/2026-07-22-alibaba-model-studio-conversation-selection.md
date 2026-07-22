# 2026-07-22 Alibaba Model Studio Conversation Selection

## Outcome

Card 087 is complete. Alibaba Model Studio's Singapore workspace-dedicated
Conversations and Responses route is the next direct-provider proof. Contract
025 is active. Card 088 is ready; card 089 remains planned behind its fixture
gate.

## Evidence

- DeepSeek now names exact V4 Pro and Flash routes, exposes a model list, maps
  compatible reasoning effort, ignores some thinking-mode fields, and retires
  legacy aliases on 2026-07-24
- Z.AI now names GLM-5.1 on its general API, while Coding Plan remains limited
  to supported tools and cannot back a Swallowtail direct adapter
- both remain stateless compatible HTTP/SSE mappings after Kimi Platform
- Alibaba recommends workspace-dedicated domains with region-specific keys and
  model lists; one key belongs to one user, workspace, and region
- Singapore has one International deployment scope and supports exact
  `qwen3.7-plus-2026-05-26`
- Conversations stores provider-owned items; deleting the conversation does
  not delete those items, so cleanup needs separate evidence
- Responses is synchronous, streams ordered provider sequences, ignores
  undocumented compatible fields, and can bind turns to the conversation

No provider account, workspace, API key, external request, subscription
mutation, remote resource, or paid inference participated.

## Decision

The first proof is a resource-free direct-inference interactive session. It
uses one operator-approved Singapore workspace endpoint, a general pay-as-you-
go Model Studio API key, and exact model `qwen3.7-plus-2026-05-26`. Singapore is
a representative one-scope fixture, not a consumer region, provider, workspace,
model, data-residency, or routing default.

Session open creates one driver-owned provider conversation. Maximum two serial
text turns use synchronous streaming Responses with `store=false`, no context
cache, no tools, and no background work. Close inventories and deletes every
conversation item before deleting the conversation and releasing the key.

Local stream closure does not prove native cancellation. A remote turn may race
cleanup; unconfirmed remote state and deletion remain visible rather than being
reported as clean removal.

## Changed

- added Research 019
- promoted Contract 025 and updated the contract index
- completed card 087
- replaced generic cards 088-089 with exact Alibaba records/fixtures and
  driver/conformance cards
- made card 088 the sole ready continuation
- refreshed roadmap, generation, long-term, research, log, and project front
  doors

## Validation

- official-source review completed against sources accessed 2026-07-22
- `effigy qa:docs` passes
- `effigy qa:northstar` passes
- `git diff --check` passes
- doctor remains at the inherited 19 findings: 12 warnings and seven errors

## Remaining Risks

- `store=false` conversation item behavior must be frozen before production
- no native synchronous Responses cancellation is documented
- remote inference may add an item after local disconnect and race cleanup
- deletion requires a complete bounded item inventory; partial inventory cannot
  produce confirmed cleanup
- endpoint, key, model list, rate, billing, and availability remain regional
  and workspace-scoped mutable evidence
- live entitlement, billing, quota, request acceptance, and deletion behavior
  remain separately gated

## Continuation Record

Card 088 is ready. Realize only Contract 025's minimum provider-neutral session
policy and deletion records, then freeze the exact offline corpus. Card 089 can
become ready only after that fixture gate closes without a new contract choice.
