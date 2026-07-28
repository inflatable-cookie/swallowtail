# 092 Session Continuity No Inventory And Currentness

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../028-session-continuity-feature-closure.md`
Depends on: card 091

## Objective

Classify every current load-session, resume-session, and native-session-close
`No` before selecting implementation routes.

## Scope

1. Parse the canonical CSV by exact provider, solution, route, and feature.
2. Reconcile all 58 starting cells with realized prepared APIs and fixtures.
3. Revalidate plausible capabilities against current official provider or
   maintained-project documentation.
4. Separate:
   - provider history replay from consumer-local reconstruction
   - replay-free provider attachment from new-session creation
   - provider-native close from process exit, disconnect, archive, or delete
5. Record version, session identity, access, transport, topology, retention,
   and support-authority constraints.
6. Rank concrete conversion candidates and identify missing shared contracts
   or route corpora.
7. Update the matrix only for demonstrated false negatives.

## Acceptance Criteria

- [x] all 58 starting cells are accounted for exactly once
- [x] every false negative cites a realized prepared path
- [x] every unstable upstream claim cites current authoritative evidence
- [x] replay, reattachment, and close effect are not silently flattened
- [x] one contract-ready or contract-gated tranche is recommended
- [x] machine checks preserve counts and classifications

## Stop Conditions

- provider evidence requires private credentials or undocumented endpoints
- route identity, session identity, or version authority is ambiguous
- tranche selection would establish product priority between equally useful
  routes

## Auto-Continuation

Continue only when one evidence-ranked tranche is unambiguous and its contract
gap is exact.

## Outcome

[Research 051](../../../research/051-session-continuity-no-currentness-and-tranche-selection.md)
classifies every starting cell:

- seven fit existing shared contracts
- four require a retained hosted-session contract branch
- one is blocked by upstream replay ordering
- ten are exact selected-route absences
- 36 do not fit the reusable provider-session operation shape

No matrix cell was stale. All 20 native-close cells remain honest `No`.

The first tranche is exact and unambiguous:

- Codex app-server load
- Claude Agent ACP load and resume
- OpenCode HTTP load and resume

Pi RPC load and resume remain the next contract-ready continuation tranche.
Card 093 freezes the exact maintained-range corpora before implementation.
