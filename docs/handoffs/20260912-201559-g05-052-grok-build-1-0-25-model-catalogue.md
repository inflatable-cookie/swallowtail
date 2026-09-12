---
title: g05.052 Grok Build 1.0.25 model catalogue
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom supplied an operator-confirmed Desktop intake on 2026-09-12 and asked Swallowtail to inspect, plan and promote the bounded Grok catalogue producer seam, with provider calls, releases, tags and Desktop mutation explicitly excluded."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
  completionNotificationAgentIds:
    - 5317069e-201f-4dea-94f3-af8f0b9faff2
tags: [coordination, handoff, worker, grok, catalogue, desktop]
---

## What This Thread Was Doing

Promote the operator-approved Swallowtail producer half of Desktop g02.086:
an exact installed Grok `1.0.25` pre-session model catalogue, or a typed ruling
that no admissible non-prompt seam exists.

## Why It Matters

Desktop currently sees only `grok_build_model_for_version`, which is an
execution compatibility choice rather than the installed CLI's truthful model
inventory. Hard-coding `grok-4.5` downstream or reusing ACP session options
would create false catalogue authority.

## Current State

- Canonical task: `docs/roadmaps/g05/052-grok-build-1-0-25-model-catalogue.md`.
- Planning base: canonical pushed `main` at the commit containing this handoff.
- Installed `/Users/tom/.grok/bin/grok` reports
  `grok 1.0.25 (f7e67d6988e2) [stable]`, SHA-256
  `9ef4a40ad60c6a5178a65caf39c2a148e6a98d0d2d350b10329dee34d9195d9c`.
- `grok --help` exposes `models — List available models and exit`; its help has
  no prompt argument. Static binary strings include a structured
  `default_models.json` document ordered `grok-4.6`, then `grok-4.5`, with
  top-level default `grok-4.6` and optional source metadata.
- No `grok models` command ran during planning. No credential was read and no
  provider contact occurred.
- The common `ModelCatalogEntry`/`ModelMetadata` vocabulary already preserves
  exact ID, optional provider identity/display metadata, default, reasoning,
  token limits and source observations.
- Grok currently has only ACP prepared operations. Its catalogue matrix cell is
  unavailable under the released `1.0.4..=1.0.5` evidence.

## Boundaries

Follow g05.052 exactly. Static artifact and fake-process work only: do not run
`grok models`, contact xAI, read or mutate credentials/config, send a prompt,
open ACP, start a model session, install/update Grok, change Desktop, repin a
consumer, create a tag/release or publish anything. Keep ACP execution claims,
`grok_build_model_for_version` and released `v0.5.0` immutable. Do not use
historical session-negotiated options as catalogue evidence.

## Important Context

Contracts 020 and 047 are the authority. First reserve Research 305 and freeze
the exact static command/output link before changing a production claim. The
embedded default-model document is evidence input, not permission to assume
the command's wire grammar. If the exact `1.0.25` artifact cannot prove that
`grok models` returns a bounded non-prompt document, write the typed
non-admission ruling, leave the feature unavailable and stop.

For an admitted seam, model it as a separate prepared `ModelCatalog` operation
with its own exact `1.0.25` claim. Require delegated Grok subscription access
readiness without exposing credential material. Execute only `models`, close
stdin unwritten, bound output, enforce deadline/cancellation, join cleanup and
never retry. Preserve array order, exact opaque IDs and the top-level default.
Map display name, description, provider identity, context window and reasoning
data only where the source field and common meaning align; omission remains
`None`. Unknown valid IDs must pass through.

Fixture destination is
`crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.25-model-catalogue/`;
record the exact files actually accepted. Expected focused gates are
`effigy validate:focused swallowtail-adapter-grok` and
`effigy package:verify-affected swallowtail-adapter-grok`, plus the remaining
g05.052 validation list.

## Suggested Next Move

Confirm Research 305 and fixture paths are still free, statically trace the
`models` subcommand to its exact output serializer, and commit that identity
evidence before implementing the prepared claim/driver.

## Completion Protocol

Meet every g05.052 oracle and obtain independent exact-head review. Return the
exact Swallowtail source head, PR/review/merge/closeout SHAs, Research 305,
Contract 020 and 047 paths, exact fixture paths, focused selector counts, and
the typed catalogue verdict. State explicitly that Desktop may consume only
the exact accepted merged source SHA under separate pin authority, or wait for
a later separately authorized source tag; `v0.5.0` does not contain the seam.
The queue must deliver this capsule directly to Desktop Chatterbox agent
`5317069e-201f-4dea-94f3-af8f0b9faff2`. No Desktop mutation, provider call,
release or tag follows automatically.
