# Research 290 — Card129 Feature Matrix Cross Evidence

Status: complete; evidence ledger; no live/provider claim
Owner: Tom
Date: 2026-09-07
Card: g05.035 / 129

## Purpose

This is the frozen evidence ledger for `provider_limitation` cells. Card129
populated it from the Research 281 route-behavior census. Card131 appends the
Codex `client_mcp_servers` row, whose basis is the Card131 corpus TSV rather
than the generic 281 census line. It does not add runtime support or promote a
provider claim. Each TSV row names one matrix row and unavailable feature,
records the unavailable finding, and points to an anchored `docs/research` or
`docs/contracts` line whose first field names the route. A guide-only or
unanchored citation cannot pass the route check.

## Machine contract

The route-matrix checker requires every provider limitation to reference one
`#L<line>` row in [the TSV ledger](290-feature-matrix-cross-evidence.tsv). The
row must match the matrix `route_id`, feature, and kind, and its `finding` must
start with `qualified route unavailable:`. The ledger's `basis` must itself be
an anchored line under `docs/research` or `docs/contracts` whose first field
names the route. Card129 rows cite Research 281. The Card131 Codex
`client_mcp_servers` row cites Research 291. Card143 appends the Grok
`selected_skill_bundle` row after its accepted live gate qualified the three
MCP/tool cells and settled the selected-skill surface as absent on frozen ACP
v1 and the exact Grok artifacts; its basis is the same Research 281 Grok
census line as the route's other limitations. Prepared integration guides are
not evidence for this disposition: the docs front door classifies guides as
non-authoritative integration usage.

Producer gaps use an existing non-complete g05 batch card. The matrix notes
carry a `Card129 producer-gap reasons:` marker with one non-empty reason for
each producer-gap feature. A withheld value follows the same producer-gap
rule; it is never a third classification.

No live provider, credential, consumer, server, tag, release, or runtime
operation was used to produce this ledger.
