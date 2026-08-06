# 044 v0.1.1 Source Patch Release

Status: active
Owner: Tom
Created: 2026-08-06
Depends on: g03.043
Vision tags: source release, compatibility maintenance, deterministic CI
Contract refs: 001, 009, 022, 036, 049, 052

## Problem

The immutable `v0.1.0` tag predates the Anthropic cancellation repair now
proven on `main`. A patch release must carry that compatible fix without
moving the original tag. The first `v0.1.1` release simulation also exposed a
contention-only Kimi fixture race: local observer cleanup completes before the
separate fixture server necessarily records the peer close frame.

## Generation Runway

Advance g03's consumer-proven defect and compatibility-maintenance goals. Keep
the existing 27-package, 33-route source-only distribution boundary.

## Execution Plan

- [x] card 132: synchronize Kimi fixture evidence and prepare the complete
      `v0.1.1` source candidate
- [ ] card 133: push the exact release commit and require all GitHub CI lanes
      to pass from canonical clean source
- [ ] card 134: create and push one annotated immutable `v0.1.1` tag

## Goals

- [ ] ship the accepted Anthropic cancellation fix as a compatible patch
- [ ] keep deterministic detachment evidence stable under workspace contention
- [ ] bind version, changelog, release notes, package graph, CI, and tag to one
      exact source identity
- [ ] leave `v0.1.0` immutable

## Boundaries

- no crates.io publication, GitHub Release object, binary, sidecar, installer,
  model artifact, or consumer mutation
- no live or authenticated provider work
- no release-gate bypass or timeout inflation
- no tag before the exact release commit passes canonical GitHub CI
- no movement or recreation of any existing tag

## Acceptance Criteria

- [x] all 11 source-release gates pass locally
- [x] Rust 1.90 and Bedrock Rust 1.94.1 floors remain exact
- [x] the isolated Git-source consumer resolves the candidate
- [ ] the exact release commit passes all six GitHub CI jobs
- [ ] annotated `v0.1.1` resolves locally and remotely to that commit
- [ ] release notes describe upgrade, rollback, fixes, and unchanged limits

## Planning Checkpoint

Stop if the candidate changes public or guaranteed behavior beyond compatible
fixes, if dependency refresh requires an unplanned migration, or if any local
or remote gate fails. A failed published tag is never moved or recreated.
