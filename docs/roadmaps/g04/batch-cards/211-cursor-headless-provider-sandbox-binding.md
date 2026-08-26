# 211 Cursor Headless Provider-Sandbox Binding

Status: planned; conditional on Research 223
Owner: Tom
Created: 2026-08-26
Milestone: [g04.076 Cursor Headless Provider Sandbox](../076-cursor-headless-provider-sandbox.md)
Depends on: card 210; promoted Research 223 with a non-empty deliver-now set

## Goal

Bind only Research 223's exact Cursor headless sandbox rows through
`HarnessIsolation::ProviderEnforced`, immutable prepared evidence,
fail-closed driver validation, and canonical `--sandbox enabled`.

## Scope

1. Extend the Cursor headless prepared input with only the typed isolation
   posture admitted by Research 223. Expose no raw string, boolean, sandbox
   enum, config map, network mode, or path policy.
2. Advertise `ProviderEnforced` only on exact admitted
   route/build/platform/access rows. Preserve omission as `AmbientHost` with no
   `--sandbox` token.
3. Bind selection through prepared input, operation requirement, immutable
   plan/evidence, request policy, driver, platform facts, and command argv.
4. Validate exact build, platform/backend prerequisites, isolation, access,
   request/plan/evidence/driver agreement, and any Research 223 configuration
   condition before spawn.
5. Add only canonical `--sandbox enabled`. Never select `disabled`, rely on a
   persisted default, or fall back to ambient execution.
6. Preserve `--mode plan` for `Read`, default mode for `ReadWrite`, `--trust`,
   exact model parameters, working resource, ambient configuration truth, and
   durable provider-state posture.
7. Preserve activity, usage, cancellation, deadline, terminal, failure,
   process ownership, and joined cleanup. Advance a private behavior revision
   only when Research 223 requires one.

## Acceptance Criteria

- [ ] only Research 223 deliver-now rows prepare
- [ ] input, capability, plan/evidence, driver, platform facts, and argv agree
- [ ] omission retains exact prior argv and `AmbientHost` behavior
- [ ] unsupported, mismatched, configurable, unavailable, fallback, or weaker
      rows reject before process work
- [ ] no runtime isolation mutation, raw config, network/path policy, approval
      response, host-isolation claim, or sibling-route behavior appears
- [ ] access, Plan, tools, model, retention, lifecycle, and cleanup claims do
      not widen

## Validation

```sh
cargo fmt -p swallowtail-adapter-cursor
effigy validate:focused swallowtail-adapter-cursor
effigy package:verify-affected swallowtail-adapter-cursor
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 212 only when preparation, platform gating, canonical
argv, omission, rejection, access composition, and lifecycle proof pass.

## Stop Conditions

- existing prepared state cannot bind every admitted platform/configuration
  fact without a generic or breaking surface
- sandbox selection can drift or fall back after preparation
- implementation needs raw config, approval exchange, host isolation,
  sibling-route work, shared contract change, or authority widening

## Out Of Scope

- shared closeout selection, another Cursor feature/route, live provider work,
  currentness, release, merge, rollover, or g04 closure
