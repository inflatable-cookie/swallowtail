# 122 OpenCode Range Dispatch

Status: completed
Owner: Tom
Updated: 2026-07-23
Milestone: `../040-cross-harness-compatibility-range-expansion.md`

## Objective

Add private exact-version dispatch for OpenCode HTTP without changing its
public operation shape or silently widening capabilities.

## Scope

- publish the proven `opencode.server` claim on the HTTP driver descriptor
- require exact configured-instance, requirement, and immutable-plan binding
- select only card 121 behavior revisions from that exact binding
- match `/global/health` against the plan before provider catalogue, session
  creation, prompt, or SSE work
- match the created session's version against the same exact binding
- preserve version-specific request, response, event, failure, and lifecycle
  mapping inside the adapter
- preserve deny-first permissions, unknown-event stop, external server
  ownership, and delegated provider authentication
- current `1.14.48` behavior regression coverage
- no installed-executable observation or compatibility facade in core

## Acceptance Criteria

- [x] every execution starts from one exact qualified point
- [x] version-specific differences remain adapter-private
- [x] narrower historical capabilities never appear current
- [x] unknown, excluded, health-mismatched, and session-mismatched versions
      remain closed
- [x] no provider, model, endpoint, credential, or route fallback appears

## Validation

- focused driver and policy tests
- workspace all-target check
- workspace warnings-denied clippy
- `git diff --check`

## Auto-Continuation

Yes, after every frozen behavior segment executes deterministically.

## Outcome

The descriptor publishes the closed `opencode.server` claim. Configured
instances, requirements, and immutable plans must agree on one exact release.
Private dispatch accepts only the 18 frozen behavior revisions.

Every catalogue and session open matches `/global/health` to the exact plan
before further harness work. Session creation must return the same release.
Missing, ambiguous, unpublished, outside-range, health-drifted, and
session-drifted versions fail closed.

All 18 behavior surfaces execute deterministically through host-approved local
HTTP fixtures. The current `1.14.48` route remains covered. The latest
qualified `1.18.4` route opens a session through the same operation shape.

Focused validation passed with 30 tests; one installed probe remained gated.
Workspace all-target check, warnings-denied clippy, formatting, and
`git diff --check` passed.
