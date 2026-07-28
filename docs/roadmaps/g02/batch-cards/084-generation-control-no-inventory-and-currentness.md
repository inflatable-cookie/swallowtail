# 084 Generation-Control No Inventory And Currentness

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../026-generation-control-feature-closure.md`
Depends on: card 083

## Objective

Classify every current output-token-limit, reasoning-selection, and
structured-output `No` before selecting implementation routes.

## Scope

1. Parse the canonical CSV by exact provider, solution, route, and feature.
2. Reconcile all 48 starting cells with realized prepared APIs and fixtures.
3. Revalidate remaining plausible capabilities against current official
   provider or maintained-project documentation.
4. Distinguish native hard limits, best-effort parameters, reasoning
   effort/mode selection, native schema enforcement, prompt conventions, and
   client post-validation.
5. Record version, model, transport, credential, and support-authority
   constraints.
6. Rank concrete conversion candidates and identify missing shared contracts.
7. Update the matrix only for demonstrated false negatives; leave
   implementation gaps as `No`.

## Acceptance Criteria

- [x] all 48 starting cells are accounted for exactly once
- [x] every false negative cites a realized prepared path
- [x] every upstream claim cites current authoritative evidence
- [x] enforcement strength is not silently flattened
- [x] one contract-ready or contract-gated tranche is recommended
- [x] machine checks preserve counts and classifications

## Result

Research 049 records:

- 4 controls ready under existing request contracts
- 18 controls needing exact shared detail or corpus
- 3 ready xAI controls retained under the operator hold
- 20 exact upstream absences
- 3 managed-agent operation-shape mismatches
- no realized matrix error

Card 085 selects OpenAI background and Realtime, Ollama attached, and OpenCode
HTTP. The tranche covers seven cells through three existing adapter crates.

## Stop Conditions

- provider evidence requires private credentials or undocumented endpoints
- route identity or version authority is ambiguous
- tranche selection would establish product priority between equally useful
  routes

## Auto-Continuation

Satisfied. Continue to card 085.
