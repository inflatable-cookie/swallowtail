# 202 Cline Headless Plan-Mode Binding

Status: planned; gated on Research 220
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.073 Cline Headless Plan Mode](../073-cline-headless-plan-mode.md)
Depends on: card 201; promoted Research 220 with a non-empty deliver-now set

## Goal

Bind only Research 220's exact Cline headless Plan row through portable
`HarnessMode::Plan`, immutable prepared evidence, fail-closed driver
validation, and canonical child argv.

## Scope

1. Extend `ClineHeadlessRunProfileInput` with only the typed Plan selection
   admitted by Research 220. Expose no raw boolean/string, provider mode enum,
   generic configuration, or Plan-to-Act operation.
2. Advertise `HarnessModeSelection(Plan)` only on the exact qualified
   route/version/behavior row admitted by Research 220. Preserve omission as
   the route default, not implicit Plan.
3. Bind the selection through preparation input, capability requirement,
   immutable plan and evidence, derived request state where required, prepared
   run, driver, and exact child arguments.
4. Validate exact package, behavior revision, mode, request/plan/evidence/
   driver agreement, and replacement state before spawn.
5. Emit only Research 220's canonical `--plan` placement. Omission must retain
   exact `--json --auto-approve false -c <cwd> <prompt>` argv and behavior.
6. Keep the selection immutable across the complete one-child run. Do not add
   runtime mode mutation, a reusable session, follow-up, steering, or resume.
7. Preserve explicit `--auto-approve false`, read-only working-resource
   policy, local-account access, `Ambient`, and `AmbientHost`. Do not convert
   provider Plan behavior into permission, tool, filesystem/network, shell,
   process, sandbox, or descendant containment.
8. Preserve activity, cancellation, deadline, terminal, failure, provider
   retention, process ownership, and joined cleanup. Advance an adapter-private
   behavior revision only when Research 220 requires one.

## Acceptance Criteria

- [ ] only Research 220 deliver-now rows prepare
- [ ] input, capability, plan/evidence, driver, and exact argv agree
- [ ] omission retains exact prior argv and provider-default behavior
- [ ] unsupported, mismatched, drifting, or behaviorally weaker rows reject
      before process work
- [ ] no runtime mode mutation or sibling-route behavior appears
- [ ] permission, tools, isolation, configuration, resources, account access,
      model, retention, lifecycle, and cleanup claims do not widen

## Validation

```sh
cargo fmt -p swallowtail-adapter-cline
effigy validate:focused swallowtail-adapter-cline
effigy package:verify-affected swallowtail-adapter-cline
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 203 only when exact preparation, capability, argv,
omission, rejection, and lifecycle proof passes.

## Stop Conditions

- existing prepared structured-run state cannot express the admitted exact
  Plan selection without a generic or breaking surface
- Plan truth can drift after preparation without fail-closed detection
- implementation needs runtime Plan-to-Act, raw flags/config, sibling-route
  work, shared contract change, or authority widening

## Out Of Scope

- shared closeout selection, another Cline feature/route, live provider work,
  currentness, release, merge, rollover, or g04 closure
