# Side-Effect-Free Preflight And Fixtures

Status: completed
Owner: Tom
Roadmap: 004 Runtime Records and Preflight
Updated: 2026-07-19

## Goal

Implement pure Contract 008 preflight and reusable testkit evidence that every
rejection occurs before side effects.

## Scope

- operation requirement records using card 010 vocabulary
- immutable preflight plan bound to driver, instance revision, route, access,
  ownership, host, requirements, and extensions
- dimensional preflight failures
- stale-plan detection
- recording zero-side-effect fixture and focused assertions

## Out Of Scope

- driver trait calls
- process, network, credential, or resource services
- routing or fallback policy
- runtime crate

## Acceptance Criteria

- missing role, host service, capability, constraint, route, access, ownership,
  or topology fails before side effects
- successful plan preserves exact selected dimensions
- materially changed instance revision invalidates the plan
- fixture failures name the rejected dimension safely

## Validation

- core and testkit focused tests
- `effigy qa`
- `git diff --check`

## Closeout

- Pure preflight rejects missing role, service, capability, constraint, route,
  access, support authority, ownership, topology, or extension dimensions.
- Successful immutable plans expose and bind the exact driver, instance
  revision, route, access profile, ownership, execution host, and operation
  requirements.
- Current-state validation rejects a materially changed instance revision as a
  stale plan.
- `swallowtail-testkit` supplies a reusable preflight fixture, dimensional
  failure cases, and a provider-side-effect recorder; all rejection cases
  leave its counter at zero.
