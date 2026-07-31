# 007 Codex Operation-Local Child Activity Ownership

Status: completed
Owner: Tom
Created: 2026-07-31
Depends on: g03.004
Vision tags: consumer stability, observable child work, exact ownership
Contract refs: 044, 045
Planning state: card 019 completed; Cursor card 012 resumed

## Problem

Nucleus proved the representation-aware callback path against Codex app-server
`0.146.0`: a typed answer resumed the exact live turn and reached a successful
`spawnAgent` call. The first child-owned activity envelope then failed because
Swallowtail admitted only the root provider thread.

Codex had already established the exact child thread through qualified spawn
topology. Rejecting that child hides supported activity; accepting arbitrary
thread ids would weaken session isolation.

## Contract Delta

Contracts 044 and 045 now permit a bounded operation-local activity ownership
set containing the root and exact children established earlier by trusted
topology evidence. Admission changes activity attribution only. Root turn,
terminal, callback, provider-request, and provider-session checks remain
unchanged. Termination clears the set.

## Goal

Admit and attribute exact child-owned Codex activity without granting child ids
session authority or carrying them across operations.

## Execution

- [x] Execute card 019.
- [x] admit bounded child ids only after successful spawn topology projection
- [x] attribute child-envelope activity to the admitted child
- [x] keep root output, terminal, turn, callback, and provider-request checks
  root-owned
- [x] reject unknown and cross-operation child ids
- [x] clear child admission on every terminal path
- [x] freeze the real child-owned envelope shape in the Codex corpus
- [x] restore Cursor card 012 as the sole next task

## Boundaries

- no consumer parsing or consumer repository edit
- no arbitrary child discovery from labels, prose, or ordinary activity
- no imported, persisted, or cross-session child authority
- no invented parentage
- no callback, terminal, root output, or provider-session widening
- no live provider or authentication effect

## Acceptance

- [x] root-owned activity remains accepted
- [x] an established child owns and emits its activity
- [x] an unknown child fails with the existing session mismatch class
- [x] another operation cannot reuse an admitted child id
- [x] terminal cleanup removes all admitted child ids
- [x] focused and affected-package Codex validation pass

## Next

After closeout, resume roadmap g03.005 at Cursor card 012. This compatibility
interruption does not alter the Cursor or Antigravity sequence.
