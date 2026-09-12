---
title: g05.053 Grok Build 1.0.25 catalogue correction
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom rejected the unnecessary provider-suppression gate and confirmed the authenticated non-inference metadata boundary on 2026-09-12; land the retained catalogue without another Grok run."
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
model catalogue through an authenticated, non-inference metadata operation.

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
- The replacement observation exited zero with 113 stdout bytes and zero
  stderr bytes. Exact binary format pieces prove the shipped output uses an
  authentication preamble and `*`/`-` bullet rows. The old bare-row parser
  rejected it before the private-home runtime proof was retained.
- The retained dirty revision fixes the bullet grammar and writes a redacted
  capsule with counts, digests, and runtime proof before any assertion.
- Focused provider-free validation is green: format, clippy including
  `live-probes`, and every `swallowtail-adapter-grok` test.
- The final persisted capsule parses `grok-4.6` as default followed by
  `grok-4.5`, exited zero, joined cleanup, opened no model session, and ran with
  zero account credit. Its auth-refresh watcher is permitted metadata-path
  activity, not inference.

## Boundaries

Follow g05.053 exactly. Do not reuse ambient ACP environment state, edit host,
fleet, requirements, MDM, or user configuration, expose credentials, send a
prompt, open a model session, perform inference, run any unbounded observation,
change Grok ACP execution compatibility, mutate Desktop, edit tagged/historical
release evidence, create a tag/release, publish, or touch workflows.

## Important Context

Recover the retained implementation and simplify it before claiming support.
Use exact argv `--no-auto-update models`. Keep the bounded parser, redacted
output evidence, exact executable binding, no-update flag, and joined process
lifecycle. Remove the private suppression file, enterprise-precedence checks,
`ProviderSuppressed` plan binding, and suppression-only public API/tests. This
operation may perform bounded authentication or catalogue metadata traffic;
it must not send a prompt, open a model session, invoke inference or tools,
update, retry, or retain raw account data. Bind
`HarnessConfigurationPosture::Ambient`; that accepts the harness metadata
configuration without granting inference authority.

The live listing remains authoritative for membership, order, and default.
The frozen exact-`1.0.25` embedded document may only supplement matching IDs.
Keep the existing parser bounds and lifecycle work and restore current-source
route 50 without changing tagged `v0.5.0` or Research 281. Run every revised
provider-free gate first. Do not run Grok again. The existing final capsule is
accepted: it records successful bullet-grammar parsing, ordered ids/default,
zero stderr, joined cleanup, and no model session or inference. The observed
auth-refresh watcher is allowed metadata-path behavior. Zero-credit success is
evidence about this observation only, not a general entitlement or billing
claim.

Expected fixture root is
`crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.25-model-catalogue/`.
Research 306 records the correction. Research 305 and its ruling log receive
a narrow correction note rather than rewritten history.

## Suggested Next Move

Simplify the retained parser/capsule revision to the corrected metadata
boundary, run the complete provider-free gate, then open the PR without any
further Grok observation.

## Completion Protocol

Meet every g05.053 oracle and obtain independent exact-head review. Return the
exact Swallowtail source head, PR/review/merge/closeout SHAs, Research 305/306,
Contracts 020/047, exact fixture and capsule paths/digests, focused selector
counts, ordered model/default result, and the release or exact-source adoption
boundary. Queue delivery goes directly to Desktop Chatterbox agent
`5317069e-201f-4dea-94f3-af8f0b9faff2`. No Desktop mutation, release, tag, or
publication follows automatically.
