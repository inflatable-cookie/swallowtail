# 267 Goose ACP Driver Core

Status: planned
Owner: Tom
Created: 2026-08-18
Milestone: `../088-goose-acp-route.md`
Depends on: Card 266; Contracts 005-006, 009-010, 023, 029, 032-033, 039-041, 044-045, 051

## Goal

Implement the smallest provider-specific driver for `goose.acp` without widening authority or projecting unsupported features.

## Scope

Add the adapter package or existing-Pi extension, discovery and compatibility binding, bounded decode, terminal outcome, activity projection, error mapping, cancellation/deadline behavior, credential-last cleanup, and deterministic fixtures named by card 266. Keep native fields private unless the selected route proves them.

## Out Of Scope

interactive continuation, catalogue/import, session lifecycle, subagents, callbacks, sandbox guarantees, retries, browser use, and alternate protocol surfaces

## Acceptance Criteria

- [ ] Driver rejects identity/protocol drift before provider work.
- [ ] Fixtures cover success, failure, malformed/unknown input, bounds, cancellation/deadline, and cleanup.
- [ ] Provider payloads do not leak into stable diagnostics.
- [ ] Focused package tests pass without credentials.

## Validation

`effigy validate:focused swallowtail-adapter-goose` after the package exists; add the smallest package-local test target.

## Stop Conditions

Stop if implementation needs a new public operation or a hidden retry, resume, or authority assumption.

## Auto-Continuation

Continue to card 268 after deterministic driver and fixture tests pass.

## Evidence

Card 266; Contracts 005-006, 009-010, 023, 029, 032-033, 039-041, 044-045, 051
