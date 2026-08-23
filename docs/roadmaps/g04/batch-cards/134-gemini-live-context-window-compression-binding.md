# 134 Gemini Live Context-Window-Compression Binding

Status: conditional
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Milestone: [g04.048 Gemini Live Context-Window Compression](../048-gemini-live-context-window-compression.md)
Depends on: card 133; promoted Research 195 with a non-empty deliver-now set

## Goal

Bind only Research 195's exact Gemini Live compression shapes through
adapter-local typed prepared state, immutable plan/evidence, driver validation,
and initial/resumed setup serialization.

## Scope

1. Add one optional typed compression selection to the Gemini Live prepared
   profile. Preserve current constructors and exact omission behavior.
2. Keep the control inside `swallowtail-adapter-gemini`. Do not add a portable
   capability, shared realtime request field, untyped provider settings map,
   or sibling-route behavior.
3. Admit only Research 195's exact configuration shapes and numeric domain.
   Reject every unsupported or malformed value before endpoint, credential, or
   socket work. Do not default, clamp, alias, substitute, or infer.
4. Bind the exact selection through configured route input, preflight plan,
   prepared evidence, bound driver state, and protocol encoding. Reject
   input/plan/evidence/model/facade/driver drift before effects when knowable.
5. Serialize exact `contextWindowCompression` in both initial and
   rollover/resume setup frames. Omission must retain prior fixture bytes.
6. Carry the same selection through fresh realtime working-state restoration.
   Rollover may not alter it even though the provider permits setup changes at
   resumption.
7. Compose with omitted and every admitted thinking level and with omitted and
   every admitted output maximum. Preserve existing capability truth for those
   portable controls; compression itself adds no portable capability.
8. Advance only the exact facade/private behavior/claim/model-route revisions
   Research 195 selects. Retain prior facade points as superseded proof.
9. Preserve media, transcription, manual activity, usage, interruption,
   latest-resumable-handle, deadline, one-rollover, credential, transport,
   provider-failure, and joined-cleanup behavior.

## Acceptance Criteria

- [ ] only Research 195 deliver-now shapes prepare
- [ ] input, plan, evidence, driver, and setup bytes agree exactly
- [ ] omission preserves prior initial and resume setup bytes
- [ ] one planned rollover and fresh restoration retain the selection
- [ ] thinking and output-maximum controls compose without drift
- [ ] unsupported values and every knowable mismatch reject before effects
- [ ] no shared runtime, portable capability, or sibling route changes
- [ ] no adapter default, alias, clamp, substitution, retry, or fallback enters
      the API

## Validation

```sh
cargo fmt -p swallowtail-adapter-gemini
effigy validate:focused swallowtail-adapter-gemini
effigy package:verify-affected swallowtail-adapter-gemini
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 135 when exact preparation, setup, restoration,
omission, composition, rejection, and lifecycle tests pass.

## Stop Conditions

- adapter-local prepared state cannot express the admitted exact set
- selected compression can drift across setup, rollover, or restoration
- implementation changes another control or realtime route
- implementation needs a portable capability, shared carrier, live proof,
  unplanned contract change, or breaking API

## Out Of Scope

- shared docs/indexes, other Gemini controls/routes/models, live provider work,
  release, or merge
