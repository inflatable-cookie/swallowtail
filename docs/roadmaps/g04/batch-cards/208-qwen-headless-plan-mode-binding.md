# 208 Qwen Headless Plan-Mode Binding

Status: complete
Owner: Tom
Created: 2026-08-26
Milestone: [g04.075 Qwen Headless Plan Mode](../075-qwen-headless-plan-mode.md)
Depends on: card 207; promoted Research 222 with a non-empty deliver-now set

## Goal

Bind only Research 222's exact Qwen headless Plan rows through portable
`HarnessMode::Plan`, immutable prepared evidence, fail-closed driver
validation, and canonical child argv.

## Scope

1. Extend the existing Qwen prepared run/session input with only portable
   `HarnessMode::Plan` admitted by Research 222. Expose no raw string, boolean,
   provider approval enum, or generic config map.
2. Advertise `HarnessModeSelection(Plan)` only on exact admitted
   route/version/behavior rows. Preserve omission as explicit provider
   `default`, not implicit Plan.
3. Bind selection through prepared input, capability requirement, immutable
   plan and evidence, request policy, prepared run/session state, driver, and
   every child command.
4. Validate exact package/revision, mode, request/plan/evidence/driver
   agreement, and replacement state before spawn.
5. Replace only the existing approval value with canonical `plan`. Omission
   must retain exact `--approval-mode default` argv.
6. Reapply the same mode on structured runs, reasoning-control children, first
   and later turn children, explicit resume, and fresh replacement. Never infer
   mode from the private provider session id.
7. Preserve `--safe-mode`, exact core/excluded tools, read-only working
   resource, model/reasoning/budgets, delegated access, `Ambient`, and
   `AmbientHost`.
8. Preserve activity, cancellation, deadline, terminal, failure, provider
   retention, process ownership, and joined cleanup. Advance an adapter-private
   behavior revision only when Research 222 requires one.

## Acceptance Criteria

- [x] only Research 222 deliver-now rows prepare
- [x] input, capability, plan/evidence, driver, and every exact child argv
      agree
- [x] omission retains exact prior argv and provider-default behavior
- [x] unsupported, mismatched, drifting, or behaviorally weaker rows reject
      before process work
- [x] no runtime mode mutation, provider-mode vocabulary, writable profile,
      or sibling-route behavior appears
- [x] permission, tools, isolation, configuration, resources, account access,
      model, reasoning, budgets, retention, lifecycle, and cleanup claims do
      not widen

## Validation

```sh
cargo fmt -p swallowtail-adapter-qwen
effigy validate:focused swallowtail-adapter-qwen
effigy package:verify-affected swallowtail-adapter-qwen
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 209 only when preparation, capability, every child argv,
omission, rejection, composition, and lifecycle proof passes.

## Stop Conditions

- existing prepared state cannot express admitted Plan without a generic or
  breaking surface
- selected mode can drift after preparation or between child processes
- implementation needs runtime mode switching, raw config, writable authority,
  sibling-route work, shared contract change, or authority widening

## Out Of Scope

- shared closeout selection, another Qwen feature/route, live provider work,
  currentness, release, merge, rollover, or g04 closure
