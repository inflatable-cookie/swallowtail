# 026 Portable Activity Key And Cross-Operation Isolation

Status: completed
Owner: Tom
Created: 2026-08-04
Depends on: g03.025
Vision tags: observable activity, consumer integration, identity isolation
Contract refs: 002, 009, 044
Planning state: cards 068-069 completed

## Problem

T3 Code reproduced durable assistant-message loss when two threads reused one
provider-backed message id. Swallowtail already scopes activity semantically by
runtime operation, but exposes no typed composite key and carries no
cross-operation collision corpus.

## Goals

- [x] expose one provider-neutral `ActivityKey`
- [x] keep provider and activity references explicitly operation-local
- [x] prove repeated Cursor provider and fallback ids remain isolated
- [x] make consumer persistence guidance use the typed composite key

## Execution Plan

### Batch 26.1 — Portable Identity

- [x] Execute card 068.
- [x] extend Contracts 009 and 044
- [x] add the public composite key and observation accessor
- [x] prove cross-operation distinction and redaction

### Batch 26.2 — Cursor And Consumer Acceptance

- [x] Execute card 069 after the runtime surface settles.
- [x] freeze explicit and absent-message-id reuse across Cursor turns
- [x] reconcile public examples and consumer posture
- [x] run focused and extracted-package validation

## Boundaries

- no provider id rewriting or global provider identity claim
- no transcript database, migration, deduplication, or consumer edit
- no new message, thread, session, or route vocabulary
- no authenticated Cursor work or installed harness probe
- no generation rollover

## Acceptance Criteria

- [x] `ActivityKey` contains the exact operation owner and activity id
- [x] equal local/provider ids under distinct operations produce distinct keys
- [x] default key formatting reveals no identity value
- [x] Cursor explicit and fallback reuse cases pass deterministically
- [x] guides forbid standalone activity or provider-reference persistence keys
- [x] focused runtime/Cursor and affected-package validation pass

## Next Planning Checkpoint

The sole Next Task has returned to the g03 evidence gate.
