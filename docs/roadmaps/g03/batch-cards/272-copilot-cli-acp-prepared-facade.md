# 272 GitHub Copilot CLI ACP Prepared Facade

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../089-copilot-cli-acp-route.md`
Depends on: Card 271; Contracts 005-006, 009-011, 023, 029, 032-033, 036-037, 039-041, 043-045, 051-052

## Goal

Expose the admitted `copilot-cli.acp` route through an adapter-local prepared constructor and typed operation.

## Scope

Bind host-approved executable or endpoint, environment, credential reference, model selection, working resource, isolation posture, timeout, and route-specific preflight evidence. Expose only the operation proved by cards 270 and 271; preserve differences through capabilities.

## Out Of Scope

consumer workflow policy, generic routing, automatic model/provider selection, public session management, and unsupported continuation

## Acceptance Criteria

- [x] Preparation fails closed on missing or mismatched authority.
- [x] Preflight names exact route and version axis.
- [x] Prepared operation drains bounded events to terminal cleanup truth.
- [x] Local and remote-authoritative host behavior is explicit where supported.

## Validation

`effigy validate:focused swallowtail-adapter-copilot` and `effigy package:verify-affected swallowtail-adapter-copilot`.

## Stop Conditions

Stop if the facade requires consumer-owned persistence, provider policy, or a capability the corpus did not prove.

## Auto-Continuation

Continue to card 273 after prepared-facade tests pass.

## Evidence

Cards 270-271; Contracts 005-006, 009-011, 023, 029, 032-033, 036-037, 039-041, 043-045, 051-052
