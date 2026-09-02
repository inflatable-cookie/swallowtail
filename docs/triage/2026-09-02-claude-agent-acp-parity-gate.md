# Claude Agent ACP Parity Census And Delivery Gate

Status: complete; evidence-only; no implementation authority
Owner: Tom
Date: 2026-09-02
Source: card 054, Research 277, Research 279, qualified ACP `0.53.0..=0.73.0`

## Result

Research 279 completes the no-filter census for the qualified
`claude-agent.acp` bridge. The route has a useful existing featureful subset,
but the advertised bridge surface is wider than the admitted Swallowtail
surface. The largest independently deliverable next tranche is exact
negotiated model-options observation on the existing adapter-owned projected
session-open seam.

This gate does not change production code, public APIs, contracts, claims,
matrices, fixtures, package pins, or release state. It does not contact a
provider or run a live session.

## Exact First-Tranche Gate

The orchestrator may compile one later adapter implementation card for
`swallowtail-adapter-claude-agent`:

- retain one exact bounded `configOptions[id=model]` snapshot after the
  existing model confirmation;
- expose it through the existing
  `InteractiveSessionHandle::negotiated_model_options` seam;
- add the established observation-only
  `feature.negotiated-model-options-observation` row only on the additive
  projected open path, with a distinct active-session source;
- keep the preserved `open_session` signature and shared lifecycle unchanged;
- treat required missing `configOptions[id=model]` as existing confirmation
  failure on both public opens; treat preserved-path snapshot-detail
  malformation as no snapshot; treat projected-path snapshot-detail
  malformation as close-and-fail with no contribution;
- keep this as negotiated session evidence, never a model catalogue,
  selectable control, provider registry, or mid-session mutation;
- do not add the row to load/resume until their response and lifecycle paths
  have the same exact parser and proof.

The existing runtime bounded type, source kinds, lifecycle, and projection
identity are sufficient. This is a proposal, not implementation authority;
implementation-card compilation belongs to the orchestrator after the native
SDK lane and joint review.

## Explicit Negative Set

These rows stay withheld and remain separate from the first tranche:

- read-write interactive access: current interactive plans bind read-only;
- session permission/mode mutation: current permission is a fixed session
  policy and `set_mode` is private setup with transport-only empty response;
- mid-session model/effort controls: no active-session config-control facade;
- Bash/terminal: no selected terminal callback or host containment proof;
- client MCP servers: the adapter sends `mcpServers: []` and exposes no
  server/process authority;
- auth readiness, subscription/login/logout, and packaging;
- list, fork, archive, restore, and provider-session import;
- images, embedded context, `@` mentions, slash-command execution, prompt
  queueing, steering, and subagent topology/control;
- persistent permission choices `allow_always`/`reject_always`;
- active effective Plan acknowledgement and model-options observation on
  load/resume;
- raw ACP payloads, untyped metadata, billed cost, or secure deletion.

Research 279 is the authoritative row-by-row disposition for this set. No
native SDK feature is attributed to ACP.

## Review Oracle

Fail a later implementation or promotion if it:

- publishes a model snapshot as a catalogue or selectable model control;
- accepts duplicate, malformed, unbounded, current-missing, or
  current-not-in-options model data;
- turns a transport-only `{}` from `set_mode` into effective mode state;
- changes `open_session` failure/cleanup behavior while adding projection;
- publishes the active row from the prepared source or reuses the prepared
  and active source ids;
- carries active model observation through load/resume without exact response,
  binding, and lifecycle proof;
- converts one-shot permission selection into persistent global authority;
- treats `mcpServers: []`, cwd, tool lists, `AmbientHost`, or process-group
  cleanup as MCP identity or terminal containment;
- treats provider subagent updates, available commands, or steering metadata
  as consumer control or stable child topology;
- attributes native SDK or Claude Code behavior to `claude-agent.acp`;
- adds a shared type, contract rule, public facade, provider claim, or live
  evidence without the orchestrator's later integration decision.

Required proof for C1 is provider-free and exact: matching, missing required
model entry (confirmation failure on both opens), snapshot-detail
malformation, duplicate option values, unbounded, current-not-in-options,
source identity disagreement, preserved-open parity, projected cleanup,
load/resume omission, and catalogue negative cases. Successful-open Absent
does not apply: confirmation already requires the model entry.

## Validation Boundary

The card's documentation-only validation is:

- `effigy qa:docs:index:research`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g05`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:links`
- `effigy qa:northstar`
- `git diff --check`

No provider contact, live ACP probe, release command, or broad test suite is
part of this gate.

## Authority

- [Research 279](../research/279-claude-agent-acp-capability-census-and-tranche-selection.md)
- [Research 277](../research/277-claude-subscription-dual-route-direction.md)
- [Contract 015](../contracts/015-acp-v1-negotiation-and-client-callbacks.md)
- [Contract 017](../contracts/017-provider-owned-session-load-replay-and-host-containment.md)
- [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 038](../contracts/038-provider-session-management-and-consumer-thread-boundary.md)
- [Contract 041](../contracts/041-input-callback-and-provider-tool-admission.md)
- [Contract 047](../contracts/047-configured-provider-instance-catalogue.md)
- [card 054](../roadmaps/g05/batch-cards/054-claude-agent-acp-parity-census-and-delivery-gate.md)
