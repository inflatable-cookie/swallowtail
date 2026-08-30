# g05.007 Claude Watcher Live Acceptance

Status: stopped before contact
Owner: Tom
Created: 2026-08-30
Updated: 2026-08-30
Depends on: completed g05.006 card 019; Contracts 044, 059, and 060
Vision tags: process watchers, live acceptance, consumer activity
Planning state: card 020 stopped before contact; the authorized turn is unconsumed; no claim

## Problem

Card 011's one live attempt did not create a watcher, so it could not prove
same-turn Stop re-entry. Card 019 has since landed the lossless lifecycle feed,
direct Stop-reentry recorder, negative counterexamples, and panic-safe cleanup.
The repaired oracle is credential-free evidence until one exact provider turn
exercises the complete sequence.

## Goal

Use exactly one newly authorized Claude Code `2.1.251` turn with exact model
`claude-haiku-4-5` to test the repaired watcher oracle. Publish only the exact
proved route point after the full oracle passes. Otherwise record one sanitized
evidence stop and keep every watcher claim withheld.

## Outcome

Card 020 stopped before provider contact on 2026-08-30. No request reached
Claude and the operator's single-turn authority is unconsumed. Details in
[the pre-contact stop log](../../logs/2026-08-30-g05-007-card-020-pre-contact-stop.md).

The worker host is `linux-x86_64`. Installed `claude --version` is exact
`2.1.251`, `ANTHROPIC_API_KEY` is absent, and the tree is clean, but the
envelope's frozen native SHA-256 is Research 261's `darwin-arm64` value and
does not describe this platform's official binary. Two named pre-contact
validation rows are also red on unchanged `main`:
`effigy package:verify-affected` (yanked `chacha20 0.10.1` pinned in
`Cargo.lock`) and `effigy package:api` (`cargo-public-api 0.52.0` absent).

All three are planning findings. The card forbids probe edits and setup work
before contact, so the milestone stops here rather than repairing.

## Execution Plan

### Batch 7.1 — Exact Live Acceptance

- [x] execute ready card 020 from current pushed `main` through its committed
      manual worker handoff — executed to the pre-contact gate
- [x] validate the unchanged repaired selector and exact identity before any
      provider request — validated; the gate is red
- [x] consume no more than one provider turn; never fall back or rerun — zero
      turns consumed
- [x] return one evidence PR for either the proved claim or the honest stop

## Acceptance Criteria

- [ ] exact installed version, frozen native digest, exact model, local
      subscription path, and one-turn budget are verified before contact —
      version, key absence, and budget verified; the frozen digest does not
      match this platform
- [ ] the live trace proves reserved tool discovery, watcher start, active Stop
      block, same-session continuation, explicit wait or stop, joined zero
      state, clean provider terminal, and joined cleanup in order — no live
      trace exists
- [ ] consumer watcher activity contains truthful started, running, and
      terminal observations without raw process or provider material — not
      reached
- [x] success publishes only the exact live-proved `2.1.251` and
      `claude-haiku-4-5` point; failure or ambiguity publishes no claim — no
      claim published
- [x] the card, milestone, evidence log, indexes, and sole Next Task record the
      consumed attempt and actual outcome — recorded as an unconsumed
      pre-contact stop

## Stop Conditions

- installed Claude is not exact `2.1.251` or its native digest differs from
  `625869b01e0050f260b2980fac248fd9cef9e462612bded4ec9d3d49ff8969a5`
- exact `claude-haiku-4-5` is unavailable, authentication needs setup or
  inspection, `ANTHROPIC_API_KEY` is present, or source state is dirty
- credential-free validation or the unchanged oracle is not green before
  provider contact
- the single provider turn fails, is inconclusive, or misses any required
  ordering fact
- review would require a second provider request, a changed probe, raw/private
  evidence, a wider version/model claim, or a new product or contract decision

## Batch Cards

- [020 Claude Code Watcher Live Acceptance](batch-cards/020-claude-code-watcher-live-acceptance.md)

## References

- [Contract 044 Observable Agent Activity And Disclosure](../../contracts/044-observable-agent-activity-and-disclosure.md)
- [Contract 059 Operation-Scoped Process Watchers](../../contracts/059-operation-scoped-process-watchers.md)
- [Contract 060 Operation-Scoped Watcher HTTP Bridge](../../contracts/060-operation-scoped-watcher-http-bridge.md)
- [Card 011 Live Stop Review](../../logs/2026-08-30-g05-003-card-011-live-stop-review.md)
- [Card 019 Watcher Proof Repair](../../logs/2026-08-30-g05-006-card-019-watcher-proof-repair.md)
- [Card 020 Pre-Contact Stop](../../logs/2026-08-30-g05-007-card-020-pre-contact-stop.md)
