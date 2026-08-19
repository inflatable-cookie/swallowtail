# 284 Pi ACP Prepared Facade

Status: superseded
Owner: Tom
Created: 2026-08-18
Milestone: `../092-pi-acp-route.md`
Depends on: Card 283; Contracts 005-006, 009-011, 023, 029, 032-033, 036-037, 039-041, 043-045, 051-052
Note: superseded by card 282; official Pi has no native ACP wire.

## Goal

Expose the admitted `pi.acp` route through an adapter-local prepared constructor and typed operation.

## Scope

Bind host-approved executable or endpoint, environment, credential reference, model selection, working resource, isolation posture, timeout, and route-specific preflight evidence. Expose only the operation proved by cards 282 and 283; preserve differences through capabilities.

## Out Of Scope

consumer workflow policy, generic routing, automatic model/provider selection, public session management, and unsupported continuation

## Acceptance Criteria

- [ ] Preparation fails closed on missing or mismatched authority.
- [ ] Preflight names exact route and version axis.
- [ ] Prepared operation drains bounded events to terminal cleanup truth.
- [ ] Local and remote-authoritative host behavior is explicit where supported.

## Validation

`effigy validate:focused swallowtail-adapter-pi` and `effigy package:verify-affected swallowtail-adapter-pi`.

## Stop Conditions

Stop if the facade requires consumer-owned persistence, provider policy, or a capability the corpus did not prove.

## Auto-Continuation

Continue to card 285 after prepared-facade tests pass.

## Evidence

Cards 282-283; Contracts 005-006, 009-011, 023, 029, 032-033, 036-037, 039-041, 043-045, 051-052
