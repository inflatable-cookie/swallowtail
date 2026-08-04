# 070 Cross-Process Reconciliation Kernel, Codex, And OpenCode

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../027-cross-process-active-operation-reconciliation.md`
Depends on: card 067

## Goal

Add one authority-free restart reconciliation role and prove exact-turn Codex
plus session-scoped OpenCode production mappings.

## Scope

1. Promote Research 099 and Contract 048.
2. Add provider-neutral state, attribution, bounded replay, plan, request,
   outcome, prepared evidence, role, registration, and capability vocabulary.
3. Implement prepared and low-level Codex reconciliation.
4. Implement prepared and low-level OpenCode reconciliation.
5. Prove exact active/terminal and session-scoped active/inactive outcomes
   without provider mutation.
6. Publish route gates without expanding the main feature CSV with permanent
   negative columns.

## Acceptance Criteria

- [x] reconciliation cannot import, resume, retry, cancel, or answer callbacks
- [x] session-scoped evidence cannot report a terminal state
- [x] Codex revalidates exact thread, turn, resource, route, and version
- [x] OpenCode revalidates exact session, resource, route, version, and status
- [x] bounded history reports snapshot completeness
- [x] all routes are classified by an actionable evidence gate
- [x] focused and affected-package validation pass
- [x] roadmap, architecture, guide, and closeout currentness reconcile

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-opencode swallowtail-adapter-codex`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-opencode swallowtail-adapter-codex`
- `effigy qa:docs`
- `git diff --check`

## Auto-Continuation

Return the sole Next Task to the g03 evidence gate. Preserve Kimi local-server
cursor checkpoints and controlled-shutdown detach semantics as separate
promotion candidates.
