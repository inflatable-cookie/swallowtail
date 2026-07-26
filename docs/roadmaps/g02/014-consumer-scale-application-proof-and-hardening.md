# 014 Consumer-Scale Application Proof And Hardening

Status: active
Posture: strict-checkpoint-gated
Owner: Tom
Created: 2026-07-25
Depends on: g02.013
Contract refs: 009, 011-013, 029, 032-037
Planning state: cards 040-041 and 045 completed; card 042 operator-paused after
a launch-target and normal-state boundary stop; cards 043-044 planned

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

- [x] Execute card 041 after Nucleus authority, worktree, and live-call gates
      are clear.
- [x] Start with read-only catalogue and Agent Chat through the normal app.
- [x] Prove version discovery, process turnover, cancellation, callbacks,
      persistence, and safe diagnostics.

### Batch 14.2a — Codex Discovery Exit Diagnostics

- [x] Execute card 045 while card 042 remains operator-paused.
- [x] Preserve the stable discovery failure code and preparation stage.
- [x] Add numeric status and bounded sanitized stderr without wrapper policy.

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
binding before the native pilot. Nucleus g05 cards 007-010 implemented those
consumer-owned rules and froze the exact tuple. Card 041 then completed all 12
planned outcomes through the normal native path at the exact 15-attempt,
6-session ceiling. Two pre-provider defects were reduced to deterministic
Swallowtail regressions before replay. Ordinary turns, inspections,
cancellation, restart recovery, deadline interruption, and joined cleanup
passed without fixture drift.

Card 042's first sustained tranche stopped after Computer Use resolved
Nucleus's shared bundle identity to a bundled app outside the proof
environment. Ten synthetic turns reached normal Nucleus state while isolated
proof evidence remained unchanged. No further provider call or record deletion
is authorized. Resume needs an exact isolated app target and a fresh bounded
reset decision. The writable tranche remains excluded pending a disposable
worktree and provider-write grant. Before this run, Soundcheck
exposed a non-zero Codex probe diagnostic gap through a host-selected wrapper.
Card 045 closed the gap with status and bounded sanitized stderr. Executable
selection remains host-owned.
