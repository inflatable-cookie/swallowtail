# 235 OpenAI Realtime Reasoning-Effort Evidence

Status: complete
Owner: Tom
Created: 2026-08-27
Milestone: [g04.083 Parallel Per-Route Feature Qualification II](../083-second-parallel-per-route-feature-qualification.md)
Depends on: OpenAI Realtime prepared facade; g04.082 closeout
Research: [236 OpenAI Realtime Reasoning-Effort Evidence](../../../research/236-openai-realtime-reasoning-effort-evidence.md)

## Goal

Freeze exact OpenAI Realtime reasoning-capable model, effort vocabulary,
session/request dispatch, acknowledgement, response, rollover, and failure
truth, then promote Research 236 with a closed deliver-now table or an honest
empty set.

## Work

1. [x] Keep route `openai.realtime`, exact opaque facade revision
       `openai-realtime-2026-07-22`, public API-key access, manual PCM profile,
       current output maximum, response cancellation, and no rollover unchanged.
2. [x] Freeze official Realtime model/session schemas, API reference, model
       pages, and dated route fixtures for `reasoning.effort`: exact models,
       values, defaults, update timing, acknowledgement, and response fields.
3. [x] Reconcile the route's fixed model with reasoning-capable Realtime models.
       Do not infer support from Responses API models or a shared OpenAI catalogue.
4. [x] Build a closed model/value/operation/lifecycle table for open, session
       update if applicable, each response, cancellation, disconnect, and fresh
       restoration. Prove unsupported rows reject before endpoint, credential,
       connection, or media work.
5. [x] Separate requested, session-encoded, response-encoded, accepted,
       effective, returned, token-usage, and observed reasoning truth. Reasoning
       tokens alone do not confirm the selected effort.
6. [x] Prove omission retains exact current session bytes, maximum-output-token
       behavior, connection lifecycle, and fresh-restoration behavior.
7. [x] Audit prepared input/evidence, plan/request agreement, protocol encoder/
       decoder, session update handling, fixtures, guide, matrices, and API
       baseline without changing production surfaces.
8. [x] Promote Research 236 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [x] exact Realtime model/value/operation table or honest empty set is recorded
- [x] Responses reasoning evidence is not promoted to Realtime
- [x] unsupported rows reject before connection and omission stays exact
- [x] accepted, effective, returned, usage, and observed truth stay separate
- [x] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-openai
effigy qa:northstar
git diff --check
```

## Stop Conditions

- the selected fixed model is not statically reasoning-capable
- effort membership or update timing is not exact for the Realtime transport
- accepted/effective selection cannot be confirmed without a provider session
- proof needs a credential, provider connection, paid media work, fixture
  rebaseline, currentness decision, or shared-contract change

## Out Of Scope

OpenAI background, Responses reasoning, tools, images, search, output-limit
changes, planned rollover, production binding, live provider work, currentness,
release, merge, shared closeout, rollover, or g04 closure.

## Outcome

Research 236 promotes five future session-scoped deliver-now rows on exact
`gpt-realtime-2.1` (`minimal`, `low`, `medium`, `high`, `xhigh`) while current
production on `openai-realtime-2026-07-22` remains an honest empty set.
Frozen evidence lives under
`crates/swallowtail-adapter-openai/tests/fixtures/openai-realtime-reasoning-effort-2026-08-27/`.
