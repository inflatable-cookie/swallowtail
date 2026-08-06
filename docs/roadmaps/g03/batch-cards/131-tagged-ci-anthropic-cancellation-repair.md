# 131 Tagged CI Anthropic Cancellation Repair

Status: completed
Owner: Tom
Created: 2026-08-06
Milestone: `../043-v0-1-0-source-release-readiness.md`
Depends on: card 130
Contract refs: 009, 022, 036

## Goal

Restore green CI after tagged-source evidence reproduced Anthropic Managed
Agents cancellation being relabelled as timeout under runner load.

## Scope

1. Preserve accepted consumer cancellation as the terminal winner when the
   attachment deadline becomes ready concurrently.
2. Wake the attachment pump directly when cancellation is accepted instead of
   relying only on network-connection shutdown.
3. Add deterministic arbitration coverage plus the existing driver and
   prepared-facade cancellation regressions.
4. Record the failed tag CI without deleting, moving, or recreating `v0.1.0`.
5. Push the repair to `main` and dispatch the existing CI workflow against the
   exact repair commit.

## Validation

- cancellation/deadline simultaneous-readiness regression
- existing Anthropic managed driver and prepared-facade interruption tests
- `effigy validate:focused swallowtail-adapter-anthropic`
- `effigy package:verify-affected swallowtail-adapter-anthropic`
- manually dispatched GitHub CI passes every job against the repair commit
- `effigy qa:docs`

## Stop Conditions

- do not weaken deadline truth when no cancellation was accepted
- do not globally extend fixture deadlines or remove the direct simultaneous-
  readiness regression; cancellation tests may isolate setup contention from
  the terminal condition they assert
- do not edit the published tag or start another release
- stop on any CI failure and retain its exact evidence

## Auto-Continuation

Yes. Continue through deterministic repair, focused validation, one exact
`main` CI dispatch, and documentation closeout. Release `0.1.1` remains a
separate operator decision.

## Completion Evidence

- `3b960b30b3e1bad01df2c06e00f067e361093eab` adds direct cancellation
  wake-up, cancellation-first attachment polling, and deterministic
  simultaneous-readiness coverage.
- `4ffbd8f8a5302b9ce31ee37687876fcab8661f58` gives cancellation tests a
  separate 30-second operation deadline so managed-resource setup contention
  cannot select the terminal condition under test.
- focused Anthropic validation passes 58 tests; extracted-package proof,
  clippy, Rust 1.90 package tests, and ten repeated concurrent interruption
  runs pass.
- local CI-shaped nextest passes 1,464 tests with 11 skipped; the complete
  Rust 1.90 non-Bedrock workspace test command passes.
- GitHub CI run `31103299936` passes all six jobs against exact repair commit
  `4ffbd8f8a5302b9ce31ee37687876fcab8661f58`.
- annotated tag `v0.1.0` remains unchanged at release commit
  `a8bef72b718d3d9e503da48b3af05da4b674d4ec`.
- no release, registry, GitHub Release, consumer, or authenticated provider
  mutation ran.
