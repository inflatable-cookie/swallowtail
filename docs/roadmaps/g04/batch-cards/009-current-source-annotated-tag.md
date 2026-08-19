# 009 Current Source Annotated Tag

Status: completed
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

- [x] the tag peels to the green commit locally and remotely
- [x] `v0.3.2` remains unchanged
- [x] no readiness-facade types are in the tagged tree

## Validation

Tag peel, canonical tag-triggered CI if configured, docs currentness.

## Auto-Continuation

No. Compile later implementation roadmaps only after this tag exists.

## Evidence

- operator authorized the annotated tag at the CI-green candidate
- annotated tag `v0.3.3` object `ca30b367e51a70c56b0998b27e7e660ba7145657`
  peels locally and remotely to
  `51d186208e75dca4c04f077dd7179ec3c2fafae9`
- `v0.3.2` remains `702f355631bb6fe8fe6cb098f48887df8ef8ca43` →
  `a859d56b47b1bc2975df7d0516ca96fd8e485b35`
- no GitHub Release or crates.io publication
- no Spec 011 facade types in the tagged tree
- tag-triggered CI run
  https://github.com/inflatable-cookie/swallowtail/actions/runs/32309276223
  passed all five jobs at the same SHA after one Stable-job rerun of a
  DeepSeek stream-cancellation flake
- this closeout commit is not the tag identity
