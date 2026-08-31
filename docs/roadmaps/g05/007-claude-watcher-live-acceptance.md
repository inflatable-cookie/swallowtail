# g05.007 Claude Watcher Live Acceptance

Status: stopped after live evidence
Owner: Tom
Created: 2026-08-30
Updated: 2026-08-31
Depends on: completed g05.006 card 019; Contracts 044, 059, and 060
Vision tags: process watchers, live acceptance, consumer activity
Planning state: card 020 complete as a Linux live evidence stop; claims withheld; turn consumed

## Problem

Card 011's one live attempt did not create a watcher, so it could not prove
same-turn Stop re-entry. Card 019 has since landed the lossless lifecycle feed,
direct Stop-reentry recorder, negative counterexamples, and panic-safe cleanup.
The repaired oracle is credential-free evidence until one exact provider turn
exercises the complete sequence.

## Goal

Repair the probe's platform-specific identity selection, then use the one
authorized Claude Code `2.1.251` turn with exact model `claude-haiku-4-5` on
the selected `linux-x86_64` host to test the repaired watcher oracle. Publish
only the exact proved route point after the full oracle passes. Otherwise
record one sanitized evidence stop and keep every watcher claim withheld.

## Execution Plan

### Batch 7.1 — Exact Live Acceptance

- [x] execute ready card 020 from current pushed `main` through its single new
      committed manual worker handoff
- [x] repair and credential-free prove the per-platform digest selection,
      commit it, then validate the clean repair head and exact Linux identity
      before any provider request
- [x] consume no more than one provider turn; never fall back or rerun
- [x] return one evidence PR for either the proved claim or the honest stop

The live ordered recorder kept only `JoinedZero` for turn
`claude-code-headless:live-claude-code-watcher`. It did not record MCP
initialize, reserved tool discovery, watcher start, Stop re-entry,
same-session continuation, explicit wait or stop, or provider success. The
first watcher claim stays unpublished. The attempt is consumed. Do not infer a
watcher range, fall back to Darwin or another model, or run a second provider
attempt without fresh authorization.

## Acceptance Criteria

- [x] exact installed version, frozen `linux-x64` native digest, exact model,
      local subscription path, and one-turn budget are verified before contact
- [ ] the live trace proves reserved tool discovery, watcher start, active Stop
      block, same-session continuation, explicit wait or stop, joined zero
      state, clean provider terminal, and joined cleanup in order
- [ ] consumer watcher activity contains truthful started, running, and
      terminal observations without raw process or provider material
- [x] success publishes only the exact live-proved `2.1.251` and
      `claude-haiku-4-5` point; failure or ambiguity publishes no claim
- [x] the card, milestone, evidence log, indexes, and sole Next Task record the
      consumed attempt and actual outcome

## Stop Conditions

- installed Claude is not exact `2.1.251`, the host is not `linux-x86_64`, or
  its native digest differs from
  `fd5f10ff0eb58daec04900466b143ea98aab50abf208a422bc008eaec13f61f7`
- exact `claude-haiku-4-5` is unavailable, authentication needs setup or
  inspection, `ANTHROPIC_API_KEY` is present, or source state is dirty
- the bounded per-platform repair, credential-free validation, or unchanged
  lifecycle oracle is not green before provider contact
- the single provider turn fails, is inconclusive, or misses any required
  ordering fact
- review would require a second provider request, a probe change beyond the
  authorized platform repair, raw/private evidence, a wider version/model
  claim, or a new product or contract decision

## Batch Cards

- [020 Claude Code Watcher Live Acceptance](batch-cards/020-claude-code-watcher-live-acceptance.md)

## References

- [Contract 044 Observable Agent Activity And Disclosure](../../contracts/044-observable-agent-activity-and-disclosure.md)
- [Contract 059 Operation-Scoped Process Watchers](../../contracts/059-operation-scoped-process-watchers.md)
- [Contract 060 Operation-Scoped Watcher HTTP Bridge](../../contracts/060-operation-scoped-watcher-http-bridge.md)
- [Card 011 Live Stop Review](../../logs/2026-08-30-g05-003-card-011-live-stop-review.md)
- [Card 019 Watcher Proof Repair](../../logs/2026-08-30-g05-006-card-019-watcher-proof-repair.md)
- [Card 020 Linux Envelope](../../logs/2026-08-31-g05-007-card-020-linux-envelope.md)
- [Card 020 Linux Live Stop](../../logs/2026-08-31-g05-007-card-020-linux-live-stop.md)
