# 014 Consumer-Scale Application Proof And Hardening

Status: active
Posture: strict-checkpoint-gated
Owner: Tom
Created: 2026-07-25
Depends on: g02.013
Contract refs: 009, 011-013, 029, 032-037
Planning state: cards 040-042 and 045 plus Soundcheck offline readiness card
091 completed; card 043 remains paused before live proof; card 044 planned

## Problem

The canonical candidate proves package reproducibility, 22 prepared routes,
and isolated consumer integration. It does not prove sustained use through a
working application's normal product path.

Publishing now would turn normal application defects into first-release
compatibility pressure. The candidate should instead be used as a hardening
baseline.

## Goals

- [x] Define an exact, bounded, consumer-owned scale envelope.
- [x] Prove Nucleus as the primary long-lived harness consumer.
- [x] Fix Swallowtail defects fixture-first and replay the same workload.
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

- [x] Execute card 042 after pilot defects close.
- [x] Run the accepted repeated and mixed workload.
- [x] Keep bounded disposable-workspace execution behind separate write
      authorization.
- [x] Reduce every Swallowtail defect to deterministic regression evidence.

### Batch 14.4 — Secondary Shape And Candidate Refresh

- [x] Support Soundcheck card 091 offline.
- [ ] Execute card 043 only after separate live-effect approval.
- [ ] Execute card 044 after both application proofs pass.

## Acceptance Criteria

- [x] at least one working consumer passes an accepted sustained workload
- [x] actual provider or harness access runs through the normal product path
- [x] exact versions, topology, access, repetitions, concurrency, effects,
      cost, diagnostics, usage, and cleanup evidence are recorded
- [x] Nucleus callback, interruption, recovery, and bounded-write cases are
      covered where applicable
- [ ] Soundcheck catalogue and structured-run behavior passes as a separate
      operation shape
- [x] every Swallowtail defect has a deterministic regression before rerun
- [x] no consumer-owned behavior migrates into Swallowtail
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
proof evidence remained unchanged. The operator chose to preserve those marked
records and approved a 60-turn, 7-launch, 12-session reset. The resumed path
must use one rebuilt bundle, an environment-bound executable launch, full-path
UI control, and a pre-turn PID/database proof. The writable tranche remains
excluded pending a disposable worktree and provider-write grant. Before this
run, Soundcheck
exposed a non-zero Codex probe diagnostic gap through a host-selected wrapper.
Card 045 closed the gap with status and bounded sanitized stderr. Executable
selection remains host-owned.

The reset then completed all 50 valid turns across 5 native launches and 10
app-server lifecycles: 35 ordinary completions, 10 read-only callbacks, 3
cancellations, and 2 controlled deadlines. Isolated evidence moved from
14 total turns to 64, ending with 57 completed, 4 cancelled, 3 timed out, and
no failed, active, or unexpected turn. The exact fixture stayed clean, no
process leaked, and the full reset stopped at its accepted 60-turn, 7-launch,
12-session ceiling after including the invalid first tranche.

Card 043's gate audit found active Soundcheck M11 work in the exact assistant
path and stopped without editing the consumer or calling the provider.
Soundcheck product baseline `7c135da` is clean; proof-support source `3566419`
completed temporary M12 card 091's deterministic assistant-data seed,
proof-only deadline, sanitized per-attempt evidence, and teardown. Soundcheck
health, QA, 24 frontend tests, and 176 Rust tests pass without provider
effects. The corrected live envelope still authorizes search for all 16
primary research attempts,
requires search progress in 2 cases, permits at most 4 secondary
repair/ranking/companion attempts, and retains the exact 20-attempt, 4-launch,
and 2-hour ceilings. It remains separately approval-gated.
