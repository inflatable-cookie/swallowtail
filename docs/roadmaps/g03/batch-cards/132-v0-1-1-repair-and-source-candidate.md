# 132 v0.1.1 Repair And Source Candidate

Status: completed
Owner: Tom
Created: 2026-08-06
Milestone: `../044-v0-1-1-source-patch-release.md`
Depends on: card 131

## Goal

Produce one complete `v0.1.1` source candidate with deterministic Kimi
detachment evidence and all local release gates passing.

## Scope

1. Make the Kimi fixture wait for its peer-close observation before asserting
   it, without changing production detachment semantics or extending operation
   deadlines.
2. Record the Anthropic and Kimi fixes in patch-release notes.
3. Deliberately refresh the dependency lock within the Rust floor and record
   every retain decision.
4. Use Effigy to prepare the workspace version and changelog, then synchronize
   the seven coupled internal requirements and lock entries exactly.
5. Rerun all 11 source-release gates after that bounded prepared-state drift
   and freeze one clean candidate commit.

## Acceptance

- [x] Kimi detachment remains cancellation-free and joins local observer work
- [x] the fixture's close observation is synchronized, not sleep-guessed
- [x] targeted Kimi validation and repeated contention-shaped proof pass
- [x] workspace version, internal dependency requirements, changelog, release
      notes, and source examples agree on `0.1.1`
- [x] all release gates and exact-revision source-consumer proof pass
- [x] no tag, registry, GitHub Release, consumer, or provider mutation runs

## Validation

- targeted Kimi detachment test under Rust 1.90
- `effigy validate:focused swallowtail-adapter-kimi`
- `effigy package:verify-affected swallowtail-adapter-kimi`
- canonical Effigy release simulation, status, and prepare plan
- complete configured release gates

## Stop Conditions

- stop rather than weakening Contract 049 detachment cleanup
- stop if synchronization needs production-only acknowledgement not present on
  the qualified wire
- stop on any release gate or dependency-floor failure

## Auto-Continuation

Yes. Continue to card 133 only after one clean release commit contains all
candidate evidence.

## Completion Evidence

- the Kimi fixture consumes and joins its server threads before reading the
  peer-close record; production WebSocket and detachment code is unchanged
- 40 repeated Rust 1.90 detachment runs pass; focused Kimi validation passes
  108 tests; extracted-package verification passes
- ten complete nextest runs pass 1,464 tests each with 11 live probes skipped
- dependency refresh advances `zerocopy` and `zerocopy-derive` from `0.8.55`
  to `0.8.56`; `agent-client-protocol-schema`, `async-tungstenite`, `base64`,
  `generic-array`, `matchit`, and `sha2` remain retained by the declared
  dependency ranges and Rust 1.90 resolver
- Effigy preparation and post-synchronization proof pass all 11 source-release
  gates; workspace and internal requirements are `0.1.1`
- the clean completion commit containing this evidence is card 133's exact
  candidate input
- no authenticated provider, consumer, tag, registry, or GitHub Release work
  ran
