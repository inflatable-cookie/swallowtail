# 307 Cline Headless Package And Route Acceptance

Status: planned
Owner: Tom
Created: 2026-08-18
Milestone: `../087-cline-headless-route.md`
Depends on: Card 306; Contracts 036-037, 044-045, 051-052

## Goal

Complete package and documentation acceptance for `cline.headless`, or record deferred/negative evidence.

## Scope

Add route descriptor, guide, compiling example, route/feature matrix rows, package index and README truth, currentness ceiling, and separately gated live evidence if justified. Keep ACP documentation separate.

## Out Of Scope

Cline ACP documentation, release publication, registry publication, consumer edits, and unbounded live qualification

## Acceptance Criteria

- [ ] focused and affected-package validation pass
- [ ] `effigy check:examples` and `effigy qa:docs` pass
- [ ] route, feature, guide, package, and architecture indexes agree
- [ ] closeout records accepted, deferred, or rejected headless truth

## Validation

`effigy validate:focused swallowtail-adapter-cline`; `effigy package:verify-affected swallowtail-adapter-cline`; `effigy check:examples`; `effigy qa:docs`.

## Stop Conditions

Stop and record disposition if package, guide, example, matrix, or process cleanup truth diverges.

## Auto-Continuation

Return to roadmap 087 and recompute the next route.

## Evidence

Cards 304-306; Research 143; Contract 052
