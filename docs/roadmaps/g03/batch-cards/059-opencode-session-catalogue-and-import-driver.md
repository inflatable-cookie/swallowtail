# 059 OpenCode Session Catalogue And Import Driver

Status: planned
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

- [ ] candidates remain bound to the exact endpoint and directory plan
- [ ] stale, missing, mismatched, or unsupported sessions issue no binding
- [ ] successful import replays bounded ordered history before readiness
- [ ] attached server and credential ownership remain unchanged
- [ ] existing OpenCode new/load/resume behavior is unchanged
- [ ] focused driver tests pass
- [ ] card 060 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-adapter-opencode swallowtail-runtime`
- `git diff --check`
- no provider prompt or broad suite

## Auto-Continuation

Yes. Continue to card 060 after focused driver acceptance.
