# 306 Cline Headless Prepared Facade

Status: planned
Owner: Tom
Created: 2026-08-18
Milestone: `../087-cline-headless-route.md`
Depends on: Card 305; Contracts 005-006, 009-011, 023, 029, 032-033, 036-037, 039-045, 051-052

## Goal

Expose `cline.headless` through an adapter-local prepared constructor and typed operation.

## Scope

Bind host-approved executable, environment, credential reference, model selection where exposed, working resource, isolation, timeout, and immutable preflight evidence. Keep headless process semantics separate from Cline ACP.

## Out Of Scope

generic routing, automatic model/provider selection, public session management, and ACP continuation

## Acceptance Criteria

- [ ] preparation fails closed on missing or mismatched authority
- [ ] preflight names exact headless route and version axis
- [ ] process reaches terminal cleanup truth
- [ ] prepared-facade tests pass

## Validation

`effigy validate:focused swallowtail-adapter-cline`; `effigy package:verify-affected swallowtail-adapter-cline`.

## Stop Conditions

Stop if the facade needs consumer persistence, provider policy, or ACP behavior not proved by card 304.

## Auto-Continuation

Continue to card 307 after prepared-facade tests pass.

## Evidence

Cards 304-305; Contracts 005-006, 009-011, 023, 029, 032-033, 036-037, 039-045, 051-052
