# 014 Consumer-Scale Application Proof And Hardening

Status: completed
Posture: strict-checkpoint-gated
Owner: Tom
Created: 2026-07-25
Depends on: g02.013
Contract refs: 009, 011-013, 029, 032-037
Planning state: cards 040-042, 044, and 045 complete; card 043 superseded by
operator scope decision after Soundcheck readiness and defect evidence

## Problem

The canonical candidate proves package reproducibility, 22 prepared routes,
and isolated consumer integration. It does not prove sustained use through a
working application's normal product path.

Publishing now would turn normal application defects into first-release
compatibility pressure. The candidate should instead be used as a hardening
baseline.

The later evidence correction keeps native application work narrow. Repeated
transport and lifecycle claims belong in adapter or consumer backend scenario
harnesses. A native authenticated run proves vertical wiring and current
provider acceptance; the UI is not the default lifecycle harness.

## Goals

- [x] Define an exact, bounded, consumer-owned scale envelope.
- [x] Prove Nucleus as the primary long-lived harness consumer.
- [x] Fix Swallowtail defects fixture-first and replay the same workload.
- [x] Retain Soundcheck's distinct structured-run integration and record the
      operator disposition of its repetitive synthetic scale workload.
- [x] Separate deterministic scenario repetition from native vertical smoke.
- [x] Refresh the candidate only after application evidence closes.
- [x] Reassess release readiness without authorizing publication.

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
- [x] Disposition card 043 without claiming its unchecked live workload.
- [x] Execute card 044 from the accepted Nucleus scale evidence and retained
      Soundcheck integration evidence.

## Acceptance Criteria

- [x] at least one working consumer passes an accepted sustained workload
- [x] actual provider or harness access runs through the normal product path
- [x] exact versions, topology, access, repetitions, concurrency, effects,
      cost, diagnostics, usage, and cleanup evidence are recorded
- [x] Nucleus callback, interruption, recovery, and bounded-write cases are
      covered where applicable
- [x] Soundcheck catalogue and structured-run integration remain represented
      as a separate operation shape; synthetic scale repetition is explicitly
      declined rather than reported as passed
- [x] every Swallowtail defect has a deterministic regression before rerun
- [x] no consumer-owned behavior migrates into Swallowtail
- [x] publication remains a later explicit operator decision

## Decision Gates

Card 040 makes no provider calls and edits no consumer. Cards 041-042 required
consumer-repository authority plus exact live-effect authorization. Card 043
is superseded. Workspace writes need a separate disposable-resource grant.
Card 044 may refresh local candidate evidence but cannot publish, push, tag,
or release.

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
primary research attempts, requires search progress in 2 cases, permits at
most 4 secondary
repair/ranking/companion attempts, and retains the exact 20-attempt, 4-launch,
and 2-hour ceilings.

The first approved 5-attempt pilot consumed one native launch and stopped
before provider execution. The tuple omitted dirty local `soundcheck-library`
and Poodle sources; cached app schema v48 rejected the newly seeded schema
v50. Card 091 is reopened. The next live envelope remains gated behind clean
dependency checkpoints and repaired offline native readiness.

Soundcheck then closed those gates at runner-repair source `49dfc7e`,
`soundcheck-library` `0cc339e`, scoped Poodle `666b985`, Signal `fe37838`, and
unchanged Swallowtail runtime `a3fbc14`. Native schema-v50 fixture open and all
offline validation passed.

The approved retry then found proof-fixture pollution before provider
execution. Soundcheck `282fa21` now suppresses host discovery only inside its
validated proof profile, and a rebuilt native bundle retains exactly 16
fixtures offline. The operator accepted the readiness and defect evidence,
declined further repetition of the fixed agent-review workflow, and
superseded card 043 without a live scale claim. Nucleus remains the accepted
sustained application proof. Card 044 is ready for local candidate
reassessment; publication remains separately prohibited.

The follow-up boundary audit confirmed that Codex adapter fixtures already
cover search events, attachments, schemas, cancellation, deadlines, process
join, and cleanup. Contracts 011 and 036 now place repeated mechanism and
consumer-mapping cases in purpose-built scenario harnesses. Future native
proof is a thin vertical smoke unless product startup or persistence is the
claim.

Card 044 closes the milestone with one reproducible source, 23 package
archives, all 22 packaged routes, isolated Nucleus and Soundcheck checks, and
the 93-test packaged Codex suite. Exact retained evidence names the frozen
source. No external release state changed. The candidate is technically
coherent and remains local for continued consumer soak.
