# 126 Codex And OpenCode Newer-Version Dispatch

Status: completed
Owner: Tom
Updated: 2026-07-24
Milestone: `../041-qualified-support-and-newer-version-execution.md`

## Objective

Permit exact newer Codex and OpenCode releases to attempt execution through
their latest-qualified private behavior without widening guaranteed support.

## Scope

- opt both ordered claims into unverified-newer attempts
- preserve the exact observed and planned version
- use latest-qualified private behavior revision for request mapping
- keep qualified capability claims unchanged
- health and session identity must still match exact OpenCode plans
- stable newer versions only; prereleases remain closed
- provider protocol drift fails safely during actual execution
- no route, model, endpoint, credential, or topology fallback

## Acceptance Criteria

- [x] newer exact versions execute as unverified, never qualified
- [x] current qualified ranges remain unchanged
- [x] known exclusions and malformed versions remain closed
- [x] diagnostics expose posture without provider payloads
- [x] Codex and OpenCode behavior remains adapter-private

## Validation

- focused Codex and OpenCode tests
- workspace all-target check
- workspace warnings-denied clippy
- `git diff --check`

## Auto-Continuation

Yes, after both adapters preserve exact identity and safe runtime failure.

## Outcome

Both Codex claims and the OpenCode server claim opt into stable ordered
forward attempts. Codex `0.146.0` dispatches through each transport's latest
qualified behavior. OpenCode `1.18.5` passes exact health and session matching
through surface 18. Neither version is qualified. Prereleases, historical
gaps, malformed values, missing bindings, and runtime drift still reject.
