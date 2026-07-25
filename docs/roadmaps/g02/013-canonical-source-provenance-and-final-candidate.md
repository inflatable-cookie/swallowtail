# 013 Canonical Source Provenance And Final Candidate

Status: active
Owner: Tom
Created: 2026-07-25
Depends on: g02.012
Contract refs: 036-037
Planning state: card 037 complete; card 038 active; card 039 ready

## Problem

The provider-wide package proof is reproducible, but its candidate builder
creates a parentless synthetic source commit. That shape cannot join canonical
`main` history without an orphan tag or a new unverified source commit.

## Goals

- [x] Promote clean canonical-history source provenance into Contract 036 and
      release topology architecture.
- [x] Separate dirty-worktree package checks from clean-HEAD final candidate
      preparation.
- [ ] Materialize the accepted source tree as one normal local `main` commit.
- [ ] Replace the candidate and rerun provider-wide and consumer evidence.
- [ ] Return one exact final candidate to the external publication decision.

## Execution Plan

### Batch 13.1 — Provenance Gate

- [x] Execute card 037.

### Batch 13.2 — Canonical Candidate

- [ ] Execute card 038 after card 037 passes.

### Batch 13.3 — Packaged Acceptance

- [ ] Execute card 039 after card 038 passes.

## Acceptance Criteria

- [ ] local package verification retains deterministic working-tree coverage
- [ ] final candidate preparation rejects dirty source state
- [ ] the final source bundle preserves a non-root commit in local `main`
      history
- [ ] package and file-list checksums reproduce from that exact bundle
- [ ] all 22 packaged facade routes pass
- [ ] Nucleus, Soundcheck, and packaged Codex proofs pass
- [ ] the prior synthetic candidate remains immutable superseded evidence
- [ ] registry, remote push, tag, release, workflow, and consumer state remain
      unchanged

## Decision Gate

After card 039, publication still requires the exact crates.io owner identity
and explicit approval of the bounded external mutation set.
