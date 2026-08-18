# 277 Mistral Vibe Headless Package And Route Acceptance

Status: planned
Owner: Tom
Created: 2026-08-18
Milestone: `../090-mistral-vibe-headless-route.md`
Depends on: Card 276; Contracts 036-037, 044-045, 051-052

## Goal

Complete production-facing documentation and acceptance for `mistral-vibe.headless`, or record a negative/deferred disposition.

## Scope

Add package metadata, route descriptors, guide and compiling normal-path example, route and feature matrix rows, README/package index updates, release-baseline handling, and any separately gated installed/live probe. Keep version posture exact and do not promote newer behavior without an evidence segment.

## Out Of Scope

release publication, registry publication, consumer edits, and unbounded live qualification

## Acceptance Criteria

- [ ] Focused and affected-package validation pass.
- [ ] `effigy check:examples` and `effigy qa:docs` pass.
- [ ] Route matrix, feature matrix, guide map, README, architecture, and package contract agree.
- [ ] Live evidence is opt-in and cannot widen the deterministic claim silently.
- [ ] Closeout records accepted, deferred, or rejected route truth.

## Validation

`effigy validate:focused swallowtail-adapter-mistral-vibe`; `effigy package:verify-affected swallowtail-adapter-mistral-vibe`; `effigy check:examples`; `effigy qa:docs`.

## Stop Conditions

Stop if package, guide, example, matrix, or cleanup truth diverges; close as deferred rather than inventing a route claim.

## Auto-Continuation

After closeout, return to roadmap 086 and recompute the next primary candidate.

## Evidence

Cards 274-276; Research 143; Contract 052
