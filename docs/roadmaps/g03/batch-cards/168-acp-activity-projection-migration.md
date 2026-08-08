# 168 ACP Activity Projection Migration

Status: cancelled
Closeout: 2026-08-08

## Disposition

Cancelled by the card-167 operator decision: the ACP projections stay
adapter-local, so there is no shared projector to migrate onto. The five
projections keep their realized behavior; the recorded boundary in card 167
governs.
Owner: Tom
Created: 2026-08-08
Milestone: `../054-remaining-duplication-tranches.md`
Depends on: card 167

## Goal

Migrate the five ACP event-to-activity projections onto the shared projector
core pinned by card 167.

## Scope

1. Build the shared projector from the card-167 contract using the card-158
   projector scaffold.
2. Migrate the five projections (claude-agent, gemini, cursor, kimi, and
   peers) with identical activity output.
3. Keep provider-specific event decoding and corpus fixtures adapter-local.

## Out Of Scope

- activity vocabulary or public API changes
- behavior changes

## Acceptance

- [ ] all five projections share the pinned contract and projector
- [ ] activity corpora pass unchanged
- [ ] non-fitting projections are recorded with reasons

## Stop Conditions

- stop if any projected activity changes

## Auto-Continuation

Yes, to the g03 evidence gate after acceptance.

## Validation

- focused validation for every touched adapter
- `effigy package:api`
- `effigy qa:routes`
