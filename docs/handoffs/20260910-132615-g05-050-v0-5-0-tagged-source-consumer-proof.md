---
title: g05.050 v0.5.0 tagged-source consumer proof
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom said Continue on 2026-09-10 after Chatterbox proposed first proving consumption from immutable v0.5.0, then resuming the preserved Desktop g02.051 task. This task is the provider-free first step only."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g05.050: prove the canonical remote `v0.5.0` tag is consumable as one
exact source and exposes the registered-only Claude SDK profile required by
Desktop g02.051.

## Why It Matters

The candidate source-consumer gate passed before tagging, but Desktop needs an
immutable tag rather than a candidate SHA or moving branch. The preserved
Desktop task is blocked on `v0.4.4`, which cannot express zero native tools
with registered-tool-driven `ReadWrite` access.

## Current State

Annotated tag `v0.5.0` is object `c772c5839806b6cbf1c9d3b495049b362e4c0c52`,
peeling to `582d01d6b6890eed5195a1fcbee0ae985304a6c7`, tree
`eebb6978be365f79ae41436912240886d0aecf78`. Tag-triggered run 34467974791 is
green 11/11. Desktop task `1e7b9baf-9b97-4e49-a6d4-17aa24bbcb5c` remains
blocked before edits or provider contact and must not be replaced.

## Boundaries

Follow `docs/roadmaps/g05/050-v0-5-0-tagged-source-consumer-proof.md`. Own only
the provider-free tag consumption proof, one evidence log, and closeout. Do not
edit code, tests, manifests, scripts, baselines, contracts, release notes,
tags, workflows, Desktop, or another consumer. Do not contact a provider,
inspect credentials, open a session, publish, or repin anything.

## Important Context

Run the existing source-consumer selector from a clean detached checkout of
the tag. Separately build a temporary external Cargo consumer using the
canonical HTTPS Git URL and only `tag = "v0.5.0"` for each direct Swallowtail
dependency. Its code must type-check the public registered-only Claude SDK
binding with zero native tools and explicit `ReadWrite`. Metadata must prove
every selected Swallowtail package resolves to the exact tag peel with no path,
branch, revision, patch, or mixed-source leak.

## Suggested Next Move

Freeze and verify the tag capsule, then create the remote-tag consumer in a
fresh temporary directory. Run the repository selector and MSRV compile,
capture one complete transcript and digest, inspect metadata, clean up, and
publish the documentation-only result.

## Completion Protocol

Open one documentation PR, run `effigy qa:docs`, `effigy qa:northstar`, and
`git diff --check`, obtain independent exact-head review, merge, and sync
canonical `main`. Return tag object/peel/tree, both consumer proofs, every
package source identity, transcript SHA-256, cleanup, PR/head/review/merge,
zero provider contact, unchanged tag, and no consumer mutation. Chatterbox
will use that capsule for the separate Desktop amendment and same-task resume.
