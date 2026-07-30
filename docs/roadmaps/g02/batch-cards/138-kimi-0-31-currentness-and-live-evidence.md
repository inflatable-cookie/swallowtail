# 138 Kimi 0.31 Currentness And Live Evidence

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../041-kimi-code-0-31-range-and-live-proof.md`

## Goal

Determine which Kimi Code `0.30.0` and `0.31.0` routes may become guaranteed.

## Scope

1. Compare exact signed tags and selected source.
2. Identify the installed executable and distribution.
3. Run bounded authenticated headless and ACP probes.
4. Keep raw credentials, payloads, and session ids out of repository evidence.
5. Select exact range changes and exclusions.

## Acceptance Criteria

- [x] exact commits, trees, and selected deltas are recorded
- [x] installed native Kimi Code identity is distinct from Python `kimi-cli`
- [x] headless stream-JSON succeeds on `0.31.0`
- [x] ACP initialize, session creation, prompt, and terminal response succeed
- [x] local-server source drift remains visible
- [x] contract fit and implementation selection are explicit

## Validation

- exact installed-version probe
- one fixed-output headless prompt
- one fixed-output ACP prompt
- tagged-source blob and diff comparison

## Stop Conditions

- Stop if authentication requires login mutation.
- Stop if a selected route emits an unbounded or incompatible protocol.
- Do not widen another route from shared executable semver alone.

## Auto-Continuation

Yes. Continue to card 139.

## Evidence

- [Research 068](../../../research/068-kimi-code-0-31-currentness-and-live-evidence.md)
- installed Kimi Code `0.31.0` version probe passed
- authenticated headless and ACP fixed-output probes passed
- no callback, tool, workspace write, destructive action, or local-server
  launch occurred

