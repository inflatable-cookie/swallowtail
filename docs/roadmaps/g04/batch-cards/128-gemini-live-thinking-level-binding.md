# 128 Gemini Live Thinking-Level Binding

Status: complete
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Milestone: [g04.046 Gemini Live Thinking Levels](../046-gemini-live-thinking-levels.md)
Depends on: card 127; promoted Research 193 with a non-empty deliver-now set

## Goal

Bind only Research 193's exact Gemini Live thinking levels through typed
prepared input, immutable plan/evidence, the realtime request, driver
validation, and initial/resumed setup serialization.

## Scope

1. Add one optional typed `ReasoningMode` selection to
   `GeminiLiveSessionProfileInput`. Preserve current constructors and exact
   omission behavior.
2. Add the minimum optional reasoning carrier to
   `OpenRealtimeMediaSessionRequest`. This is a portable field, not a generic
   provider settings map.
3. Admit only exact Research 193 values for exact model, facade point, and
   private behavior revision. Reject `off`, `default`, `xhigh`, `max`, numeric
   budgets, aliases, and foreign values before endpoint, credential, or socket
   work.
4. Bind `Capability::ReasoningSelection` and its exact
   `CapabilityConstraint::ReasoningMode` through configured instance/model
   route where applicable, operation requirements, preflight plan, prepared
   evidence, request, and low-level driver.
5. Serialize the exact selected uppercase level in both initial and
   rollover/resume setup frames. When selection is omitted, retain the current
   exact `MINIMAL` setup frames and do not add a reasoning capability.
6. Carry the same request and plan through fresh realtime working-state
   restoration. Reject request, plan, evidence, model, facade, or driver drift
   before effects when knowable.
7. Update the exact Contract 029 facade point/private behavior revision as
   Research 193 requires. Do not rewrite or backfill the current behavior.
8. Keep OpenAI Realtime's low-level route fail-closed: an unsupported reasoning
   field on the shared request must be rejected before endpoint, credential,
   or socket work, while its normal absent path remains byte-for-byte
   unchanged.
9. Preserve Gemini media, transcription, manual activity, usage, interruption,
   deadline, one-rollover, credential, transport, provider-failure, and joined
   cleanup behavior.

## Acceptance Criteria

- [x] only Research 193 deliver-now values prepare
- [x] input, capability constraint, plan, evidence, request, driver, and setup
      bytes agree exactly
- [x] omission preserves current initial and resume setup bytes
- [x] one planned rollover and fresh restoration retain the selected level
- [x] unsupported values and every knowable drift reject before effects
- [x] sibling OpenAI Realtime rejects an unsupported shared-request reasoning
      field and retains its ordinary absent behavior
- [x] no alias, clamp, substitution, numeric translation, retry, or fallback
      enters the API

## Validation

```sh
cargo fmt -p swallowtail-runtime -p swallowtail-adapter-gemini -p swallowtail-adapter-openai
effigy validate:focused swallowtail-runtime swallowtail-adapter-gemini swallowtail-adapter-openai
effigy package:verify-affected swallowtail-runtime swallowtail-adapter-gemini swallowtail-adapter-openai
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 129 when exact preparation, initial/resume setup,
restoration, omission, rejection, and sibling fail-closed tests pass.

## Stop Conditions

- the portable vocabulary cannot carry the admitted exact values
- adding the shared request carrier weakens another realtime route
- selected reasoning can drift across setup, rollover, or restoration
- implementation needs a generic option map, live proof, contract amendment,
  or breaking API

## Out Of Scope

- shared docs/indexes, other Gemini controls/routes/models, live provider work,
  release, or merge
