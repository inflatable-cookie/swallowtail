# 151 Claude SDK Repaired-MCP Zero-Credit First Turn

Status: complete; Desktop PR 179 merged at `d7d0fb12`
Owner: Desktop live-evidence harness owner
Created: 2026-09-09
Milestone: `../036-v0-4-4-release-readiness.md`
Depends on: Card 150 complete; Desktop Card 315 merge `dfc9c6a6`

## Goal

Capture the first Claude provider-turn result while the operator-confirmed
balance remains zero, through the repaired registered-MCP startup path.

## Scope

1. Use Desktop main after Card 315 and exact source-linked Swallowtail
   `0d120067cd260b1f5127835cab0b1a3ad020a29d`. Preserve SDK `0.3.259`, native
   `2.1.259` Darwin arm64, Node `22.23.2`, sidecar `0.4.4`, MCP `2025-11-25`,
   default permission, persistence false, and `claude-sonnet-5`.
2. Run the existing provider-free checks before live execution. Then run
   exactly one fresh open through registered MCP and submit exactly one prompt:
   `Reply with exactly: ready`.
3. Record the immutable courier digest, authenticated MCP connection,
   provider readiness, prompt digest, bounded start/terminal result, action
   counts, terminal state, and cleanup in one new immutable redacted capsule.
4. Close immediately after the first bounded failure or successful turn.
   Never retry.

Owned mutable paths: Desktop Card 132/314 runner and wrapper only if an
offline-before-live assertion needs repair; focused tests; one new capsule,
factual log, card result, and additive closeout docs. Queue closeout owns
shared Desktop indexes.

Forbidden: second open or prompt; registered-tool invocation; permission
response; control, retry, replay, reconnect, or respawn; top-up or account
mutation; raw provider/error text, account identity, billing detail, secrets,
paths, endpoints, or environment values; Swallowtail production changes;
qualification, matrix, candidate, tag, or release changes.

## Acceptance Criteria

- [ ] provider-free checks pass before the live path becomes reachable
- [ ] exactly one live open and at most one prompt occur, with no retry
- [ ] MCP is declared and authenticated through the immutable courier before readiness
- [ ] one new capsule records stable bounded fields and no sensitive provider text
- [ ] cleanup is confirmed under the exact Card 313 oracle
- [ ] every prior capsule remains byte-identical
- [ ] both Contract 061 cells remain unqualified

## Validation

- Desktop `effigy check:claude-registered-tool-live --check`
- Swallowtail `effigy validate:card116-mediated-stdio`
- immutable-capsule digest checks
- Desktop docs and Northstar spine QA
- `git diff --check`
- exact-head independent review

## Decision Tree

- First-turn start rejection or terminal provider failure: accept the bounded
  zero-credit observation without naming billing as the cause unless a safe
  typed field does so; close and stop.
- Successful prompt: record `credit_exhaustion_not_observed_at_first_turn`,
  close and stop.
- MCP startup failure, unexpected callback/tool/control, leakage, count excess,
  ambiguous identity, or unconfirmed cleanup: defect; preserve evidence and
  stop without retry.

## Review Oracle

Reject if MCP is bypassed, the courier is not the repaired immutable artifact,
live execution can precede checks, more than one open or prompt can occur,
old capsules change, redaction weakens, or cleanup is inferred.

## Auto-Continuation

No. Top-up waits for this capsule to merge and return to Chatterbox.

## Result

Desktop Card 316 reached provider readiness with MCP connected through the
immutable courier, submitted exactly one prompt, and ended with typed
`swallowtail.claude-agent.sdk.provider_failed`. The result exposed
`api_error_status` and `terminal_reason` as present fields, but the current
sidecar retained presence only. Zero tools, callbacks, controls, retries, or
reconnects occurred; cleanup matched the route-qualified degraded macOS
posture. Capsule SHA-256:
`d16b7f8d86513633704a82a6901746dba0cf8dd57d36a5948f1f43e007aa4c28`.

PR 179 accepted head `b129d3bbb0170aaed6e34071acf2a537234f45c8`
merged at `d7d0fb1288f325869d2f3596d646d13fa1936188` after independent
review comment `5596986943`; Desktop canonical closeout is
`6a9ba2c5fc02ee3f1613dada2b9a47ae57b3deb4`. Both Contract 061 cells remain
unqualified. Research 300 and Card 152 own the lost structured failure facts.
