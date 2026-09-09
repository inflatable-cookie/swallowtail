---
title: g05.045 Claude SDK consumer multi-turn editing corrected-oracle acceptance
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom answered 'Yes' on 2026-09-09 to the corrected mediation oracle and one final Desktop two-turn Sonnet session. This Swallowtail task remains provider-free."
queue:
  dependsOn: [bc4acd98-91a3-4e14-86e0-38376b037da7]
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Replace undispatched queue task `84237429-af77-4131-8212-86e75c36369d`, whose
immutable handoff used the superseded tool-start ordering check. Deliver
g05.045 by consuming the exact merged Desktop g02.049 capsule provider-free.

## Why It Matters

g05.029 still lacks one chain combining a real consumer, two editing turns in
one session, and consumer mediation before execution. The SDK may stream a
tool-use proposal/activity row before `canUseTool`; treating visibility as
execution would reject a correctly mediated route.

## Current State

Released `v0.4.4` peels to `49c9e3b291609c9ebf5b35a284c08302f3b8d5e3`.
Desktop task `bc4acd98-91a3-4e14-86e0-38376b037da7` retains PR #205, two
immutable stopped capsules, the mapping repair, and the corrected check-only
harness. Tom authorized its one final two-turn `claude-sonnet-5` session. This
task waits for the Desktop task to reach `done` and owns no provider budget.

## Boundaries

Follow `docs/roadmaps/g05/045-claude-sdk-consumer-multi-turn-editing-acceptance.md`.
Read the capsule only from the exact merged Desktop tree. Own Research 303, one
provider-free evidence fixture/test, the exact guide evidence note, a bounded
g05.029 follow-up, one log, and their indexes. Preserve g05.029's historical
stopped verdict. Do not edit runtime/sidecar/API behavior, matrix truth,
versions, ranges, Desktop, credentials, releases, or tags. Do not contact a
provider.

## Important Context

Apply the corrected oracle literally. A tool-use proposal or activity-start
row may precede the callback. For every native call, the exact Desktop
allow/deny must precede dispatch, result, and filesystem effect, and the
fixture digest must remain unchanged through the decision. Both turns still
need a successful write effect in one stable provider session. Absence of Git,
Bash, web, MCP, outside-path effects, retries, fallback, and a third turn must
be recorded. A deterministic replay binds the live capsule but cannot replace
it.

## Suggested Next Move

After the dependency closes, freeze its PR, review, merge, closeout, capsule,
and source identities. Score every oracle row before editing. Pass only on the
literal evidence chain; otherwise stop without a provider retry.

## Completion Protocol

Run `effigy validate:focused swallowtail-adapter-claude-agent`, `effigy
package:verify-affected swallowtail-adapter-claude-agent`, `effigy qa:docs`,
`effigy qa:northstar`, and `git diff --check`. Open one PR for independent
exact-head review. Return both repositories' exact identities, capsule digest,
tuple/model/counts/transitions/cleanup, validation, review, merge, and
closeout. No provider rerun, range widening, release, or tag follows.
