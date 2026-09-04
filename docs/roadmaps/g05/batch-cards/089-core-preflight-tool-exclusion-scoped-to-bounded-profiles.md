# 089 Core Preflight Tool Exclusion Scoped To Bounded Profiles

Status: ready
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: Contract 013 as clarified 2026-09-04; card 080's stop record on PR 221

## Goal

Make the shared preflight guard match Contract 013: reject `ReadWrite` plus
`Capability::ToolCalls` only when a filesystem boundary is claimed (the
bounded profile), and admit the pairing under an ambient profile that claims
no boundary. This unblocks the write half of card 080 without touching any
adapter.

## Scope

1. In `crates/swallowtail-core/src/preflight/session_access.rs`, change the
   guard at the "Bounded writable sessions cannot declare consumer tools"
   rejection so it fires only when `policy.filesystem_boundary()` is `Some`
   (or the isolation posture is enforced) together with `ReadWrite` and a
   `ToolCalls` requirement. `SessionAccessPolicy::ambient_harness(ReadWrite)`
   and `ambient_harness_with_consumer_mediated_requests(ReadWrite, ..)` with
   `ToolCalls` pass. `bounded_workspace(..)` with `ToolCalls` still fails
   with the same message.
2. Add testkit session-access fixture cases and one portable assertion:
   ambient read-write with tool calls admitted; bounded read-write with tool
   calls rejected; read-only with tool calls unchanged. Name the assertion
   `assert_consumer_tool_exclusion_keys_on_boundary_claim` and call it from
   the existing session-access suite.
3. Prove every existing adapter plan still preflights identically: run the
   workspace-wide session-access fixtures and the focused validation for
   core and testkit. No adapter source changes.
4. Regenerate the core and testkit API baseline files if any public item
   changed; expect none.
5. `CHANGELOG.md` `[Unreleased]`: one line under a non-breaking heading.

## Out Of Scope

Adapter changes; card 080's route work; Codex bounded profile behaviour;
Contract 013 text (already clarified); any new policy dimension.

## Acceptance Criteria

- [ ] ambient read-write with `ToolCalls` passes shared preflight
- [ ] bounded read-write with `ToolCalls` still fails with the same message
- [ ] the Codex Agent Chat and Bounded Workspace plans preflight unchanged
- [ ] the new assertion is in the portable suite
- [ ] public API unchanged, or additive only

## Validation

- `cargo fmt -p swallowtail-core -p swallowtail-testkit -- --check`
- `effigy validate:focused swallowtail-core swallowtail-testkit`
- `effigy package:verify-affected swallowtail-core swallowtail-testkit`
- `effigy package:api`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: the exclusion keys on the boundary claim. Smallest counterexample:
a bounded plan with tool calls that now passes, or an ambient plan that still
fails.

## Auto-Continuation

No. Stop after one reviewable PR; the coordinator then resumes card 080's
worker for its second PR.
