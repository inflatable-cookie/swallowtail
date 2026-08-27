# 236 OpenAI Realtime Reasoning-Effort Binding

Status: done
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Milestone: [g04.084 OpenAI Realtime Reasoning Effort](../084-openai-realtime-reasoning-effort.md)
Depends on: card 235; promoted Research 236 with five exact future rows

## Goal

Bind only Research 236's exact session-scoped Realtime effort values through
typed prepared input, immutable plan/evidence/request state, the OpenAI driver,
`session.update`, and matching `session.updated` acknowledgement.

## Scope

1. Add one optional typed `ReasoningMode` selection and builder to
   `OpenAiRealtimeSessionProfileInput`. Preserve constructors and omission.
2. Admit only `minimal|low|medium|high|xhigh` for exact model
   `gpt-realtime-2.1` and the new facade/private behavior point. Reject every
   other value before endpoint, credential, socket, or media work.
3. Bind `Capability::ReasoningSelection` and exact
   `CapabilityConstraint::ReasoningMode` through configured instance/model
   route, requirements, preflight plan, prepared evidence, and the existing
   `OpenRealtimeMediaSessionRequest` carrier.
4. Serialize the selected value only as
   `session.update.session.reasoning.effort`. Do not add a per-response override.
5. Decode the optional effort from `session.updated`. For explicit selection,
   require an exact match before returning a usable session. Missing, foreign,
   or mismatched values fail and perform normal joined cleanup. Omission keeps
   the existing acknowledgement behavior and makes no default claim.
6. Mint facade `openai-realtime-reasoning-2026-08-27` with private behavior
   `openai.realtime-manual-pcm-reasoning-v2`. Retain
   `openai-realtime-2026-07-22` and its v1 proof as a superseded historical
   point rejected by the current driver.
7. Preserve exact omission setup bytes. Compose selection independently with
   the existing positive output-token maximum through 4,096.
8. Preserve the selected request through fresh context-losing working-state
   restoration. Planned rollover remains disabled.
9. Preserve media, transcription, activity, usage, cancellation, deadline,
   provider failure, disconnect, connection invalidation, and credential-last
   joined cleanup.

## Acceptance Criteria

- [x] only the five Research 236 values prepare
- [x] input, constraint, plan, evidence, request, setup bytes, and acknowledged
      value agree exactly
- [x] mismatch and missing explicit acknowledgement fail before session return
- [x] omission retains current bytes and no selected/default reasoning claim
- [x] every admitted value composes with the existing output maximum
- [x] fresh restoration keeps the immutable selected value
- [x] historical facade proof stays named and the current driver rejects it
- [x] no alias, clamp, substitution, fallback, retry, or provider-effectiveness
      claim enters the API

## Validation

```sh
cargo fmt -p swallowtail-adapter-openai
effigy validate:focused swallowtail-adapter-openai
effigy package:verify-affected swallowtail-adapter-openai
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 237 when exact preparation, wire, acknowledgement,
omission, composition, restoration, and failure tests pass.

## Stop Conditions

- the portable vocabulary cannot carry all five exact values
- selected effort cannot be confirmed before returning the session
- omission cannot keep current bytes and acknowledgement behavior
- fresh restoration can drift from the prepared selection
- implementation needs a shared runtime change, live proof, contract amendment,
  per-response control, or breaking API

## Out Of Scope

Shared planning closeout and Next Task, live provider work, effective reasoning,
reasoning-token usage, sibling routes, currentness, release, merge, rollover, or
g04 closure.

## Evidence

Bound on worker branch `t3code/openai-realtime-reasoning-effort`. Focused
realtime suite `realtime_prepared_facade` passed 22/22 including preparation,
dispatch, acknowledgement failure cleanup, omission bytes, composition,
restoration, and superseded-facade rejection.
