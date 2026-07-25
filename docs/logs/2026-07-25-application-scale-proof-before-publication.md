# Application-Scale Proof Before Publication

Date: 2026-07-25
Decision: hold first publication
Roadmap: g02.014

## Change

The canonical `0.1.0` candidate remains a local hardening baseline. Package
reproducibility, 22 packaged route proofs, and isolated Nucleus and Soundcheck
tests do not by themselves justify a first public release.

Contract 036 now requires at least one operator-selected working application to
pass an accepted sustained workload through its normal product path before
publication.

## Sequence

Nucleus is the primary proof. It exercises long-lived Codex sessions,
callbacks, application persistence, cancellation, interruption, recovery,
read-only chat, and bounded workspace execution.

Soundcheck follows as the secondary shape. It exercises catalogue discovery
and bounded structured runs with schema, attachments, reasoning, search,
progress, cancellation, and deadline behavior.

Scale is not an invented concurrency target. Card 040 must first record exact
repetitions, lifecycle turnover, supported concurrency, spend, effects, test
data, diagnostics, and stop conditions. Live provider calls, consumer edits,
and workspace writes remain separately gated.

## Current State

- Swallowtail posture: `strict-paused`
- card 040: completed, read-only
- cards 041-044: planned
- Nucleus and Soundcheck worktrees contain active unrelated user work
- crates.io, branch push, tag, GitHub release, and workflow work: held
- active candidate artifacts: unchanged regression baseline; not publishable
  because they predate this hold

## Audit Result

Nucleus's normal Agent Chat path is real Swallowtail app-server integration,
but the desktop fixes state under `$HOME/.nucleus/state` and exposes no normal
turn cancellation action. Its lower-level live smoke cannot substitute for
normal-path proof. Soundcheck already has an isolated app-data override and
normal structured-run cancellation.

Card 040 fixes the recommendation:

- pilot: 12 planned Nucleus turns, 15 maximum, 3 launches, 2 restarts, 6
  app-server lifecycles, 60 minutes
- sustained chat: 50 planned turns, 55 maximum, 5 launches, 10 app-server
  lifecycles, 4 hours
- optional writable tranche: 10 disposable-worktree task attempts under
  separate authority
- Soundcheck comparison: 16 workflows, 20 provider attempts maximum, 4
  launches, 2 hours

No consumer edit, provider call, credential read, workspace write, registry
mutation, push, tag, or release occurred.

## Next Move

After Nucleus's active desktop work is checkpointed or confirmed
non-overlapping, authorize a companion roadmap for isolated state, normal chat
cancellation, proof deadline control, and a native proof selector. Separately
approve or revise the 15-turn live pilot ceiling before card 041.
