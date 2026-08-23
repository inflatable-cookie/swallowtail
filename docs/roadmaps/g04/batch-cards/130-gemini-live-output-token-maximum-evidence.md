# 130 Gemini Live Output-Token-Maximum Evidence

Status: ready
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Milestone: [g04.047 Gemini Live Output-Token Maximum](../047-gemini-live-output-token-maximum.md)
Depends on: Research 021; Research 193; g04.046

## Goal

Freeze current exact-model and Live-facade `maxOutputTokens` behavior, then
define the smallest positive Gemini Live output-maximum subset that can
satisfy Contracts 027, 037, and 040.

## Method

1. Freeze the current official exact-model page, Live WebSocket reference,
   and generation-config reference for model
   `gemini-3.1-flash-live-preview`, `v1beta` `BidiGenerateContentSetup`,
   `generationConfig.maxOutputTokens`, and the model output-token limit.
   Record retrieval dates, page dates, stable references, and specimen
   digests.
2. Prove or reject exact applicability. `BidiGenerateContentSetup` accepts a
   `GenerationConfig` and its unsupported-field list does not name
   `maxOutputTokens`, but the general reference warns that not every parameter
   is configurable for every model. Do not convert absence from an exclusion
   list or the model catalogue limit into support without exact composed
   evidence.
3. Close the numeric domain. Test the candidate positive range up to the exact
   model limit of 65,536 against official evidence. Classify zero, values above
   the model limit, negative/fractional/overflowing forms, aliases, clamping,
   and provider defaults explicitly.
4. Freeze current route source and deterministic fixtures for model, endpoint,
   facade axis/point, private behavior revision, initial setup, resume setup,
   prepared input, plan, request, driver validation, reasoning selection, and
   fresh restoration.
5. Prove the omission boundary. Current initial and resume fixtures have no
   `maxOutputTokens`; omission must retain those exact bytes and must not add
   `OutputTokenLimit` capability evidence.
6. Prove whether one selected maximum is valid and immutable across initial
   setup, provider-planned rollover/resume setup, and fresh realtime
   working-state restoration. Prove composition with every Research 193
   thinking level without changing either control's meaning.
7. Classify the exact model/facade applicability and decide the new opaque
   facade point and adapter-private behavior revision. Do not rewrite the
   thinking-capable point or extend another version/model by inference.
8. Classify Contract 040 evidence truth: deterministic setup bytes can prove
   dispatch; provider acceptance and effective generated length remain
   separate unless this exact surface returns explicit confirmation.
9. Replace Research 194's reservation with exact route/value dispositions and
   a deliver-now table. Do not edit shared architecture, matrices, programme,
   indexes, changelog, or roadmap front doors.

No credential, account inspection, provider request, paid operation, browser
login, or live Gemini call is authorized. Current official documentation and
secret-free repository/source inspection are sufficient for this gate.

## Acceptance Criteria

- [ ] exact model, Live facade, field applicability, and numeric domain are
      source-backed or explicitly withheld
- [ ] initial, resume, omission, restoration, reasoning composition, and
      current behavior truth are explicit
- [ ] every candidate value class has a deliver-now, gated, withheld, or
      obsolete disposition
- [ ] facade point/private behavior revision and exact applicability are
      explicit
- [ ] acceptance and effective generated length are not inferred from dispatch
- [ ] Research 194 is promoted with an exact route/value deliver-now table
- [ ] no production code, capability, guide, matrix, or compatibility claim
      changes during evidence
- [ ] `effigy validate:focused swallowtail-adapter-gemini` passes
- [ ] `effigy qa:northstar` and `effigy qa:docs:index:research` pass
- [ ] `git diff --check` passes

Auto-continue to card 131 only when Research 194 admits a non-empty exact
positive domain and preserves current omission, reasoning, rollover,
restoration, and contract boundaries.

## Stop Conditions

- current official evidence cannot prove the field on the exact model/facade
- the positive domain cannot be closed without a live call or inference
- omission would change current setup bytes
- rollover, restoration, or reasoning composition cannot retain one immutable
  maximum
- delivery needs client-side truncation, a shared carrier change, contract
  change, version inference, or breaking API

## Out Of Scope

- production binding, guide/matrix claims, other Gemini routes/models, token
  counting, client truncation, live work, or shared closeout surfaces
