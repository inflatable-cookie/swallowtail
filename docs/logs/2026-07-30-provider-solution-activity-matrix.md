# 2026-07-30 Provider Solution Activity Matrix

## Outcome

Card 135 is complete. Swallowtail now publishes one machine-readable
provider-solution activity inventory and one concise consumer guide.

The matrix covers 55 exact route-operation pairs:

- 32 available ordinary structured-run or interactive-session profiles
- 23 not-applicable catalogue, inventory, provider-session-management,
  realtime-media, and serving operations
- all 26 production routes
- four auxiliary hosted catalogue identities

Every row links a public prepared entry, prepared-facade conformance test, and
exact harness or direct activity inventory record.

## Decisions

- Intermediate assistant content and final-answer content remain separate
  semantic channels. Streaming final-answer deltas do not imply intermediate
  harness messages.
- Lifecycle strength uses complete, update-and-completion, completion-only,
  mixed, and profile-dependent values. It is not a boolean.
- Disclosure distinguishes provider display, adapter summary, and
  identity-and-lifecycle-only evidence.
- `profile-dependent` directs consumers to exact prepared evidence. It does
  not mean unresearched.
- Tool display input and output cover activity content only. Callback and
  direct-tool exchange bodies remain on their typed surfaces.
- Catalogue, inventory, provider-session-management, realtime-media, and
  serving operations remain explicitly not applicable to ordinary agent
  activity.
- Sensitive-content retention, transcript projection, labels, grouping,
  collapsed UI, and thread ownership remain downstream.

## Machine Gate

`scripts/check-provider-activity-matrix.py` validates:

- exact operation inventory from the harness and direct fixtures
- stable row order and allowed values
- 32 available and 23 not-applicable dispositions
- 26 production and four auxiliary route identities
- prepared entry, conformance test, and exact evidence file references
- no not-applicable feature cells inside an available operation

The checker now runs under `effigy qa:routes`.

## Validation

- `effigy qa:routes`
- `effigy qa:docs`
- `effigy package:api`

All passed. `effigy doctor` retains the pre-existing oversized-file debt: 130
findings, including 32 errors.

## Next

Card 136 is ready: assemble all local package archives and prove selected rich,
thin, direct, realtime, not-applicable, and unverified-newer profiles from
extracted artifacts. Publication and retained-candidate replacement remain
out of scope.
