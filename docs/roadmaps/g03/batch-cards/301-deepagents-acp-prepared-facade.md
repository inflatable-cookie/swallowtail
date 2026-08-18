# 301 Deep Agents ACP Prepared Facade

Status: planned
Owner: Tom
Created: 2026-08-18
Milestone: `../096-deep-agents-acp-route.md`
Depends on: Card 300; Contracts 005-006, 009-011, 017, 023, 029, 032-033, 036-037, 039-045, 051-052

## Goal

Expose `deepagents.acp` through an adapter-local prepared constructor only after authority and cleanup are proven.

## Scope

Bind exact endpoint or executable, credential reference, model or agent selection, working resource, host service, remote topology, timeout, and isolation posture. Expose only the operation and capability subset proved by cards 299 and 300.

## Out Of Scope

consumer product workflows, automatic route selection, generic provider options, implicit persistence, and unsupported session management

## Acceptance Criteria

- [ ] Preparation fails closed on missing or mismatched authority.
- [ ] Preflight records route, topology, version axis, and cleanup posture.
- [ ] Operation reaches terminal truth with bounded event delivery.
- [ ] Host identity and remote attachment rules are explicit where relevant.

## Validation

`effigy validate:focused swallowtail-adapter-deepagents` and `effigy package:verify-affected swallowtail-adapter-deepagents`.

## Stop Conditions

Stop if the facade silently owns remote resources or claims provider retention/control not established by the corpus.

## Auto-Continuation

Continue to card 302 after prepared-facade tests pass.

## Evidence

Cards 299-300; Contracts 005-006, 009-011, 017, 023, 029, 032-033, 036-037, 039-045, 051-052
