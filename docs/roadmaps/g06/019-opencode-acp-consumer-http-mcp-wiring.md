# g06.019 OpenCode ACP Consumer HTTP MCP Wiring

Owner: Tom
Created: 2026-09-24
Depends on: Contract 063 (Consumer-Supplied HTTP MCP Placement); Research 337; completed g06.016
Vision tags: OpenCode ACP, MCP placement, production MCP, Longhorn

## Outcome

Wire the consumer-supplied streamable-HTTP MCP entry into `opencode.acp`
production `session/new`, so a consumer can point OpenCode straight at
Longhorn's Contract 022 `agent-control` server with no stdio carrier.
Provider-free tests only.

## Why It Matters

g06.016 built the route and modelled the URL-plus-header shape behind a
refusal (`mcp_remote_not_admitted`) because no contract admitted it. Contract
063 admitted the shape on 2026-09-24. `opencode.acp` is the first route whose
provider takes the production MCP directly.

## Ready-State Rubric

- [x] Contract 063 admits the placement shape and names `opencode.acp` as its
      first route.
- [x] Research 337 freezes the provider's `http`/`sse` acceptance and header
      forwarding.
- [x] The modelled type and its refusal exist in
      `crates/swallowtail-adapter-opencode/src/acp/mcp.rs`.
- [x] Scope, acceptance, validation, evidence and stop conditions are explicit.

## Decisions

- Emit the ACP `http` form only. `sse` stays modelled and unemitted.
- A session declares at most one consumer MCP entry, stdio or HTTP, under the
  route-owned name; both at once is a typed refusal.
- Validate structure only: non-empty name, absolute `http`/`https` URL,
  well-formed header names. Pass values verbatim.
- URL and header values never reach failures, diagnostics, activity, receipts,
  `Debug` output, or plan fingerprints.
- Feature cells for honouring stay gated: emission is proven, a working remote
  call is not. The Contract 061 placement projection names the HTTP placement.

## Dispatch manifest

- **State:** ready; implementation lane; independent of g06.018 and g06.020.
- **Completion:** production `session/new` carries a validated HTTP entry;
  refusal and redaction tests pass; guide and matrix rows state emission, not
  honouring; independent exact-head review accepts the head.
- **Owned mutable paths:** `crates/swallowtail-adapter-opencode/**`; the
  OpenCode ACP prepared guide; the `opencode.acp` route and feature matrix
  rows; `CHANGELOG.md` `[Unreleased]`; `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Excluded:** this card's prose; contracts; `opencode.http`; other routes;
  core vocabulary changes beyond what the adapter needs (stop and ask);
  Longhorn; any live session; release, tag, publication.
- **Worker evidence:** the authenticated run result.
- **Escalation:** operator via Chatterbox for scope; Queue coordinator for
  mechanical blockers.

## Work

1. Replace the refusal with a production encoder for the admitted `http` form.
2. Extend the prepared facade so a consumer declares one HTTP entry.
3. Add refusal tests (invalid URL, bad header name, two entries, wrong name)
   and redaction tests over every failure and debug path.
4. Update the Contract 061 placement projection, guide, and matrix rows.
5. Run the named validation.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Values are verbatim | The URL is normalised or a header rewritten | encoder test compares exact input to wire |
| Secrets stay private | A bearer appears in a failure, `Debug`, or activity | redaction tests over each surface |
| One entry per session | Stdio and HTTP both declared | typed refusal |
| Emission is not honouring | A matrix cell says `Yes` for remote tool calls | the cell stays gated with its live-gate reason |
| HTTP route untouched | `opencode.http` code or tests change | no such path in the diff |

## Stop conditions

Stop and ask if the wiring needs a shared core or runtime vocabulary change, or
if the frozen evidence does not support a header shape the consumer needs.

## Evidence

Research 337. Focused and affected-package validation for
`swallowtail-adapter-opencode`, route and docs QA.

## Next Task

Chatterbox asks the operator whether to run a live gate: one declared remote
entry against a loopback streamable-HTTP server and one tool call.
