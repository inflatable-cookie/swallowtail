# 105 DeepSeek Reasoning Binding

Status: ready after 104
Owner: Tom
Created: 2026-08-22
Milestone: [g04.038 DeepSeek Continuation Reasoning Controls](../038-deepseek-continuation-reasoning-controls.md)
Depends on: card 104; promoted Research 186

## Goal

Bind only Research 186 deliver-now DeepSeek reasoning controls through prepared
input, immutable plan/evidence, the configured driver, and every exact request
attempt without weakening continuation replay.

## Scope

1. Extend the existing portable `ReasoningMode` path only with exact values and
   profiles admitted by Research 186. Reject aliases and unsupported modes.
2. If Research 186 admits a thinking-mode choice, add only its exact typed
   adapter-local representation. Do not overload portable effort or expose a
   raw boolean/string map.
3. Bind exact reasoning capability constraints into the model route and plan.
4. Retain the selection in `DeepSeekPreparedEvidence` and configure the
   low-level driver with the same values. Preserve the current no-new-selection
   high/enabled path.
5. Emit exact `reasoning_effort` and `thinking.type` values on structured runs
   and every admitted continuation attempt.
6. Keep one fixed session selection across tool-result requests, later turns,
   and fresh restoration. Preserve provider-private `reasoning_content` bounds.
7. Reject input/plan/evidence/driver/request mismatch before endpoint or
   credential use. Add deterministic preparation and protocol tests.

## Acceptance Criteria

- only Research 186 deliver-now combinations prepare
- no provider alias silently maps to another portable mode
- input, plan, evidence, driver, and every request attempt agree exactly
- current high/enabled behavior remains unchanged on the existing path
- private continuation remains adapter-held, bounded, and correctly replayed
- known failures occur before network work

## Validation

```sh
cargo fmt -p swallowtail-adapter-deepseek
effigy validate:focused swallowtail-adapter-deepseek
effigy package:verify-affected swallowtail-adapter-deepseek
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 106 when exact binding, replay, and compatibility tests
pass.

## Stop Conditions

- the change needs a new facade revision or Contract 030 amendment
- selection drifts across continuation attempts or restoration
- thinking disable falsifies capability or private-continuation truth
- preserving current public behavior requires a breaking change

## Out Of Scope

- guide, matrix, architecture, programme, or changelog closeout
- provider acceptance or effective-depth claims
- other DeepSeek models, facades, tools, limits, or live work

