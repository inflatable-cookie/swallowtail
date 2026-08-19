# 263 Cline ACP Driver Core

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../086-cline-acp-route.md`
Depends on: Card 262; Contracts 005-006, 009-010, 023, 029, 032-033, 039-041, 044-045, 051

## Goal

Implement the smallest Cline ACP driver without projecting headless-only or unsupported ACP features.

## Scope

Add discovery, ACP initialize/capability binding, bounded decode, activity projection, terminal outcome, error mapping, cancellation/deadline, credential-last cleanup, and deterministic fixtures from card 262.

## Out Of Scope

Cline headless, ACP continuation not proved by fixtures, teams, scheduling, session import, retries, browser/UI behavior, and consumer policy

## Acceptance Criteria

- [x] ACP drift is rejected before provider work
- [x] fixtures cover success, failure, malformed/unknown input, bounds, cancellation/deadline, and cleanup
- [x] native fields do not leak into stable diagnostics
- [x] focused package tests pass without credentials

## Validation

`effigy validate:focused swallowtail-adapter-cline` after the package exists.

## Stop Conditions

Stop if the driver needs a new public operation or hidden retry, resume, or authority assumption.

## Auto-Continuation

Continue to card 264 after deterministic driver and fixture tests pass.

## Evidence

Card 262; Contracts 005-006, 009-010, 023, 029, 032-033, 039-041, 044-045, 051
