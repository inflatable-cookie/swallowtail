# 292 Kiro Headless Driver Core

Status: planned
Owner: Tom
Created: 2026-08-18
Milestone: `../094-kiro-headless-route.md`
Depends on: Card 291; Contracts 005-006, 009-011, 017, 023, 029, 032-033, 039-041, 044-045, 051

## Goal

Implement the smallest driver for `kiro.headless` while keeping remote/local ownership and native provider controls explicit.

## Scope

Add discovery, compatibility, bounded decode, activity, terminal outcomes, cancellation/deadline, error mapping, resource ownership, and joined cleanup for the operation admitted by card 291. Keep remote server state, text/Git mutation, ACP lifecycle, and provider sessions separate.

## Out Of Scope

automatic workspace/repository mutation policy, session import or management, retries, generic routing, and unsupported alternate modes

## Acceptance Criteria

- [ ] Fixtures cover success, failure, malformed/unknown input, bounds, cancellation/deadline, and cleanup.
- [ ] Remote or local resource ownership is testable.
- [ ] Driver rejects protocol/version drift before provider work.
- [ ] Focused package tests pass without credentials.

## Validation

`effigy validate:focused swallowtail-adapter-kiro` after the package exists.

## Stop Conditions

Stop if the driver cannot distinguish provider authority from consumer policy or needs a new public operation without a promoted contract.

## Auto-Continuation

Continue to card 293 after deterministic tests pass.

## Evidence

Card 291; Contracts 005-006, 009-011, 017, 023, 029, 032-033, 039-041, 044-045, 051
