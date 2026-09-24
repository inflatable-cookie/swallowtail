# g06.031 ACP Routes HTTP MCP Acceptance Evidence

Owner: Tom
Created: 2026-09-24
Depends on: Contract 063 (Consumer-Supplied HTTP MCP Placement); Research 336, 337, 349; g06.005
Vision tags: MCP placement, ACP routes, production MCP, Longhorn

## Outcome

For each remaining ACP route, settle from frozen artifacts whether the
provider accepts a consumer-supplied streamable-HTTP MCP entry, and propose the
per-route wiring task for every route that does. Evidence only; no route code.

## Why It Matters

`opencode.acp` proved that a harness can reach Longhorn's Contract 022
`agent-control` MCP directly (Research 349). Every other ACP route that can do
the same drops Longhorn's stdio carrier from its path. Research 336 left six
ACP routes as producer gaps with carrier candidacy open; this settles them.
It advances g06.005 without replacing it.

## Ready-State Rubric

- [x] Contract 063 admits the placement shape and sets per-route admission
      on frozen evidence.
- [x] Research 337 is the method template: ACP `session/new` `mcpServers`
      forms, `mcpCapabilities`, and header forwarding from shipped source.
- [x] Tom approved the lane on 2026-09-24.
- [x] Scope, acceptance, validation, evidence and stop conditions are explicit.

## Decisions

- Routes: `cline.acp`, `copilot-cli.acp`, `gemini-cli.acp`, `goose.acp`,
  `kiro.acp`, `deepagents.acp`, `claude-agent.acp`, and `grok-build.acp` at
  its current maintained ceiling. Use each route's current qualified or
  official point.
- Per route, record from the shipped artifact: advertised
  `mcpCapabilities`, accepted `mcpServers` forms (`stdio`, `http`, `sse`),
  whether declared headers reach the transport, and any gating flag.
- Classify each route `direct-http-candidate`, `carrier-required`
  (stdio only), or `provider-limitation` (no client MCP seam), with the
  citation.
- For each `direct-http-candidate`, write a proposed wiring outline (route,
  version, forms, gates) in the research record. Chatterbox compiles the
  implementation tasks; this lane edits no route code.
- Update Research 336's route list only by a dated addendum line pointing at
  the new record.
- Provider-free: hash and read artifacts only.

## Dispatch manifest

- **State:** ready; evidence lane; independent of g06.032.
- **Completion:** every listed route is classified with a citation; guides and
  matrix cells unchanged except typed-gap references; independent exact-head
  review accepts the head.
- **Owned mutable paths:** one new research record with any fixtures under
  `docs/research/`, its index line; a dated addendum line in Research 336;
  the `client_mcp_servers` gap references for the listed routes in the feature
  matrix and cross-evidence records; `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Excluded:** this card's prose; route code; contracts; any matrix cell
  moving to available; `opencode.acp`; Longhorn; any live session; release,
  tag, publication.
- **Worker evidence:** the authenticated run result and the research record.
- **Escalation:** operator via Chatterbox for scope; Queue coordinator for
  mechanical blockers.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Evidence from shipped source | A form is inferred from docs or a schema | shipped-source citation per route |
| No claim moves | A cell becomes available | no available cell in the diff |
| Every route classified | A listed route is skipped | one row per listed route |
| Headers proven, not assumed | `http` accepted but headers dropped goes unnoticed | header path cited or recorded as unproven |

## Stop conditions

Stop and ask if a route's artifact cannot be obtained without login or
install, or if a route needs a contract decision beyond Contract 063.

## Evidence

Research 336, 337, 349; the new record. `effigy qa:docs`, `effigy qa:routes`.

## Next Task

Chatterbox compiles one wiring task per `direct-http-candidate` and updates
Longhorn's route list.
