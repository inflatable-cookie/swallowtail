# 265 Cline ACP Package And Route Acceptance

Status: planned
Owner: Tom
Created: 2026-08-18
Milestone: `../086-cline-acp-route.md`
Depends on: Card 264; Contracts 036-037, 044-045, 051-052

## Goal

Complete package and documentation acceptance for `cline.acp`, or record deferred/negative evidence.

## Scope

Add package metadata, ACP route descriptor, guide, compiling normal-path example, route/feature matrix rows, README/package index updates, currentness ceiling, and separately gated live evidence if justified.

## Out Of Scope

Cline headless documentation, release publication, registry publication, consumer edits, and unbounded live qualification

## Acceptance Criteria

- [ ] focused and affected-package validation pass
- [ ] `effigy check:examples` and `effigy qa:docs` pass
- [ ] route, feature, guide, package, and architecture indexes agree
- [ ] closeout records accepted, deferred, or rejected ACP truth

## Validation

`effigy validate:focused swallowtail-adapter-cline`; `effigy package:verify-affected swallowtail-adapter-cline`; `effigy check:examples`; `effigy qa:docs`.

## Stop Conditions

Stop and record disposition if package, guide, example, matrix, or ACP cleanup truth diverges.

## Auto-Continuation

Return to roadmap 086 and recompute the next route.

## Evidence

Cards 262-264; Research 143; Contract 052
