# 276 Mistral Vibe Headless Prepared Facade

Status: planned
Owner: Tom
Created: 2026-08-18
Milestone: `../090-mistral-vibe-headless-route.md`
Depends on: Card 275; Contracts 005-006, 009-011, 023, 029, 032-033, 036-037, 039-041, 043-045, 051-052

## Goal

Expose the admitted `mistral-vibe.headless` route through an adapter-local prepared constructor and typed operation.

## Scope

Bind host-approved executable or endpoint, environment, credential reference, model selection, working resource, isolation posture, timeout, and route-specific preflight evidence. Expose only the operation proved by cards 274 and 275; preserve differences through capabilities.

## Out Of Scope

consumer workflow policy, generic routing, automatic model/provider selection, public session management, and unsupported continuation

## Acceptance Criteria

- [ ] Preparation fails closed on missing or mismatched authority.
- [ ] Preflight names exact route and version axis.
- [ ] Prepared operation drains bounded events to terminal cleanup truth.
- [ ] Local and remote-authoritative host behavior is explicit where supported.

## Validation

`effigy validate:focused swallowtail-adapter-mistral-vibe` and `effigy package:verify-affected swallowtail-adapter-mistral-vibe`.

## Stop Conditions

Stop if the facade requires consumer-owned persistence, provider policy, or a capability the corpus did not prove.

## Auto-Continuation

Continue to card 277 after prepared-facade tests pass.

## Evidence

Cards 274-275; Contracts 005-006, 009-011, 023, 029, 032-033, 036-037, 039-041, 043-045, 051-052
