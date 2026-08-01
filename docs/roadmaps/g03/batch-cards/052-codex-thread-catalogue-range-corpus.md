# 052 Codex Thread Catalogue Range Corpus

Status: completed
Owner: Tom
Created: 2026-08-01
Milestone: `../020-codex-external-thread-discovery-and-import.md`
Depends on: card 051

## Goal

Freeze exact Codex thread discovery and import behavior without projecting
current app-server documentation backward across the maintained range.

## Scope

1. Inspect every maintained Codex app-server behavior milestone.
2. Freeze `thread/list`, `thread/read`, history inclusion, cwd/source filters,
   pagination, status, and `thread/resume` presence and response shape.
3. Name the first complete guaranteed catalogue/import segment.
4. Preserve unsupported legacy points and later unverified-newer posture.
5. Add deterministic normalized corpus assertions before claim changes.

## Out Of Scope

- production driver or facade changes
- experimental fork, compaction, rename, descendants, or item pagination
- rollout files, state DB access, live Codex, or consumer edits

## Acceptance Criteria

- [x] every claim has exact release and source identity
- [x] method introduction and behavior changes are explicit milestones
- [x] current main documentation is corroboration, not historical authority
- [x] legacy supported points remain usable without catalogue import
- [x] no production capability changes in this card
- [x] focused Codex corpus tests pass
- [x] card 053 becomes the sole ready and next task

## Closeout

Research 093 and the deterministic compatibility corpus select `0.105.0` as
the first complete operation-specific floor. Earlier qualified Codex releases
retain all existing operations without advertising catalogue/import. Current
documentation remains corroboration only.

`effigy validate:focused swallowtail-adapter-codex` passed 149 tests. No live,
broad, production capability, or consumer change ran.

## Validation

- `effigy validate:focused swallowtail-adapter-codex`
- `git diff --check`
- no live or broad suite

## Auto-Continuation

Yes. Continue to card 053 when the complete segment is settled.
