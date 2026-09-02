# 054 Claude Agent ACP Parity Census And Delivery Gate

Status: complete; evidence-only; implementation deferred to orchestrator integration
Owner: Tom
Created: 2026-09-02
Milestone: `../022-claude-agent-dual-route-parity.md`
Depends on: Research 277; qualified Claude Agent ACP `0.53.0..=0.73.0`

## Goal

Map the complete featureful-session surface of the qualified Claude Agent ACP
bridge and select the largest honest, independently deliverable expansion of
`claude-agent.acp` without flattening native SDK semantics.

## Scope

1. Re-derive the exact current Swallowtail ACP route surface from production
   source, tests, matrices, and qualified corpora. Do not trust the triage gap
   list as complete.
2. Inventory the qualified bridge's read-write interactive tools, permission
   requests and persistent choices, `session/set_mode`,
   `session/set_config_option`, Bash/terminal methods, client MCP servers,
   load/resume/list/fork/close/delete, images, embedded context, @-mentions,
   slash commands, edit review, prompt queueing, steering, usage, auth, and
   subagent transcript metadata.
3. For every capability, classify protocol presence, Swallowtail parsing,
   prepared-plan admission, effective acknowledgement, active observation,
   lifecycle ownership, process authority, public facade, consumer projection,
   and withholding reason.
4. Select bounded delivery tranches. Prefer read-write interactive access,
   session-scoped permission/mode control, and mid-session model/effort controls
   first. Keep Bash/terminal behind consumer mediation and host-process proof.
   Keep client MCP, auth readiness, packaging, session management, attachments,
   commands, and subagent projection distinct when their dependencies differ.
5. Identify exact shared-contract or public-API gaps. Do not invent those
   surfaces; return them to the joint orchestrator integration step.
6. Write Research 279 and an ACP delivery gate with exact first-tranche scope,
   negative set, validation, review oracles, and implementation-card proposal.
   Do not change production code, claims, matrices, fixtures, or package pins.

## Acceptance Criteria

- complete route and bridge capability partition with no filter or exception
  list
- exact current support/withholding proof per capability
- at least one useful first tranche, or an honest evidence stop proving why no
  tranche can ship independently
- terminal/process, permission, acknowledgement, session, and projection
  boundaries remain distinct
- no native SDK-only feature is attributed to ACP
- no provider contact or live authenticated session

## Validation

- `effigy qa:docs:index:research`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g05`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:links`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: every proposed ACP feature has protocol, admission, effectiveness,
observation, lifecycle, and authority proof appropriate to its claim.

Smallest counterexample: `set_mode` dispatches but its effective value is
discarded, a session permission choice silently becomes a global allow, client
MCP identity changes on resume, or Bash executes outside the mediated cwd and
consumer callback boundary.

Required proof: exact capability matrix, source/wire cross-references,
prepared/active state transitions, mixed-session counterexamples, process and
MCP identity tests, and explicit withheld rows.

## Auto-Continuation

No. Implementation compilation follows joint review with card 053.

## Closeout

Research 279 and the ACP parity delivery gate complete the full capability
partition for qualified `claude-agent.acp`. The census covers protocol
presence, parsing, prepared admission, effective acknowledgement, active
observation, lifecycle, host/process authority, public facade, consumer
projection, and withholding for every capability named by this card.

The largest independent next tranche is exact negotiated model-options
observation on the existing adapter-owned projected session-open seam. It
retains one bounded `configOptions[id=model]` snapshot through the existing
`InteractiveSessionHandle::negotiated_model_options` seam and publishes an
observation-only active-session row only after exact validation. It does not
create a model catalogue or mid-session control. Read-write interactive
access, session permission/mode mutation, mid-session model/effort controls,
terminal/Bash, MCP, auth, management expansion, richer prompt content,
queueing, steering, and subagent topology remain explicitly withheld for the
joint integration step.

No Rust, claim, matrix, fixture, contract, package pin, release, provider, or
shared front-door surface changed. No provider contact or live probe occurred.
The exact worker head, validation transcript, and PR identity are reported in
the worker handoff and final closeout; this card does not authorize merge or
implementation.

## Evidence

- [Research 279](../../../research/279-claude-agent-acp-capability-census-and-tranche-selection.md)
- [ACP parity delivery gate](../../../triage/2026-09-02-claude-agent-acp-parity-gate.md)
- [ACP parity lane log](../../../logs/2026-09-02-claude-agent-acp-parity-gate.md)
- [Research 277](../../../research/277-claude-subscription-dual-route-direction.md)
- [Contract 015](../../../contracts/015-acp-v1-negotiation-and-client-callbacks.md)
- [Contract 017](../../../contracts/017-provider-owned-session-load-replay-and-host-containment.md)
- [Contract 029](../../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 038](../../../contracts/038-provider-session-management-and-consumer-thread-boundary.md)
- [Contract 041](../../../contracts/041-input-callback-and-provider-tool-admission.md)
- [Contract 047](../../../contracts/047-configured-provider-instance-catalogue.md)

## Stop Conditions

Stop on an unqualified bridge dependency, provider contact need, missing
effective acknowledgement, unbounded process or MCP authority, a required
shared public API decision, or overlap with the SDK worker's owned surfaces.
