# 013 Canonical Source Provenance And Final Candidate

Status: completed
Owner: Tom
Created: 2026-07-25
Depends on: g02.012
Contract refs: 036-037
Planning state: cards 037-039 complete

## Problem

The provider-wide package proof was reproducible, but its candidate builder
created a parentless synthetic source commit. That shape could not join
canonical `main` history without an orphan tag or a new unverified source
commit.

## Goals

- [x] Promote clean canonical-history source provenance into Contract 036 and
      release topology architecture.
- [x] Separate dirty-worktree package checks from clean-HEAD final candidate
      preparation.
- [x] Materialize the accepted source tree as one normal local `main` commit.
- [x] Replace the candidate and rerun provider-wide and consumer evidence.
- [x] Return one exact final candidate to the external publication decision.

## Execution Plan

### Batch 13.1 — Provenance Gate

- [x] Execute card 037.

### Batch 13.2 — Canonical Candidate

- [x] Execute card 038 after card 037 passes.

### Batch 13.3 — Packaged Acceptance

- [x] Execute card 039 after card 038 passes.

## Acceptance Criteria

- [x] local package verification retains deterministic working-tree coverage
- [x] final candidate preparation rejects dirty source state
- [x] the final source bundle preserves a non-root commit in local `main`
      history
- [x] package and file-list checksums reproduce from that exact bundle
- [x] all 22 packaged facade routes pass
- [x] Nucleus, Soundcheck, and packaged Codex proofs pass
- [x] the prior synthetic candidate remains immutable superseded evidence
- [x] registry, remote push, tag, release, workflow, and consumer state remain
      unchanged

## Decision Gate

After card 039, publication still requires the exact crates.io owner identity
and explicit approval of the bounded external mutation set.

## Closeout

The active candidate source and exact parent are recorded in
`release-candidates/0.1.0/candidate.env`. Its complete normal-history bundle
reproduces all 23 packages and audited file lists. Packaged proof passes 20
suites across all 22 production routes, Nucleus, Soundcheck, and the full
packaged Codex suite.

The former parentless candidate is retained at
`.effigy/release-candidates/superseded/0.1.0-73c7f5b5b561/`. No remote,
registry, tag, release, workflow, credential, provider, or consumer mutation
occurred.
