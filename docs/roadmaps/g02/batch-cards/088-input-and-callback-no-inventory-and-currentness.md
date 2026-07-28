# 088 Input And Callback No Inventory And Currentness

Status: ready
Owner: Tom
Created: 2026-07-28
Milestone: `../027-input-and-callback-feature-closure.md`
Depends on: card 087

## Objective

Classify every current attachment, consumer-tool, approval-or-question, and
external-search `No` before selecting implementation routes.

## Scope

1. Parse the canonical CSV by exact provider, solution, route, and feature.
2. Reconcile all 74 starting cells with realized prepared APIs and fixtures.
3. Revalidate plausible capabilities against current official provider or
   maintained-project documentation.
4. Separate:
   - attachment transport from filesystem or workspace access
   - consumer tool callbacks from provider-owned tool execution
   - approval and question callbacks from observed-and-stopped requests
   - provider search controls from arbitrary external network authority
5. Record version, model, transport, credential, topology, and support
   authority constraints.
6. Rank concrete conversion candidates and identify missing shared contracts.
7. Update the matrix only for demonstrated false negatives.

## Acceptance Criteria

- [ ] all 74 starting cells are accounted for exactly once
- [ ] every false negative cites a realized prepared path
- [ ] every unstable upstream claim cites current authoritative evidence
- [ ] callback and authority strength are not silently flattened
- [ ] one contract-ready or contract-gated tranche is recommended
- [ ] machine checks preserve counts and classifications

## Stop Conditions

- provider evidence requires private credentials or undocumented endpoints
- route identity or version authority is ambiguous
- tranche selection would establish product priority between equally useful
  routes

## Auto-Continuation

Continue to card 089 only when the selected tranche and missing contracts are
exact.
