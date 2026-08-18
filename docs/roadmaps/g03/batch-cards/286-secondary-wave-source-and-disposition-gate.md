# 286 Secondary Wave Source And Disposition Gate

Status: planned
Owner: Tom
Created: 2026-08-18
Milestone: `../097-harness-route-expansion-intake-and-watchlist.md`
Depends on: Research 143; g03.086

## Goal

Refresh secondary candidates and decide which are ready for route evidence, which need a new contract/spec, and which should close as watchlist or negative evidence.

## Scope

Recheck OpenHands Agent Server, Kiro CLI, Aider, Deep Agents, Crush, Continue, MiMo Code, Kilo Code, Roo Code, and registry-only leads. Record source identity, machine-facing surface, topology, authority, cleanup, maturity, overlap, and the smallest safe first operation.

## Out Of Scope

new packages, implementation, live work, provider login, and automatic promotion from the primary wave

## Acceptance Criteria

- [ ] Every candidate has add/defer/reject/revisit disposition.
- [ ] OpenHands, Kiro, Aider, and Deep Agents have explicit route candidates or blockers.
- [ ] Watchlist entries do not appear in the production route matrix.
- [ ] New contract/spec requirements are named before implementation.

## Validation

`effigy qa:northstar` and read-only source/route comparison.

## Stop Conditions

Stop if a candidate has no maintained machine-facing surface or only duplicates an existing route without information gain.

## Auto-Continuation

Continue only to the first admitted secondary candidate; card 303 may close the watchlist independently.

## Evidence

Research 143; current route matrix; existing ACP, OpenCode, Pi, and remote transport contracts
