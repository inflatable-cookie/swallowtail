# 131 Gemini Live Output-Token-Maximum Binding

Status: done
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Milestone: [g04.047 Gemini Live Output-Token Maximum](../047-gemini-live-output-token-maximum.md)
Depends on: card 130; promoted Research 194 with a non-empty deliver-now set

## Goal

Bind only Research 194's exact Gemini Live output-token maximum through typed
prepared input, immutable plan/evidence, the existing realtime request, driver
validation, and initial/resumed setup serialization.

## Outcome

Bound `1..=65_536` through
`GeminiLiveSessionProfileInput::with_maximum_output_tokens`, existing
`OpenRealtimeMediaSessionRequest`, `OutputTokenLimit` /
`OutputTokenMaximum`, and setup `generationConfig.maxOutputTokens`. Advanced
facade to `...thinking-output-max-2026-08-23`, behavior
`...thinking-output-max-v3`, claim `gemini.live-preview-window-3`, and
model-route `prepared-3`, retaining the thinking-capable point as superseded.

## Scope

1. Add one optional typed `NonZeroU64` maximum to
   `GeminiLiveSessionProfileInput`. Preserve current constructors and exact
   omission behavior.
2. Reuse `OpenRealtimeMediaSessionRequest::with_maximum_output_tokens`; do not
   add or change a shared runtime carrier.
3. Admit only Research 194's exact positive domain for the exact model, facade
   point, and private behavior revision. Reject out-of-domain values before
   endpoint, credential, or socket work. Do not clamp, alias, substitute, or
   infer a default.
4. Bind `Capability::OutputTokenLimit` and exact
   `CapabilityConstraint::OutputTokenMaximum` through configured instance/model
   route where applicable, operation requirements, preflight plan, prepared
   evidence, request, and low-level driver.
5. Serialize exact `generationConfig.maxOutputTokens` in both initial and
   rollover/resume setup frames. When omitted, retain the current exact setup
   frames and add no output-limit capability.
6. Carry the same request and plan through fresh realtime working-state
   restoration. Reject request, plan, evidence, model, facade, or driver drift
   before effects when knowable.
7. Compose the maximum with omitted reasoning and every admitted Research 193
   thinking level. Do not change `thinkingLevel`, omission, capability, or
   facade truth beyond the revision Research 194 requires.
8. Update the exact Contract 029 facade point/private behavior revision as
   Research 194 requires. Do not rewrite or backfill the current
   thinking-capable behavior.
9. Preserve Gemini media, transcription, manual activity, usage, interruption,
   deadline, one-rollover, credential, transport, provider-failure, and joined
   cleanup behavior. Do not change sibling realtime routes.

## Acceptance Criteria

- [x] only Research 194 deliver-now values prepare
- [x] input, capability constraint, plan, evidence, request, driver, and setup
      bytes agree exactly
- [x] omission preserves current initial and resume setup bytes
- [x] one planned rollover and fresh restoration retain the selected maximum
- [x] selected and omitted reasoning compose without drift
- [x] unsupported values and every knowable mismatch reject before effects
- [x] no shared runtime or sibling realtime behavior changes
- [x] no alias, clamp, substitution, truncation, retry, or fallback enters the
      API

## Validation

```sh
cargo fmt -p swallowtail-adapter-gemini
effigy validate:focused swallowtail-adapter-gemini
effigy package:verify-affected swallowtail-adapter-gemini
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 132 when exact preparation, initial/resume setup,
restoration, omission, composition, and rejection tests pass.

## Stop Conditions

- the existing portable carrier or capability constraint cannot express the
  admitted exact domain
- selected maximum can drift across setup, rollover, or restoration
- implementation changes reasoning or a sibling realtime route
- implementation needs client-side truncation, live proof, contract amendment,
  shared runtime change, or breaking API

## Out Of Scope

- shared docs/indexes, other Gemini controls/routes/models, live provider work,
  release, or merge
