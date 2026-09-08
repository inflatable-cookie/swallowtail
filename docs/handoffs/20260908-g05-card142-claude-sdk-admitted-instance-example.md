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
queue_approval: "Tom authorized the switch to northstar-queue dispatch and retirement of the Swallowtail coordinator in the Chatterbox conversation on 2026-09-08, with standing direction to keep promoted ready cards moving."
tags: [coordination, handoff, worker, claude-agent-sdk, examples, admission]
---

## What This Thread Was Doing

Implementing g05 card 142: ship an executable `claude-agent.sdk`
example that composes an `AdmittedInstanceRecord`, lifts it with
`ClaudeAgentSdkSessionPreparation::from_admitted`, and continues through the
card 125 registered-tool binding, mediated stdio proxy attachment, and proxy
recipe.

## Why It Matters

Bovine Desktop needs exactly this composition to build the runner for
the card 132 Claude registered-tool live gate, which is one of the two
consumer gates blocking the release scope. Swallowtail ships no example of it,
and Desktop has been told not to work around the gap.

## Current State

`examples/prepared_claude_agent_sdk.rs` documents the inputs in prose
and then accepts an already-constructed preparation; it never builds the
admitted record and never calls `from_admitted`. The only example that
constructs admission is `examples/connection_lifecycle.rs`, which is
`claude-agent.acp`, a different route with different inputs. The guide's
Explicit Inputs section names `from_admitted` correctly, so the gap is the
executable example rather than the prose. Card 125 merged the registered-tool
route binding at `1cbc21ad`. No work has started.

## Boundaries

Owned and forbidden paths are the card 142 manifest row and they
bind. Provider-free only: the example demonstrates composition and types as
the existing examples do, and spends no live route. Do not change the
admission API, the ACP example, or any claim, matrix cell, or contract.

## Important Context

The composition is: an interpreted-script launch recipe binding the
approved Node runtime and the source-tagged sidecar entry; an environment
reference carrying `CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE`, `_NATIVE_BINARY`,
and `_MANIFEST`; a delegated subscription credential reference; a model route;
and the open deadline. Swallowtail never installs, vendors, or updates the
Node runtime, the SDK package, or the native binary; the consumer provisions
them and passes opaque host-owned references. Cross-reference the finished
example from the admission section of
`docs/guides/claude-agent-sdk-prepared-integration.md` and from
`docs/handoffs/20260907-g05-card132-desktop-claude-sdk-registered-tool-live-gate.md`,
replacing the incorrect pointer to `prepared_claude_agent_sdk.rs`.

## Suggested Next Move

Write the example, then continue it through the registered-tool
preparation, attachment, and proxy recipe to open, so it covers the exact
composition the card 132 packet requires. Named validation:
`cargo fmt -p swallowtail-adapter-claude-agent -- --check`;
`effigy validate:focused swallowtail-adapter-claude-agent`;
`effigy package:verify-affected swallowtail-adapter-claude-agent`;
`effigy qa:northstar`; `git diff --check`. If the composition cannot be
expressed without changing the admission API, stop and escalate to Chatterbox
rather than changing that API in an example card.

## Completion Protocol

Stop at exact-head review. Do not merge, tag, release, or touch any consumer
repository. Report the exact head SHA, the named validation output, and
anything found but deliberately not changed. The card at
`docs/roadmaps/g05/batch-cards/142-claude-sdk-admitted-instance-worked-example.md` and its manifest row bind: owned paths,
forbidden paths, acceptance criteria, and stop conditions are as written
there.
