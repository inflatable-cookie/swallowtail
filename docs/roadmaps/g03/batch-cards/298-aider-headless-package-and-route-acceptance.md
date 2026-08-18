# 298 Aider Headless Package And Route Acceptance

Status: planned
Owner: Tom
Created: 2026-08-18
Milestone: `../095-aider-headless-route.md`
Depends on: Card 297; Contracts 036-037, 044-045, 051-052

## Goal

Complete package and documentation for `aider.headless`, or close the candidate as deferred or negative evidence.

## Scope

Add package metadata, route descriptor, guide and compiling example, route/feature matrix rows, README package map, architecture notes, release baseline handling, and separately gated live evidence where justified. Keep remote, Git, API-key, and retention limitations visible.

## Out Of Scope

release publication, registry publication, consumer edits, and broad live qualification

## Acceptance Criteria

- [ ] Focused and affected-package validation pass.
- [ ] `effigy check:examples` and `effigy qa:docs` pass.
- [ ] Route, feature, guide, package, and architecture indexes agree.
- [ ] Deferred/negative decisions do not enter the production route matrix.

## Validation

`effigy validate:focused swallowtail-adapter-aider`; `effigy package:verify-affected swallowtail-adapter-aider`; `effigy check:examples`; `effigy qa:docs`.

## Stop Conditions

Stop and record disposition if authority, cleanup, or route identity is not honest; do not widen scope to rescue the candidate.

## Auto-Continuation

Return to roadmap 087 and recompute the remaining secondary wave.

## Evidence

Cards 295-297; Research 143; Contract 052
