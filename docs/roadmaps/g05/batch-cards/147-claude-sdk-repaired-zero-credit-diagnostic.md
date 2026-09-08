# 147 Claude SDK Repaired Zero-Credit Diagnostic

Status: complete; Desktop PR 174 merged at `7646db459d3fb8f194729a46b56acdfa49d41fb2`
Owner: Desktop live-evidence owner
Created: 2026-09-08
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: card 146 merged at `13dee542`; operator confirmation that credit remains zero

## Goal

Repeat the zero-credit diagnostic once on the repaired Claude SDK route and
capture whether exhaustion reaches the bounded failed-open receipt, without a
prompt turn or qualification claim.

## Scope

1. Desktop links its complete development graph to exact source-linked
   Swallowtail `0d120067cd260b1f5127835cab0b1a3ad020a29d` without committing a
   machine-local lock or changing the released dependency pin.
2. Preserve the frozen Card 145 tuple and use `claude-sonnet-5`, the cheapest
   model in Desktop's admitted Sonnet/Opus inventory.
3. Run exactly one fresh prepared registered-session open while the
   operator-confirmed account remains at zero usage credit.
4. Record the stable route code, failed-open stage, bounded sidecar subcode,
   provider-readiness truth, cleanup disposition, redaction result, and exact
   source and artifact identities.
5. If open succeeds, close immediately and record that exhaustion was not
   observed at open. Do not submit a prompt.

Forbidden: prompt or turn; registered-tool dispatch; permission callback;
Deny, cancellation, stale-callback, or other control attempt; retry, replay,
reconnect, or respawn; account mutation or top-up; qualification or matrix
availability; candidate, tag, or release work; raw provider content, account
identity, billing details, credentials, paths, endpoints, or environment
values in the capsule.

## Acceptance Criteria

- [ ] exactly one fresh open runs against source-linked `0d120067`; no second attempt follows
- [ ] `claude-sonnet-5` is used; prompt, turn, tool, permission, and control counts stay zero
- [ ] the capsule distinguishes bounded account/usage rejection, another typed rejection, and successful open
- [ ] cleanup and redaction are observed rather than inferred
- [ ] exact source, SDK, native, Node, sidecar, protocol, model, permission, and persistence identities are frozen
- [ ] both Contract 061 cells remain unqualified and no release consequence is inferred

## Validation

- Desktop provider-free runner and capsule schema/redaction checks
- exact one-open and zero-turn/tool/permission/control/retry assertions
- Swallowtail `effigy validate:card116-mediated-stdio`
- Desktop docs and Northstar QA
- exact-head independent review

## Decision Tree

- Bounded account/usage subcode: accept zero-credit observability evidence;
  keep both cells unqualified; wait for operator top-up before a separately
  authorized credited qualification.
- Successful open: close immediately without a turn; record
  `credit_exhaustion_not_observed`; keep both cells unqualified.
- `mcp_status_invalid`, another non-account rejection, ambiguous identity,
  missing cleanup, leakage, or any extra action: stop and return the exact
  contradiction; do not retry.

## Auto-Continuation

No. The single live-attempt authority is consumed exactly once. The credited
qualification suite remains a later, separate action after credit restoration.

## Result

Desktop task `22fa30cb-a2da-4120-94cb-be260c433923` completed through PR 174.
Accepted head `a935727e3f25e7df0b11325bd60ba5b7d0fba5da`; review comment
`5591728802`; merge `7646db459d3fb8f194729a46b56acdfa49d41fb2`; canonical
Desktop closeout `1a8652b64c22415e6c3a9499f8cd2b7d434816af`.

The single open reached ready/connected state while credit remained zero, then
closed immediately. Counts are one open and zero prompts, turns, tools,
permission callbacks, controls, retries, reconnects, or respawns. Capsule
SHA-256 is
`9f6af32bc0011e01f2d84071c5990a9a657e660b9d90d31e802f13755799cf33`.

Close reported `Degraded`, with joined reapers and zero leases, listeners, or
survivors. Research 298 identifies the Desktop runner defect: it rejected the
Contract 019-qualified ordinary macOS degraded posture because it accepted
only `Clean | NotApplicable`. Card 148 owns the provider-free oracle repair.
The capsule stays immutable. Both Contract 061 cells remain unqualified.
