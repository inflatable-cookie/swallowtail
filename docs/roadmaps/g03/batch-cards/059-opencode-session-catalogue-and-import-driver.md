# 059 OpenCode Session Catalogue And Import Driver

Status: completed
Owner: Tom
Created: 2026-08-01
Milestone: `../022-opencode-external-session-discovery-and-import.md`
Depends on: card 058

## Goal

Implement bounded OpenCode session discovery and explicit import through the
approved attached HTTP endpoint.

## Scope

1. Implement resource-scoped list and exact candidate lookup.
2. Bind endpoint, directory, host, access lease, server revision, model, and
   policy across catalogue and import.
3. Issue an imported binding only after exact revalidation.
4. Reuse existing ordered replay and continuation paths.
5. Preserve attached-server ownership and credential cleanup.

## Out Of Scope

- starting, stopping, updating, or owning the OpenCode server
- cross-directory project scanning or automatic synchronization
- share, fork, rename, revert, summarize, archive, or delete
- consumer persistence or UI

## Acceptance Criteria

- [x] candidates remain bound to the exact endpoint and directory plan
- [x] stale, missing, mismatched, or unsupported sessions issue no binding
- [x] successful import replays bounded ordered history before readiness
- [x] attached server and credential ownership remain unchanged
- [x] existing OpenCode new/load/resume behavior is unchanged
- [x] focused driver tests pass
- [x] card 060 becomes the sole ready and next task

## Evidence

- the attached facade exposes separate resource-scoped catalogue and explicit
  import operations only for qualified server revisions
- page projection distinguishes inactive roots, children, active sessions,
  archived sessions, missing status, and incompatible revisions without
  granting unavailable candidates import authority
- import repeats exact health, lookup, directory, title, update-time, revision,
  root, archive, and idle-status checks before issuing a binding
- the imported binding enters the unchanged four-item load/replay path under
  local and remote-authoritative host identities
- a deterministic title-drift case fails at import revalidation and issues no
  binding
- `effigy validate:focused swallowtail-adapter-opencode swallowtail-runtime`
  passed 195 tests
- `git diff --check` passed

## Validation

- `effigy validate:focused swallowtail-adapter-opencode swallowtail-runtime`
- `git diff --check`
- no provider prompt or broad suite

## Auto-Continuation

Yes. Continue to card 060 after focused driver acceptance.
