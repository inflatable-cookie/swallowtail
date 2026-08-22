# 117 Cline Thinking Control Binding

Status: blocked
Owner: Tom
Created: 2026-08-22
Updated: 2026-08-22
Milestone: [g04.042 Cline Thinking Controls](../042-cline-thinking-controls.md)
Depends on: card 116; promoted Research 190 with a non-empty deliver-now set

## Goal

Bind only Research 190 deliver-now Cline route/value rows through the owning
route's prepared input, immutable plan/evidence, request or session policy,
configured driver, and exact child argv.

## Scope

1. Add optional typed reasoning input only to the prepared ACP session and/or
   headless run profiles admitted by Research 190. Preserve existing
   constructors and absent behavior.
2. Expose only exact portable `ReasoningMode` values justified by Research
   190. Reject raw strings, upstream aliases, unqualified route/value rows,
   and incompatible package or behavior evidence.
3. Add exact `ReasoningSelection` capability constraints to the selected
   configured instance, requirements, and plan only when requested.
4. Bind the same selection into request/session policy, prepared evidence, and
   an explicitly configured low-level driver. ACP and headless bindings remain
   separate even when their typed input is structurally similar.
5. Emit only the qualified argv: one fixed ACP child-spawn selection and/or
   one headless run selection. Expose no generic argv, setting, environment,
   provider, or model choice.
6. Preserve exact current arguments when reasoning is absent:
   `cline --acp` for ACP and the existing
   `--json --auto-approve false ...` headless command.
7. Repeat an admitted ACP selection when
   `prepare_working_state_restoration` creates a fresh context-losing child.
   Do not claim ACP load/resume. Keep headless selection local to one run.
8. Ensure ambient or persisted configuration cannot override the planned
   value where Research 190 proves that check. Do not create a synthetic home
   or mutate user/project settings.
9. Validate request/session, plan, evidence, route, package, behavior, and
   driver agreement before task or process work. Preserve existing resource,
   cancellation, terminal, and cleanup behavior.

## Acceptance Criteria

- [ ] only Research 190 deliver-now rows prepare
- [ ] portable input, plan, evidence, policy, driver, and argv agree exactly
- [ ] ACP and headless capability claims remain independent
- [ ] absent reasoning preserves current command and public behavior
- [ ] no alias, default, model inference, or generic parameter map enters the
      public API
- [ ] fresh ACP replacement repeats the same immutable selection when ACP is
      admitted
- [ ] known mismatch and unsupported failures occur before process work

## Validation

```sh
cargo fmt -p swallowtail-adapter-cline
effigy validate:focused swallowtail-adapter-cline
effigy package:verify-affected swallowtail-adapter-cline
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 118 when exact route-local binding, absent-path, and
zero-process failure tests pass.

## Stop Conditions

- portable reasoning cannot represent a Research 190 row exactly
- prepared evidence and driver cannot remain in exact agreement
- ACP and headless would need one combined capability claim
- ambient or persisted state can override the selected value
- fresh ACP replacement loses or changes an admitted selection
- compatibility requires a new contract, unplanned currentness work, or a
  breaking public lifecycle change

## Out Of Scope

- route guides, matrices, architecture, programme, changelog, or shared
  closeout
- provider-effective reasoning claims not proved by Research 190
- other Cline settings, models, modes, permissions, tools, or live work

## Closeout

Not executed. Research 190 admits no deliver-now route/value row. `cline.acp`
discards the parsed flag entirely, and every `cline.headless` value depends on
a provider and model the route does not select, so binding a portable
`ReasoningSelection` would claim a capability from argv acceptance or accept
Cline's own clamp and substitution.
