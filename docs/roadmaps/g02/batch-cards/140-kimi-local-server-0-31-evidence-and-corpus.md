# 140 Kimi Local-Server 0.31 Evidence And Corpus

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../042-kimi-code-0-31-local-server-guarantee.md`

## Goal

Resolve the `0.31.0` broadcaster gate with exact source, deterministic wire,
and bounded installed-server evidence.

## Scope

1. Compare selected local-server source across `0.29.2`, `0.30.0`, and
   `0.31.0`.
2. Freeze the richer subagent status payload.
3. Prove that portable activity continues to ignore provider status content.
4. Run one bounded foreground startup and authenticated read-only smoke.
5. Keep secrets and live payloads out of repository state.

## Acceptance Criteria

- [x] exact source delta and upstream test intent are recorded
- [x] deterministic corpus contains full subagent status
- [x] corpus decodes as progress without portable content
- [x] installed `0.31.0` health, bearer rejection, metadata, and catalogue pass
- [x] the foreground child is stopped and reaped

## Validation

- focused local-server corpus and activity tests
- bounded installed foreground smoke
- fixture provenance assertions

## Stop Conditions

- Stop if the wire event type or required control schema changed.
- Stop if live proof would require provider inference or account mutation.
- Do not expose the bearer token in logs or fixtures.

## Auto-Continuation

Yes. Continue to card 141.

## Evidence

- [Research 069](../../../research/069-kimi-code-0-31-local-server-qualification.md)
- exact signed-source and upstream broadcaster-test comparison
- bounded installed foreground smoke
- 18 focused local-server library tests pass
