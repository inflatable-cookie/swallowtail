# 049 Cline Clippy Result Large Err Proof

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../017-cline-stable-clippy-result-large-err.md`
Depends on: card 048

## Goal

Prove Clippy 1.98-style workspace `-D warnings` no longer reports
`result_large_err` on ACP `start_session` helpers, and close the papercut.

## Scope

1. Run
   `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`.
2. Mark the Cline/ACP `result_large_err` papercut closed, or leave the
   merge SHA for orchestrator closeout.
3. Do not restack PR 13 in this lane.

## Out Of Scope

- DeepSeek
- other open papercuts

## Acceptance Criteria

- [x] the named workspace clippy command exits 0
- [x] the papercut is marked closed, or the PR says the orchestrator
      will close it on merge
- [x] `public-api-0.3.3` is unchanged

## Validation

- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `git diff --check`

## Auto-Continuation

No. After this PR lands, restack g04.016 / PR 13. Do not start Claude
Agent ACP addable or llama.cpp.

## Stop Conditions

- Stop if clippy still fails on any `start_session` Err pair.
- Stop if a workspace-wide allow is about to land.
