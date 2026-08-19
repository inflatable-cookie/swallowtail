# 264 Cline ACP Prepared Facade

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../086-cline-acp-route.md`
Depends on: Card 263; Contracts 005-006, 009-011, 023, 029, 032-033, 036-037, 039-045, 051-052

## Goal

Expose `cline.acp` through an adapter-local prepared constructor and typed operation.

## Scope

Bind host-approved ACP executable/endpoint, environment, credential reference, model selection where exposed, working resource, isolation, timeout, and preflight evidence. Preserve ACP capability differences without adding headless behavior.

## Out Of Scope

generic routing, automatic model/provider selection, public session management, and unsupported ACP continuation

## Acceptance Criteria

- [x] preparation fails closed on missing or mismatched authority
- [x] preflight names exact ACP route and version axis
- [x] operation drains bounded events to terminal cleanup truth
- [x] prepared-facade tests pass

## Validation

`effigy validate:focused swallowtail-adapter-cline`; `effigy package:verify-affected swallowtail-adapter-cline`.

## Stop Conditions

Stop if the facade requires consumer persistence, provider policy, or an ACP capability not proved by card 262.

## Auto-Continuation

Continue to card 265 after prepared-facade tests pass.

## Evidence

Cards 262-263; Contracts 005-006, 009-011, 023, 029, 032-033, 036-037, 039-045, 051-052
