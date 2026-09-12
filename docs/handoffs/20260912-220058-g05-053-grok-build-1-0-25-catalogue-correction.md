---
title: g05.053 Grok Build 1.0.25 catalogue correction
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom rejected g05.052's non-admission as wrong and explicitly instructed Swallowtail Chatterbox to fix the Grok catalogue seam on 2026-09-12."
queue:
  capability: complex
  skipPRReview: false
  notifyOriginOnCloseout: true
  completionNotificationAgentIds:
    - 5317069e-201f-4dea-94f3-af8f0b9faff2
tags: [coordination, handoff, worker, grok, catalogue, correction, desktop]
---

## What This Thread Was Doing

Correct g05.052 and deliver the exact installed Grok `1.0.25` pre-session
model catalogue through a genuinely provider-suppressed prepared operation.

## Why It Matters

PR #315's final non-admission missed Grok's generic `GROK_CONFIG` overlay and
treated the absence of a dedicated `GROK_REMOTE_FETCH` variable as absence of
bindable suppression. Desktop needs the installed catalogue rather than the
single ACP compatibility model or a hard-coded downstream list.

## Current State

- Canonical task: `docs/roadmaps/g05/053-grok-build-1-0-25-catalogue-correction.md`.
- g05.052 completed through PR #315 at merge `5d0c49f1`; its admitted
  implementation remains recoverable from preserved head
  `06ba3176a1f84437a9a7abcb0a174ca89308c88e`.
- Exact installed CLI is `grok 1.0.25 (f7e67d6988e2) [stable]`, executable
  SHA-256 `9ef4a40ad60c6a5178a65caf39c2a148e6a98d0d2d350b10329dee34d9195d9c`.
- Shipped exact-version docs state `GROK_CONFIG` is an allowlisted inline JSON
  overlay for `features`; `remote_fetch` and `managed_config` are boolean
  feature keys. Requirements/MDM remain higher priority.
- Host-local launches with `env_clear()` before applying the exact approved
  bootstrap and environment references.
- No catalogue command or provider call was run during corrective planning.

## Boundaries

Follow g05.053 exactly. Do not reuse ambient ACP environment state, edit host,
fleet, requirements, MDM, or user configuration, expose credentials, send a
prompt, open a model session, perform inference, run any unbounded observation,
change Grok ACP execution compatibility, mutate Desktop, edit tagged/historical
release evidence, create a tag/release, publish, or touch workflows.

## Important Context

Recover the admitted PR #315 implementation, then correct it before claiming
support. The first live observation disproved the overlay route: it exited zero
but started Grok's auth-refresh watcher and wrote a freshly fetched
remote-origin `models_cache.json`; raw command output was not retained, so
parser grammar was also unproved. Preserve that as failed evidence.

The catalogue needs a distinct exact environment reference containing an
authorized isolated `GROK_HOME`, no ambient `GROK_CONFIG` or
`GROK_CONFIG_PATH`, and a generated operation-private `$GROK_HOME/config.toml`
containing only:

```toml
[features]
remote_fetch = false
managed_config = false
```

This is temporary operation materialization, not a host configuration edit.
Use exact argv `--no-auto-update models`. Represent both the instance and
operation requirements as `HarnessConfigurationPosture::ProviderSuppressed`.
Require positive host evidence that no higher-priority requirements/MDM pin
defeats suppression; reject before process start otherwise. Exact `1.0.25`
documents that overlay-reachable `features` still include security gates that
read raw disk layers and that `remote_fetch` has fleet precedence. Do not use
the overlay for this gate.

The live listing remains authoritative for membership, order, and default.
The frozen exact-`1.0.25` embedded document may only supplement matching IDs.
Keep the existing parser bounds and lifecycle work, add mutation-sensitive
environment/pre-spawn regressions, and restore current-source route 50 without
changing tagged `v0.5.0` or Research 281. Run every revised provider-free gate
first. Then run exactly one replacement authenticated, provider-suppressed
catalogue listing. Capture bounded raw stdout and stderr before parsing. Admit
only if the exact-version log proves remote fetch disabled and the isolated
home has neither a fresh remote-origin cache write nor an auth-refresh watcher.
No third observation or further retry is authorized. Tom's 2026-09-12
instruction to fix the seam after rejecting non-admission authorizes this
materially corrected replacement.

Expected fixture root is
`crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.25-model-catalogue/`.
Research 306 records the correction. Research 305 and its ruling log receive
a narrow correction note rather than rewritten history.

## Suggested Next Move

Freeze the failed overlay and private-file precedence evidence in Research 306,
recover the preserved admitted tree, then make the pre-process and runtime
suppression proofs fail closed before restoring
the route.

## Completion Protocol

Meet every g05.053 oracle and obtain independent exact-head review. Return the
exact Swallowtail source head, PR/review/merge/closeout SHAs, Research 305/306,
Contracts 020/047, exact fixture and capsule paths/digests, focused selector
counts, ordered model/default result, and the release or exact-source adoption
boundary. Queue delivery goes directly to Desktop Chatterbox agent
`5317069e-201f-4dea-94f3-af8f0b9faff2`. No Desktop mutation, release, tag, or
publication follows automatically.
