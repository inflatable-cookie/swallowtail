# 008 Portable Watcher Lifecycle Core

Status: complete
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: Contract 059

## Goal

Implement the provider-neutral watcher identity, lifecycle, ownership, control,
and activity vocabulary without starting a process or selecting a route.

## Scope

1. Add bounded watcher ids, owning-turn keys, states, terminal causes,
   revisions, summaries, and requester identity in `swallowtail-core`.
2. Add object-safe model/operator control roles and optional host-service
   registration in `swallowtail-runtime`.
3. Add a pure deterministic registry/state machine for start acceptance,
   running, terminal, joined, wait, stop, races, bounds, and foreign ids.
4. Add watcher activity projection on the existing turn stream.
5. Extend testkit assertions without a process executor or adapter.

## Non-Goals

- local process launch, arbitrary command schemas, PIDs, or host registry
- injected skill, MCP, hook, adapter, or provider behavior
- automatic wait, detached work, persistence, recovery, or raw logs
- consumer UI or route documentation

## Acceptance Criteria

- [x] identities are turn-scoped, opaque, bounded, and redacted
- [x] model and operator roles stay distinct over one registry state
- [x] lifecycle and race transitions are deterministic
- [x] wait, stop, cancellation, deadline, and join truth are representable
- [x] no host or provider effect is possible in this card

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-testkit`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-testkit`
- `git diff --check`

## Stop Conditions

- public records need a PID, executable, command, path, or raw output
- the core must choose a provider tool or executor
- one role cannot preserve model versus operator requester identity
- implementation requires shared route or consumer policy

## Auto-Continuation

No. Return one reviewable PR. Cards 009-011 remain planned.
