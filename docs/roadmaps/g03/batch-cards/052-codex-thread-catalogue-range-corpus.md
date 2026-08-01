# 052 Codex Thread Catalogue Range Corpus

Status: ready
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

- [ ] every claim has exact release and source identity
- [ ] method introduction and behavior changes are explicit milestones
- [ ] current main documentation is corroboration, not historical authority
- [ ] legacy supported points remain usable without catalogue import
- [ ] no production capability changes in this card
- [ ] focused Codex corpus tests pass
- [ ] card 053 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-adapter-codex`
- `git diff --check`
- no live or broad suite

## Auto-Continuation

Yes. Continue to card 053 when the complete segment is settled.
