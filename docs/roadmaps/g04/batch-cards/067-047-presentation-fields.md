# 067 047 Presentation Fields

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../023-047-presentation-metadata.md`
Depends on: card 066

## Goal

Realize the named optional 047 presentation fields.

## Scope

1. Additive API in `public-api-unreleased`.
2. `public-api-0.3.3` stays immutable.
3. Snapshot still has no emails, tokens, or targets.

## Out Of Scope

- overlay hide/favourite as 047 fields
- accent color
- hosted OAuth

## Acceptance Criteria

- [x] named fields project onto 047 without changing `Ready` / `NotReady`
- [x] overlay markers remain overlay
- [x] `public-api-0.3.3` stays immutable

## Realization

`ConfiguredProviderInstanceAdmission::with_label` carries the optional
Contract 057 host-owned `InstanceLabel` into
`ConfiguredProviderInstanceRecord::label`. The snapshot stores no overlay
markers, accent color, authenticated-subject values, emails, tokens, or
targets. Focused tests cover labelled `Ready` and `NotReady` records and show
that the label is not part of the readiness calculation.

The additive surface is recorded in
`release-baselines/public-api-unreleased/swallowtail-runtime.txt`.

## Evidence

Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-1e989752`
Worker branch: `t3code/presentation-metadata-handoff`

Validation passed: focused core/runtime/testkit validation (396 tests),
`git diff --check`, and `effigy package:api`. The immutable
`release-baselines/public-api-0.3.3/` baseline is unchanged.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-testkit`
- `git diff --check`
- `effigy package:api`

## Auto-Continuation

No. Named addable implementations from g04.022 wait until this
milestone closes.

## Stop Conditions

- Stop if `Ready` / `NotReady` changes.
- Stop if overlay markers become 047 snapshot fields.
