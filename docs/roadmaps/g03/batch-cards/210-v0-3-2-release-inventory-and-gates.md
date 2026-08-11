# 210 v0.3.2 Release Inventory And Gates

Status: completed
Owner: Tom
Created: 2026-08-11
Milestone: `../067-v0-3-2-source-patch-release.md`

## Goal

Make the selected compatible release shape exact before any version mutation.

## Scope

- `v0.3.2` compatibility classification and `[Unreleased]` payload
- 30-package dependency order and metadata truth
- separate unreleased Command Code and idioms semantic API baselines
- read-only release simulation and prepare plan

## Acceptance

- [x] changelog contains the complete post-`v0.3.1` payload
- [x] metadata and semantic API checks pass for 30 current packages
- [x] release package order contains Command Code and places idioms before
      runtime
- [x] read-only `v0.3.2` simulation and prepare plan select no excluded action
- [x] no version, lock, candidate state, commit, push, or tag mutation runs

## Evidence

- `[Unreleased]`: nine entries — four added, three changed, two fixed
- current source: 30 packages, 36 production routes, Rust `1.95.0`
- semantic API: 28 immutable release baselines plus separate Command Code and
  idioms first-release baselines; five compatible current-source overrides
- release order: 30 exact packages; idioms precedes runtime; Command Code and
  the four previously omitted adapters are included
- `effigy release prepare --plan --version 0.3.2`: ready, no gates or mutations
- `effigy release simulate --version 0.3.2`: all 11 configured gates pass with
  Effigy `v0.11.0+local.53a4971`, including 1,625 workspace tests and the
  isolated source consumer
- `.release-prepared.json`: absent; workspace version remains `0.3.1`

Card 211 is ready but remains an operator-authorized release mutation. No
candidate preparation is implied by the green simulation.

## Stop Conditions

- stop on incompatible API evidence, count or dependency drift, or a release
  plan containing registry, GitHub Release, workflow, consumer, or provider
  mutation
