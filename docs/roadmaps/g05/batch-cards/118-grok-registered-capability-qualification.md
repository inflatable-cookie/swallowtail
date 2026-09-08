# 118 Grok common capability qualification

Status: complete; PR 281 merged at `98543d0b`; provider-free callable seam built on the merged Card114/115 kernel; Contract 061 row stays Unqualified with real_route_gate_pending and the live gate remains separate
Owner: Tom
Created: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`

## Admission Evidence — 2026-09-07

Four live attempts across `1.0.4` and `1.0.5` (two on card 128's harness, two
on card 133's repaired oracle) all recorded `client_mcp_admitted` true,
`client_mcp_tools_listed` true, and on the reruns `echo_helper_live` true.
Grok Build accepts a client-supplied ACP `mcpServers` declaration, spawns the
server, connects it, and enumerates its tools. No attempt has produced a
`tools/call`, and the reruns did not establish a session at all
(`session_new_unanswered`), which card 137 attributes to the probe not
answering client requests during `session/new`.

That is sufficient to build the route-local seam provider-free under Research
289's adapter mapping, in the same posture cards 116 and 125 used for Claude:
a callable seam with the Contract 061 row held `Unqualified` until a real
route gate passes. It is not sufficient to claim support, and no matrix cell
moves on it.

## Goal

Preserve merged one-shot permission semantics. Freeze exact Grok consumer-tool surface before implementing registration; an unsupported finding is truthful but does not complete operator full-MCP goal. No empty-list-to-support flag change.

## Scope

crates/swallowtail-adapter-grok/**; route research/fixtures and its public API baseline. Coordinator reserves all docs front doors, contracts, roadmap/card
status, shared Cargo manifests and unrelated owners' paths for serial closeout.
No product repository, credential/global settings, live client data or release.

## Acceptance And Review Oracle

Contract063 is the falsification oracle, including binding/revocation, bounded
payloads, lifecycle/cancellation/unknown outcomes and no mutating replay.
Exact-head independent cross-model review must inspect actual mounted/callable
wiring, not just isolated type fixtures. Missing evidence remains a blocked
capability. Public API changes are additive; incompatible surfaces stop before
mutation. Provider cards prove their exact route only, never another provider.

## Validation

- `effigy validate:focused swallowtail-adapter-grok`
- `effigy package:verify-affected swallowtail-adapter-grok`
- `effigy qa:docs` and `git diff --check`
- Card114 retains unchanged watcher and Claude Code compatibility fixtures;
  coordinator adds `swallowtail-adapter-claude-agent` to its focused compatibility
  round. No broad workspace reruns solely for unrelated docs movement.
- Real route probes use disposable inputs only after deterministic checks and
  explicit exact-route access authority. No tag/release authority in this card.

## Auto-Continuation

Coordinator dispatches the next ready manifest lane after accepted merge and
clean-main closeout. Worker stops at review; it never merges or releases.

## Independent Evidence Preparation

The retained route owner may start bounded read-only protocol/corpus research
now, before114/115. Publish exact surface evidence and proposed adapter mapping;
no runtime edits, live credential mutation or support claim in that preparation.
Runtime adoption waits for114/115 and a qualified surface. Missing evidence is
the preparation outcome to resolve, not a prerequisite for starting research.

## Result

The route-local seam is callable and provider-free. `GrokRegisteredToolBinding`
qualifies one selection, and the Swallowtail-owned mediated-stdio courier is
declared as one reserved entry in the ACP `session/new` `mcpServers` list. Grok
spawns that child; this route owns the bridge lease only.

The lease is bound to one exact turn attempt, which is Contract 063's one active
provider turn per server lease. Cancellation settles rather than observes, so an
outstanding call is cancelled; terminal, cancellation, deadline, and the
transport-failure path all settle before the consumer sees terminal; a lease the
host cannot join fails its own turn and refuses every later one. Settlement is
serialized, so concurrent settlers share one cleanup truth. Opening is bounded
by the lesser of the caller's deadline and Contract 063's ten-second ceiling,
including the courier ready barrier. A failed registered cleanup never becomes a
clean close and retains the working resource and credential at session close, at
open abort, and at the ready barrier.

Omission is byte-identical: an open without a binding still sends
`mcpServers: []` and creates no lease, listener, or courier.

Nothing here claims support. Contract 061 stays `Unqualified` with
`real_route_gate_pending`, no feature-matrix cell moves, and the four Card 128
`evidence_pending` cells are untouched. The disposable real-route gate is
separately authorized and has not run.

Five exact-head cross-model review rounds raised twelve blockers, all repaired:
turn-lifecycle binding, cleanup-truth propagation and ownership retention,
bounded open including the ready barrier, failed-start settlement, and
serialized settlement. Three of those rounds corrected the evidence rather than
the implementation, so the concurrency guarantee is now a unit test that drives
both settlements by hand with no scheduling in it.
