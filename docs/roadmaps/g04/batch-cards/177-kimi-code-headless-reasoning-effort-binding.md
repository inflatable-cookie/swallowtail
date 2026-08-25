# 177 Kimi Code Headless Reasoning-Effort Binding

Status: blocked; Research 210 empty deliver-now set
Owner: Tom
Created: 2026-08-25
Milestone: [g04.063 Kimi Code Headless Reasoning Effort](../063-kimi-code-headless-reasoning-effort.md)
Depends on: card 176; non-empty Research 210 deliver-now table

## Goal

Bind only Research 210's exact reasoning-effort rows through typed prepared
input, immutable evidence, selected-model agreement, and process-local child
environment construction.

## Work

1. Add optional typed `ReasoningMode` selection to the smallest Kimi headless
   prepared input. Do not expose raw strings, config objects, or environment
   maps.
2. Admit the selection only for exact Research 210 executable-version,
   selected-model, and value rows. Preserve existing absence behavior.
3. Carry the same selection through prepared evidence, request policy, driver,
   and child launch. Advertise `ReasoningSelection` only where exact rows exist.
4. Set only the exact qualified process-local child environment key. Do not
   mutate user configuration or create a synthetic Kimi config root.
5. Resolve inherited environment/config precedence before process creation.
   Reject contradiction, shadowing, fallback exposure, or missing selected-
   model support without launching a child.
6. Keep access, model selection, one-prompt stream-json arguments, retention,
   managed recovery, retry, cancellation, deadlines, and diagnostics exact.
7. Keep thought content and reasoning-summary activity absent. Redact any
   selected value where existing diagnostic policy requires it.
8. Add focused fixtures, tests, example/API baseline, and guide changes only
   as required by the delivered surface.

## Acceptance Criteria

- [ ] only Research 210 exact rows prepare and dispatch
- [ ] request, evidence, selected model, driver, and child env agree
- [ ] unsupported, shadowed, substituted, or defaulted values fail pre-spawn
- [ ] omission preserves existing arguments, environment, and behavior
- [ ] no user config, ambient durable state, or raw setting surface mutates
- [ ] no thought content or reasoning-summary activity becomes public
- [ ] lifecycle, access, retention, recovery, and retry truth remains exact
- [ ] no shared contract/runtime or breaking public API change
- [ ] `cargo fmt -p swallowtail-adapter-kimi` passes
- [ ] `effigy validate:focused swallowtail-adapter-kimi` passes
- [ ] `effigy package:verify-affected swallowtail-adapter-kimi` passes
- [ ] `git diff --check` passes

## Stop Conditions

- Research 210 is empty or contradicts the planned typed mapping
- safe dispatch requires config mutation, ambient authority, live confirmation,
  generic settings, shared authority, or a breaking API

## Out Of Scope

- ACP/local-server/Python routes, unrelated Kimi controls, live provider work,
  currentness, release, merge, generation rollover, or g04 closure
