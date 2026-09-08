# 149 Claude SDK Zero-Credit First-Turn Diagnostic

Status: ready; one bounded Desktop live attempt authorized
Owner: Desktop live-evidence harness owner
Created: 2026-09-08
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: card 148 complete; operator confirmation that credit remains zero

## Goal

Exercise the first Claude SDK query exactly once while account credit remains
zero, then preserve the bounded harness error and cleanup truth before top-up.

## Scope

1. Extend Desktop's Card 132 runner with a distinct one-turn diagnostic mode.
   Prove its capsule, redaction, one-attempt budget, terminal branches, and
   cleanup oracle provider-free before any live process starts.
2. Use the frozen Card 147 tuple and exact source-linked Swallowtail
   `0d120067cd260b1f5127835cab0b1a3ad020a29d`: SDK `0.3.259`, native
   `2.1.259` Darwin arm64, Node `22.23.2`, sidecar `0.4.4`, MCP `2025-11-25`,
   `claude-sonnet-5`, default permission, persistence false.
3. Open once and submit exactly one prompt: `Reply with exactly: ready`.
   Observe start-turn and terminal completion as separate bounded stages.
4. Close immediately after the first terminal result or rejection. Record the
   stable route code, bounded `turn_ended` fields, progress presence, session
   terminal state, cleanup disposition, redaction result, exact prompt digest,
   identities, and action counts.
5. If the prompt unexpectedly succeeds, preserve that outcome and stop. No
   second prompt or attempt follows.

Forbidden: retry, replay, reconnect, respawn, second open, second turn,
registered-tool invocation, permission response, control attempt, account
mutation or top-up, raw provider/error text, account identity, billing detail,
credentials, paths, endpoints, or environment values. No qualification,
matrix availability, candidate, tag, release, or dependency-pin change.

## Acceptance Criteria

- [ ] provider-free fixtures pass before live execution and prove no live path is reachable from check mode
- [ ] exactly one open and one prompt submission occur; every retry and extra-action count is zero
- [ ] first-turn start rejection, terminal provider failure, and unexpected success remain distinct
- [ ] the capsule carries only stable codes and bounded terminal metadata; raw provider text is absent
- [ ] close accepts only Clean, NotApplicable, or the exact Card 148-qualified Degraded posture with every companion invariant satisfied
- [ ] the immutable Card 312 capsule and prior evidence remain byte-identical
- [ ] no outcome qualifies Contract 061 cells or authorizes top-up, candidate, tag, or release

## Validation

- Desktop provider-free runner, wrapper, schema, redaction, count, and cleanup fixtures
- immutable Card 312 capsule digest check
- `effigy qa:docs` and `effigy qa:northstar:spine`
- `git diff --check`
- exact-head independent review

## Decision Tree

- First-turn rejection or terminal provider failure: accept the bounded harness
  error as the zero-credit observation; do not claim an exact billing cause
  unless a safe typed field says so; close and stop.
- Successful prompt: record `credit_exhaustion_not_observed_at_first_turn`,
  close, and stop.
- Leakage, ambiguous identity, missing cleanup, unexpected callback/tool/control,
  or any attempt-count excess: defect; preserve the capsule and stop without retry.

## Auto-Continuation

No. The live authority is consumed once. Top-up follows only after Chatterbox
reports that this capsule is merged and preserved.

## Result

Pending.
