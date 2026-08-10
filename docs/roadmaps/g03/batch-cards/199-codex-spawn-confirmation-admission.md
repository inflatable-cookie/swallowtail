# 199 Codex Subagent Spawn-Confirmation Admission Implementation

Status: dispatched
Owner: Tom
Created: 2026-08-10
Milestone: none yet (consumer-proven hardening)
Depends on: g03 batch 198 (contract 045 amendment)
Auto-start next card: no

## Objective

Implement the admission fix mapped by research note 120 and contracted by
card 198: admit a codex collab child thread when the parent envelope
projects `subAgentActivity` (`kind=started`), so a child `turn/started`
that races ahead of the spawn `collabAgentToolCall` item/completed no
longer fails the whole turn.

## Governing Refs

- `docs/research/120-codex-collab-spawn-admission-evidence.md` — the event
  sequence, the race, and this fix direction
- Contract 045 as amended by card 198 — the admission evidence set and
  ordering tolerance are authoritative there
- `crates/swallowtail-adapter-codex/src/turn_state/notifications.rs` —
  `admit_spawned_children` (312-339), `verify_child_lifecycle_owner`
  (180-205)
- Evidence test `evidence_197_collab_child_lifecycle_precedes_spawn_completion`
  (`turn_state/tests.rs`) — this card flips it to assert admission

## Worker Rules

- Execute the card exactly; no planning authority; no sub-agents.
- Do NOT touch roadmap/milestone/card/dispatch status files — deliverables +
  batch log only.
- Commit on branch `thread/199-codex-spawn-confirmation-admission` and push
  with `git push -u origin thread/199-codex-spawn-confirmation-admission`;
  no merge.

## Scope

- Admit the child when the parent envelope projects `subAgentActivity`
  (`kind=started`) with an exact `agentThreadId` — same bounded
  `MAX_ADMITTED_CHILD_THREADS` set, cleared at operation terminal, same
  fail-closed posture for never-observed ids.
- Keep the existing `collabAgentToolCall` spawnAgent item/completed
  admission (covers v1 flows and non-spawn collab actions).
- Flip the evidence test to assert admission under the live ordering;
  rename it out of the `evidence_197_` prefix per test conventions.
- Add ordering fixtures: lifecycle-before-spawn-completion admitted and
  observed; never-observed id still fails closed; admission set cleared at
  terminal.
- Run the adapter test suite and the repo's standard validation per
  AGENTS.md (`effigy` tasks where they apply).
- Batch log `docs/logs/2026-08-10-codex-spawn-confirmation-admission.md`.

Out of scope: contract changes (card 198 owns them), other providers,
deferred child-lifecycle buffering (only if the residual race
materializes — see Stop Conditions), consumer (nucleus) changes.

## Acceptance

- [ ] child admitted on parent-envelope `subAgentActivity` (`kind=started`)
- [ ] lifecycle-before-completion ordering admitted and observed; unknown
  ids still fail closed
- [ ] evidence test flipped and renamed; new ordering fixtures pass
- [ ] adapter suite + standard validation green; batch log pushed

## Evidence

- Batch log with commands + exit states and fixture names.
- Live verification is a follow-up operator step: re-run the Nucleus collab
  spawn against this build; if a capture shows the lifecycle racing ahead
  even of `subAgentActivity`, that is the residual-risk case from research
  note 120 and becomes its own card (bounded deferral), not silent scope
  growth here.

## Stop Conditions

- Admitting on `subAgentActivity` would weaken the fail-closed posture for
  unknown ids → stop with citations
- The amended contract 045 text contradicts this implementation shape →
  stop and report
- Projection changes needed beyond the codex adapter's subagent activity
  module → stop and report
