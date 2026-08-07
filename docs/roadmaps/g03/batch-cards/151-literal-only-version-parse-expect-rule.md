# 151 Literal-Only Version-Parse Expect Rule

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../050-provider-reachable-panic-closure.md`
Depends on: card 150

## Goal

Add a CI rule that keeps `InterfaceVersion::new(...).expect(...)` and its
axis sibling on string literals only, so a provider-reachable panic cannot
regress.

## Scope

1. Add a small source-scan check (extend an existing script or add one next
   to the other check scripts) that rejects `InterfaceVersion::new(` or
   `InterfaceVersionAxis::new(` followed by a non-literal argument within the
   same statement ending in `.expect(`.
2. Wire the check into an existing gate task in `effigy.toml` (for example
   `qa:docs` or a code-scan task) and into CI.
3. Verify the check passes the current tree and fails on a synthetic
   non-literal regression.

## Out Of Scope

- runtime behavior changes
- other panic classes outside version parsing

## Acceptance

- [ ] the check runs in CI and fails a non-literal version-parse expect
- [ ] the current tree passes the check

## Stop Conditions

- stop if the scan produces false positives on macros or generated code

## Auto-Continuation

Yes, to card 152 after acceptance.

## Validation

- run the new gate task; confirm the synthetic negative fails and the tree
  passes
- `effigy qa:docs`
