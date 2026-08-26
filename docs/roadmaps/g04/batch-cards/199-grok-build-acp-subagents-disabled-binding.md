# 199 Grok Build ACP Subagents-Disabled Binding

Status: planned; gated on Research 219
Owner: Tom
Created: 2026-08-26
Milestone: [g04.072 Grok Build ACP Subagents Disabled](../072-grok-build-acp-subagents-disabled.md)
Depends on: card 198; promoted Research 219 with a non-empty deliver-now set

## Goal

Bind only Research 219's exact Grok Build subagents-disabled rows through a
closed adapter-local selection, immutable prepared evidence, fail-closed driver
validation, and canonical child argv.

## Scope

1. Add only the adapter-local disabled profile or named builder selected by
   Research 219. Expose no raw boolean/string, explicit enabled value, generic
   topology map, agent definitions, or portable capability.
2. Bind the selection through installation or operation preparation input as
   Research 219 requires, immutable plan and evidence, prepared state, driver,
   and exact child arguments.
3. Validate exact package, profile membership, lifecycle applicability,
   request/plan/evidence/driver agreement, and replacement state before spawn.
4. Emit only Research 219's canonical `--no-subagents` placement. Omission must
   preserve exact prior argv and public behavior.
5. Keep one immutable selection across initialize, every session and prompt,
   operation-private sessions, attachment recovery, and fresh child
   replacement.
6. Preserve existing permission observe-and-stop behavior. Never add
   auto-approval, tool filtering, agent definitions, callbacks, or direct child
   control.
7. Keep `AmbientHost` exact. Do not infer filesystem, network, sandbox,
   ordinary process-tool, read-only, or OS descendant-process containment.
8. Preserve access, model selection, activity, cancellation, deadline,
   terminal, failure, process ownership, and joined cleanup truth. Advance only
   an exact private behavior revision selected by Research 219.

## Acceptance Criteria

- [ ] only Research 219 deliver-now rows prepare
- [ ] input, plan/evidence, driver, and exact startup argv agree
- [ ] omission retains exact prior argv and route behavior
- [ ] unsupported, mismatched, drifting, and overrideable rows reject before
      process work
- [ ] child observation/control, permission, tool, sandbox, filesystem/network,
      and process-containment claims do not widen
- [ ] existing lifecycle and cleanup behavior remains exact

## Validation

```sh
cargo fmt -p swallowtail-adapter-grok
effigy validate:focused swallowtail-adapter-grok
effigy package:verify-affected swallowtail-adapter-grok
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 200 only when exact preparation, argv, omission,
rejection, replacement, topology separation, and lifecycle proof passes.

## Stop Conditions

- existing prepared state cannot express the admitted exact profile
- restriction truth can drift after preparation without fail-closed detection
- implementation needs explicit enabling, raw flags, generic topology, shared
  contract change, sibling-route work, or a breaking API

## Out Of Scope

- shared closeout selection, another Grok feature/route, live provider work,
  currentness, release, merge, rollover, or g04 closure
