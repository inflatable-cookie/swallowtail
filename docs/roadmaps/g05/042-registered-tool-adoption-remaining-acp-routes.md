# g05.042 Registered-Tool Adoption For Remaining ACP Routes

Status: planned; no dispatch authorization
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-09
Depends on: shared registered-tool kernel (card 114 merged); Contract 060-derived lease
Governing refs: Contracts 012, 017, 019, 028, 029, 037, 041, 047, 051, 057, 058, 060-063
Former card: 134 Registered-Tool Adoption For Remaining ACP Routes (folded g05.038; no scope change)

## Outcome

Give the `client_mcp_servers` producer gap on the remaining ACP routes a
durable, honest reference: `cline.acp`, `copilot-cli.acp`, `gemini-cli.acp`
(with `gemini-cli.headless`), `goose.acp`, `kiro.acp`, and `deepagents.acp`.

The shared registration snapshot, bridge lease, and dispatch kernel exist, so
the gap on these routes is no longer "the kernel does not exist". It is that
no route-local adoption has been built and no consumer has required one. Each
route needs its own exact surface evidence before any binding, exactly as the
Claude, Codex, and Grok adoptions did.

## Ready-State Rubric

- [x] Objective is bounded enough to finish without fresh planning decisions if promoted.
- [x] Governing refs point at current canonical surfaces.
- [ ] Per-route scope and review oracle are explicit (below).
- [ ] A consumer requirement plus the operator's direction exists (absent: no dispatch authorization).
- [ ] No unresolved planning gaps beyond that promotion gate.

## Decisions

None. Promotion is per route, never as a batch, and requires a consumer
requirement and the operator's direction, in the same way the Bovine three
were promoted.

## Dispatch manifest

- **State:** planned; not in any dispatch manifest; no worker, no siblings, no concurrency.
- **Completion (if a route is promoted):** exact ACP MCP declaration surface and version segment frozen for that route; mapped to the shared snapshot and bridge without a second registry or lease; proved provider-free; then a real-route gate under the consumer's own test authority.
- **Owned mutable paths:** none until promoted; on promotion, that route's adapter paths, fixtures, guide section, and matrix cell.
- **Reserved closeout surfaces:** `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, `docs/roadmaps/generation-index.md`, `docs/logs/README.md`.
- **Worker:** none (no dispatch authorization).
- **Excluded:** dispatch, runtime, probes, claims, matrix changes; batch promotion across routes.
- **Escalation:** operator via Chatterbox for promotion.

## Work

1. If a route is promoted: freeze the exact ACP MCP declaration surface and version segment for that route.
2. Map it to the shared snapshot and bridge without inventing a second registry or lease.
3. Prove it provider-free; then run a real-route gate under the consumer's own test authority.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Per-route exactness | One route's adoption implies another's | Independent per-route evidence and review |
| No second registry | A route-local registry or lease duplicates the kernel | Adapter diff showing shared-snapshot reuse |
| Matrix honesty | An unavailable `client_mcp_servers` cell is reported as a provider limitation without that route's frozen evidence | Matrix names this task as the producer gap |

## Out Of Scope

This task authorizes no dispatch, runtime, probe, claim, or matrix change.
Its only function is to give those crosses a reference that names who owes
the work and why it is unbuilt. Promotion requires a consumer requirement and
the operator's direction.

## Review Oracle

An unavailable `client_mcp_servers` cell on these routes names this task
because Swallowtail has not built route-local adoption; it must not be
reported as a provider limitation without that route's frozen evidence, and
it must not imply a scheduled lane.

## Stop conditions

- No consumer requirement or operator direction: no dispatch.
- A route's surface cannot map to the shared snapshot without a second registry.

## Evidence

On completion (if ever promoted per route), record: route, frozen surface, validation actually run, PR link, reviewed exact head, merge commit, and material limits.

## Next task

None. This stub authorizes no successor; each promotion is separately compiled by Chatterbox.
