# 188 Command Code Session Catalogue And Export Disposition

Status: completed
Owner: Tom
Created: 2026-08-09
Milestone: `../061-command-code-session-catalogue-export-disposition.md`
Depends on: card 187

## Goal

Freeze catalogue/import/export as evidence-backed absences on Command Code
`1.15.1` without adding driver roles.

## Scope

1. Align feature and route matrices with Research 118.
2. Document TTY-only `/export` and the private on-disk project tree.
3. Name the promotion gate for a future machine list/export surface.

## Acceptance

- [x] no filesystem catalogue implementation lands
- [x] guides and matrices agree
- [x] `effigy qa:routes` and `effigy qa:guides` pass

## Evidence

- Feature matrix: catalogue/import `Not applicable`; notes deny export.
- Route import classification keeps `command-code.headless` in
  `not applicable (14)`.
- Lifecycle row: no public load/resume; project transcripts preserved locally.
- Guide Unsupported plus promotion gate; architecture records
  `~/.commandcode/projects/…` as private provider state.
- Activity fixture exact absences: public load/resume, catalogue, import,
  provider export.

## Validation

- `effigy qa:routes` — passed
- `effigy qa:guides` — passed

## Stop Conditions

- stop if the card would invent catalogue from `~/.commandcode/projects`

## Auto-Continuation

No. Return to the operator.
