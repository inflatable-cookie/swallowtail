# Codex And OpenCode Module Decomposition

Date: 2026-07-30

Card 152 removes all remaining Codex and OpenCode error-level structural
findings.

## Changes

- Codex app-server, active-turn, RPC, session, and scripted test-server files
  are split by existing protocol and lifecycle responsibilities.
- OpenCode prepared operations, protocol, structured-run, interactive-role,
  event, protocol-test, and HTTP fixture-server files are split by operation
  family.
- Trait implementations and test names remain in the same module namespace.
  No version corpus, fixture, capability, callback, lifecycle, or cleanup rule
  changed.
- OpenCode's exported declarations remain in their original file while large
  preparation bodies move behind private inner methods. This preserves the
  repository's path-sensitive declaration baseline.

## Evidence

- Codex package: 129 tests passed
- OpenCode package: 82 tests passed, including a full rerun after restoring
  declaration paths
- focused warnings-denied clippy: passed
- 24-crate public-API declaration baseline: passed
- doctor: 146 findings, 129 warnings, 17 high errors, no Codex or OpenCode
  error finding

## Next

Card 153 removes the ten runtime, Claude Agent, Gemini, and Kimi high findings.
