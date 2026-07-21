# 037 Direct Run, Catalogue, And Provider Evidence

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../011-hosted-transport-foundations.md`

## Objective

Remove process-harness assumptions from direct structured runs and add the
smallest typed catalogue, usage, rate, and quota observations.

## Governing References

- Contract 006
- Contract 008
- Contract 009
- Contract 014

## Scope

- optional working-resource binding on structured runs
- explicit Codex requirement for its existing resource
- optional model input/output token limits
- typed token-usage snapshots
- distinct rate-limit and quota observations
- semantic runtime event delivery for provider observations

## Out Of Scope

- billing or cost calculation
- retry policy
- provider-specific token semantics in core
- direct provider adapter

## Implementation Steps

1. Make structured-run resource binding optional and migrate current callers.
2. Keep Codex exec fail-closed when its resource is absent.
3. Add mutable model token-limit metadata.
4. Add safe provider observation records and semantic event kinds.
5. Prove cumulative snapshot replacement semantics in fixtures.

## Acceptance Criteria

- [x] direct structured run can exist without a resource reference
- [x] Codex behavior and request shape remain unchanged
- [x] absent catalogue limits mean unknown
- [x] usage, rate limit, and quota are distinct types
- [x] no observation triggers retry, fallback, or billing behavior

## Validation

- focused core, runtime, Codex, and testkit tests
- `git diff --check`

## Evidence Required

- direct no-resource fixture
- Codex missing-resource rejection before process start
- catalogue and provider-observation unit tests

## Evidence

- `StructuredRunRequest` now treats the working resource as an optional binding.
- Codex exec requires that binding before process start; every existing Codex
  request still supplies it.
- model token limits are optional mutable metadata.
- token usage, rate-limit state, and quota state are separate provider
  observations; Codex exec emits typed usage instead of formatted prose.

## Stop Conditions

- direct inference gains a separate catch-all prompt API
- provider-specific field names enter stable common records
- existing Codex behavior widens

## Auto-Continuation

Yes. Continue to card 038 when focused validation passes.
