# Contract 045 Spawn-Confirmation Admission Amendment

Date: 2026-08-10
Roadmap: g03 batch 198
Card: 198 (contract amendment)

## Outcome

Amended Contract 045's Admission paragraphs so the operation-local admission
evidence set includes the provider's spawn-confirmation observation, and so
ordering between child-lifecycle envelopes and admission evidence is not
assumed. Observation-only posture unchanged. No adapter or test changes.

Governing evidence: `docs/research/120-codex-collab-spawn-admission-evidence.md`.

## Amended Admission Text

Qualified admission evidence now includes:

- a completed spawn collaboration item
- the provider's spawn-confirmation observation — for Codex app-server, the
  parent-envelope `subAgentActivity` (`kind=started`) carrying the exact
  `agentThreadId`

Ordering: the adapter may not assume ordering between child-lifecycle
envelopes and admission evidence. Child lifecycle for an id already
established by spawn-confirmation topology evidence is admitted and observed
without failing the operation. Never-observed ids still fail closed.

Control, callbacks, terminal, and session authority unchanged.

## Surfaces Touched

- `docs/contracts/045-subagent-topology-observation-and-control.md` —
  Admission paragraphs; `Updated: 2026-08-10`
- `docs/contracts/contract-index.md` — Updated date; 045 scope names
  operation-local admission
- `docs/contracts/contract-summaries.md` — 045 summary notes spawn-confirmation
  admission and ordering tolerance
- `docs/research/README.md` — index research note 120 (gate-required; left
  unindexed by card 197)
- `docs/logs/README.md` — index this batch log and the card 197 evidence log
  (gate-required)
- this batch log

## Validation

- `effigy docs:contract-drift` / `effigy docs:spec-drift` — not defined in this
  repo; used the docs QA equivalents below
- `effigy qa:docs:index:logs` — exit 0
- `effigy qa:docs:index:research` — exit 0
- `effigy qa:docs` — exit 0
- `effigy qa:northstar` — exit 0

## Next

Implementation card 199: admit on parent-envelope `subAgentActivity`
(`kind=started`), keep collab item/completed admission, flip the evidence
test.
