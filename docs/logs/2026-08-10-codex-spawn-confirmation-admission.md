# Codex Spawn-Confirmation Admission

Date: 2026-08-10
Roadmap: g03 batch 199
Card: 199 (implementation)

## Outcome

Codex app-server admission now accepts the provider's spawn-confirmation
observation: the parent-envelope `subAgentActivity` (`kind=started`) carrying
the exact `agentThreadId` admits that child before its lifecycle envelopes
arrive. The live 0.147.0 race where a child `turn/started` beats the spawn
`collabAgentToolCall` item/completed no longer fails the whole turn.

Governing refs: `docs/research/120-codex-collab-spawn-admission-evidence.md`,
Contract 045 Admission paragraph (amended by card 198).

## What Changed

- `crates/swallowtail-adapter-codex/src/turn_state/notifications.rs`
  - `admit_spawned_children` now also takes the raw envelope and adds the
    `subAgentActivity` (`kind=started`) `agentThreadId` to the candidate set.
  - New `spawn_confirmation_thread` helper: root-envelope only, exact
    `agentThreadId`, `kind=started` only; `interacted`/`interrupted` and
    child-envelope items never admit. Fail-closed posture for never-observed
    ids unchanged.
  - Existing `collabAgentToolCall` spawnAgent item/completed admission kept
    (v1 flows and non-spawn collab actions).
- `crates/swallowtail-adapter-codex/src/turn_state/tests.rs`
  - Evidence test flipped and renamed out of the `evidence_197_` prefix:
    `collab_child_lifecycle_precedes_spawn_completion_is_admitted`.
  - Ordering fixtures:
    - `never_observed_child_lifecycle_still_fails_closed_after_spawn_confirmation`
    - `interacted_subagent_activity_is_observation_only_and_does_not_admit`
    - `spawn_confirmation_admission_is_cleared_at_operation_terminal`
- this batch log (indexed in `docs/logs/README.md`)

No contract, roadmap, milestone, card, or dispatch status files touched.
Out of scope as written: contract changes, other providers, child-lifecycle
deferral, consumer (nucleus) changes.

## Validation

- `cargo test -p swallowtail-adapter-codex` — passed (all test binaries; 9/9
  `turn_state::tests` including the four fixtures above)
- `effigy validate:focused swallowtail-adapter-codex` — exit 0 (nextest +
  clippy `-D warnings`)
- `effigy package:verify-affected swallowtail-adapter-codex` — exit 0
- `effigy qa:docs:index:logs` — exit 0 (log indexed)

## Next

Live verification is the operator follow-up (card 199 Evidence): re-run the
Nucleus collab spawn against this build; a capture where the lifecycle races
ahead even of `subAgentActivity` is the residual-risk case from research note
120 and becomes its own bounded-deferral card.
