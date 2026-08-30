# g05.006 Watcher Proof Repair

Status: complete
Owner: Tom
Created: 2026-08-30
Updated: 2026-08-30
Depends on: stopped g05.003 card 011; Contracts 044, 059, and 060
Vision tags: process watchers, consumer activity, review oracle
Planning state: merged through PR 126 at `c8691e84`; no live provider authorization; claims unpublished

## Problem

Card 011 consumed one authorized Claude turn without reaching a host watcher.
Its branch-only prototype also has three independent defects: it cannot prove
Stop re-entry, it publishes terminal-only watcher activity by duplicating the
runtime projector, and failed live assertions bypass temporary-workspace
cleanup. Merging that branch would preserve the wrong proof and consumer path.

## Goal

Salvage the credential-free value from prototype head `49f2692f` into one
reviewable repair based on current `main`. Make watcher lifecycle activity
lossless and make a future live selector capable of proving the exact Stop
counterexample. Do not run Claude or publish a watcher capability.

## Execution Plan

### Batch 6.1 — Credential-Free Repair

- [x] execute ready card 019 from current pushed `main`
- [x] retain safe deterministic fixtures and binding work from `49f2692f`
      selectively; do not merge or cherry-pick the whole prototype commit
- [x] return one PR with no provider contact, claim, or live authorization

## Acceptance Criteria

- [x] watcher accepted, running, and terminal transitions reach the existing
      ordered turn event stream independently of provider output cadence
- [x] the existing provider-neutral watcher activity projector owns lifecycle
      mapping; joined cleanup emits no duplicate completion
- [x] a future live probe can distinguish exact MCP discovery and invocation,
      active Stop blocking, same-session continuation, explicit wait or stop,
      joined watcher cleanup, and provider terminal success
- [x] the proof recorder retains only bounded safe facts and no raw HTTP,
      provider, credential, endpoint, path, command, or output material
- [x] every temporary live-probe resource has cleanup established before any
      provider contact or assertion
- [x] deterministic fixtures cover the review-oracle counterexamples without
      contacting Claude
- [x] route capability, matrix, guide, and version-range claims stay withheld

## Stop Conditions

- lossless activity needs a new product or consumer policy choice beyond
  Contracts 044 and 059
- direct Stop attribution requires raw provider payload retention or private
  bridge material to cross the test boundary
- the repair needs ambient Claude settings, project mutation, provider login,
  a second provider turn, or a wider watcher version segment
- salvaging the prototype would preserve terminal-only activity or its
  terminal local rejection as successful same-turn evidence
- a public generic HTTP/MCP, process, event-bus, or consumer facade is required

## Batch Cards

- [019 Watcher Proof Oracle And Activity Delivery Repair](batch-cards/019-watcher-proof-oracle-and-activity-delivery-repair.md)

## References

- [Contract 044 Observable Agent Activity And Disclosure](../../contracts/044-observable-agent-activity-and-disclosure.md)
- [Contract 059 Operation-Scoped Process Watchers](../../contracts/059-operation-scoped-process-watchers.md)
- [Contract 060 Operation-Scoped Watcher HTTP Bridge](../../contracts/060-operation-scoped-watcher-http-bridge.md)
- [Card 011 Live Stop Review](../../logs/2026-08-30-g05-003-card-011-live-stop-review.md)
