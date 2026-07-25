# 018 Shared Facade Evidence And Conformance

Status: completed
Owner: Tom
Created: 2026-07-25
Completed: 2026-07-25
Milestone: `../007-provider-wide-facade-contract-and-foundation.md`

## Objective

Add the smallest provider-neutral records and assertions needed by
adapter-local prepared facades and typed bound operations.

## Governing Refs

- Contract 037
- Contracts 008-011 and 029
- card 017

## Scope

1. Inventory Codex prepared records before adding shared types.
2. Add only evidence common to all runtime shapes: selected driver and role,
   instance/host binding, compatibility where applicable, access provenance,
   expanded safe specification, and drift identity.
3. Add testkit assertions for visible evidence, explicit operation input,
   failure-before-effects, lifecycle delegation, and low-level accessibility.
4. Avoid a generic operation trait when existing role traits suffice.
5. Keep adapter-private plans and provider records in adapter crates.

## Acceptance Criteria

- [x] shared records contain no provider-specific fields
- [x] facade evidence is inspectable without secrets or raw payloads
- [x] assertions work for installed, hosted, and attached routes
- [x] bound execution cannot bypass existing preflight or cleanup
- [x] no central provider construction or selection appears

## Validation

- focused runtime and testkit tests
- Codex prepared regression tests
- `cargo check --workspace`
- `git diff --check`

## Stop Conditions

- common evidence requires a provider-specific enum
- a new generic operation role would flatten lifecycle
- diagnostics would expose a target path, secret, or provider payload

## Execution Evidence

- runtime owns `PreparedOperationEvidence`, its safe binding, exact interface
  assessments, access provenance, and immutable preflight plan
- testkit asserts exact plan agreement across installed-harness, hosted-direct,
  and attached-runtime fixtures
- mismatched access evidence fails before provider effects
- Codex prepared profiles embed the shared evidence while retaining their
  public plan, request, and low-level execution paths
- 209 focused runtime, testkit, and Codex tests pass
- full repository QA passes
- Doctor remains at the pre-existing 19 oversized-file findings; this card
  adds no structural finding
- additive core, runtime, testkit, and Codex public-API drift remains visible
  against the held candidate baseline for card 036 to replace

## Auto-Continuation

Completed. Card 019 is active.
