---
title: g05 Card 142 Claude SDK admitted-instance worked example worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-08
updated: 2026-09-08
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom authorized the switch to northstar-queue dispatch and retirement of the Swallowtail coordinator thread in the Chatterbox conversation on 2026-09-08, and has standing direction to keep promoted ready cards moving."
tags: [coordination, handoff, worker, claude-agent-sdk, examples, admission]
---

## Objective

Ship an executable `claude-agent.sdk` example that composes an
`AdmittedInstanceRecord` from opaque host-owned references, lifts it with
`ClaudeAgentSdkSessionPreparation::from_admitted`, and continues through the
card 125 registered-tool binding, mediated stdio proxy attachment, and proxy
recipe.

## Current State

Bovine Desktop needs exactly this composition to build the runner for the
card 132 live gate, and Swallowtail ships no example of it.
`examples/prepared_claude_agent_sdk.rs` documents the inputs in prose and then
accepts an already-constructed preparation; it never builds the admitted
record and never calls `from_admitted`. The only example that constructs
admission is `examples/connection_lifecycle.rs`, which is `claude-agent.acp`,
a different route with different inputs. The guide's Explicit Inputs section
names `from_admitted` correctly, so the gap is the executable example.
Desktop has been told not to work around it.

## Scope

Add the example under `crates/swallowtail-adapter-claude-agent/examples/`:
the interpreted-script launch recipe binding the approved Node runtime and the
source-tagged sidecar entry, the environment reference carrying
`CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE`, `_NATIVE_BINARY`, and `_MANIFEST`, the
delegated subscription credential reference, the model route, and the open
deadline; then `from_admitted`; then the registered-tool preparation,
attachment, and proxy recipe through to open. Cross-reference it from
`docs/guides/claude-agent-sdk-prepared-integration.md`'s admission section and
from `docs/handoffs/20260907-g05-card132-desktop-claude-sdk-registered-tool-live-gate.md`.

## Acceptance

An executable `claude-agent.sdk` example constructs `AdmittedInstanceRecord`
and calls `from_admitted`; it continues through the registered-tool binding,
attachment, and proxy recipe; the guide and the card 132 packet point at it;
it is provider-free and spends no live route.

## Stop Conditions

The composition cannot be expressed without changing the admission API: stop
and return to Chatterbox rather than changing that API in an example card.

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy qa:northstar`
- `git diff --check`

## Completion Protocol

Stop at exact-head review. Do not merge, tag, release, or touch any consumer
repository. Report the exact head SHA, the validation output, and anything you
found but did not change. The card is `docs/roadmaps/g05/batch-cards/142-claude-sdk-admitted-instance-worked-example.md`;
its manifest row carries the owned and forbidden paths and they bind.
