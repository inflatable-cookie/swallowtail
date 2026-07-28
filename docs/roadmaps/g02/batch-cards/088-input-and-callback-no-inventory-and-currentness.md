# 088 Input And Callback No Inventory And Currentness

Status: completed
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

- [x] all 74 starting cells are accounted for exactly once
- [x] every false negative cites a realized prepared path
- [x] every unstable upstream claim cites current authoritative evidence
- [x] callback and authority strength are not silently flattened
- [x] one contract-ready or contract-gated tranche is recommended
- [x] machine checks preserve counts and classifications

## Result

Research 050 records:

- 4 routes ready under existing portable contracts
- 36 cells needing shared contract detail or exact route corpus
- 5 composite-solution cells that can become only `Partial`
- 3 xAI cells retained under the operator hold
- 25 exact selected-route absences
- 1 realtime-media input that is not an attachment
- no realized matrix error

Card 089 selects six cells:

- Pi RPC attachment input
- OpenCode HTTP attachment and approval-or-question exchange
- Anthropic Messages attachment, consumer-tool exchange, and external search

Anthropic tool exchange requires one adjacent interactive-session role under
Contract 030. The selected tranche covers all four audited feature columns
across three transports.

## Stop Conditions

- provider evidence requires private credentials or undocumented endpoints
- route identity or version authority is ambiguous
- tranche selection would establish product priority between equally useful
  routes

## Auto-Continuation

Satisfied. Continue to card 089.
