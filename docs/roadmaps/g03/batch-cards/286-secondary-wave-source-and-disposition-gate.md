# 286 Secondary Wave Source And Disposition Gate

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../097-harness-route-expansion-intake-and-watchlist.md`
Depends on: Research 143; g03.086-g03.092; cards 260-261

## Goal

Refresh secondary candidates and decide which are ready for route evidence, which need a new contract/spec, and which should close as watchlist or negative evidence.

## Scope

Recheck OpenHands Agent Server, Kiro CLI, Aider, Deep Agents, Crush, Continue, MiMo Code, Kilo Code, Roo Code, and registry-only leads. Record source identity, machine-facing surface, topology, authority, cleanup, maturity, overlap, and the smallest safe first operation.

## Out Of Scope

new packages, implementation, live work, provider login, and automatic promotion from the primary wave

## Acceptance Criteria

- [x] Every candidate has add/defer/reject/revisit disposition.
- [x] OpenHands, Kiro, Aider, and Deep Agents have explicit route candidates or blockers.
- [x] Watchlist entries do not appear in the production route matrix.
- [x] New contract/spec requirements are named before implementation.

## Validation

`effigy qa:northstar` and read-only source/route comparison.

## Stop Conditions

Stop if a candidate has no maintained machine-facing surface or only duplicates an existing route without information gain.

## Auto-Continuation

Continue to card 287. Card 303 may close the watchlist independently.

## Evidence

Research 153; `docs/logs/2026-08-19-secondary-wave-source-and-disposition.md`.
`effigy qa:northstar` passed. No production claim. No new provider-neutral contract.
