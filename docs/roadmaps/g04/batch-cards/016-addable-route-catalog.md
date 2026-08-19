# 016 Addable Route Catalog

Status: planned
Owner: Tom
Created: 2026-08-19
Milestone: `../006-addable-catalog-admission-and-config-fields.md`
Depends on: completed g04.005

## Goal

Let a consumer assemble an addable-route catalog from adapter-local
descriptors without a registry crate.

## Scope

1. Catalog assembly over adapter-local descriptors.
2. Hosted, installed, and local-runtime grouping.
3. Available, unavailable, and unsupported observations.
4. Testkit fixture descriptors only.

## Out Of Scope

- production adapter descriptors
- admission (card 017)
- Contract 008 discovery of one selected driver
- sign-in execution

## Acceptance Criteria

- [ ] absence of a descriptor means the consumer did not link that adapter
- [ ] unavailable can name a missing install, runtime, or host service
- [ ] unsupported is distinct from unavailable
- [ ] discovery candidates are not catalog rows

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-testkit`
- `git diff --check`

## Auto-Continuation

Yes, into card 017, after this card is marked ready.

## Stop Conditions

- Stop if an umbrella registry crate appears.
- Stop if topology is folded into `ExecutionLayer`.
