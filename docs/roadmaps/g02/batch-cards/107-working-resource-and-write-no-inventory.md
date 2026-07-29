# 107 Working Resource And Write No Inventory

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../032-working-resource-and-workspace-authority-feature-closure.md`
Depends on: card 106

## Objective

Classify every working-resource and bounded-workspace-text-write `No` before
selecting implementation routes.

## Scope

1. Parse and freeze the 31-cell starting inventory from the canonical CSV.
2. Reconcile every cell with realized prepared APIs, contracts, and fixtures.
3. Revalidate plausible capabilities against current official provider or
   maintained-project documentation.
4. Separate:
   - resource selection from resource access
   - working directory from filesystem boundary
   - callback I/O from ambient harness access
   - bounded text replacement from provider tools and shell execution
   - provider enforcement from host enforcement
5. Record exact version, route, host, resource, access, topology, failure,
   cleanup, and support-authority constraints.
6. Rank concrete conversion candidates and identify missing contracts or
   route corpora.
7. Change matrix cells only for demonstrated false negatives.

## Acceptance Criteria

- [x] all 31 starting cells are accounted for exactly once
- [x] every false negative cites a realized prepared path
- [x] every unstable claim cites current authoritative evidence
- [x] resource selection, writes, ambient authority, and containment remain
      distinct
- [x] one contract-ready or contract-gated tranche is recommended
- [x] machine checks preserve counts and classifications

## Evidence

- Research 058 classifies all 31 cells: 24 operation-shape
  non-applicabilities, six selected-surface absences, and one contract-gated
  Gemini ACP write profile.
- There are no realized matrix false negatives.
- Gemini CLI `0.51.0` already mediates in-workspace text writes through the
  ACP client. The current Swallowtail profile intentionally disables that
  callback and selects Plan Mode.
- The selected tranche remains `AmbientHost`; it adds no sandbox or process
  containment claim.

## Stop Conditions

- evidence requires private credentials, undocumented endpoints, or live
  provider effects
- route, resource, access, host, version, or support authority is ambiguous
- tranche selection would establish product priority between equally useful
  routes

## Auto-Continuation

Continue only when one evidence-ranked tranche is unambiguous and its contract
gap is exact.
