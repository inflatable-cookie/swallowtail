# 088 Alibaba Conversation Records And Fixtures

Status: completed
Owner: Tom
Updated: 2026-07-22
Milestone: `../029-remaining-direct-provider-breadth.md`

## Objective

Realize Contract 025's minimum provider-conversation session records and freeze
the exact Alibaba Model Studio Singapore Conversations and Responses corpus.

## Governing References

- Research 019
- Contracts 005, 006, 009, 011, 014, 020, and 025
- roadmap 029
- existing session, durable-retention, remote-deletion, and hosted transport
  records

## Scope

- resource-free interactive direct-inference session posture
- explicit durable provider-conversation retention and delete-on-close policy
  on session open, requirements, capabilities, and preflight
- generic owned conversation and aggregate conversation-item deletion kinds
- no connection-scoped, harness, workspace-resource, or resume requirement
- fixture-only `swallowtail-adapter-alibaba-model-studio` crate
- exact Singapore workspace-dedicated endpoint and regional audience
- general Model Studio API-key, pay-as-you-go, provider-supported access
- exact `qwen3.7-plus-2026-05-26` route
- Conversations create/list-items/delete-item/delete-conversation corpus
- synchronous streaming Responses corpus with `conversation`, `stream=true`,
  `store=false`, `reasoning.effort=none`, no tools, and no cache header
- provider sequence, text delta/completion, returned model, usage, request,
  errors, unknowns, disconnect, and cleanup-race fixtures
- default tests use no workspace, credential, network, or paid inference

## Ordered Work

1. Add the minimum provider-state session policy and pure compatibility checks.
2. Add generic conversation and conversation-item deletion evidence without
   provider identity in core or runtime.
3. Extend deterministic testkit assertions for direct conversation retention,
   serial turns, deletion ordering, and unconfirmed remote races.
4. Add the fixture-only Alibaba adapter descriptor, access, route, request,
   stream, failure, inventory, and deletion corpus.
5. Prove unsupported fields, Coding Plan access, aliases, other regions,
   response storage, cache, tools, background work, retry, and fallback reject
   before effects.
6. Update architecture only for records that exist, then close the card with
   focused validation and a doctor delta review.

## Acceptance Criteria

- [x] session retention and deletion posture is explicit and preflight-bound
- [x] core/runtime records remain provider-neutral and need no compatibility shim
- [x] conversation, aggregate item deletion, response, turn, stream, route,
      model, workspace, region, and host identities remain separate
- [x] deleting a conversation cannot report its items deleted
- [x] cancellation and deadline preserve unconfirmed remote-turn truth
- [x] the dated corpus fixes exact request, sequence, output, usage, failure,
      item inventory, deletion, and exclusion behavior offline
- [x] card 089 has a complete production seam with no unresolved contract gap

## Evidence Required

- pure preflight rejection tests for every new policy mismatch
- provider-neutral retention and deletion assertion tests
- offline fixture tests for every selected route and cleanup boundary
- dependency audit showing no Alibaba identity in core, runtime, or testkit
- explicit list of unsupported or ignored provider fields
- doctor delta with no new oversized-file debt

## Validation

- focused core, runtime, testkit, and Alibaba fixture tests
- focused warnings-denied clippy
- `effigy qa:docs`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- the session policy requires copying structured-run policy wholesale
- generic records need Alibaba, workspace, region, or Responses identity
- official evidence cannot prove `store=false` conversation behavior or item
  deletion independently from conversation deletion
- a deterministic fixture requires a real workspace, key, or provider request
- response cancellation is inferred from local SSE closure

## Auto-Continuation

No. Make card 089 ready only after the records and corpus prove the production
boundary without a new contract decision.

## Completion Evidence

- `SessionProviderStatePolicy` defaults to `Prohibited` and permits only the
  explicit durable-conversation-delete-on-close opt-in; open requests,
  requirements, capability constraints, preflight plans, and runtime request
  validation must agree
- generic `Conversation` and aggregate `ConversationItems` owned-resource kinds
  keep deletion outcomes independent; confirmed conversation deletion cannot
  imply item deletion
- the reusable provider-conversation assertion pack covers pure mismatch
  rejection, one active turn, two-turn maximum, deletion truth, and local
  cancellation/deadline races without an eleventh synthetic profile
- fixture-only `swallowtail-adapter-alibaba-model-studio` freezes the exact
  Singapore workspace audience, general API-key pay-as-you-go access, exact
  model, request fields, fragmented SSE, sequence, output/model/usage agreement,
  safe failures, complete item inventory, ordered deletion, and exclusions
- no Alibaba, workspace, region, Qwen, or Responses identity appears in core,
  runtime, or testkit; no compatibility shim was added
- focused core, runtime, testkit, and Alibaba tests plus warnings-denied clippy
  pass; full validation is recorded in the batch log
- doctor remains at the inherited 19 findings: 12 warnings and seven errors,
  with no new file from this batch
