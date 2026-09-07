# 126 Claude SDK Selected-Skill Bundle Binding

Status: ready
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: card 115 merged (`SelectedSkillBundle::{new,resolve}`); the reconciliation capsule (cell 3); Contract 063 Context, Instructions, Skills, And References

## Goal

Bind a resolved selected-skill bundle to a `claude-agent.sdk` session as Contract 063's third input, separate from session instructions and the per-turn `TurnRequest`, so Desktop's repository skill and its required references reach the provider with identity, digest, and bounds validated.

## Scope

1. `ClaudeAgentSdkSessionProfile::with_selected_skill_bundle(ResolvedSkillBundle)` (additive; absence preserves today's behaviour).
2. Transport: the bundle's bounded resolved content and reference descriptors cross to the sidecar as one distinct labelled input, never merged into instructions or user text, never as raw client paths; the sidecar presents it to the SDK through the exact frozen 0.3.259 surface the card identifies (system prompt append or the SDK's own skills input, whichever the declaration supports without ambient file loading: `settingSources` stays empty, `skills` stays explicit). Record the chosen surface with anchors.
3. Validation before provider work: identity/revision/digest re-checked at open; oversize, missing, or foreign references fail typed; the bundle is immutable for the session.
4. Fake-SDK fixtures: bundle present and delivered as the labelled input; absent bundle unchanged; tampered digest fails; oversize fails; reference outside bounds fails.
5. Contract 061 projection: `registered-tool.selected-skill-bundle` row becomes available for this route only.
6. Guide, changelog `[Unreleased]`, additive baseline.

## Out Of Scope

Skill discovery (Contract 062 host side); deciding relevance; any claim the provider followed the skill; Codex and Grok routes.

## Acceptance Criteria

- [ ] resolved bundle bound at open through the additive profile API; absence unchanged
- [ ] delivered as a distinct labelled input, never as instructions or user text
- [ ] digest/bounds/reference failures typed before provider work
- [ ] fixtures as listed; Contract 061 row available for this route; guide, changelog, baseline; one PR

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: the skill content the provider receives is byte-identical to the resolved bundle Swallowtail validated, and nothing else about the session changes. Smallest counterexample: a bundle delivered by writing a file into the workspace.

## Stop Conditions

No frozen 0.3.259 surface can carry the bundle without ambient file loading (record; return to Chatterbox).

## Auto-Continuation

No. Stop for exact-head review. Provider-free only; the real-route gate runs under separate operator authority.
