---
title: g05.045 Claude SDK consumer multi-turn editing acceptance
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom said 'Go for it' on 2026-09-09, authorizing the bounded Desktop live proof and its provider-free Swallowtail consumption. This task performs only the latter and must not contact a provider."
queue:
  dependsOn: [bc4acd98-91a3-4e14-86e0-38376b037da7]
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g05.045 by consuming the exact merged Desktop g02.049 capsule and
deciding whether it closes g05.029's failed consumer multi-turn editing clause.

## Why It Matters

The SDK route's individual capabilities are already shipped, but the final
audit found no evidence chain combining a real consumer, two user turns in one
session, two edits, and a consumer decision before every tool effect. Desktop
g02.049 owns exactly one cost-bounded live attempt to produce that chain.

## Current State

Swallowtail g05.029 is historically stopped through audit PR #307 at merge
`55595c38fa644b4e2a4111ecef4d5fbfb58fb733`; clauses 2–5 passed and clause 1
failed. Released `v0.4.4` peels to
`49c9e3b291609c9ebf5b35a284c08302f3b8d5e3`. Desktop task
`bc4acd98-91a3-4e14-86e0-38376b037da7` is queued to repair its
`acceptEdits` mapping and run one two-turn `claude-sonnet-5` session. This task
cannot start until that queue task reaches `done`.

## Boundaries

Follow `docs/roadmaps/g05/045-claude-sdk-consumer-multi-turn-editing-acceptance.md`.
Read the capsule only from the exact merged Desktop tree and recompute its
digest. Own Research 303, one provider-free evidence fixture/test, the exact
Claude SDK guide evidence note, a bounded follow-up subsection in g05.029, one
log, and their indexes. Preserve g05.029's historical stopped verdict. Do not
edit runtime/sidecar/API behavior, feature-matrix disposition, versions,
ranges, Desktop, credentials, releases, or tags. Do not contact Claude or any
provider.

## Important Context

Apply the literal seven-row oracle. Two turns means one open and one stable
provider session with two ordered user-turn identities, not two sessions.
Every native tool call—not only edits—needs a correlated Desktop callback and
allow/deny decision before tool start and any filesystem effect. Both turns
need a successful write-tool effect. `acceptEdits` fails; the capsule must show
SDK `default`. Absence of forbidden effects, retries, fallback, and third turns
must be recorded rather than inferred. A fake replay binds accepted evidence
but cannot replace the live Desktop capsule.

## Suggested Next Move

After the dependency closes, freeze its PR, review, merge, closeout, capsule,
and source identities. Score every oracle row before editing. If all pass, add
the smallest provider-free evidence binding and factual docs. If one fails,
record the exact stop and do not seek or run a retry.

## Completion Protocol

Run `effigy validate:focused swallowtail-adapter-claude-agent`, `effigy
package:verify-affected swallowtail-adapter-claude-agent`, `effigy qa:docs`,
`effigy qa:northstar`, and `git diff --check`. Open one PR. Independent
exact-head review verifies merged-source identity, capsule immutability,
literal oracle scoring, deterministic evidence binding, historical-truth
preservation, zero provider use, and scope. Return both repositories' exact
heads/merges/reviews, capsule SHA-256, tuple/model/counts/transitions/cleanup,
validation, and canonical closeout. No provider rerun, range widening, release,
or tag follows.
