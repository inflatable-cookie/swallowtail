# 152 Codex And OpenCode Module Decomposition

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../045-error-level-structural-health-stabilization.md`

## Goal

Remove the remaining five Codex and seven OpenCode error-level findings while
preserving their qualified-range and operation behavior.

## Scope

1. Split Codex app-server, turn-state, RPC, session, and app-server test
   support by existing private responsibility.
2. Split OpenCode prepared operations, protocol, run, roles, events, protocol
   tests, and HTTP support by exact operation and lifecycle concern.
3. Keep range selection, callbacks, activity, and cleanup semantics unchanged.

## Acceptance Criteria

- [x] Codex has no error-level structural finding
- [x] OpenCode has no error-level structural finding
- [x] all exact version and behavior corpora remain unchanged
- [x] public declaration hashes remain unchanged
- [x] focused adapter tests and warnings-denied clippy pass

## Validation

- `cargo test -p swallowtail-adapter-codex`
- `cargo test -p swallowtail-adapter-opencode`
- warnings-denied clippy for both adapters
- public-API and doctor delta checks

## Stop Conditions

- Stop if an extraction would move provider invariants into shared code.
- Stop if qualified behavior, correlation, or cleanup changes.
- Do not broaden validation beyond the two packages until closeout.

## Auto-Continuation

Yes. Continue to card 153 after focused validation.

## Evidence

- Five Codex high files are split across app-server roles, provider requests,
  RPC pumping, interactive-session handling, and scripted test-process
  concerns.
- Seven OpenCode high files are split across prepared operations, HTTP
  protocol families, structured-run roles, interactive roles, event parsing,
  protocol tests, and fixture-server handling.
- OpenCode public declarations remain in their original source file. Extracted
  preparation logic uses private inner methods so the path-sensitive public-API
  baseline remains exact.
- Codex passed 129 package tests. OpenCode passed 82 package tests, including a
  complete rerun after the API-path correction.
- Warnings-denied clippy passed for both adapters. The 24-crate public-API
  declaration baseline passed.
- Doctor now reports 146 findings: 129 warnings, 17 high errors, and no Codex
  or OpenCode error finding.
