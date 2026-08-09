# 061 Command Code Session Catalogue And Export Disposition

Status: completed
Owner: Tom
Created: 2026-08-09
Depends on: g03.060; Research 118
Vision tags: installed harness, Command Code, provider session
Contract refs: 017, 038, 043, 046, 052

## Problem

Operators asked for session catalogue/export after g03.059. Research 118 shows
no non-TTY list or export surface on Command Code `1.15.1`. Inventing a
`~/.commandcode/projects` scanner would violate Contract 046.

## Generation Runway

Closes the session-surface honesty gap without shipping fake catalogue/export
capability.

## Execution Plan

- [x] card 188: update route/feature/activity matrices, guide, and failure notes
      so catalogue/import/export remain explicit absences with Research 118
      evidence; no new driver roles

## Goals

- [x] publish evidence-backed `No` / `Not applicable` truth for catalogue,
      import, and provider export on `command-code.headless`
- [x] document on-disk transcript privacy and the TTY-only `/export` gap
- [x] leave a promotion gate if Command Code later ships a machine list/export

## Boundaries

- no filesystem catalogue driver
- no TUI automation
- no public 017 resume binding from private 043 continuity
- no Provider API work

## Acceptance Criteria

- [x] matrices, guide, and architecture agree that catalogue/export are
      unsupported on `1.15.1`
- [x] `effigy qa:routes` and `effigy qa:guides` pass
- [x] closeout names the promotion gate for a future machine list/export

## Planning Checkpoint

Return to the operator. Reopen only if Command Code exposes a non-interactive
session list or export, or if the operator selects another lane.
