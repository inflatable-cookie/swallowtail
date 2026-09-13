---
title: Swallowtail v0.5.1 annotated source tag
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom explicitly authorized local annotated-tag creation and tag push for v0.5.1 at exact SHA e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27 on 2026-09-13. Tom approved the exact annotation text quoted in this handoff. No GitHub Release, registry, artifact, provider, or consumer authority was granted."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
  completionNotificationAgentIds:
    - 5317069e-201f-4dea-94f3-af8f0b9faff2
tags: [coordination, handoff, worker, release, patch, desktop]
---

## What This Thread Was Doing

Execute the operator-authorized source-only `v0.5.1` annotated tag at exact
candidate merge `e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27`, then publish the
normal tag record and documentation closeout under g05.055.

## Why It Matters

Desktop g02.089 Phase A qualified the exact source carried by this candidate
and is held for an ordinary released-pin proof. The immutable tag unblocks
Phase B without replacing the qualified package tree.

## Current State

- Reviewed candidate head: `755a2669185f4937dbccde95effc5d82dfaa0324`.
- Tag target: merge `e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27`.
- Candidate/merge tree: `d375b3227985e8e552ba9346d8f9b8631936db5f`.
- Canonical branch and remote: `main` at
  `git@github.com:inflatable-cookie/swallowtail.git`.
- Independent review `5649975685` accepted the identical candidate tree.
- Exact-SHA push CI run `34731113171` succeeded with all 11 jobs green.
- Qualified `crates/` tree is `186f3ba42a4aa896c03a3bddcacb007b1afbd8cc`;
  Grok subtree is `1a6f777f84f9aef80c146badf6944811568e23cd`;
  both match Desktop-qualified source `0209dd7f`.
- Workspace version is `0.5.1`; every package has `publish = false`.
- Local and remote `v0.5.1` were absent at planning promotion.
- Current `main` includes closeout `c712b902` and is not the tag target.

## Boundaries

Authorization covers exactly two external mutations: create one local
annotated tag `v0.5.1` at the named candidate merge, then push only that exact
tag ref to `origin`. Never move, replace, delete, recreate, or force-push it.
Do not run release prepare/execute, edit source or workflows, create a GitHub
Release, publish to crates.io, upload artifacts, contact providers, or mutate
Desktop or another consumer.

The exact approved annotation is:

```text
Swallowtail v0.5.1

OpenCode HTTP 1.18.30 qualification and Grok Build 1.0.25 authenticated model catalogue. Canonical hosted CI run 34731113171 passed against e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27.

The source-only release contains no crates.io publication, GitHub Release object, binary, sidecar, installer, or model artifact.
```

## Important Context

Follow `docs/roadmaps/g05/055-v0-5-1-annotated-source-tag.md`. The earlier
prepare transaction is spent; do not rerun it. Perform one exact read-only
preflight immediately before mutation. After push, verify tag object, peel,
tree, message, remote agreement, qualified package trees, package metadata,
and tag-triggered CI. A post-tag failure keeps the tag immutable and enters
next-patch recovery. Registry absence is structural unless an authoritative
registry query succeeds; do not turn a blocked query into a stronger claim.

## Suggested Next Move

Perform the full preflight once. If every invariant holds, create the tag at
the explicit SHA, re-read it locally, push only its exact ref, then verify the
remote and tag-triggered CI before writing the documentation closeout.

## Completion Protocol

Record tag object SHA, peeled candidate/tree, exact annotation, local/remote
agreement, qualifying pre-tag run `34731113171`, tag-triggered CI and all job
results, unchanged qualified package trees, version and `publish = false`, and
registry/GitHub Release/artifact evidence. Update Contract 036, the `0.5.1`
release note/index, g05.055, roadmap front doors, and one tag log through a
documentation PR. Run `effigy qa:docs`, `effigy qa:northstar`, and
`git diff --check`; obtain independent exact-head review and merge through the
queue. Return the immutable release capsule directly to Desktop Chatterbox
agent `5317069e-201f-4dea-94f3-af8f0b9faff2`. No consumer, provider, GitHub
Release, registry, binary, or other publication work follows automatically.
