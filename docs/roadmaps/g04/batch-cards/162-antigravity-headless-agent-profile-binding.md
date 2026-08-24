# 162 Antigravity Headless Agent Profile Binding

Status: conditional
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Milestone: [g04.058 Antigravity Headless Agent Profile Selection](../058-antigravity-headless-agent-profile-selection.md)
Depends on: card 161; non-empty Research 205 deliver-now table

## Goal

Bind only Research 205's admitted Antigravity agent-profile rows through typed
prepared inputs, immutable plan/request agreement, exact command construction,
and stream-JSON confirmation.

## Work

1. Add the smallest adapter-local typed profile-id surface admitted by Research
   205. Do not expose raw profile definitions, provider settings, or a generic
   string map.
2. Add optional profile selection to admitted structured-run and continuation
   preparation shapes only. Omission must preserve existing public
   construction and capability truth.
3. Carry the exact optional id through immutable prepared evidence, plan
   constraints, request agreement, run/session state, and every admitted child.
4. Emit one adjacent `--agent <id>` pair after the exact model selection and
   before the prompt/continuation-specific tail. Emit nothing on omission.
5. Extend the event parser to require exact `init.agent` equality for selected
   children and the Research 205 omission shape for unselected children.
6. Map invalid profile, missing confirmation, foreign confirmation, malformed
   init, provider rejection, and plan/request mismatch to stable safe failures.
7. Preserve model, effort, schema, resource access, isolation, permission mode,
   host deadline, cancellation, terminal, conversation, and join behavior.
8. Add focused tests and examples/guidance only for delivered rows.

## Acceptance Criteria

- [ ] only Research 205 deliver-now rows prepare
- [ ] explicit selection appears in capability/constraint, prepared evidence,
      immutable plan, request, command, and confirmation truth
- [ ] omission keeps byte-identical argv and existing behavior
- [ ] every selected child emits exactly one `--agent <id>`
- [ ] missing or mismatched `init.agent` fails before output is accepted
- [ ] no invalid value falls back to ambient/default agent selection
- [ ] selected continuation, if admitted, reasserts and confirms one immutable
      profile on every child
- [ ] access, isolation, permission, model, effort, schema, deadline,
      cancellation, conversation, failure, and cleanup behavior stay intact
- [ ] public API baseline and examples reflect only the admitted typed surface
- [ ] `cargo fmt -p swallowtail-adapter-antigravity` passes
- [ ] `effigy validate:focused swallowtail-adapter-antigravity` passes
- [ ] `effigy package:verify-affected swallowtail-adapter-antigravity` passes
- [ ] `git diff --check` passes

## Stop Conditions

- Research 205 is empty or ambiguous
- binding requires shared contract/runtime changes, generic configuration,
  profile mutation, or public exposure of private provider data
- exact confirmation cannot precede accepted output
- continuation cannot preserve one immutable profile across child replacement

## Out Of Scope

- other Antigravity flags or routes, agent management/catalogue UI, account
  work, currentness, release, publication, merge, generation rollover, or g04
  closure
