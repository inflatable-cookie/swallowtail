# 103 Retained Execution No Inventory And Currentness

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../031-retained-execution-and-recovery-feature-closure.md`
Depends on: card 102

## Objective

Classify every retained-background-execution, stream-reattachment, and
provider-managed-recovery `No` before selecting implementation routes.

## Scope

1. Parse and freeze the 59-cell starting inventory from the canonical CSV.
2. Reconcile every cell with realized prepared APIs, contracts, and fixtures.
3. Revalidate plausible capabilities against current official provider or
   maintained-project documentation.
4. Separate:
   - retained provider execution from durable session state
   - retrieval from stream reattachment
   - stream reattachment from transport reconnect
   - provider-managed recovery from consumer retry or reconstruction
5. Record exact version, route, state identity, cursor, access, cancellation,
   deadline, topology, retention, and support-authority constraints.
6. Rank concrete conversion candidates and identify missing contracts or
   route corpora.
7. Change matrix cells only for demonstrated false negatives.

## Acceptance Criteria

- [x] all 59 starting cells are accounted for exactly once
- [x] every false negative cites a realized prepared path
- [x] every unstable claim cites current authoritative evidence
- [x] retrieval, reattachment, reconnect, and recovery remain distinct
- [x] one contract-ready or contract-gated tranche is recommended
- [x] machine checks preserve counts and classifications

## Stop Conditions

- evidence requires private credentials, undocumented endpoints, or live
  provider effects
- route, state, cursor, version, or support authority is ambiguous
- tranche selection would establish product priority between equally useful
  routes

## Auto-Continuation

Continue only when one evidence-ranked tranche is unambiguous and its contract
gap is exact.

## Outcome

Research 056 classifies all 59 starting cells: 32 operation-shape
non-applicable, 22 selected-surface absences, two separate-route candidates,
and three shared-contract candidates.

No realized false negative changed the matrix. Kimi is the unambiguous first
tranche because qualified headless and local-server retry records currently
pass under a contradictory prohibited-recovery policy. Local-server
WebSocket v2 also supplies an exact bounded cursor-reattachment surface across
the same qualified range.

The route-matrix check freezes every classification and exact count.
