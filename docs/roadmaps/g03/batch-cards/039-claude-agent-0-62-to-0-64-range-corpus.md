# 039 Claude Agent 0.62 To 0.64 Range Corpus

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../015-claude-agent-0-64-standalone-range-maintenance.md`
Depends on: card 038

## Goal

Freeze exact Claude Agent package and selected behavior evidence while keeping
the production ceiling at `0.61.0`.

## Scope

1. Record exact wrapper, ACP SDK, Agent SDK, tag, integrity, and source digests.
2. Prove `0.62.0` is selected-source identical to `0.61.0`.
3. Freeze `0.63.0` tool/subagent correlation and `0.64.0` steering/form deltas.
4. Record current Anthropic subscription posture without changing access types.
5. Add deterministic corpus assertions before any claim movement.

## Acceptance Criteria

- [x] every release has exact artifact and source identity
- [x] selected deltas are classified without new portable authority
- [x] fixtures contain no secret, account, path, prompt, or provider payload
- [x] exact `0.61.0` remains the production ceiling
- [x] focused Claude and ACP corpus tests pass

## Evidence

- exact npm, tarball, package, and selected-source identities for
  `0.61.0..=0.64.0`
- `0.62.0` selected output matches `0.61.0`
- `0.63.0` and `0.64.0` retain separate private behavior milestones
- `effigy validate:focused swallowtail-protocol-acp
  swallowtail-adapter-claude-agent` — 157 passed

## Validation

- `effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-claude-agent`
- `git diff --check`
- no live provider work

## Auto-Continuation

Yes. Continue to card 040 when the corpus settles every behavior boundary.
