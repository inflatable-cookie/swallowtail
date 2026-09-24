# g06.014 Consumer-Supplied HTTP MCP Acceptance Per Route

Owner: Tom
Created: 2026-09-22
Depends on: Contract 029; Contracts 012, 017, 019, 041, 063; Spec 014; g06.013
Vision tags: route truth, MCP, harness capability, Longhorn boundary

## Outcome

Establish, for every route that documents an MCP client configuration seam,
whether it accepts a **consumer-supplied streamable-HTTP MCP server entry** —
loopback URL, per-instance bearer header, origin-constrained — and publish the
resulting route list and truthful feature-matrix cells. The list decides which
routes connect directly to the consumer's production MCP and which need
Longhorn's stdio carrier.

## Why It Matters

Longhorn's 2026-09-22 direction makes the Contract 022 `agent-control` server
the production MCP and withdraws the registered-tool bridge's production role,
leaving Swallowtail as harness-side MCP client configuration only. Longhorn is
building a stdio carrier for the single 022 instance and validates it against
the routes this lane names, so the carrier is sized against measured need rather
than assumption.

Today no route is proven to accept a consumer-supplied HTTP MCP entry. The only
MCP client surfaces Swallowtail admits are two Swallowtail-owned stdio couriers
(`claude-agent.sdk`, `grok-build.acp`), and every other route's
`client_mcp_servers` cell is a provider limitation.

## Ready-State Rubric

- [x] The production MCP boundary is settled and recorded in Contract 063 and
      Spec 014.
- [x] The current MCP client surfaces are known: two Swallowtail-owned stdio
      couriers, everything else a provider limitation.
- [x] The question is bounded to acceptance of a consumer-supplied HTTP entry,
      not to implementing one.
- [x] Scope, acceptance, validation, evidence, and stop conditions are explicit.
- [x] Review oracle is present because the result is exact, per-route, and
      partly negative.

## Decisions

- This lane produces **evidence and matrix truth**, not a connector. It adds no
  listener, courier, registry, lease, or production MCP server.
- Acceptance means a harness can be configured with a streamable-HTTP MCP server
  entry carrying a loopback URL and a bearer header, and that the entry is
  honoured rather than ignored or rejected.
- A route that cannot is stated as needing the Longhorn stdio carrier, or as a
  provider limitation. It must not be described as HTTP-capable on inference.
- `subscriptions/listen` and the `longhorn://agent-control/...` resources are
  deliberately **not claimed** in this pass. Record them as typed unsupported
  unless a named route needs them, and give the reopen condition.
- No provider prompt, login, install, or host update. Frozen provider artifacts,
  published documentation, and CLI help are the evidence; a live gate would need
  separate operator authorization that this lane does not have.

## Dispatch manifest

- **State:** ready; evidence lane; one lane; no automatic successor.
- **Completion:** a research record classifies every MCP-seam route as
  `direct-http`, `carrier-required`, or `provider-limitation` with its evidence
  citation, the feature-matrix `client_mcp_servers` cells agree with it, the
  Longhorn stdio carrier is named as the dependent where a route needs it, and
  the route list is published so Chatterbox can hand it to Longhorn.
- **Owned mutable paths:** the new research record and one research-index line;
  the `client_mcp_servers` cells and their cross-references in
  `docs/guides/provider-solution-feature-matrix.csv`; the affected prepared
  guides where they state MCP transport support; the standing-lane or
  architecture note that records the boundary consequence; `PAPERCUTS.md`
  append only.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Worker:** evidence-first route-currentness worker; frozen-artifact and
  published-documentation analysis; no provider credentials.
- **Excluded:** implementing any connector, carrier, or client; changing a
  route's admission; Contracts 060 or 063 semantics; Longhorn code; release,
  tag, or publication; any live probe.
- **Escalation:** operator via Chatterbox for a route whose acceptance cannot be
  settled without a live session; Queue coordinator for mechanical blockers.

## Work

1. Inventory every route with a documented MCP client configuration seam, from
   Contracts 012, 017, 041 and 063, the prepared guides, and each route's frozen
   provider artifacts. Record the seam's exact shape per route.
2. Classify each route for a consumer-supplied streamable-HTTP entry: transport
   types accepted, whether arbitrary URLs are honoured, whether headers or a
   bearer can be supplied, and whether the entry can be supplied per run rather
   than only through ambient configuration.
3. Record the two existing Swallowtail-owned stdio couriers separately, since
   they are non-production and do not evidence HTTP acceptance.
4. Publish the route list and reconcile the matrix cells and any guide statement
   that overstates transport support.
5. Stop and record a typed gap where acceptance cannot be settled provider-free.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Every MCP-seam route is classified | A route with a config seam is omitted or inferred HTTP-capable | inventory covers the contracts' seam list with a citation each |
| Classification rests on provider evidence | Acceptance asserted from a tool name or a sibling route | each cell cites frozen artifacts or published documentation |
| The boundary is not widened | The lane adds a listener, courier, or client | no production surface added; matrix and guide edits only |
| Longhorn's dependency is named | A carrier-required route is not reported | the route list names it as a carrier dependent |

## Stop conditions

Stop and ask if a route's acceptance cannot be settled without a live provider
session, if two routes disagree on the same mechanism, or if classifying a route
would require changing its admission or a contract's semantics.

## Evidence

The new research record, the route list, and the reconciled matrix cells and
cross-references. Provider artifacts are hashed and never executed.

## Next Task

Chatterbox sends the published route list to the Longhorn thread so the stdio
carrier's acceptance evidence covers exactly the routes that need it, then
reconciles the standing lane.
