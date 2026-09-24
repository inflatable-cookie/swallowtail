# g06.035 Gemini CLI ACP Consumer HTTP MCP Wiring

Owner: Tom
Created: 2026-09-25
Depends on: Contract 063 (Consumer-Supplied HTTP MCP Placement); Research 351; g06.019 as the pattern
Vision tags: MCP placement, ACP routes, production MCP, Longhorn

## Outcome

Wire the consumer-supplied streamable-HTTP MCP entry into `gemini-cli.acp`
production `session/new` at the route's current point (`0.59.0` in Research
351). Provider-free tests only.

## Why It Matters

Research 351 classified `gemini-cli.acp` as a `direct-http-candidate` from its shipped
artifact. Wiring the entry is the step before a live gate can prove this
harness reaches Longhorn's Contract 022 `agent-control` MCP without the stdio
carrier. Tom approved compiling the candidate wiring tasks on 2026-09-24.

## Ready-State Rubric

- [x] Contract 063 admits the placement shape.
- [x] Research 351 cites the provider's accepted forms, header path and gates.
- [x] g06.019 (`opencode.acp`) is the reference implementation.
- [x] Scope, acceptance, validation, evidence and stop conditions are explicit.

## Decisions

- Emit `http`, which the provider maps to `httpUrl`, with headers in `requestInit.headers`. The entry is only honoured after `authenticate` completes; an unauthenticated session must fail typed, not drop the entry.
- Follow the g06.019 shape: validate structure only (non-empty name, absolute
  `http`/`https` URL, well-formed header names); pass values verbatim; one
  consumer entry per session under a route-owned name; URL and header values
  never reach failures, `Debug`, activity, receipts or fingerprints.
- Omission keeps today's `mcpServers` behaviour byte-identical.
- `gemini-cli.headless` and its MCP-disabled path stay untouched.
- Emission is not honouring: the `client_mcp_servers` cell stays unavailable
  as a producer gap naming the live gate, not `Yes`.

## Dispatch manifest

- **State:** ready; implementation lane; independent of its sibling wiring
  lanes.
- **Completion:** production `session/new` carries a validated HTTP entry;
  omission, refusal and redaction tests pass; guide and matrix text state
  emission only; independent exact-head review accepts the head.
- **Owned mutable paths:** `crates/swallowtail-adapter-gemini/**` for `gemini-cli.acp` only; `docs/guides/gemini-cli-prepared-integration.md`; the `gemini-cli.acp` route
  and feature matrix rows; the working public-API baseline for `swallowtail-adapter-gemini` if
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

Research 351. Focused and affected-package validation for `swallowtail-adapter-gemini`, route and
docs QA.

## Next Task

Chatterbox asks the operator whether to run a live gate for `gemini-cli.acp`.
