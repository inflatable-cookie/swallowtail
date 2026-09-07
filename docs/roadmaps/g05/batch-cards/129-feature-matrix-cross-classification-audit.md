# 129 Feature Matrix Cross Classification Audit

Status: ready
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: the operator's 2026-09-07 ruling that an unexplained cross is a work item, not a result; `docs/guides/provider-solution-feature-matrix.csv`; Contract 061

## Rule (operator, 2026-09-07)

Every unavailable cell in the feature matrix is exactly one of:

- **provider limitation**: the provider or harness cannot do it on the
  qualified version, with a citation to the frozen evidence (research doc,
  corpus line, vendor doc); or
- **producer gap**: the provider can do it and Swallowtail has not built the
  seam, with the card that builds it.

A cross with neither is a defect in the matrix. "Withheld" is not a third
kind: it is a producer gap with a reason.

## Scope

1. Add two columns to the matrix CSV and its rendered guide: `cross_kind`
   (`provider_limitation` | `producer_gap`) and `cross_ref` (evidence path or
   card id), populated for every unavailable cell on every route, starting
   with the three Bovine routes across MCP/tools, permissions, skills, model
   inventory/selection, cancellation/reconciliation, packaging, and registered
   tools; then the remaining 46 routes.
2. Add the missing capability columns the 2026-09-07 matrix exposed:
   registered tools, client MCP servers, selected-skill bundle, persistent
   permission grants, pre-session model catalogue.
3. `scripts/check-provider-route-matrix.sh`: fail when an unavailable cell
   lacks a kind and reference, or when a `producer_gap` references a card
   that does not exist or is complete.
4. Output a ranked backlog of every `producer_gap` on a route a consumer
   currently pins (Bovine: Claude SDK, Codex app-server, Grok ACP; plus the
   Nucleus routes), for Chatterbox to promote in consumer priority order.

## Out Of Scope

Building any seam; changing any claim; live probes.

## Acceptance Criteria

- [x] no unavailable cell without kind and reference, enforced by the check
- [x] every `provider_limitation` cites frozen evidence that says so
- [x] every `producer_gap` names an existing, non-complete card, or the audit
      creates a stub card the backlog lists
- [x] the ranked producer-gap backlog for pinned routes is in the card Result

## Result

The feature matrix now has 39 capability columns. The five new columns are
`registered_tools`, `client_mcp_servers`, `selected_skill_bundle`,
`persistent_permission_grants`, and `pre_session_model_catalogue`.
`cross_kind` and `cross_ref` are JSON maps keyed by unavailable feature. The
checker rejects missing or extra keys, unsupported kinds, missing frozen
evidence, and producer gaps that point at a complete card. Withheld Grok
cells are producer gaps with the non-complete Card128 evidence gate.

Ranked producer-gap backlog for currently pinned routes:

| Rank | Consumer | Route | Cell | Card | Reason |
| ---: | --- | --- | --- | --- | --- |
| 1 | Bovine | `claude-agent.sdk` | `consumer_tool_exchange` | [125](125-claude-sdk-registered-tool-route-binding.md) | route binding is not exposed |
| 2 | Bovine | `claude-agent.sdk` | `registered_tools` | [125](125-claude-sdk-registered-tool-route-binding.md) | prepared registration is not bound into open |
| 3 | Bovine | `claude-agent.sdk` | `selected_skill_bundle` | [126](126-claude-sdk-selected-skill-bundle-binding.md) | selected bundle transport is not bound |
| 4 | Bovine | `codex.app-server` | `selected_skill_bundle` | [127](127-codex-selected-skill-bundle-binding.md) | selected bundle transport is not bound |
| 5 | Bovine | `grok-build.acp` | `client_mcp_servers` | [128](128-grok-acp-client-mcp-probe-harness.md) | provider-route surface is unproven; probe owns the disposition |
| 6 | Bovine | `grok-build.acp` | `consumer_tool_exchange` | [128](128-grok-acp-client-mcp-probe-harness.md) | consumer-tool surface is unproven; probe owns the disposition |
| 7 | Bovine | `grok-build.acp` | `registered_tools` | [128](128-grok-acp-client-mcp-probe-harness.md) | registered-tool/MCP surface is unproven; probe owns the disposition |
| 8 | Bovine | `grok-build.acp` | `selected_skill_bundle` | [128](128-grok-acp-client-mcp-probe-harness.md) | labelled-input surface is unproven; probe owns the disposition |
| 9 | Nucleus | `codex.app-server` | `selected_skill_bundle` | [127](127-codex-selected-skill-bundle-binding.md) | shared Codex producer gap; no second dispatch is authorized |

This is a ranked producer-gap backlog for Chatterbox. It is not dispatch
authorization. The g05.035 ledger remains distinct: 14/18 producer seams are
merged on `main`, while Desktop live acceptance remains 0/18. No live/provider,
tag, release, or consumer action was taken.

Validation on this branch: `python3 scripts/provider_route_matrix/validate.py
docs/guides/provider-solution-feature-matrix.csv`, the extended route-matrix
check, and `git diff --check` are the card-local checks; the card-named
`effigy qa:routes` and `effigy qa:docs` runs remain the coordinator gate.

## Validation

- `effigy qa:routes`; `effigy qa:docs`; `git diff --check`

## Review Oracle

Invariant: reading a cross tells you who owes the work. Smallest
counterexample: a cross whose reference is a card marked complete.

## Auto-Continuation

No. Stop for exact-head review; Chatterbox promotes the backlog.
