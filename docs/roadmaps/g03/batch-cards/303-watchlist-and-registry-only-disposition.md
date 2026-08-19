# 303 Watchlist And Registry-Only Disposition

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../097-harness-route-expansion-intake-and-watchlist.md`
Depends on: Card 286; Research 153

## Goal

Close watchlist entries as add, defer, reject, or revisit decisions without creating adapter packages or production route rows.

## Scope

Cover Crush, Continue, MiMo Code, Kilo Code, Roo Code, Amp, Auggie, CodeBuddy, Cortex Code, Devin, Factory Droid, Junie, GLM Agent, and other registry-only leads. Record route surface, overlap, authority, source maturity, next evidence needed, and owner of the next decision.

## Out Of Scope

implementation, package creation, live provider work, and production matrix changes

## Acceptance Criteria

- [x] Every named watchlist candidate has a disposition.
- [x] Deferred candidates have a concrete revisit condition.
- [x] Duplicate/fork/UI-only candidates have explicit negative reasoning.
- [x] No watchlist candidate appears as a production route.

## Validation

`effigy qa:northstar` and documentation/index validation.

## Stop Conditions

Stop if the work turns into an implementation queue rather than a disposition record.

## Auto-Continuation

Close roadmap 087 or promote one explicitly selected candidate to a new roadmap after operator review.

## Evidence

Research 158; `docs/logs/2026-08-19-watchlist-and-registry-only-disposition.md`.
ACP registry `1.0.0` / 38 agents fetched 2026-08-19. Add: none. Reject
community wrappers and Agoragentic. Defer the rest with revisit conditions.
g03.087 was already complete; no candidate was promoted. Closed g03.097.
Counts stay 40 packages / 47 production routes. No install, login, or live
work. `effigy qa:northstar` and `effigy qa:docs` passed.
