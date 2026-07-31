# 012 Grok Build 0.2.117 Range Maintenance

Status: completed
Owner: Tom
Created: 2026-07-31
Depends on: g03.011
Vision tags: maintained compatibility, installed harnesses, ACP
Contract refs: 011, 015, 023, 029, 032, 037, 039, 044
Planning state: cards 030-032 completed

## Problem

Swallowtail guarantees exact Grok Build `0.2.114`, while official stable and
the installed executable are `0.2.117`. The ACP handshake remains compatible,
but `0.2.117` changes ACP-visible background-agent termination and task-control
behavior. Treating every newer release as either identical or incompatible
would misstate both the evidence and Swallowtail's forward-compatibility
posture.

## Goal

Guarantee Grok Build ACP from baseline `0.2.114` through installed boundary
`0.2.117` with exact artifact identities, one explicit task-control behavior
milestone, unchanged portable authority, and visible unverified-newer posture.

## Execution Plan

### Batch 12.1 — Exact Range Corpus

- [x] Execute card 030.
- [x] freeze exact launcher, platform, executable, and source identities
- [x] record selected ACP initialization and release deltas
- [x] keep the existing exact `0.2.114` production claim unchanged

### Batch 12.2 — Claim And Conformance

- [x] Execute card 031.
- [x] extend the maintained claim through `0.2.117`
- [x] map `0.2.117` to a distinct private behavior revision
- [x] prove discovery, planning, run, cancellation, and activity boundaries

### Batch 12.3 — Installed And Package Acceptance

- [x] Execute card 032.
- [x] prove installed exact `0.2.117` without a provider prompt
- [x] run focused and extracted-package validation
- [x] reconcile public route truth and return to the maintenance checkpoint

## Boundaries

- no Grok installation, update, authentication, session creation, or model call
- no new direct task-control, subagent-control, or provider-session operation
- no change to delegated cached-token authority or endpoint audience
- no implicit sandbox, credential, model, endpoint, executable, or route fallback
- no Claude, Gemini, consumer, candidate, or registry-publication work
- no prerelease or unpublished-version qualification

## Acceptance Criteria

- [x] exact stable `0.2.114`, `0.2.115`, `0.2.116`, and `0.2.117` are frozen
- [x] selected ACP identity, capability, access, and lifecycle evidence is explicit
- [x] the `0.2.117` task-control delta has a named private behavior revision
- [x] baseline `0.2.114` remains maintained
- [x] installed exact `0.2.117` classifies as qualified
- [x] later stable versions remain permitted and visibly unverified
- [x] unsupported, malformed, revision-mismatched, and prerelease versions fail
  before provider work
- [x] no new portable operation or authority is implied
- [x] focused and extracted-package validation pass
- [x] architecture, route truth, front doors, roadmap state, and closeout evidence
  are current

## Decision Gates

- Stop if an exact stable point lacks official package or executable identity.
- Stop if selected ACP initialization differs from the recorded compatibility
  evidence.
- Stop if the task-control delta requires new portable authority or a contract
  change.
- Stop if range proof depends on authentication, a provider prompt, or durable
  provider mutation.

## Next Planning Checkpoint

Return to the g03 compatibility-maintenance checkpoint after card 032.
Standalone Claude ACP and Gemini range maintenance remain paused.
