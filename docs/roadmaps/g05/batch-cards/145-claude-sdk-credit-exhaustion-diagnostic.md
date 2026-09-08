# 145 Claude SDK Credit-Exhaustion Diagnostic

Status: complete; Desktop PR 172 merged at `117e09e02dafd505d9f1d6e2b1380b56bbb22a19`
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

- [x] one and only one fresh live open is attempted while usage credit is still operator-confirmed exhausted
- [x] `claude-sonnet-5` is used and no prompt, tool call, or control attempt runs
- [x] the redacted capsule carries the exact Card 144 receipt fields and source/artifact tuple
- [x] cleanup is observed without inference and no automatic retry/reconnect/respawn occurs
- [x] the outcome is preserved as bounded non-account `mcp_status_invalid`; no quota inference follows

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

Desktop task `9dc9b50f-5bb2-4912-be7d-a6159e8c758d` completed through PR 172.
Independent review accepted head
`8ac46df322bec428e744adb5d22fba3a6ad9a7cc`; review comment `5589754115`;
merge `117e09e02dafd505d9f1d6e2b1380b56bbb22a19`; canonical Desktop closeout
`5d7f1681222222d6f1586176c05733c6f1daf7c3`.

The immutable capsule SHA-256 is
`81cb0fd6c5a717e9ca66b73c7a746e66870219ebf2c1330081dfd76549608569`.
It records exactly one open and zero prompt turns, tool dispatches, permission
callbacks, controls, retries, reconnects, or respawns. The route returned
`typed_failure` at `sidecar_rejected`, bounded subcode `mcp_status_invalid`,
before provider readiness. Cleanup was confirmed with joined task reapers,
zero survivors, listeners, and registered leases.

This did not observe the exhausted-credit state. It exposed a pre-readiness
MCP-status contradiction outside the three expected decision-tree outcomes.
Research 297 freezes the return. Card 146 owns provider-free diagnosis and
repair before any credited live suite. Both Contract 061 cells remain
unqualified. No quota, qualification, candidate, tag, or release inference
follows.
