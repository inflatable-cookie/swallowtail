# 014 Consumer-Scale Application Proof And Hardening

Status: active
Posture: strict-checkpoint-gated
Owner: Tom
Created: 2026-07-25
Depends on: g02.013
Contract refs: 009, 011-013, 029, 032-037
Planning state: card 040 completed; cards 041-044 planned

## Problem

The canonical candidate proves package reproducibility, 22 prepared routes,
and isolated consumer integration. It does not prove sustained use through a
working application's normal product path.

Publishing now would turn normal application defects into first-release
compatibility pressure. The candidate should instead be used as a hardening
baseline.

## Goals

- [x] Define an exact, bounded, consumer-owned scale envelope.
- [ ] Prove Nucleus as the primary long-lived harness consumer.
- [ ] Fix Swallowtail defects fixture-first and replay the same workload.
- [ ] Prove Soundcheck's materially different bounded structured-run path.
- [ ] Refresh the candidate only after application evidence closes.
- [ ] Reassess release readiness without authorizing publication.

## Non-Goals

- [ ] Do not publish crates, push branches or tags, create a GitHub release, or
      change registry ownership.
- [ ] Do not turn consumer prompts, workflows, persistence, or UI into
      Swallowtail policy.
- [ ] Do not use uncontrolled provider spend, live user data, or unbounded
      workspace writes.
- [ ] Do not add parallelism unsupported by the selected application or route.
- [ ] Do not call one successful smoke test scale evidence.

## Execution Plan

### Batch 14.1 — Proof Envelope

- [x] Execute card 040.
- [x] Audit existing native, live, packaged, and diagnostic surfaces.
- [x] Produce the exact Nucleus-first workload, effect, cost, and stop
      envelope.

### Batch 14.2 — Nucleus Native Pilot

- [ ] Execute card 041 after Nucleus authority, worktree, and live-call gates
      are clear.
- [ ] Start with read-only catalogue and Agent Chat through the normal app.
- [ ] Prove version discovery, process turnover, cancellation, callbacks,
      persistence, and safe diagnostics.

### Batch 14.3 — Sustained Nucleus Hardening

- [ ] Execute card 042 after pilot defects close.
- [ ] Run the accepted repeated and mixed workload.
- [ ] Add bounded disposable-workspace execution only under separate write
      authorization.
- [ ] Reduce every Swallowtail defect to deterministic regression evidence.

### Batch 14.4 — Secondary Shape And Candidate Refresh

- [ ] Execute card 043 after the active Soundcheck lane permits consumer work.
- [ ] Execute card 044 after both application proofs pass.

## Acceptance Criteria

- [ ] at least one working consumer passes an accepted sustained workload
- [ ] actual provider or harness access runs through the normal product path
- [ ] exact versions, topology, access, repetitions, concurrency, effects,
      cost, diagnostics, usage, and cleanup evidence are recorded
- [ ] Nucleus callback, interruption, recovery, and bounded-write cases are
      covered where applicable
- [ ] Soundcheck catalogue and structured-run behavior passes as a separate
      operation shape
- [ ] every Swallowtail defect has a deterministic regression before rerun
- [ ] no consumer-owned behavior migrates into Swallowtail
- [ ] publication remains a later explicit operator decision

## Decision Gates

Card 040 makes no provider calls and edits no consumer. Cards 041-043 require
consumer-repository authority plus exact live-effect authorization. Workspace
writes need a separate disposable-resource grant. Card 044 may refresh local
candidate evidence but cannot publish, push, tag, or release.

Card 040 found that Nucleus needed an app-scoped isolated state root, normal
Agent Chat cancellation, a proof-only deadline, and a disposable fixture
binding before the native pilot. Nucleus g05 cards 007-010 now implement those
consumer-owned rules, pass deterministic evidence, and freeze the exact
source, executable, access, topology, model, fixture, workload, and stop tuple
without a provider call. Card 041 is ready except for operator acceptance of
the ChatGPT-backed 15-turn and 60-minute live envelope.
