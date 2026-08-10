# 197 Codex Collab Spawn Child-Thread Admission Evidence

Status: dispatched
Owner: Tom
Created: 2026-08-10
Milestone: none yet (consumer-proven hardening)
Depends on: none
Auto-start next card: no

## Objective

Research card. A live Codex multi-agent (collab) spawn through Nucleus
failed the whole turn with:

```
swallowtail.codex.app_server.lifecycle_owner_mismatch
Codex app-server lifecycle belongs to an unknown operation thread
```

emitted by `verify_child_lifecycle_owner`
(`crates/swallowtail-adapter-codex/src/turn_state/notifications.rs:180-205`):
a `turn/started` lifecycle event arrived for a child thread that was never
registered in `admitted_child_threads`.

Map exactly how codex app-server (installed 0.147.0) announces sub-agent
spawns on a multi-agent/collab turn, why the existing admission path
(g03/007, g03/008) did not cover it, and produce the implementation-ready
delta — which notification admits the child, where admission must happen,
and what the contract/implementation cards look like.

## Governing Refs

- `docs/roadmaps/g03/007-codex-operation-local-child-activity-ownership.md`
- `docs/roadmaps/g03/008-codex-child-turn-lifecycle-ownership.md`
- `docs/contracts/045-subagent-topology-observation-and-control.md` —
  subagent topology is observation-only
- The codex adapter turn-state module and its tests
  (`crates/swallowtail-adapter-codex/src/turn_state/`)

## Worker Rules

- Execute the card exactly; no planning authority; no sub-agents.
- Do NOT touch roadmap/milestone/card/dispatch status files — deliverables +
  batch log only.
- This is an evidence card: no production code changes. Test-only additions
  (e.g. a failing fixture that reproduces the gap) are allowed if they are
  clearly marked as evidence, not the fix.
- Commit on branch `thread/197-codex-collab-spawn-admission-evidence` and
  push with
  `git push -u origin thread/197-codex-collab-spawn-admission-evidence`;
  no merge.

## Scope

- Identify every codex app-server notification/method that carries a child
  (sub-agent) thread identity today, and which of them the adapter admits
  on. Cite source and, where possible, codex app-server protocol evidence
  (installed schema/fixtures/logs under the harness or adapter test
  corpora).
- Reconstruct the admission gap: what sequence does a collab spawn produce
  vs. what `admitted_child_threads` expects? Determine whether admission
  must move to an earlier notification, or whether a new spawn notification
  must be observed.
- Check whether the failed turn leaves recoverable state (the child did
  emit one activity before the failure) and whether reconciliation cards
  (g03/026-038) already cover it.
- Deliverable: a research note under `docs/research/` (next number per
  local convention) with the spawn event sequence, the admission gap, and a
  proposed card split (contract delta if any + implementation).
- Batch log `docs/logs/2026-08-10-codex-collab-spawn-admission-evidence.md`.

Out of scope: implementing the admission fix, changing adapter behavior,
contract amendments (recommend them; do not write them).

## Acceptance

- [ ] the collab-spawn notification sequence is documented with citations
- [ ] the exact admission gap is named (missing observation vs. ordering)
- [ ] a card split for the fix is proposed with contract impact stated
- [ ] research note + batch log committed and pushed

## Evidence

- Research note with source citations; batch log with commands + exit
  states.

## Stop Conditions

- The installed codex app-server's collab spawn flow cannot be determined
  from local evidence (schema, fixtures, logs) → stop with what is missing
- The gap contradicts g03/007-008's ownership model rather than extending
  it → stop with citations
- The fix requires a protocol capability codex does not expose → stop and
  report
