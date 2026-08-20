# 049 Cline Clippy Result Large Err Proof

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../017-cline-stable-clippy-result-large-err.md`
Depends on: card 048

## Goal

Prove Clippy 1.98-style `-D warnings` on the Cline package no longer
reports `result_large_err`, and close the papercut.

## Scope

1. Run
   `cargo clippy -p swallowtail-adapter-cline --all-targets --all-features -- -D warnings`.
2. Mark
   `Stable clippy result_large_err on unchanged Cline driver` closed in
   `PAPERCUTS.md` with the PR SHA once known, or leave the close note for
   merge closeout if the SHA is the merge commit.
3. Do not restack PR 13 in this lane.

## Out Of Scope

- workspace-wide clippy
- DeepSeek
- other open papercuts

## Acceptance Criteria

- [ ] the named clippy command exits 0
- [ ] the Cline papercut is marked closed, or the PR says the orchestrator
      will close it on merge
- [ ] `public-api-0.3.3` is unchanged

## Validation

- `cargo clippy -p swallowtail-adapter-cline --all-targets --all-features -- -D warnings`
- `effigy validate:focused swallowtail-adapter-cline`
- `git diff --check`

## Auto-Continuation

No. After this PR lands, restack g04.016 / PR 13. Do not start Claude
Agent ACP or llama.cpp.

## Stop Conditions

- Stop if clippy still fails on `start_session`.
- Stop if a workspace-wide allow is about to land.
