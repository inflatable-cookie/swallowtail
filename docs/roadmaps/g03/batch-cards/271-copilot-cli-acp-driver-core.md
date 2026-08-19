# 271 GitHub Copilot CLI ACP Driver Core

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../089-copilot-cli-acp-route.md`
Depends on: Card 270; Contracts 005-006, 009-010, 023, 029, 032-033, 039-041, 044-045, 051

## Goal

Implement the smallest provider-specific driver for `copilot-cli.acp` without widening authority or projecting unsupported features.

## Scope

Add the adapter package or existing-Pi extension, discovery and compatibility binding, bounded decode, terminal outcome, activity projection, error mapping, cancellation/deadline behavior, credential-last cleanup, and deterministic fixtures named by card 270. Keep native fields private unless the selected route proves them.

## Out Of Scope

interactive continuation, catalogue/import, session lifecycle, subagents, callbacks, sandbox guarantees, retries, browser use, and alternate protocol surfaces

## Acceptance Criteria

- [x] Driver rejects identity/protocol drift before provider work.
- [x] Fixtures cover success, failure, malformed/unknown input, bounds, cancellation/deadline, and cleanup.
- [x] Provider payloads do not leak into stable diagnostics.
- [x] Focused package tests pass without credentials.

## Validation

`effigy validate:focused swallowtail-adapter-copilot` after the package exists; add the smallest package-local test target.

## Stop Conditions

Stop if implementation needs a new public operation or a hidden retry, resume, or authority assumption.

## Auto-Continuation

Continue to card 272 after deterministic driver and fixture tests pass.

## Evidence

Card 270; Contracts 005-006, 009-010, 023, 029, 032-033, 039-041, 044-045, 051
