# g06.013 Claude Agent ACP 0.79.0 Useful Newer

Owner: Tom
Created: 2026-09-21
Depends on: Contract 029; Contracts 015 and 038; Research 309, 331/335; completed g05.059
Vision tags: route currentness, Claude Agent ACP, useful-newer, contract review

## Outcome

Qualify the `claude-agent.acp` family from the qualified ceiling `0.76.0`
through official npm/GitHub/ACP-registry stable `0.79.0`, and bring the durable
contract ranges in Contracts 015 and 038 up to the raised ceiling.

## Why It Matters

g05.059 left the ceiling at `0.76.0`. Official stable moved to `0.79.0`. This
lane is the highest review risk in the operator-approved external suite because
it edits durable contract ranges rather than only selection claims and
documentation.

## Ready-State Rubric

- [x] Official npm, GitHub and ACP-registry channels agree through `0.79.0`.
- [x] Every published hop `0.77.0`, `0.78.0` and `0.79.0` was retrieved and
      inventoried, and the unpublished interior points were proved unpublished.
- [x] The identity record names a compatible extension of
      `initialize-meta-extensions-v7`.
- [ ] The Contracts 015 and 038 range amendments are reviewed explicitly.

## Decisions

- One family: `claude-agent.acp` only. The Claude Agent SDK sidecar and Claude
  Code stay separate families.
- Compatible extension: keep the baseline, claim id, behavior revision,
  `AllowUnverified`, exclusion `0.58.0`, and the unmapped pins (ACP SDK
  `1.4.0` holds; the Agent SDK pin and capability-gated compaction stay
  unmapped).
- The contract edits are in scope only as the honest record of the raised
  qualified range. Any change beyond that range statement needs a scoped
  ruling before merge.
- Later `UnverifiedNewer` is the next unpublished stable.
- Host `claude` is absent from `PATH`. Missing host is not a gap. Do not
  install, and do not execute downloaded binaries. No provider prompt or live
  session.
- Do not edit the Next Task pointer, generation status, handoffs, or northstar
  loop state.

## Dispatch manifest

- **State:** ready; one family useful-newer lane entering at review, with
  explicit contract-review attention.
- **Owned mutable paths on revision:** this PR's existing Claude Agent ACP
  selection source, its ACP identity and delta-ledger tests and fixtures under
  `crates/swallowtail-adapter-claude-agent/`, the Claude Agent prepared guide,
  the affected provider and activity matrix rows, Contracts 015 and 038 **only
  as the qualified-range record**, the CHANGELOG `[Unreleased]` entry, the
  lane's research record and its indexes/logs.
- **Excluded:** other families, the Claude Agent SDK sidecar, Claude Code, the
  watcher, any new contract rule beyond the range statement, workflow,
  dependency, release, provider mutation, the Next Task pointer,
  `docs/roadmaps/README.md`, generation runway and lifecycle projection.
- **Escalation:** operator via Chatterbox for any contract question; Queue
  coordinator for mechanical blockers.

## Work

1. Freeze identity for `0.79.0` with the complete dist inventory and the
   unpublished-interior proof. No production claim.
2. Recheck official stable. If it moved before the claim landed, extend hops
   under Contract 029 In-Run Latest Movement.
3. Apply the compatible-extension claim and record the raised range in
   Contracts 015 and 038, changing nothing else in either contract.
4. Run focused adapter validation and the named docs gates.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| The window extends through `0.79.0` | A range is inferred past the current stable | selection claims and both contract statements name the raised ceiling |
| Contract edits only record the range | A contract rule changes beyond the range statement | contract diff shows the range line and nothing else |
| Unmapped surfaces stay unmapped | The Agent SDK pin or compaction silently becomes mapped | claim revision and matrix rows unchanged for those surfaces |
| No provider or host mutation | The CLI is installed or an artifact executed | absence from `PATH` recorded; artifacts hashed only |

## Stop conditions

Stop and ask if a hop changes the selected ACP initialize, mode, permission, or
elicitation shape; if the npm, GitHub and registry channels disagree; if one
contract cannot express the raised range without a new rule; or if the contract
diff is broader than the range statement.

## Evidence

The lane's research record (renumbered from the colliding `331` on admission).
Fixtures under
`crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.79.0/`.
Probe artifacts live outside the repository.

## External PR Review Entry Authority

Tom authorized the Queue Oracle on 2026-09-21 to prepare and take over the
remaining externally authored currentness PRs through review entry, accepted the
pilot-first sequence, attested Grok 4.6 as the author model for the cloud
sessions, and directed that this Claude Agent ACP PR be held until last with
its Contracts 015 and 038 edits receiving explicit scope and review attention.

The external work and its evidence are submitted claims, not independent
acceptance. Tom declared Grok 4.6 as the external author model; canonical
declaration: `xai/grok-4.6`, GitHub author `betterthanclay`, authoring system
Cursor cloud driven by Grok Bot.

Queue starts an independent reviewer, not a worker. Changes requested authorize
one ordinary revision loop in the same workspace and branch, scoped to this
Claude Agent ACP qualification and its existing changed surfaces. Queue owns
merge and lifecycle closeout. Intake preparation may publish this planning and
merge it into the existing branch; the exclusions above constrain revision
workers, not this operator-approved planning preparation or hook-owned closeout.

Canonical handoff: `docs/handoffs/20260921-g06-013-external-pr-review-entry.md`.

Preserve the external branch and workspace after completion. This is the last
admission of the operator-approved suite; the additional external pull requests
that appeared later are not dispatched by this approval. Research numbers `331`
through `334` are retained by earlier lanes; this lane takes the next free
number.
