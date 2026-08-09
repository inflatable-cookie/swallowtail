# 060 Command Code Interactive Continuity

Status: completed
Owner: Tom
Created: 2026-08-09
Depends on: g03.059; Research 118
Vision tags: installed harness, Command Code, interactive continuity
Contract refs: 005-006, 009-010, 017, 023, 032-033, 039-041, 043-046, 051-052

## Problem

`command-code.headless` only owns structured runs with `--no-session`. Research
118 proves same-cwd exact `--resume <uuid>` continuation under plan-mode NDJSON.
Ambient `--continue` and `--fork-session` must stay rejected.

## Generation Runway

Advances g03's installed-harness goal by adding Contract 043 private continuity
on the existing Command Code package without inventing catalogue, export, or
Provider API surfaces.

## Execution Plan

- [x] card 185: freeze retained-session and resume NDJSON fixtures plus failure
      shapes for bad id, cross-cwd, continue, and fork
- [x] card 186: implement interactive driver role, private resume command
      binding, retention policy split, and prepared session facade
- [x] card 187: matrix/guide/live acceptance for interactive continuity; close
      into the catalogue/export disposition checkpoint

## Goals

- [x] expose interactive session on `command-code.headless` without collapsing
      structured-run `--no-session` behavior
- [x] first turn omits resume; later turns pass exact private `--resume <id>`
- [x] reject `--continue` and `--fork-session` in prepared evidence
- [x] keep public 017 load/resume and 046 catalogue/import/export out of scope

## Boundaries

- no ambient latest-session selector
- no home-directory project scan as catalogue
- no TUI `/export` or `/sessions` automation
- no Provider API work in this milestone
- no version bump, tag, or registry mutation

## Acceptance Criteria

- [x] deterministic fixtures cover first turn, exact resume, bad id, and
      forbidden selectors
- [x] focused and extracted-package validation pass without credentials
- [x] one operator-gated two-turn plan-mode live probe passes in one working
      resource
- [x] matrices and guide state interactive continuity honestly without claiming
      catalogue or export

## Planning Checkpoint

After card 187, run g03.061 to freeze catalogue/export as unsupported on
`1.15.1` unless new machine evidence appears.
