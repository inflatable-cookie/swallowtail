# 121 Claude SDK First-Turn Rejection Terminal State

Status: ready
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: card 119 (shared sidecar; serial after card 120)

## Goal

Stop the poisoned-retry misreport: after a first-turn rejection the session has already consumed `system/init`, so a consumer retry on the same session fails as `init_missing`, which misnames the cause. Make the session explicitly terminal after a first-turn rejection and make any further turn fail with a code that says so.

## Scope

1. Sidecar: after `query_rejected` / `supported_model_rejected` (and every other first-turn rejection), mark the session terminal; subsequent `turn` commands fail `session_rejected_terminal` carrying the original code; close still works and reports honestly.
2. Rust: the handle reflects terminal state; a consumer retry returns the typed code instead of `init_missing`.
3. Fixtures: rejection then retry; rejection then close; the existing `init_missing` fixture unchanged for the genuinely missing case.
4. Guide (retry means a new session), changelog `[Unreleased]`, additive baseline.

## Out Of Scope

Automatic re-open or retry inside Swallowtail; any live turn.

## Acceptance Criteria

- [ ] a retry after first-turn rejection fails with the terminal code naming the original rejection
- [ ] `init_missing` is reserved for a genuinely missing init
- [ ] fixtures green; guide, changelog, baseline; one PR

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: a failed session never reports a different failure on retry than the one that killed it. Smallest counterexample: `init_missing` after a recorded rejection.

## Stop Conditions

Terminal state cannot be represented without an SDK API the fixture cannot model (return to Chatterbox).

## Auto-Continuation

No. Stop for exact-head review. No live tier; a later live gate needs fixtures first and separate operator authority; never read or replay the Desktop candidate.

## Result

Implemented on the Card121 review branch. The sidecar now records every
first-turn rejection as terminal state; a later query fails with bounded
`session_rejected_terminal` correlation to the original sidecar code, without
another SDK input or provider-work attempt. Explicit close remains available,
and genuine first-message shape failures retain `init_missing`.

Provider-free sidecar and Rust-host fixtures cover rejection-then-retry and
rejection-then-close, including ordered cleanup, no SDK-input replay, and no
new credential acquisition. Card119 model-qualification evidence and its
fail-soft observer behavior, plus Card120 loaded-module verification, remain
green. The API baseline is unchanged.

Focused validation (427/427), affected-package verification, semantic API,
Northstar, formatting, diff, and Rust-quality closeout checks pass. The
affected-package closeout records the repository's pre-existing unused
`swallowtail-protocol-openai-chat` patch warning. No live SDK/provider,
credentials/auth/config, Desktop candidate, Card124 work, release, tag, merge,
or review workspace action was taken. Exact-head review remains the stop.
