# 290 OpenHands Agent Server Package And Route Acceptance

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../093-openhands-agent-server-route.md`
Depends on: Card 289; Contracts 036-037, 044-045, 051-052

## Goal

Complete package and documentation for `openhands.agent-server`, or close the candidate as deferred or negative evidence.

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

`effigy validate:focused swallowtail-adapter-openhands`; `effigy package:verify-affected swallowtail-adapter-openhands`; `effigy check:examples`; `effigy qa:docs`.

## Stop Conditions

Stop and record disposition if authority, cleanup, or route identity is not honest; do not widen scope to rescue the candidate.

## Auto-Continuation

Continue to card 291 Kiro ACP identity after this closeout.

## Evidence

Cards 287-289; Research 155; `docs/logs/2026-08-19-openhands-agent-server-package-and-route-acceptance.md`.
Deferred: live HTTP/WebSocket conversation stays unwired. Package kept.
No production route, guide, example, or matrix row.
