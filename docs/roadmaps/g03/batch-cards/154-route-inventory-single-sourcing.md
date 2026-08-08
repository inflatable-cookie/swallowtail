# 154 Route Inventory Single-Sourcing

Status: completed
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

- [x] adding one route to the CSV updates every consumer without a second
      edit
- [x] `effigy qa:routes` and `effigy qa:docs` pass unchanged

## Stop Conditions

- stop if the shared module changes what the checks validate

## Auto-Continuation

Yes, to card 155 after acceptance.

## Validation

- `effigy qa:routes`, `effigy qa:docs`, `effigy check:examples`

## Completion Evidence

- new `scripts/provider_route_matrix/route_inventory.py` owns the
  authoritative 34-route inventory derived from the feature-matrix CSV
  (splitting composite `route_id` cells like the previous parser) and
  validates count and uniqueness on every call; it also owns the per-route
  provider-session lifecycle posture table, failing loudly on a missing
  route
- `check-provider-route-matrix.sh` now generates both its expected lists
  (route ids and lifecycle posture rows) from the module instead of the two
  heredocs
- `check-integration-guide-coverage.py` and `check-provider-activity-matrix.py`
  derive from the module and keep their document regexes as a doc-versus-CSV
  cross-check, so validation strength is unchanged
- `check-consumer-front-door.py` compares the module inventory against the
  frozen `v0.2.0` baseline instead of re-parsing the document
- single-source property proven by mutating the CSV: every consumer reacted
  to the change without an edit, then the matrix was restored byte-identical
- `effigy qa:routes`, `effigy qa:docs` (all fifteen checks), and
  `effigy check:examples` pass unchanged
