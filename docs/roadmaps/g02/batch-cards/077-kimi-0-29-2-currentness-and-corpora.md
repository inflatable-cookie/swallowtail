# 077 Kimi 0.29.2 Currentness And Corpora

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../024-kimi-structured-coverage-and-matrix-closeout.md`

## Objective

Qualify maintained Kimi Code `0.29.2` before extending structured-run claims.

## Governing Refs

- Research 006, 010, 040-041, 044
- Contracts 015, 017, 029, 034, 038-039
- exact Kimi Code `0.29.1` and `0.29.2` tagged source

## Scope

1. Compare ACP, headless, local REST/WebSocket, lifecycle, catalogue,
   reasoning, profile, disabled-tool, authentication, and version surfaces
   against `0.29.0`.
2. Add behavior milestones or exact exclusions where required.
3. Freeze bounded deterministic corpora for selected changed surfaces.
4. Extend the guaranteed ceiling only after every claimed operation passes.
5. Retain visible unverified-newer posture for later stable releases.

## Acceptance Criteria

- [x] exact source and release provenance is recorded
- [x] every selected surface has an explicit delta disposition
- [x] changed protocol behavior gets a new revision
- [x] `0.29.2` is never inferred compatible from SemVer alone
- [x] no live credential, installed upgrade, provider call, or session effect
- [x] older qualified milestones remain passing

## Validation

- full Kimi deterministic suite
- exact range and corpus tests
- strict Clippy, docs, routes, and `git diff --check`

Completed evidence:

- 64 deterministic Kimi tests pass; one live installed probe remains gated and
  ignored
- ACP and local-server operations execute at every claimed milestone
- strict Clippy, rustdoc, route, docs, format, and diff checks pass
- Research 046 records exact release, source, corpus, and delta provenance

## Stop Conditions

- tagged source or release identity is ambiguous
- a claimed surface requires live account evidence
- changed behavior cannot coexist with the current public contract

## Auto-Continuation

Yes. Continue to card 078 after qualification passes.
