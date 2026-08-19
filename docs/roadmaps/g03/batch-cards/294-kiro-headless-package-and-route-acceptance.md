# 294 Kiro Headless Package And Route Acceptance

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../094-kiro-headless-route.md`
Depends on: Card 293; Contracts 036-037, 044-045, 051-052

## Goal

Complete package and documentation for `kiro.acp`, or close the candidate as deferred or negative evidence.

## Scope

Add package metadata, route descriptor, guide and compiling example, route/feature matrix rows, README package map, architecture notes, release baseline handling, and separately gated live evidence where justified. Keep remote, Git, API-key, and retention limitations visible.

## Out Of Scope

release publication, registry publication, consumer edits, and broad live qualification

## Acceptance Criteria

- [x] Focused and affected-package validation pass.
- [x] `effigy check:examples` and `effigy qa:docs` pass.
- [x] Route, feature, guide, package, and architecture indexes agree.
- [x] Deferred/negative decisions do not enter the production route matrix.

## Validation

`effigy validate:focused swallowtail-adapter-kiro swallowtail-testkit`; `effigy package:verify-affected swallowtail-adapter-kiro`; `effigy check:examples`; `effigy qa:docs`; `effigy qa:routes`; `effigy qa:guides`.

## Stop Conditions

Stop and record disposition if authority, cleanup, or route identity is not honest; do not widen scope to rescue the candidate.

## Auto-Continuation

Return to roadmap 087 and recompute the remaining secondary wave.

## Evidence

Cards 291-293; Research 156; Contract 052;
`docs/logs/2026-08-19-kiro-acp-package-and-route-acceptance.md`.
Accepted `kiro.acp` as an unreleased additive production route. Current source
is 39 packages and 46 routes. Immutable `v0.3.2` stays 30 packages and 36
routes. Live install, login, and prompt were not justified.
