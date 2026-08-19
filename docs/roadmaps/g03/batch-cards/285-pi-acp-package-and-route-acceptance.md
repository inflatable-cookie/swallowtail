# 285 Pi ACP Package And Route Acceptance

Status: superseded
Owner: Tom
Created: 2026-08-18
Milestone: `../092-pi-acp-route.md`
Depends on: Card 284; Contracts 036-037, 044-045, 051-052
Note: superseded by card 282; official Pi has no native ACP wire.

## Goal

Complete production-facing documentation and acceptance for `pi.acp`, or record a negative/deferred disposition.

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

`effigy validate:focused swallowtail-adapter-pi`; `effigy package:verify-affected swallowtail-adapter-pi`; `effigy check:examples`; `effigy qa:docs`.

## Stop Conditions

Stop if package, guide, example, matrix, or cleanup truth diverges; close as deferred rather than inventing a route claim.

## Auto-Continuation

After closeout, return to roadmap 086 and recompute the next primary candidate.

## Evidence

Cards 282-284; Research 143; Contract 052
