# 122 Selected Harness Range Dispatch

Status: planned
Owner: Tom
Updated: 2026-07-23
Milestone: `../040-cross-harness-compatibility-range-expansion.md`

## Objective

Add private exact-version dispatch for the selected harness without changing
its public operation shape or silently widening capabilities.

## Scope

- immutable exact version and behavior-revision selection
- version-specific invocation, framing, schema, and lifecycle mapping
- exact configured-instance capabilities and request policy
- rejection before harness work on mismatches
- current behavior regression coverage
- no compatibility facade in provider-neutral core

## Acceptance Criteria

- [ ] every execution starts from one exact qualified point
- [ ] version-specific differences remain adapter-private
- [ ] narrower historical capabilities never appear current
- [ ] unknown or excluded versions remain closed
- [ ] no provider, model, endpoint, credential, or route fallback appears

## Validation

- focused driver and policy tests
- workspace all-target check
- workspace warnings-denied clippy
- `git diff --check`

## Auto-Continuation

Yes, after every frozen behavior segment executes deterministically.
