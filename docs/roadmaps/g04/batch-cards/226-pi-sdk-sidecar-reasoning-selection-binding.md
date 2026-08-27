# 226 Pi SDK Sidecar Reasoning Selection Binding

Status: blocked; Research 228 empty deliver-now set
Owner: Tom
Created: 2026-08-27
Milestone: [g04.081 Pi SDK Sidecar Reasoning Selection](../081-pi-sdk-sidecar-reasoning-selection.md)
Depends on: card 225; promoted Research 228 with a non-empty deliver-now set

## Goal

Bind only Research 228's exact Pi SDK-sidecar model/value/lifecycle rows through
portable `ReasoningSelection`, immutable prepared evidence, canonical sidecar
bootstrap, and effective-state agreement before readiness.

## Scope

1. Expose one optional typed `ReasoningMode` through the prepared
   `pi.sdk-sidecar` session surface. Do not expose raw Pi strings, cycling,
   settings, model switching, or a generic options map.
2. Admit only exact Research 228 provider/model/value/lifecycle rows. Reject
   foreign models, values, versions, and combinations before launch recipe,
   environment, credential, resource, or provider work.
3. Bind the exact selection in `SessionOptions`, capability requirements,
   immutable plan, prepared request/evidence, attachment requests, and fresh
   restoration. Preserve exact request-plan-driver agreement.
4. Encode canonical `thinkingLevel` only when selected. Omission must retain
   the exact current bootstrap object and claim no selected/default value.
5. Compare bootstrap and every post-switch state snapshot with the exact
   requested mode. Treat clamp, substitution, missing/unknown state, model
   fallback, or stored-state conflict as fail-closed drift before readiness.
6. Reapply the caller-declared mode through the runtime replacement factory on
   load and resume only for lifecycles admitted by Research 228. Do not mutate
   an arbitrary durable session through an unsupported operation.
7. Advance the private wire, behavior, and source tag only if Research 228
   proves the semantic change requires it; keep all four interface axes exact.
8. Preserve provider/model/resource/session verification, replay bounds,
   attachments, cancellation, durable provider state, close/join, and
   credential-last cleanup.

## Acceptance Criteria

- [ ] only exact Research 228 rows prepare and dispatch
- [ ] request, plan, evidence, command, and effective state agree
- [ ] clamp, substitution, mismatch, and unsupported lifecycle fail closed
- [ ] omission retains exact prior bytes and capability posture
- [ ] load/resume/restoration follow Contracts 012 and 017
- [ ] no unrelated Pi route, sidecar control, or shared contract widens

## Validation

```sh
cargo fmt -p swallowtail-adapter-pi
effigy validate:focused swallowtail-adapter-pi
effigy package:verify-affected swallowtail-adapter-pi
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 227 only when exact preparation, plan agreement,
canonical dispatch, state confirmation, omission, attachment, and restoration
proof pass.

## Stop Conditions

- exact selection cannot be rejected before effects or confirmed before ready
- the sidecar must accept unbounded strings or silently tolerate clamp
- durable attachment cannot preserve caller-declared session-option meaning
- delivery needs live provider work, account state, or ambient settings

## Out Of Scope

- mode changes after readiness, cycling, model switching, arbitrary settings,
  `pi.rpc`, newer SDK currentness, live provider work, release, merge, rollover,
  or g04 closure
