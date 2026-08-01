# 053 Codex Thread Catalogue And Import Driver

Status: ready
Owner: Tom
Created: 2026-08-01
Milestone: `../020-codex-external-thread-discovery-and-import.md`
Depends on: card 052

## Goal

Expose exact resource-scoped Codex thread discovery and explicit import behind
the existing prepared Codex facade.

## Scope

1. Map qualified `thread/list` pagination and cwd/source filters.
2. Project bounded candidate title, preview, time, and status content.
3. Revalidate one selection through qualified `thread/read` or equivalent.
4. Issue an imported binding only after exact plan agreement.
5. Reuse existing Codex load replay and resume paths unchanged.

## Out Of Scope

- direct rollout or state-database reads
- account-wide discovery, automatic import, polling, or synchronization
- archive, delete, fork, rename, compaction, or model fallback

## Acceptance Criteria

- [ ] only the exact corpus-qualified segment advertises catalogue/import
- [ ] one approved resource scope cannot return or import another cwd
- [ ] stale, missing, active-when-forbidden, and mismatched threads issue none
- [ ] provider content is bounded and diagnostic-safe
- [ ] imported load and resume preserve existing access and lifecycle truth
- [ ] focused Codex tests pass
- [ ] card 054 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-adapter-codex`
- scoped Codex formatting
- `git diff --check`
- no live or broad suite

## Auto-Continuation

Yes. Continue to card 054 after the production mapping passes.
