# 114 Oh My Pi Package And Route Acceptance

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../040-oh-my-pi-rpc-foundation.md`
Depends on: card 113

## Goal

Close package, public API, route-matrix, docs, and deterministic validation for
the first OMP route.

## Validation

- `effigy validate:focused swallowtail-adapter-oh-my-pi`
- `effigy package:verify-affected swallowtail-adapter-oh-my-pi`
- `effigy qa:docs`
- `effigy qa:routes`
- operator-only: `SWALLOWTAIL_LIVE_OMP_PROMPT=1 effigy probe:omp-luna-low`

## Acceptance

- [x] opt-in package and public prepared types compile independently
- [x] feature and activity matrices add `oh-my-pi.rpc` without changing `pi.rpc`
- [x] deterministic acceptance runs without live or authenticated provider work
- [x] later operator-gated Luna/low proof uses the same prepared facade
- [x] g03 returns to its evidence gate

## Completion

- focused adapter and shared activity-inventory validation: 124 tests passed;
  Clippy passed
- affected-package proof: 85 packaged files compiled from the extracted crate
- docs validation passed
- route validation passed: 33 routes, 26 solutions, and 66 activity operations
- installed identity-only probe classified `omp/17.2.9` through its exact Bun
  launcher
- authenticated prepared smoke selected `openai-codex` / `gpt-5.6-luna` /
  `low`, completed one bounded answer with usage, and joined cleanly
- live lifecycle findings are frozen: startup display clears are droppable;
  model and thinking changes are session lifecycle, not turn-owned activity

The live smoke was separately operator-authorized. No write-capable provider
tool ran and no credential value entered Swallowtail evidence.
