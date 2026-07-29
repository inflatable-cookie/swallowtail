# 029 Pi RPC Session Continuity

Status: paused
Owner: Tom
Created: 2026-07-28
Depends on: g02.028
Vision tags: exact lifecycle, installed harnesses, persistent sessions
Contract refs: 003, 005, 009, 012, 017, 037-038
Planning state: card 096 complete; cards 097-098 paused

## Problem

Pi RPC is the last session-continuity route classified ready under existing
shared contracts. Exact `0.80.10` exposes persisted-session switching plus
ordered message and append-order entry reads. The current Swallowtail Pi
profile deliberately uses `--no-session` and prohibits provider state.

Adding load and resume therefore requires a separate persistent profile. It
cannot silently widen the existing ephemeral session or structured-run paths.

## Goals

- [x] Revalidate the current maintained Pi release and exact RPC continuity
      surface.
- [x] Specify load replay, replay-free resume, binding, cancellation, and
      cleanup phases before production changes.
- [ ] Add a separate durable-provider-session prepared profile.
- [ ] Implement public prepared load and resume without changing ephemeral
      Pi behavior.
- [ ] Convert exactly two matrix cells after packaged evidence exists.
- [ ] Select the next evidence-ranked feature family from the retained matrix.

## Non-Goals

- treating a session path as sufficient attachment authority
- exposing transcript paths, entry ids, messages, or raw RPC payloads through
  stable diagnostics
- making process exit a provider-native close claim
- changing Pi's existing `--no-session` structured-run or ephemeral
  interactive paths
- adding archive, restore, delete, containment, endpoint, model, credential,
  or provider fallback
- consumer edits, live authentication, publication, or release mutation

## Execution Plan

### Batch 29.1 — Currentness And Corpus Gate

- [x] Execute card 096.
- [x] Revalidate the maintained release and exact persistence commands.
- [x] Stop before a false positive corpus when resource binding fails.
- [x] Confirm existing Contracts 009, 017, and 038 remain sufficient.

### Batch 29.2 — Persistent Profile And Operations

- [ ] Execute card 097 only when card 096 leaves no shared-contract gap.
- [ ] Add separate prepared load and resume operations.
- [ ] Preserve exact version, host, resource, access, model, and provider-state
      binding.

### Batch 29.3 — Package Closeout And Matrix Continuation

- [ ] Execute card 098.
- [ ] Prove both cells from extracted packages and re-audit continuity counts.
- [ ] Select the next feature family without rolling to g03.

## Acceptance Criteria

- [ ] load returns bounded ordered replay before readiness
- [ ] resume exposes no replay phase
- [ ] arbitrary paths or copied ids cannot mint attachment authority
- [ ] ephemeral and persistent Pi profiles remain separate
- [ ] cancellation, disconnect, overflow, and close leave no detached work
- [ ] provider state survives runtime close without a native-close claim
- [ ] every changed matrix cell maps to a public packaged prepared path

## Decision Gates

- Stop if the maintained RPC surface cannot prove replay completeness before
  readiness.
- Stop if a persisted session cannot be rebound to the exact selected host,
  resource, model, access, and provider-state posture.
- Promote a narrow contract delta before implementation if exact Pi evidence
  contradicts Contracts 009, 017, or 038.
- Do not infer a native close, archive, restore, or delete operation.

## Next Planning Checkpoint

Research 053 records the upstream public-interface gate. Cards 097-098 remain
paused until Pi can attach a session with the exact caller-bound cwd and
corroborate it. Roadmap 030 returns to the matrix. Stay in g02.
