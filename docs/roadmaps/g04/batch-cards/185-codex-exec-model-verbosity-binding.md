# 185 Codex Exec Model Verbosity Binding

Status: complete
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Milestone: [g04.066 Codex Exec Model Verbosity](../066-codex-exec-model-verbosity.md)
Depends on: card 184; promoted Research 213 with a non-empty deliver-now set

## Goal

Bind only Research 213's exact Codex Exec model-verbosity rows through typed
adapter-local prepared state, immutable evidence, driver validation, and exact
child arguments.

## Scope

1. Add one closed adapter-local `CodexModelVerbosity` with only Research 213's
   admitted values. Preserve current constructors and exact omission behavior.
2. Add an optional typed selection only to `CodexExecProfileInput`. Do not add
   a shared capability, generic settings map, app-server field, or sibling-
   route behavior.
3. Bind the selection to the exact selected model/provider/version/profile
   rows through preparation, model route, plan/evidence, and driver state.
   Reject drift before process, credential, or provider effects.
4. Emit only the exact `--config model_verbosity=<encoded-value>` form admitted
   by Research 213. Do not mutate user/project config or rely on ambient
   defaults.
5. Preserve current argv byte-for-byte when omitted. Do not serialize the
   model's default verbosity on behalf of an omitted caller selection.
6. Reject unknown values, unsupported models/providers/versions, stale model
   rows, and incompatible behavior segments before spawn. Never infer support
   from a model-name prefix or silently drop the selection.
7. Compose with absent and every admitted reasoning value, external search,
   JSON Schema output, and one image without changing their semantics.
8. Preserve access, retention, working resource, activity, usage, provider
   failure, cancellation, deadline, terminal, and joined cleanup truth.
9. Advance only Research 213's selected private behavior/claim/model-route
   revisions. Retain prior claims as superseded proof where required.

## Acceptance Criteria

- [x] only Research 213 deliver-now rows prepare
- [x] input, model route, plan/evidence, driver, and child argv agree exactly
- [x] omission preserves the prior command
- [x] unsupported values/models/providers/versions and drift reject before
      effects
- [x] reasoning, search, schema, image, access, retention, and lifecycle
      behavior remain unchanged
- [x] no shared runtime, portable capability, generic settings, sibling route,
      provider-acceptance, effective-output, or billing claim enters the API

## Validation

```sh
cargo fmt -p swallowtail-adapter-codex
effigy validate:focused swallowtail-adapter-codex
effigy package:verify-affected swallowtail-adapter-codex
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 186 only when exact preparation, command, omission,
composition, rejection, and lifecycle proof passes.

## Stop Conditions

- adapter-local prepared state cannot express the admitted exact set
- selected model support or verbosity truth can drift after preparation
- implementation needs ambient config, a portable capability, generic settings,
  app-server promotion, live proof, unplanned contract change, or breaking API

## Out Of Scope

- shared closeout selection, another Codex control/route, live provider work,
  currentness, release, merge, rollover, or g04 closure
