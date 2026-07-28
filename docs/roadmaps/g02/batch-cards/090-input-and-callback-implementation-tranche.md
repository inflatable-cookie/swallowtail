# 090 Input And Callback Implementation Tranche

Status: planned
Owner: Tom
Created: 2026-07-28
Milestone: `../027-input-and-callback-feature-closure.md`
Depends on: card 089

## Objective

Implement the contract-ready input/callback tranche through existing prepared
route identities.

## Scope

1. Implement only routes selected by card 088 and frozen by card 089.
2. Keep each input or callback on its exact operation shape.
3. Bind request, plan, dispatch, callback admission, response, and cleanup.
4. Reject undeclared, mismatched, late, duplicate, or unsupported exchanges
   before they can become authority.
5. Preserve cancellation, deadlines, topology, version posture, and redacted
   diagnostics.
6. Update matrix cells only after public prepared paths and conformance exist.

## Acceptance Criteria

- [ ] every changed matrix cell has a realized prepared operation
- [ ] input and callback authority agree from request through completion
- [ ] malformed and unsupported inputs fail deterministically
- [ ] callbacks remain consumer-executed and exactly correlated
- [ ] topology, cleanup, and version posture remain unchanged
- [ ] package examples compile without live access

## Auto-Continuation

No while planned. Card 089 must make the implementation envelope exact.
