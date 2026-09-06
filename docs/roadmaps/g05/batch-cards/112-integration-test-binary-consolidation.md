# 112 Integration Test Binary Consolidation

Status: planned; ready now; touches no release surface
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Milestone: `../034-release-lane-simplification.md`
Depends on: the g05.034 measured causes table

## Goal

Cut link time, which dominates the test suite: 370 integration test files each build a separate binary.

## Scope

1. Per crate, move `tests/*.rs` into one or a few `tests/<group>/main.rs` binaries with `mod` files, preserving every test name and module path in the output. Start with the six largest: claude-agent (32), kimi (30), codex (28), testkit (24), openai (20), gemini (20). Then opencode, protocol-acp, pi, host-local, anthropic, alibaba-model-studio.
2. Keep process-spawning tests in their own binary per crate so the `ci-process` nextest profile still selects them.
3. Record nextest binary count and wall clock before and after per crate, locally under the card 093 load method.
4. No `src` change; no test deleted or renamed.

## Acceptance Criteria

- [ ] binary count per crate reduced to a handful; test count identical
- [ ] `ci` and `ci-process` profiles still select the same tests
- [ ] before/after timings recorded

## Validation

- `effigy validate:focused <crate>` per touched crate (up to four per run)
- `cargo nextest list --workspace` count before and after
- `git diff --check`

## Review Oracle

See the g05.034 manifest row.

## Auto-Continuation

No. Stop for exact-head review.
