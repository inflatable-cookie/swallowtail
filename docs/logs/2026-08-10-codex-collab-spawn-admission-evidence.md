# Codex Collab Spawn Child-Thread Admission Evidence

Date: 2026-08-10
Roadmap: g03 batch 197
Card: 197 (evidence)

## Outcome

Research note `docs/research/120-codex-collab-spawn-admission-evidence.md`
maps the 0.147.0 collab spawn sequence, names the admission gap, and proposes
the card split. Evidence test
`evidence_197_collab_child_lifecycle_precedes_spawn_completion` in
`crates/swallowtail-adapter-codex/src/turn_state/tests.rs` reproduces the live
ordering and documents today's fail-closed behavior. No production code
changed.

## Findings

- Live 2026-08-10 16:24 UTC Nucleus collab spawn (codex-cli 0.147.0) failed
  the whole turn with `lifecycle_owner_mismatch` from
  `verify_child_lifecycle_owner` (`turn_state/notifications.rs:180-205`):
  the child `turn/started` arrived before any admission.
- The only admission source is the parent envelope's `collabAgentToolCall`
  (`spawnAgent`) item/completed (`admit_spawned_children`,
  `notifications.rs:312-339`). `subAgentActivity` items carry the exact child
  id but project `control: None` and never admit.
- Gap: ordering race. The app-server emits the child `turn/started` from
  inside the spawn call; the collab item/completed only after the spawn
  handler returns. The 0.146.0 frozen corpus order
  (`root_spawn_completed -> child_turn_started`) is not guaranteed.
- The earliest child-identity notification is the parent-envelope
  `subAgentActivity` (`kind=started`), live-observed ~3 ms before the child
  lifecycle; the proposed fix admits on it (contract delta + implementation).
- Reconciliation g03/026-038 do not cover the gap; the child thread remains
  recoverable via thread catalogue/import as a new operation.

## Commands

- `codex app-server generate-json-schema --out /tmp/codex-schema-0.147.0` —
  exit 0; installed 0.147.0 protocol schema
- `codex --version` — `codex-cli 0.147.0`
- Rollout inspection: `~/.codex/sessions/2026/08/10/rollout-2026-08-10T17-23-25-*.jsonl`,
  `...17-24-30-*.jsonl` — spawn event timestamps 16:24:30.049 / .120 / .123
- Source citations: openai/codex @ `rust-v0.147.0` (spawn handlers v1/v2,
  agent control, app-server listener attach)

## Validation

- `cargo test -p swallowtail-adapter-codex turn_state::tests::evidence_197_collab_child_lifecycle_precedes_spawn_completion` — passed
- `cargo test -p swallowtail-adapter-codex turn_state::tests` — passed

## Next

Implementation card per research note 120: admit on parent-envelope
`subAgentActivity` (`kind=started`), keep collab item/completed admission,
flip the evidence test, contract 045 amendment for the widened admission
evidence set and ordering tolerance.
