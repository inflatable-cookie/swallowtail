# 063 Remaining Addable Gap Classification

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../022-further-addable-inventory.md`
Depends on: card 062

## Goal

Classify remaining routes as reuse, descriptor work, or gated.

## Scope

1. Hosted OAuth stays parked.
2. Owned, headless, and response-only stay off sibling addable rows.
3. OpenHands stays without a production route.

## Out Of Scope

- adapter-local descriptors
- compiling the first implementation roadmap
- live probes

## Acceptance Criteria

- [ ] each inventoried route is reuse, descriptor work, or gated
- [ ] hosted OAuth remains parked
- [ ] no production code changes

## Validation

- `effigy qa:docs:index:research`
- `git diff --check`

## Auto-Continuation

Yes, into card 064.

## Stop Conditions

- Stop if a gated route is classified as the next addable descriptor.
