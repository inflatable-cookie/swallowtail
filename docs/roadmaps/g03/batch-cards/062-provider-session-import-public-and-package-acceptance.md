# 062 Provider Session Import Public And Package Acceptance

Status: planned
Owner: Tom
Created: 2026-08-01
Milestone: `../023-provider-session-import-acceptance-and-handoff.md`
Depends on: card 061

## Goal

Reconcile public capability truth and prove the common, Codex, Kimi, and
OpenCode import packages assemble independently.

## Scope

1. Update provider route and feature matrices from card 061 classifications.
2. Document catalogue, import, load, resume, and management as separate
   operations.
3. Add prepared-facade examples for browse, select, import, replay, and resume.
4. Warn against raw-id attachment, implicit scope widening, and automatic sync.
5. Assemble and compile one extracted target containing all selected packages.

## Out Of Scope

- crates.io publication or release preparation
- consumer adoption or consumer database design
- live provider prompt or broad workspace suite

## Acceptance Criteria

- [ ] public matrices match tested route capability truth
- [ ] examples use only prepared, validated operations
- [ ] no guidance implies automatic synchronization or raw-id authority
- [ ] all selected packages compile from the extracted target
- [ ] docs and Northstar validation pass
- [ ] card 063 becomes the sole ready and next task

## Validation

- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-codex swallowtail-acp`
- `effigy package:verify-affected swallowtail-adapter-kimi swallowtail-adapter-opencode swallowtail-testkit`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- no registry or broad suite

## Auto-Continuation

Yes. Continue to card 063 after package acceptance.
