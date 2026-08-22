# 108 xAI Responses Control Binding

Status: complete
Owner: Tom
Created: 2026-08-22
Milestone: [g04.039 xAI Responses Reasoning And Output Bounds](../039-xai-responses-reasoning-output-bounds.md)
Depends on: card 107; promoted Research 187

## Goal

Bind only Research 187 deliver-now reasoning and output controls through xAI
prepared input, immutable plan/evidence, configured driver policy, and exact
WebSocket requests.

## Scope

1. Add portable `ReasoningSelection` only for exact Research 187 model/value
   rows. Reject aliases and multi-agent semantics.
2. Add the existing portable maximum-output-token control only for exact
   admitted models and profiles. Do not synthesize truncation.
3. Bind exact reasoning and output capability constraints into the model route
   and preflight plan.
4. Retain both selections in `XaiPreparedEvidence` and configure the low-level
   driver with the same policy.
5. Emit exact Responses-body `reasoning.effort` and `max_output_tokens` fields
   on first and later WebSocket `response.create` requests.
6. Keep one fixed session selection across later turns and fresh replacement.
7. Preserve byte-identical absent-control request bodies and current constructors
   where compatible.
8. Reject input/plan/evidence/driver/request mismatch before endpoint or
   credential use. Add deterministic preparation and protocol tests.

## Acceptance Criteria

- [x] only Research 187 deliver-now combinations prepare
- [x] input, plan, evidence, driver, and every request agree exactly
- [x] reasoning and output controls can be selected independently where evidence
  allows
- [x] no alias or multi-agent meaning silently maps to portable reasoning
- [x] absent controls preserve current public and wire behavior
- [x] known failures occur before network work

## Validation

```sh
cargo fmt -p swallowtail-adapter-xai
effigy validate:focused swallowtail-adapter-xai
effigy package:verify-affected swallowtail-adapter-xai
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 109 when exact binding, continuation, and compatibility
tests pass.

## Stop Conditions

- implementation needs a facade revision or Contract 040 amendment
- selection drifts across later turns or fresh replacement
- output bounds require local truncation or a false effectiveness claim
- preserving current public behavior requires a breaking change

## Out Of Scope

- guide, matrix, architecture, programme, or changelog closeout
- provider acceptance, effective-depth, or exact-length claims
- search, tools, multi-agent, currentness, or live work

## Closeout

The additive prepared inputs now retain optional `ReasoningMode` and positive
maximum-output-token selections. Exact constraints are carried in the model
route and plan, exposed through `XaiPreparedEvidence`, and checked again by
the run/session drivers before endpoint or credential use. The protocol emits
only selected `reasoning.effort` and `max_output_tokens` fields; omitted
controls keep the prior body. Commit `537ca567` carries the binding and
deterministic coverage.
