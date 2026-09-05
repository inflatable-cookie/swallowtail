---
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
---

# Card 081 — Claude SDK Bash under mediation

## Dispatch

- Planning base: `0dea0cd9e8c8209df845a6b568298f74102519f9` (`origin/main`)
- Card: `docs/roadmaps/g05/batch-cards/081-claude-sdk-bash-under-mediation.md`
- Manifest: `docs/roadmaps/g05/029-claude-sdk-interactive-parity.md`, section `Card 081 Manifest`
- Worker branch: `worker/g05-card081-claude-sdk-bash`
- Dedicated workspace: `g05-card081-claude-sdk-bash`
- Coordinator: `af21d886-4053-4156-ae6a-e878dfb99985`

This is an implementation worker-pr-loop. Keep the worker and its exact
workspace alive through independent review, merge, and closeout. The reviewer
must run in this same workspace, be independently routed, and receive the
exact PR head.

## Owned paths

- `crates/swallowtail-adapter-claude-agent/src/sdk/**`
- `crates/swallowtail-adapter-claude-agent/sidecar/**`
- `crates/swallowtail-adapter-claude-agent/tests/**`
- `crates/swallowtail-adapter-claude-agent/README.md` or the named SDK guide
- Claude SDK cells in the route/feature matrices
- `CHANGELOG.md` `[Unreleased]` entries for this card
- `release-baselines/public-api-0.4.1/swallowtail-adapter-claude-agent.txt` (additive only)
- Card 081 `## Result`
- append-only `PAPERCUTS.md` evidence if required by the card

## Forbidden paths and surfaces

- Every other crate and package
- `claude_code_*` and ACP modules
- Contracts and shared runtime/core/testkit surfaces
- SDK version pins
- BashOutput, KillShell, NotebookEdit, Task, WebFetch, WebSearch, resume,
  model-change, MCP, or other later-card surfaces
- Credentials, live Claude/provider calls, or consumer mutations

## Required implementation oracle

Implement the Claude SDK Bash permission-policy lane exactly as the manifest
states. Bash is available only under a read-write lease and only after a host
allow on every call under every permission mode. `acceptEdits` may auto-approve
edits but never shell. Add a bounded, truncation-flagged command view to the
callback record for Bash only; retain the full command in the sidecar and
return it unchanged on allow. The never-available tools remain unavailable.
Add provider-free fake-SDK proofs, including truncation and denial paths.

Preserve the default profile and existing permission semantics. Keep provider-
neutral vocabulary and dependency direction intact. Do not broaden scope to
solve later cards.

## Validation and closeout

Run the card-named focused validation, affected-package verification, fake-SDK
proofs, formatting, package API baseline check, route/guides/docs/Northstar QA,
and `git diff --check`. Regenerate only the owned adapter baseline, additive
only. Report exact test counts and any pinned 1.95.0 evidence required by the
manifest. Push one reviewable PR; do not merge it from the worker.

On the PR head, the coordinator will create an independent cross-model review
agent in this same workspace. Merge only after acceptance and required checks
are green. If pinned MSRV hits the known Pi race while Card 094's remainder is
unmerged, rerun that job once at the same SHA; do not weaken the implementation
or retag anything. Then perform reserved closeout and notify Chatterbox.

No tags or publications are authorized by this handoff.
