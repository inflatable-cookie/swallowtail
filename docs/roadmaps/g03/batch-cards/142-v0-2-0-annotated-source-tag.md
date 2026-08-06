# 142 v0.2.0 Annotated Source Tag

Status: completed
Owner: Tom
Created: 2026-08-06
Milestone: `../046-v0-2-0-muse-and-rust-floor-source-release.md`
Depends on: card 141

## Goal

After separate exact authorization, create and push one annotated `v0.2.0`
source tag at the exact CI-green release commit.

## Scope

1. Reconfirm clean local and remote candidate identity and tag absence.
2. Preview the exact Effigy execution mutation.
3. Create and push annotated `v0.2.0` without a GitHub Release or registry
   publication.
4. Verify local and remote annotated-tag identity and close release evidence.

## Acceptance

- [x] local and remote peeled tag resolve to the exact green release commit
- [x] the tag annotation is `v0.2.0`
- [x] `v0.1.0` and `v0.1.1` remain unchanged
- [x] no crates.io, GitHub Release, binary, consumer, or provider mutation runs

## Stop Conditions

- stop without a separate exact operator authorization
- stop if the tag exists, candidate identity drifts, or execution plans any
  excluded side effect
- never move or recreate a failed or partially published tag

## Auto-Continuation

No. Stop after exact tag and closeout evidence.

## Completion Evidence

- annotated tag object: `643373ccb794c854a594297d823972dc3621fd3c`
- peeled release commit: `0104b8948ad141f5c42ad752127203b9b1d72db5`
- annotation: `v0.2.0`
- local and remote tag identities match
- GitHub CI run `31129147745`, dispatched against immutable ref `v0.2.0`,
  completed successfully with all five jobs at the peeled release commit
- `v0.1.0` remains at tag object
  `630d33a0d1ff285d20787ee038147dc3493f8b88`; `v0.1.1` remains at
  `d7cb439ef3b6808013950d209c2ffcf7930ec81a`
- Effigy execution remained fail-closed against stale prepared state; the
  repository runbook's explicit annotated-tag fallback created only the tag
- no crates.io publication, GitHub Release, binary, consumer, or authenticated
  provider work ran
