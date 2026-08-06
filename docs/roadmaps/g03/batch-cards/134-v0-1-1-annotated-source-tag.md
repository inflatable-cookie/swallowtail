# 134 v0.1.1 Annotated Source Tag

Status: completed
Owner: Tom
Created: 2026-08-06
Milestone: `../044-v0-1-1-source-patch-release.md`
Depends on: card 133

## Goal

Create and push one annotated `v0.1.1` source tag at the exact CI-green release
commit.

## Scope

1. Reconfirm clean local and remote candidate identity and tag absence.
2. Preview the exact Effigy execution mutation.
3. Create and push annotated `v0.1.1` without a GitHub Release or registry
   publication.
4. Verify local and remote annotated-tag identity and close release evidence.

## Acceptance

- [x] local and remote peeled tag resolve to the exact green release commit
- [x] the tag annotation is `v0.1.1`
- [x] `v0.1.0` remains unchanged
- [x] no crates.io, GitHub Release, binary, consumer, or provider mutation runs

## Stop Conditions

- stop if the tag exists, candidate identity drifts, or execution plans any
  excluded side effect
- never move or recreate a failed or partially published tag

## Auto-Continuation

Yes. The operator explicitly authorized this patch release. Stop immediately
after exact tag and closeout evidence.

## Completion Evidence

- annotated tag object: `d7cb439ef3b6808013950d209c2ffcf7930ec81a`
- peeled release commit: `bd3f4bbdffc403897ece4499ee0904b1e8116639`
- annotation: `v0.1.1`
- local and remote tag identities match; `v0.1.0` remains unchanged at tag
  object `630d33a0d1ff285d20787ee038147dc3493f8b88`
- a fresh clone at `v0.1.1` passes the exact Git-source consumer proof for
  `swallowtail-core`, `swallowtail-runtime`, `swallowtail-host-local`, and
  `swallowtail-adapter-codex`
- Effigy's binary-oriented install verifier is inapplicable: it attempted to
  install an `effigy` binary from this library-only repository; no release
  identity or source-consumer failure occurred
- no crates.io publication, GitHub Release, binary, consumer, or authenticated
  provider work ran
