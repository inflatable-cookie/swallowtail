# 147 Claude SDK Repaired Zero-Credit Diagnostic

Status: ready; one live open explicitly authorized while Claude usage credit remains exhausted
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

Pending.
