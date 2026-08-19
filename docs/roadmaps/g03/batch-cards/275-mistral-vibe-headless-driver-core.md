# 275 Mistral Vibe Headless Driver Core

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../090-mistral-vibe-headless-route.md`
Depends on: Card 274; Contracts 005-006, 009-010, 023, 029, 032-033, 039-041, 044-045, 051

## Goal

Implement the smallest provider-specific driver for `mistral-vibe.headless` without widening authority or projecting unsupported features.

## Scope

Add the adapter package or existing-Pi extension, discovery and compatibility binding, bounded decode, terminal outcome, activity projection, error mapping, cancellation/deadline behavior, credential-last cleanup, and deterministic fixtures named by card 274. Keep native fields private unless the selected route proves them.

## Out Of Scope

interactive continuation, catalogue/import, session lifecycle, subagents, callbacks, sandbox guarantees, retries, browser use, and alternate protocol surfaces

## Acceptance Criteria

- [x] Driver rejects identity/protocol drift before provider work.
- [x] Fixtures cover success, failure, malformed/unknown input, bounds, cancellation/deadline, and cleanup.
- [x] Provider payloads do not leak into stable diagnostics.
- [x] Focused package tests pass without credentials.

## Validation

`effigy validate:focused swallowtail-adapter-mistral-vibe` after the package exists; add the smallest package-local test target.

## Stop Conditions

Stop if implementation needs a new public operation or a hidden retry, resume, or authority assumption.

## Auto-Continuation

Continue to card 276 after deterministic driver and fixture tests pass.

## Evidence

Card 274; Research 150; `docs/logs/2026-08-19-mistral-vibe-headless-driver-core.md`.
`effigy validate:focused swallowtail-adapter-mistral-vibe` passed (23 tests,
Clippy warnings denied). No live install or prompt. Production claim stays
card 277.
