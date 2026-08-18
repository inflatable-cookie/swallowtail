# 303 Watchlist And Registry-Only Disposition

Status: planned
Owner: Tom
Created: 2026-08-18
Milestone: `../097-harness-route-expansion-intake-and-watchlist.md`
Depends on: Card 286; Research 143

## Goal

Close watchlist entries as add, defer, reject, or revisit decisions without creating adapter packages or production route rows.

## Scope

Cover Crush, Continue, MiMo Code, Kilo Code, Roo Code, Amp, Auggie, CodeBuddy, Cortex Code, Devin, Factory Droid, Junie, GLM Agent, and other registry-only leads. Record route surface, overlap, authority, source maturity, next evidence needed, and owner of the next decision.

## Out Of Scope

implementation, package creation, live provider work, and production matrix changes

## Acceptance Criteria

- [ ] Every named watchlist candidate has a disposition.
- [ ] Deferred candidates have a concrete revisit condition.
- [ ] Duplicate/fork/UI-only candidates have explicit negative reasoning.
- [ ] No watchlist candidate appears as a production route.

## Validation

`effigy qa:northstar` and documentation/index validation.

## Stop Conditions

Stop if the work turns into an implementation queue rather than a disposition record.

## Auto-Continuation

Close roadmap 087 or promote one explicitly selected candidate to a new roadmap after operator review.

## Evidence

Research 143; ACP latest registry; official candidate sources named there
