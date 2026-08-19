# 305 Cline Headless Driver Core

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../087-cline-headless-route.md`
Depends on: Card 304; Contracts 005-006, 009-010, 023, 029, 032-033, 039-041, 044-045, 051

## Goal

Implement the smallest Cline headless process driver without inheriting ACP lifecycle or team semantics.

## Scope

Add executable discovery, bounded JSON/stream decode, activity projection, terminal exit mapping, process cancellation/deadline, error mapping, credential-last cleanup, and deterministic fixtures from card 304.

## Out Of Scope

ACP continuation, teams, scheduling, session import, retries, browser/UI behavior, and consumer repository policy

## Acceptance Criteria

- [x] framing and process drift are rejected before provider work
- [x] fixtures cover success, failure, malformed/unknown input, bounds, cancellation/deadline, and cleanup
- [x] native output does not leak into stable diagnostics
- [x] focused package tests pass without credentials

## Validation

`effigy validate:focused swallowtail-adapter-cline` after the package exists.

## Stop Conditions

Stop if implementation needs ACP-only operations or a hidden retry, resume, or authority assumption.

## Auto-Continuation

Continue to card 306 after deterministic driver and fixture tests pass.

## Evidence

Card 304; Contracts 005-006, 009-010, 023, 029, 032-033, 039-041, 044-045, 051
