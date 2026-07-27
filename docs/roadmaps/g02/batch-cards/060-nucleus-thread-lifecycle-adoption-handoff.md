# 060 Nucleus Thread Lifecycle Adoption Handoff

Status: planned
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

- [ ] Nucleus can implement local-only lifecycle for every route
- [ ] provider deletion is optional, explicit, and capability-gated
- [ ] local success never hides provider failure or uncertainty
- [ ] ACP history removal is presented no stronger than proven
- [ ] Kimi, Gemini, and not-applicable routes have honest local-only behavior
- [ ] consumer confirmation, persistence, UI, and retry policy remain in
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

