# 118 Codex Legacy Version Dispatch

Status: completed
Owner: Tom
Updated: 2026-07-23
Milestone: `../039-installed-harness-compatibility-range-audit.md`

## Objective

Add private legacy behavior dispatch to both existing Codex drivers while
preserving current isolated and experimental behavior.

## Scope

- deprecated exec segments with exact config and retention requirements
- legacy boolean and mode-based search arguments
- omission of unsupported ephemeral and suppression flags
- deprecated app-server default-stdio and explicit-listener segments
- stable read-only catalogue, open, resume, turn, interruption, and cleanup
- no legacy dynamic tools, provider requests, or bounded workspace roots
- exact configured-instance capabilities per observed version
- current `0.122.0+` exec and `0.110.0+` app-server behavior unchanged
- no new public operation shape, v1 facade, route fallback, auth work, or
  consumer edit

## Acceptance Criteria

- [x] every dispatch starts from the immutable exact version binding
- [x] ambient config and durable retention require explicit request agreement
- [x] current suppressed exec never degrades to ambient
- [x] app-server never sends an unsupported listener or request field
- [x] legacy capability absence rejects before process work
- [x] unpublished, prerelease, malformed, and unknown versions remain closed
- [x] both existing driver identities remain honest

## Validation

- focused Codex driver and policy tests
- workspace all-target check
- workspace warnings-denied clippy
- `git diff --check`

Completed with 72 passing Codex adapter tests. Workspace all-target check and
warnings-denied clippy pass. Doctor remains at the inherited 19 findings: seven
errors and twelve warnings.

## Auto-Continuation

Yes. Continue to card 119 after both drivers execute their frozen segments.
