# g06.037 Kiro ACP Consumer HTTP MCP Wiring

Owner: Tom
Created: 2026-09-25
Depends on: Contract 063 (Consumer-Supplied HTTP MCP Placement); Research 351; g06.019 as the pattern
Vision tags: MCP placement, ACP routes, production MCP, Longhorn

## Outcome

Wire the consumer-supplied streamable-HTTP MCP entry into `kiro.acp`
production `session/new` at the route's current point (`2.21.4` in Research
351). Provider-free tests only.

## Why It Matters

Research 351 classified `kiro.acp` as a `direct-http-candidate` from its shipped
artifact. Wiring the entry is the step before a live gate can prove this
harness reaches Longhorn's Contract 022 `agent-control` MCP without the stdio
carrier. Tom approved compiling the candidate wiring tasks on 2026-09-24.

## Ready-State Rubric

- [x] Contract 063 admits the placement shape.
- [x] Research 351 cites the provider's accepted forms, header path and gates.
- [x] g06.019 (`opencode.acp`) is the reference implementation.
- [x] Scope, acceptance, validation, evidence and stop conditions are explicit.

## Decisions

- Emit `type: "http"` with headers. The provider drops session-injected servers when MCP is governance-disabled and its `initialize` advertisement is unproven, so the route must surface a typed outcome when the entry is dropped and must not claim acceptance from emission.
- Follow the g06.019 shape: validate structure only (non-empty name, absolute
  `http`/`https` URL, well-formed header names); pass values verbatim; one
  consumer entry per session under a route-owned name; URL and header values
  never reach failures, `Debug`, activity, receipts or fingerprints.
- Omission keeps today's `mcpServers` behaviour byte-identical.
- Agent-configured servers are not overridden silently: record ACP-entry precedence in the guide.
- Emission is not honouring: the `client_mcp_servers` cell stays unavailable
  as a producer gap naming the live gate, not `Yes`.

## Dispatch manifest

- **State:** ready; implementation lane; independent of its sibling wiring
  lanes.
- **Completion:** production `session/new` carries a validated HTTP entry;
  omission, refusal and redaction tests pass; guide and matrix text state
  emission only; independent exact-head review accepts the head.
- **Owned mutable paths:** `crates/swallowtail-adapter-kiro/**`; `docs/guides/kiro-acp-prepared-integration.md`; the `kiro.acp` route
  and feature matrix rows; the working public-API baseline for `swallowtail-adapter-kiro` if
  its API changes; `CHANGELOG.md` `[Unreleased]`; `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Excluded:** this card's prose; contracts; other routes; core vocabulary
  changes (stop and ask); any live session; release, tag, publication.
- **Worker evidence:** the authenticated run result.
- **Escalation:** operator via Chatterbox for scope; Queue coordinator for
  mechanical blockers.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Values verbatim | URL normalised or a header rewritten | encoder test compares input to wire |
| Secrets private | A bearer appears in a failure, `Debug` or activity | redaction tests per surface |
| Omission unchanged | An empty declaration changes the wire | omission fixture byte-identical |
| Emission is not honouring | A cell says `Yes` | cell stays a producer gap naming the live gate |
| Provider gates surfaced | A dropped or refused entry looks accepted | typed outcome test for each Research 351 gate |

## Stop conditions

Stop and ask if the wiring needs a shared core or runtime vocabulary change,
or if the route's current point differs from Research 351 in the seam.

## Evidence

Research 351. Focused and affected-package validation for `swallowtail-adapter-kiro`, route and
docs QA.

## Next Task

Chatterbox asks the operator whether to run a live gate for `kiro.acp`.
