# 198 Contract 045 Subagent Spawn-Confirmation Admission Amendment

Status: completed
Owner: Tom
Created: 2026-08-10
Milestone: none yet (consumer-proven hardening)
Depends on: g03 batch 197 (evidence, merged `370105d1`)
Auto-start next card: no

## Objective

Amend contract 045's admission paragraph so the operation-local admission
evidence set covers the provider's spawn-confirmation observation, and so
ordering between child-lifecycle envelopes and admission evidence is
explicitly not assumed. This is the contract half of the card split proposed
by research note 120; the implementation follows in a separate card.

## Governing Refs

- `docs/research/120-codex-collab-spawn-admission-evidence.md` — the evidence
  base; the amendment text below is its recommendation
- `docs/contracts/045-subagent-topology-observation-and-control.md` — the
  contract being amended (Admission paragraph)
- Live failure 2026-08-10: `lifecycle_owner_mismatch` on a 0.147.0 collab
  spawn — child `turn/started` beat the spawn `collabAgentToolCall`
  item/completed

## Worker Rules

- Execute the card exactly; no planning authority; no sub-agents.
- Do NOT touch roadmap/milestone/card/dispatch status files — deliverables +
  batch log only.
- Contract amendment only: no adapter code changes, no test flips (the
  evidence test stays fail-closed until the implementation card).
- Commit on branch `thread/198-contract-045-spawn-confirmation-admission`
  and push with
  `git push -u origin thread/198-contract-045-spawn-confirmation-admission`;
  no merge.

## Scope

- `docs/contracts/045-subagent-topology-observation-and-control.md`: amend
  the Admission paragraph to state, per research note 120's proposal:
  - The qualified admission evidence set widens from "completed spawn
    collaboration item" to also include the provider's spawn-confirmation
    observation — for codex app-server, the parent-envelope
    `subAgentActivity` (`kind=started`) carrying the exact `agentThreadId`.
  - The adapter may not assume ordering between child-lifecycle envelopes
    and admission evidence; child lifecycle for an id already established
    by spawn-confirmation topology evidence is admitted and observed
    without failing the operation. Never-observed ids still fail closed.
  - Keep the amendment additive: observation-only posture unchanged; no
    change to control, callbacks, terminal, or session authority.
- Run the contract drift gates (`effigy docs:contract-drift`,
  `effigy docs:spec-drift`, or this repo's equivalent per AGENTS.md) and
  update any index/summary surfaces the gates or conventions require
  (`contract-index.md`, `contract-summaries.md`).
- Batch log `docs/logs/2026-08-10-contract-045-spawn-confirmation-admission.md`.

Out of scope: adapter implementation, evidence-test flip, any other
contract.

## Acceptance

- [x] contract 045 Admission paragraph states the widened evidence set and
  ordering tolerance, additive and observation-only
- [x] contract index/summary surfaces updated per convention
- [x] drift gates pass
- [x] batch log committed and pushed

## Closeout

Merged to main as `1d3e7c4b` (worker commit `75f70fc7`, grok medium, clean
first run). The amendment matches research note 120's proposal verbatim in
substance: spawn-confirmation observation (`subAgentActivity` kind=started
with exact `agentThreadId`) joins the admission evidence set; no assumed
ordering between child-lifecycle envelopes and admission evidence;
never-observed ids still fail closed. The docs gates also required indexing
research 120 and the 197 evidence log — done on the branch.

## Evidence

- Batch log with the amended paragraph, gate commands + exit states.

## Stop Conditions

- The amendment as scoped conflicts with another contract clause → stop
  with citations
- The drift gates demand changes beyond the admission paragraph → stop and
  report
