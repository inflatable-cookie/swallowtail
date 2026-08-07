# 154 Route Inventory Single-Sourcing

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../051-validation-machinery-and-index-closure.md`
Depends on: card 153

## Goal

Single-source the 34-route inventory so a route touch is one edit, not five.

## Scope

1. Add one shared Python module (alongside `scripts/provider_route_matrix/`)
   that owns the authoritative route id list, derived from the feature-matrix
   CSV.
2. Derive the shell heredoc check, both regex parsers, and the frozen baseline
   comparison from that module:
   - `scripts/check-provider-route-matrix.sh:19-54,118-153`
   - `scripts/provider_route_matrix/assertions.py:299-305`
   - `scripts/check-integration-guide-coverage.py:104-117`
   - `scripts/check-provider-activity-matrix.py:251-256`
   - `release-baselines/production-routes-0.2.0.txt` consumers
3. Keep the cross-checks that compare the sets; the goal is one edit point,
   not weaker validation.

## Out Of Scope

- route, feature, or lifecycle truth changes
- renaming or renumbering any route

## Acceptance

- [ ] adding one route to the CSV updates every consumer without a second
      edit
- [ ] `effigy qa:routes` and `effigy qa:docs` pass unchanged

## Stop Conditions

- stop if the shared module changes what the checks validate

## Auto-Continuation

Yes, to card 155 after acceptance.

## Validation

- `effigy qa:routes`, `effigy qa:docs`, `effigy check:examples`
