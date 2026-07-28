# 060 Nucleus Thread Lifecycle Adoption Handoff

Status: completed
Owner: Tom
Created: 2026-07-26
Milestone: `../019-provider-session-lifecycle-acceptance-and-handoff.md`

## Objective

Give Nucleus a bounded adoption envelope that keeps local thread lifecycle and
optional provider-session management separate.

## Governing Refs

- Contract 038
- repository authority map
- cards 058-059
- existing Nucleus prepared Codex handoff evidence

## Scope

1. Document Nucleus-local archive, restore, and delete as the universal path.
2. Document optional provider archive, restore, or delete only when the bound
   route exposes a prepared operation.
3. Define separate local and provider result persistence, partial success,
   retry, warning, and unverified-newer presentation requirements.
4. Define inactive-session ordering: stop local work, close the runtime handle,
   then request provider management.
5. Provide deterministic consumer scenario cases without prescribing UI copy.
6. Record exact package/API inputs, rollback, unsupported routes, and
   acceptance evidence.
7. Do not edit Nucleus.

## Acceptance Criteria

- [x] Nucleus can implement local-only lifecycle for every route
- [x] provider deletion is optional, explicit, and capability-gated
- [x] local success never hides provider failure or uncertainty
- [x] ACP history removal is presented no stronger than proven
- [x] Kimi, Gemini, and not-applicable routes have honest local-only behavior
- [x] consumer confirmation, persistence, UI, and retry policy remain in
      Nucleus

## Validation

- handoff links and examples
- package API compile check for handoff snippets
- docs and route-matrix checks
- `git diff --check`

## Stop Conditions

- the handoff invents Nucleus product policy or UI
- consumer edits would be required
- a provider action becomes implicit on local archive or deletion
- a partial provider result cannot be represented honestly

## Auto-Continuation

No. Stop for separate Nucleus planning and edit authority.

## Outcome

The
[Nucleus lifecycle handoff](../../../releases/0.1.0-nucleus-provider-session-lifecycle-handoff.md)
defines separate local and provider actions, exact supported routes, inactive
handle ordering, partial success, effect uncertainty, version posture,
compile-checked examples, and rollback. Nucleus was not edited.

The current public management binding has no stable persistence codec. Initial
adoption is same-process only. Missing bindings and post-restart actions stay
local-only; durable export and import is deferred as separate contract work.

## Validation Evidence

- all workspace examples compile
- public API baseline passes for 23 crates
- provider route, lifecycle, and feature matrices pass
- Northstar and docs checks pass
- formatting and `git diff --check` pass
- `effigy doctor` remains red only on the recorded 66 file-size findings
