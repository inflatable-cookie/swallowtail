# 028 Session Continuity Feature Closure

Status: active
Owner: Tom
Created: 2026-07-28
Depends on: g02.027
Vision tags: exact lifecycle, provider breadth, persistent sessions
Contract refs: 003, 005, 009, 012, 015, 017, 038
Planning state: cards 092-093 completed; card 094 ready; card 095 planned

## Problem

The current matrix has 58 session-continuity `No` cells:

- 20 load-session gaps
- 18 resume-session gaps
- 20 native-session-close gaps

These columns describe different provider effects. A route may replay history,
reattach without replay, close active work while retaining history, expose
only local teardown, or have no provider session at all. One generic resume or
close flag would erase those differences.

## Goals

- [x] Revalidate every starting `No` against its exact route and maintained
      version posture.
- [x] Detect false negatives against realized prepared paths.
- [x] Keep load replay, replay-free resume, native close, local teardown,
      archive, deletion, and direct private continuation separate.
- [x] Confirm no missing shared contract is needed by the selected tranche.
- [ ] Implement a representative tranche across materially different
      lifecycle shapes.
- [ ] Re-audit all 58 starting cells and retain honest absence.

## Non-Goals

- treating consumer-local thread restoration as provider load or resume
- treating a new session plus copied prompt history as native load
- treating process exit or connection drop as confirmed provider-native close
- treating close as archive, deletion, secure erasure, or durable export
- adding implicit provider, route, session, endpoint, credential, or version
  fallback
- consumer edits, live authentication, publication, or release mutation

## Execution Plan

### Batch 28.1 — Exact Currentness Audit

- [x] Execute card 092.
- [x] Classify all 58 starting cells by exact route, operation shape, replay
      semantics, close effect, and authority.
- [x] Rank conversions by consumer value and architectural information.

### Batch 28.2 — Contract And Corpus Gate

- [x] Execute card 093 only after the audit selects exact routes.
- [x] Promote narrow shared distinctions only where Contracts 009, 017, and
      038 do not already settle them.
- [x] Freeze deterministic route corpora before production changes.

### Batch 28.3 — Representative Implementation

- [ ] Execute card 094 only for contract-ready routes.
- [ ] Keep replay, attachment, cancellation, provider-state, and cleanup truth
      exact.

### Batch 28.4 — Matrix Closeout

- [ ] Execute card 095.
- [ ] Prove package truth and select provider retention next unless the audit
      changes the evidence-ranked runway.

## Acceptance Criteria

- [x] all 58 starting cells are classified exactly once
- [x] load and resume remain distinct
- [x] native close never implies archive or deletion
- [ ] every changed cell maps to a public prepared path
- [ ] version, access, topology, and provider-state authority remain exact
- [ ] machine counts and classification drift fail deterministically

## Decision Gates

- Ask the operator if selecting among equally useful route tranches would set
  product priority.
- Stop if maintained evidence cannot distinguish replay, reattachment, or
  provider-native close from local reconstruction or teardown.
- Do not borrow lifecycle capability from a sibling route in a composite
  solution.
- Keep honest `No` when a route has no provider-owned persistent session.

## Next Planning Checkpoint

Card 094 implements Codex app-server load, Claude Agent ACP load and resume,
and OpenCode HTTP load and resume through their existing prepared identities.
The exact card 093 corpora and Contracts 009, 017, and 038 bound the work. Stay
in g02.
