# 113 Runtime Ownership And Rollover Implementation Tranche

Status: superseded
Owner: Tom
Created: 2026-07-28
Milestone: `../033-runtime-ownership-and-connection-rollover-feature-closure.md`
Depends on: card 112

## Objective

Implement only the contract-ready ownership and rollover routes selected by
cards 111-112.

## Scope

1. Add exact prepared inputs and capability claims for selected routes.
2. Bind runtime, connection, host, topology, version, and cleanup before
   effects.
3. Reject ownership, continuity, model, cursor, or topology drift.
4. Preserve provider completion and cleanup as independent outcomes.
5. Join runtime, connection, task, and cleanup work.
6. Change matrix cells only after focused conformance passes.

## Acceptance Criteria

- [ ] every converted cell has a public prepared path
- [ ] attached or provider-owned resources cannot mint owned lifecycle
- [ ] reconnect cannot masquerade as replay or continuation
- [ ] cancellation and cleanup failures remain visible
- [ ] focused exact-range conformance passes offline

## Auto-Continuation

No. Research 060 selected no implementation candidate.
