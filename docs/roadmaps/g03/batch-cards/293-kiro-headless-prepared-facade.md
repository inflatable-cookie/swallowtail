# 293 Kiro Headless Prepared Facade

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../094-kiro-headless-route.md`
Depends on: Card 292; Contracts 005-006, 009-011, 017, 023, 029, 032-033, 036-037, 039-045, 051-052

## Goal

Expose `kiro.acp` through an adapter-local prepared constructor only after authority and cleanup are proven.

## Scope

Bind exact endpoint or executable, credential reference, model or agent selection, working resource, host service, remote topology, timeout, and isolation posture. Expose only the operation and capability subset proved by cards 291 and 292.

## Out Of Scope

consumer product workflows, automatic route selection, generic provider options, implicit persistence, and unsupported session management

## Acceptance Criteria

- [x] Preparation fails closed on missing or mismatched authority.
- [x] Preflight records route, topology, version axis, and cleanup posture.
- [x] Operation reaches terminal truth with bounded event delivery.
- [x] Host identity and remote attachment rules are explicit where relevant.

## Validation

`effigy validate:focused swallowtail-adapter-kiro` and `effigy package:verify-affected swallowtail-adapter-kiro`.

## Stop Conditions

Stop if the facade silently owns remote resources or claims provider retention/control not established by the corpus.

## Auto-Continuation

Continue to card 294 after prepared-facade tests pass.

## Evidence

Cards 291-292; Research 156; `docs/logs/2026-08-19-kiro-acp-prepared-facade.md`.
`effigy validate:focused swallowtail-adapter-kiro` passed (30 tests,
Clippy warnings denied). `effigy package:verify-affected swallowtail-adapter-kiro`
passed. No live install, login, or prompt. Production claim stays card 294.
