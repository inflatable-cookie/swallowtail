# 009 Current Source Annotated Tag

Status: planned
Owner: Tom
Created: 2026-08-19
Milestone: `../003-current-source-tag-before-readiness.md`
Depends on: card 008; separate exact operator authorization

## Goal

Create and push one annotated immutable source tag for the green candidate.

## Scope

1. Annotated tag on the exact CI-green commit.
2. Confirm local and remote peel.
3. Record that later readiness implementation may now be compiled as ready.

## Out Of Scope

- moving or recreating the tag
- GitHub Release object
- registry publication
- starting facade implementation in this card

## Acceptance Criteria

- [ ] the tag peels to the green commit locally and remotely
- [ ] `v0.3.2` remains unchanged
- [ ] no readiness-facade types are in the tagged tree

## Validation

Tag peel, canonical tag-triggered CI if configured, docs currentness.

## Auto-Continuation

No. Compile later implementation roadmaps only after this tag exists.
