---
title: Swallowtail v0.5.0 annotated source tag
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: merged
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom explicitly authorized local annotated-tag creation and tag push for v0.5.0 at exact SHA 582d01d6b6890eed5195a1fcbee0ae985304a6c7 on 2026-09-10. Tom separately approved the exact annotation text quoted in this handoff. No GitHub Release, registry, artifact, provider, or consumer authority was granted."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Execute the operator-authorized source-only `v0.5.0` annotated tag at exact
candidate SHA `582d01d6b6890eed5195a1fcbee0ae985304a6c7`, then publish the normal
tag record and documentation closeout under g05.049.

## Why It Matters

The repaired candidate passed every source and hosted gate. Consumers need an
immutable exact tag before the registered-only Claude SDK route can unblock
Desktop g02.051 without selecting a moving branch.

## Current State

- Candidate commit: `582d01d6b6890eed5195a1fcbee0ae985304a6c7`.
- Candidate tree: `eebb6978be365f79ae41436912240886d0aecf78`.
- Canonical branch and remote: `main` at
  `git@github.com:inflatable-cookie/swallowtail.git`.
- Independent review `5616849978` accepted the identical tree.
- Exact-SHA push CI run 34464717829 succeeded with all 11 jobs green,
  including both pinned-MSRV jobs.
- Workspace version is `0.5.0`; `publish = false`; MSRV is `1.95`.
- Local and remote `v0.5.0` were absent at planning promotion.
- Current `main` includes later documentation closeout and is not the tag
  target.

## Boundaries

Authorization covers exactly two external mutations: create one local
annotated tag `v0.5.0` at the named candidate SHA, then push only that exact
tag ref to `origin`. Never move, replace, delete, recreate, or force-push it.
Do not run release prepare/execute, edit source or workflows, create a GitHub
Release, publish to crates.io, upload artifacts, contact providers, or mutate
Desktop or another consumer.

The exact approved annotation is:

```text
Swallowtail v0.5.0

Registered-only Claude SDK sessions, corrected native mediation truth, and Pi RPC 0.85.1 qualification. Canonical hosted CI run 34464717829 passed against 582d01d6b6890eed5195a1fcbee0ae985304a6c7.

The source-only release contains no crates.io publication, GitHub Release object, binary, sidecar, installer, or model artifact.
```

## Important Context

Follow `docs/roadmaps/g05/049-v0-5-0-annotated-source-tag.md`. The earlier
prepare transaction is spent; do not fabricate or rerun it. Perform one exact
read-only preflight immediately before mutation. After push, verify tag object,
peel, tree, message, and remote agreement, then wait for tag-triggered CI. A
post-tag failure keeps the tag immutable and enters next-patch recovery.

## Suggested Next Move

Perform the full preflight once. If every invariant holds, create the tag at
the explicit SHA, re-read it locally, push only its exact ref, then verify the
remote and tag-triggered CI before writing the documentation closeout.

## Completion Protocol

Record tag object SHA, peeled candidate/tree, exact annotation, local/remote
agreement, qualifying pre-tag run 34464717829, tag-triggered CI and all job
results. Update Contract 036, the `0.5.0` release note/index, g05.049, roadmap
front doors, and one tag log through a documentation PR. Run `effigy qa:docs`,
`effigy qa:northstar`, and `git diff --check`; obtain independent exact-head
review and merge through the queue. No consumer, provider, GitHub Release,
registry, binary, or publication work follows automatically.
