# 145 Claude SDK Credit-Exhaustion Diagnostic

Status: ready; operator authorized one exhausted-credit diagnostic open on 2026-09-08
Owner: Desktop live-evidence owner
Created: 2026-09-08
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: card 144 merged at `cc53c81a`; Desktop PR 170 runner; operator-confirmed exhausted Claude usage credit

## Goal

Capture how the exact Claude SDK route reports known exhausted usage credit
through Card 144's structured failed-open receipt, without spending a prompt
turn or making a qualification claim.

## Scope

1. Desktop links the complete development graph to exact Swallowtail
   `6a93f1d916945aa2b402df7329dc994570e005c8` and proves the Card 144 receipt
   API is compiled into its existing Card 132 runner.
2. Preserve the frozen Card 132 tuple except for the evidence purpose. Use
   `claude-sonnet-5`, the cheapest model in Desktop's already-admitted
   `claude-sonnet-5` / `claude-opus-5` inventory.
3. Run exactly one fresh prepared registered-session open while the operator
   confirms usage credit remains exhausted. Submit no prompt and run no Deny,
   cancellation, stale-callback, or tool-call control.
4. Record the stable route code, Card 144 stage, exact bounded sidecar subcode
   when present, provider-readiness truth, cleanup disposition, and the same
   redaction and source/artifact identities as Card 132.
5. If open unexpectedly succeeds, close immediately, record
   `credit_exhaustion_not_observed`, and perform no turn. If it returns an
   account/usage subcode, preserve it exactly. If it remains generic
   construction-class rejection, preserve that contradiction for Chatterbox.

Forbidden: a prompt turn; registered-tool invocation; control attempts;
automatic retry, replay, reconnect, or respawn; account mutation or top-up;
qualification or matrix changes; tag, release, or candidate work; raw provider
content, account identity, billing details, credentials, paths, endpoints, or
environment values in the capsule.

## Acceptance Criteria

- [ ] one and only one fresh live open is attempted while usage credit is still operator-confirmed exhausted
- [ ] `claude-sonnet-5` is used and no prompt, tool call, or control attempt runs
- [ ] the redacted capsule carries the exact Card 144 receipt fields and source/artifact tuple
- [ ] cleanup is observed without inference and no automatic retry/reconnect/respawn occurs
- [ ] the outcome is classified only as observed account/usage rejection, generic live-only rejection, or successful-open-without-turn

## Validation

- existing Desktop provider-free runner checks
- Swallowtail `effigy validate:card116-mediated-stdio`
- capsule schema/redaction and one-attempt-count checks
- Desktop docs QA and exact-head independent review

## Decision Tree

- Bounded account/usage subcode: accept as evidence that the harness surfaces
  exhaustion; keep both Contract 061 cells unqualified; wait for operator top-up
  before a separately authorized qualification suite.
- Generic construction rejection: retain the live-only observability gap and
  both unqualified cells; no retry.
- Successful open: close without a turn; record that exhaustion was not
  observed at open; both cells remain unqualified.
- Any leakage, ambiguous identity, missing cleanup, extra attempt, turn, tool
  call, or control attempt: stop and return the defect to Chatterbox.

## Auto-Continuation

No. The credited qualification suite requires a later operator confirmation
that credit was restored and separate live authority.

## Result

Pending Desktop capsule.
