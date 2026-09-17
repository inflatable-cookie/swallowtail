---
kind: northstar-handoff
title: "Swallowtail Chatterbox continuation"
handoff_mode: chatterbox-continuation
chatterbox_mode: conversational-planning
dispatch_authority: chatterbox
status: ready-to-launch
base_required: pushed-main
---

## What This Thread Was Doing

Act as Swallowtail's long-running Northstar Chatterbox. The thread took over
the project after the earlier Chatterbox exhausted its context, used Northstar
Queue to complete the `v0.4.4`, `v0.5.0`, and `v0.5.1` release lanes, repaired
the Claude SDK registered-tool and zero-credit evidence path, qualified the
Grok catalogue, and ran the nineteen-family Research 308 version-currentness
campaign through Oh My Pi `18.1.22`.

Tom requested a fresh same-workspace Chatterbox on DeepSeek V4.1 Flash because
this thread is context-heavy. This is an ownership transfer, not an
implementation assignment or a project/workspace replacement.

## Why It Matters

Canonical docs hold the durable project state, but the successor also needs
Tom's process preferences and the live decision boundary after Research 308.
The transfer must preserve the Swallowtail checkout, Queue provenance, and
every existing task/thread identity while making one fresh thread the sole
active Swallowtail planning authority.

## Current State

- Source Chatterbox: `8b7bf4df-3098-448d-9939-3299a5a60ce7`.
- Workspace: `wks_f4a6ebd6fbb93734`, local checkout
  `/Users/tom/Dev/projects/swallowtail`.
- Canonical `main` and `origin/main` at handoff creation:
  `a085d5c95c409e5c69fd0d4e6150a41c91f20ff1` (`Adopt reviewed-head Queue hooks`).
- Queue origin-transfer preflight plan:
  `5d5ea26b6d794300708c70afac23f0662c790ee7827f86ded8fe3e319f2777f3`.
  It found no unfinished Queue tasks currently targeting the source
  Chatterbox; the exact transfer set is empty.
- [g05.079](../roadmaps/g05/079-oh-my-pi-18-1-22-major-line-qualification.md)
  is terminal. PR #340 reviewed head
  `93a30cc1a662ffef461bc945972d1e286b355c27`, review comment `5673967249`,
  merge `dd0b28bd00a52191e62b896753a31868c5052ab8`, and lifecycle closeout
  `a3f2f2d06503899b9056c7fd3c3756eafafb1885` close Research 308.
- The [roadmap front door](../roadmaps/README.md) now requires a generation
  reassessment. No currentness family is ready. Goose remains exact
  `QualifiedOnly` `1.46.0` at the typed provider-auth failure change; Qoder
  remains exact `QualifiedOnly` `1.1.25` at the newly effective
  `--max-turns 8` boundary. Both need an operator policy ruling before renewed
  qualification.
- Planned but undispatched live-evidence stubs remain at g05.066 (Claude Agent
  SDK registered-tool requalification on `0.3.270`/`2.1.270`) and g05.069
  (Command Code `1.54.0` live requalification). g05.035, g05.039-g05.042 also
  remain planned or gated; none is ready merely by appearing in the index.
- `docs/triage/` contains older unresolved route, watcher, feature-projection,
  and provider-session leads. Treat them as intake, not an executable backlog;
  reconcile or prune them only through normal Chatterbox promotion.

## Boundaries

Remain the Swallowtail Chatterbox in the existing local workspace. Planning
and confirmed promotion belong here; implementation, independent review,
merge, and closeout belong to Northstar Queue. Do not create a Chatterbox
worktree, archive or rename the source workspace, alter task provenance, or
resubmit completed work.

No release, tag, publication, provider call, live probe, host update, consumer
mutation, or new task is authorized by this handoff. Currentness is a standing
lane and must not keep g05 open by itself. New product policy, including the
Goose and Qoder stop rulings, remains Tom's decision.

Remain read-only until the source sends the exact follow-up `Ownership transfer
complete`. Do not compete with the source Chatterbox before that message.

## Important Context

- Tom expects autonomous follow-through once authority exists. Do not ask him
  to repeat authorization, and do not treat a Queue completion notice as new
  authority.
- Keep tasks proportionate. Provider-free retrieval, parsing, comparison,
  correction, and validation are normally retryable; do not impose artificial
  one-shot limits. Prefer one selected-source ledger and official release notes
  as discovery authority over repetitive binary archaeology.
- Live provider work must be narrowly budgeted and use the cheapest adequate
  model. Preserve exact calls and typed failures, but do not let incidental MCP
  or harness machinery obscure the provider result.
- A blocked Queue task remains an obligation. Inspect full detail, preserve its
  worker/workspace/PR, rule from existing authority when possible, and use the
  supported same-task recovery control. Never duplicate or discard a lane to
  quiet the board.
- Release gates are explicit and exact-SHA. A candidate, green CI, merge, or
  closeout never implies tag or publication authority.
- The requested successor configuration is the configured Paseo profile
  `Deepseek v4 Flash Worker`: provider `pi`, model
  `deepseek/deepseek-flash` (catalogue label `DeepSeek V4.1 Flash`), high
  thinking. This model choice changes no planning authority.

## Suggested Next Move

After ownership transfer, reload canonical `main` and run a concise generation
reassessment rather than dispatching a stale planned item. Present Tom with the
smallest meaningful choices: resolve the Qoder turn-binding and Goose typed-auth
stops; compile the already-planned Claude SDK and Command Code live gates if
their provider-operation authority is still intended; or close/refocus g05
around a new operator-selected goal. Inspect the open triage set only as needed
to support that choice.

## Completion Protocol

After receiving `Ownership transfer complete`, become the sole active
Swallowtail Chatterbox in workspace `wks_f4a6ebd6fbb93734`. Continue from this
handoff and current canonical docs, not from stale chat recollection. Promote
confirmed planning in coherent docs-only commits, dispatch only explicitly
authorized ready handoffs through Northstar Queue, and inspect full task detail
before any blocker ruling or recovery control.

The source remains visible as history and must not compete. A later refresh
repeats this exact sequence: push one seven-section continuation handoff,
preflight Queue attention, create one same-workspace successor with the exact
requested settings, transfer the unchanged preflight set, verify it, then send
`Ownership transfer complete`.
