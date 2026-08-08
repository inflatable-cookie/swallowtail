# 165 Discovery Failure Stage Mapper Consolidation

Status: done
Closeout: 2026-08-08
Owner: Tom
Created: 2026-08-08
Milestone: `../054-remaining-duplication-tranches.md`
Depends on: checkpoint after 053

## Goal

Consolidate the discovery failure-stage mappers onto the shared probe code
table, or record them as intentional.

## Scope

1. Replace the hand-written `DiscoveryStatus -> code` tables in
   `adapter-codex/src/discovery/outcome.rs` and
   `adapter-cursor/src/discovery/outcome.rs` with the shared
   `installed_probe_codes!` table fields, so the eight outcome-code strings
   live once in `swallowtail-runtime`.
2. Record antigravity's staged outcome mapper and codex's stderr-sanitizing
   exit mapper as intentionally adapter-local (already recorded in card 160;
   confirm and cross-reference).

## Out Of Scope

- public API, diagnostic-code, or behavior changes
- the staged-outcome and stderr-sanitization mappers

## Acceptance

- [x] codex and cursor outcome codes come from the shared table
- [x] every outcome stays byte-identical (same codes, same messages)
- [x] adapter-local mappers are recorded with reasons

## Closeout

- codex and cursor `status_code` tables now reference the shared
  `installed_probe_codes!("swallowtail.codex"|"swallowtail.cursor")` table
  fields; the eight outcome-code strings live once in `swallowtail-runtime`.
  Each adapter keeps a local `InstalledProbeCodes` const (no new public
  runtime API).
- adapter-local mappers confirmed intentional (cross-reference card 160):
  antigravity's staged outcome mapper and codex's stderr-sanitizing
  `exit_failed` carry classification the shared table does not express.
- validation: 10 focused suites green; `validate:focused` passed for both
  packages; `package:api` unchanged at the v0.3.0 baseline.

## Stop Conditions

- stop if a mapper's outcome codes or messages change

## Auto-Continuation

Yes, to card 166 after acceptance.

## Validation

- `effigy validate:focused swallowtail-adapter-codex`
- `effigy validate:focused swallowtail-adapter-cursor`
- `effigy package:api`
