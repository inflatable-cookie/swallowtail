# 062 Provider Session Import Public And Package Acceptance

Status: completed
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

- [x] public matrices match tested route capability truth
- [x] examples use only prepared, validated operations
- [x] no guidance implies automatic synchronization or raw-id authority
- [x] all selected packages compile from the extracted target
- [x] docs and Northstar validation pass
- [x] card 063 becomes the sole ready and next task

## Evidence

- the provider feature CSV adds separate `provider_session_catalogue` and
  `provider_session_import` columns, remains sorted by provider, and reports
  `Yes` only for the three tested production solution rows
- the route matrix publishes the split 19-route classification and exact
  promotion boundaries for discovery-only, attachment-only, and blocked routes
- the central import guide separates browse, select, import, replay, resume,
  management, and consumer persistence and rejects raw-id authority and sync
- Codex, Kimi ACP, and OpenCode examples use only prepared catalogue, import,
  load, and resume operations; the OpenCode example compiles
- the stale validation selector `swallowtail-acp` was corrected to the exact
  workspace package `swallowtail-protocol-acp`
- both `effigy package:verify-affected` groups passed for seven selected
  packages
- `effigy qa:docs`, `effigy qa:northstar`, and `git diff --check` passed

## Validation

- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-codex swallowtail-protocol-acp`
- `effigy package:verify-affected swallowtail-adapter-kimi swallowtail-adapter-opencode swallowtail-testkit`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- no registry or broad suite

## Auto-Continuation

Yes. Continue to card 063 after package acceptance.
