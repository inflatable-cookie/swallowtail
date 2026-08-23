# 136 OpenAI Background Service-Tier Evidence

Status: ready
Owner: Tom
Created: 2026-08-23
Milestone: [g04.049 OpenAI Background Service Tier](../049-openai-background-service-tier.md)
Depends on: Research 102 and 191; g04.044

## Goal

Freeze current exact-model and Responses-facade service-tier behavior, then
define the smallest adapter-local subset that can satisfy Contracts 021, 029,
037, 040, 048, and 049.

## Method

1. Freeze current official Responses create, retrieve, streaming-events,
   background-mode, service-tier/Flex/Fast material, and exact `gpt-5.6-sol`
   model page. Record retrieval dates, page dates when present, stable URLs,
   and complete specimen digests.
2. Freeze current route source and deterministic fixtures for exact model and
   facade, prepared input, plan/evidence, driver agreement, create request,
   reasoning, structured output, one reattachment, retrieval, cancellation,
   deletion, controlled detachment, restart reconciliation, and terminal
   response parsing.
3. Enumerate the exact current request and response enum domains from the
   authoritative schema. Classify omission, `auto`, `default`, `flex`, Fast /
   Priority spellings, Ultrafast, any schema-only value, and unknown future
   strings. Distinguish canonical selections from accepted aliases and
   provider-returned canonical values.
4. Close exact model applicability and access posture. Public API-key payg
   access does not prove project settings, Flex/Fast/Priority/Ultrafast
   enrollment, quota, capacity, or account entitlement.
5. Distinguish requested, planned, dispatched, provider-accepted, returned,
   effective, billed, and observed truth. The official response may differ
   from the request. Do not infer cost, latency, capacity, fallback, or
   substitution from a successful response.
6. Classify ordinary runs, one in-process stream reattachment, controlled
   active-run detachment, and restart reconciliation separately. Determine
   what selected and returned tier truth survives each boundary without a
   shared checkpoint or contract change.
7. Prove omission retains the prior create-request bytes. Prove composition
   with absent and every admitted reasoning value plus absent and selected
   structured output without changing existing lifecycle behavior.
8. Decide whether an exact returned-tier observation can be exposed through
   the current route-local API. If not, state the dispatch-only boundary and
   withhold any value/profile whose safe use requires resolved-tier evidence.
9. Decide the exact opaque facade point, private behavior revision, claim id,
   and model-route revision for any admitted subset. Retain the current
   corrected reasoning point as frozen superseded proof.
10. Replace Research 196's reservation with exact value/profile dispositions
    and a deliver-now table. Do not edit production code or shared closeout
    surfaces.

No credential, account/project inspection, provider request, paid operation,
or live OpenAI call is authorized. Current official OpenAI documentation and
secret-free repository/source inspection are sufficient for this gate.

## Acceptance Criteria

- [ ] exact request/response domains, aliases, defaults, model applicability,
      access gates, and unknown-value posture are source-backed or withheld
- [ ] requested, dispatched, returned, effective, and billed truth are distinct
- [ ] ordinary, reattached, detached, and reconciled profile dispositions are
      explicit
- [ ] omission, reasoning/structured-output composition, and lifecycle truth
      are explicit
- [ ] facade/private behavior/claim/model-route revisions are explicit
- [ ] Research 196 is promoted with an exact deliver-now table
- [ ] no production code, capability, matrix, or compatibility claim changes
- [ ] `effigy validate:focused swallowtail-adapter-openai` passes
- [ ] `effigy qa:northstar` and `effigy qa:docs:index:research` pass
- [ ] `git diff --check` passes

Auto-continue to card 137 only when Research 196 admits a non-empty exact
value/profile set that preserves omission, composition, lifecycle, access,
checkpoint, observation, and contract boundaries.

## Stop Conditions

- exact value, model, access, request, response, or lifecycle applicability
  cannot be closed without a live call or inference
- requested and returned tier truth cannot remain distinct
- detachment/reconciliation needs an unplanned shared checkpoint change
- delivery needs a portable capability, currentness work, unplanned contract
  change, or breaking API

## Out Of Scope

- production binding, guide/matrix claims, another OpenAI route/model, live
  work, or shared closeout surfaces
