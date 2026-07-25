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
- Nucleus companion contract and roadmap: commit `962d1901`

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

## Companion Planning

The operator authorized the Nucleus companion lane. Nucleus Spec 014 promoted
the settled shape into Contracts 008, 010, and 030 plus roadmap g05.003:

- `NUCLEUS_DESKTOP_DATA_ROOT` isolates database, snapshots, and UI config
- normal Agent Chat gains cancellation outside its serialized service mutex
- completed, cancelled, timed-out, and failed outcomes persist distinctly
- a bounded proof deadline uses the production Swallowtail deadline path
- Effigy launches and inspects the credential-free native proof profile

Nucleus commit `962d1901` records the plan without touching the user's active
code changes. Cards 007-009 remain planned because current sidebar card 006
modifies the same Tauri, Agent Chat, and server files.

## Next Move

Complete and checkpoint Nucleus card 006, then execute Nucleus cards 007-009
without credentials or provider calls. Card 010 returns the exact pilot.
Separately approve or revise the 15-turn live ceiling before Swallowtail card
041.
