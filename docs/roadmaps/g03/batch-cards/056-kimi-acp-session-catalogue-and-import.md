# 056 Kimi ACP Session Catalogue And Import

Status: planned
Owner: Tom
Created: 2026-08-01
Milestone: `../021-acp-session-list-and-kimi-import.md`
Depends on: card 055

## Goal

Implement explicit Kimi ACP session discovery and import while preserving its
exact state-root, resource, access, and version identity.

## Scope

1. Freeze exact Kimi `session/list` behavior across qualified milestones.
2. Implement a resource-scoped catalogue through the negotiated ACP route.
3. Revalidate the selected session, host, state root, resource, access, model,
   version, and policy before issuing a binding.
4. Load ordered history through the existing Kimi replay path.
5. Resume only through the imported binding.

## Out Of Scope

- Kimi local-server or Python CLI capability promotion
- account-wide state-directory scanning
- deletion, synchronization, consumer persistence, or UI
- live provider prompt

## Acceptance Criteria

- [ ] every supported Kimi milestone has exact list/import evidence
- [ ] candidates cannot cross state roots, resources, hosts, or plans
- [ ] stale or mismatched selections issue no binding
- [ ] successful import completes replay before readiness
- [ ] existing Kimi-created session behavior is unchanged
- [ ] focused Kimi and ACP tests pass
- [ ] card 057 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-adapter-kimi swallowtail-acp`
- `git diff --check`
- no authentication mutation or broad suite

## Auto-Continuation

Yes. Continue to card 057 after focused Kimi acceptance.
