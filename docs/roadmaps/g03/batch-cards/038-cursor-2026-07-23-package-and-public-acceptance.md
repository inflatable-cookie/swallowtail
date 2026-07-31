# 038 Cursor 2026.07.23 Package And Public Acceptance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../014-cursor-agent-2026-07-23-range-maintenance.md`
Depends on: card 037

## Goal

Accept the current exact Cursor milestone through package and public-truth
evidence without modifying the installed CLI.

## Acceptance Criteria

- [x] extracted Cursor package compiles
- [x] route and feature matrices name both exact milestones
- [x] architecture and Northstar currentness are reconciled
- [x] no provider prompt, installation, consumer edit, or publication runs

## Validation

- `effigy validate:focused swallowtail-adapter-cursor`
- `effigy package:verify-affected swallowtail-adapter-cursor`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

Completed. Return to the g03 maintenance checkpoint.

## Result

Focused validation passed 34 tests across six binaries in two seconds. The
54-file Cursor package assembled and compiled independently in three seconds.
Route, lifecycle, feature, activity, docs, Northstar, and diff-hygiene checks
passed. No live provider or consumer operation ran.
