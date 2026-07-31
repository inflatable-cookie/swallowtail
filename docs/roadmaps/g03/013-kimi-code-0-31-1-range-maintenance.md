# 013 Kimi Code 0.31.1 Range Maintenance

Status: completed
Owner: Tom
Created: 2026-07-31
Depends on: g03.012
Vision tags: maintained compatibility, installed harnesses, ACP, local server
Contract refs: 011, 023, 029, 032, 037, 038, 042, 044
Planning state: cards 033-035 completed

## Problem

Kimi Code `0.31.1` became stable after Swallowtail qualified `0.31.0`. ACP and
default headless retain their selected behavior, while the local server changes
workspace-scoped session lookup, catalogue refresh stability, and optional
turn-interruption detail.

## Goal

Guarantee Kimi Code ACP, default headless, and local-server routes through
`0.31.1` using exact release identity, one local-server behavior milestone,
unchanged portable authority, and visible unverified-newer posture.

## Execution Plan

### Batch 13.1 — Exact Release Corpus

- [x] Execute card 033.
- [x] freeze exact source, package, artifact, and selected blob identity
- [x] add bounded ACP, headless, and local-server delta fixtures
- [x] keep the production ceiling at `0.31.0`

### Batch 13.2 — Claims And Conformance

- [x] Execute card 034.
- [x] extend all three maintained claims through `0.31.1`
- [x] retain ACP and headless behavior revisions
- [x] map exact local-server `0.31.1` to its refresh-stable revision

### Batch 13.3 — Artifact And Package Acceptance

- [x] Execute card 035.
- [x] accept the exact signed artifact without changing the installed CLI
- [x] run focused and extracted-package proof
- [x] reconcile public truth and return to the maintenance checkpoint

## Boundaries

- no Kimi installation, update, login, session creation, or model call
- no experimental v2 headless qualification
- no new portable interruption, catalogue, MCP, or session authority
- no implicit executable, credential, model, endpoint, route, or sandbox fallback
- no Claude, Gemini, consumer, candidate, or registry-publication work

## Acceptance Criteria

- [x] exact stable `0.31.1` identity is frozen
- [x] ACP and default headless selected behavior remains unchanged
- [x] local-server refresh and optional interruption deltas have one named
  private behavior revision
- [x] all three exact `0.31.1` routes classify as qualified
- [x] later stable versions remain permitted and visibly unverified
- [x] malformed, prerelease, unsupported, and version-mismatched inputs fail
  before provider work
- [x] focused and extracted-package validation pass
- [x] architecture, route truth, front doors, roadmap state, and closeout evidence
  are current

## Decision Gates

- Stop if exact artifact identity cannot be corroborated from official sources.
- Stop if selected ACP or default headless protocol differs from the recorded
  evidence.
- Stop if the local-server change requires a new public operation or contract.
- Stop if acceptance depends on authentication, a provider prompt, or durable
  provider mutation.

## Next Planning Checkpoint

Return to the g03 compatibility-maintenance checkpoint after card 035.
Standalone Claude ACP and Gemini range maintenance remain paused.
