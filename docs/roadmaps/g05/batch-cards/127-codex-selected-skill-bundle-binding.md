# 127 Codex App-Server Selected-Skill Bundle Binding

Status: ready
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: card 115 merged; card 117 merged; the reconciliation capsule (cell 9); Contract 063 Context, Instructions, Skills, And References

## Goal

Bind a resolved selected-skill bundle to a `codex.app-server` session as Contract 063's third input, on the same rules as card 126, so the Codex route reaches skill parity for Desktop.

## Scope

1. `CodexSessionProfileInput::with_selected_skill_bundle(ResolvedSkillBundle)` (additive; absence unchanged).
2. Transport through the exact app-server surface the route already qualifies for session developer instructions or an equivalent labelled input, recorded with anchors; never raw paths, never merged into user text, never a working-resource write.
3. Validation before provider work as card 126.
4. Fixtures on the existing Codex app-server fixture server: present, absent, tampered digest, oversize, foreign reference.
5. Contract 061 row available for this route only; guide, changelog `[Unreleased]`, additive baseline.

## Out Of Scope

Codex provider-direct MCP (withheld); skill discovery; relevance; Claude and Grok routes.

## Acceptance Criteria

- [ ] resolved bundle bound at open through the additive input API; absence unchanged
- [ ] delivered as a distinct labelled input on the qualified app-server surface
- [ ] typed failures before provider work; fixtures as listed
- [ ] Contract 061 row available for this route; guide, changelog, baseline; one PR

## Validation

- `cargo fmt -p swallowtail-adapter-codex -- --check`
- `effigy validate:focused swallowtail-adapter-codex`
- `effigy package:verify-affected swallowtail-adapter-codex`
- `effigy package:api`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: as card 126, on the Codex route. Smallest counterexample: bundle text appended to the user's turn.

## Stop Conditions

The app-server exposes no labelled input separable from user text (record; return to Chatterbox; the row stays Unavailable / route_dimension_unsupported).

## Auto-Continuation

No. Stop for exact-head review. Provider-free only; the real-route gate runs under separate operator authority.
