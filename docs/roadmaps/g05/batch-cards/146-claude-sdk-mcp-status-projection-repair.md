# 146 Claude SDK MCP-Status Projection Diagnosis And Repair

Status: ready; operator authorized promotion and queue dispatch on 2026-09-08
Owner: Claude SDK adapter owner
Created: 2026-09-08
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: card 144 merged at `cc53c81a`; card 145 and Research 297

## Goal

Diagnose and repair the provider-free MCP-status projection contradiction that
stopped Card 145 before provider readiness. Do not contact Claude or spend a
live attempt.

## Scope

1. Freeze Research 297's exact source, tuple, receipt, capsule, and Desktop
   lifecycle identities in the Claude adapter's bounded regression corpus.
2. Reconcile the exact official SDK `0.3.259` `McpServerStatus` declaration
   and shipped implementation with `projectMcpStatuses`, the Rust readiness
   validator, and the fake SDK. Enumerate connected, pending, failed,
   needs-auth, and disabled rows plus every declared optional metadata field.
3. Prove or falsify the leading mismatch: the official type permits
   `serverInfo`, `error`, `config`, `scope`, and `tools`, while the sidecar
   rejects some optional metadata and the fake emits none. Accept only the
   exact declared shape needed for the frozen route. Discard all non-projectable
   metadata; never serialize raw error text, configuration, URLs, headers,
   paths, tool descriptions, or other provider content.
4. If a deterministic producer mismatch is confirmed, repair the projection
   and prove required-server connected admission, bounded failed/needs-auth
   rejection, optional-server status, undeclared/duplicate/malformed rows,
   omission, strict configuration, registered mediation, and joined cleanup.
   If the frozen artifact cannot settle the shape, return the exact unresolved
   observation needed from a future separately authorized open; do not guess.
5. Update the Claude guide, Contract 063's bounded MCP-status evidence, the
   additive `0.4.4` public API baseline only if the public surface changes,
   and `[Unreleased]`. Keep both Contract 061 cells unqualified.

Owned mutable paths: `crates/swallowtail-adapter-claude-agent/**`; the Claude
SDK sections of Contract 063 and its integration guide; additive
`release-baselines/public-api-0.4.4/swallowtail-adapter-claude-agent.txt` only
if required; `CHANGELOG.md` `[Unreleased]`; this card's `## Result`;
`PAPERCUTS.md` append only. Queue closeout owns shared roadmap/index/handoff
status surfaces.

Forbidden: live Claude/native provider execution; prompt, turn, tool, or
account activity; Desktop or other consumer edits; weakened
`strictMcpConfig`; projection of raw optional metadata; qualification, matrix
availability, candidate, tag, release, or dependency-version changes.

## Acceptance Criteria

- [ ] exact `0.3.259` declaration and implementation evidence settle every accepted MCP-status row shape
- [ ] faithful fixtures include declared optional metadata instead of only `{name, status}`
- [ ] safe projection retains only name, bounded status, and fixed local failure codes
- [ ] required and optional server semantics, malformed/foreign rows, strict omission, registered mediation, and cleanup remain fail-closed
- [ ] the Card 145 tuple no longer fails for a confirmed producer-only projection mismatch, or the unresolved live observation is named without speculative repair
- [ ] both Contract 061 cells remain unqualified and no live attempt occurs

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy validate:card116-mediated-stdio`
- `effigy package:api` if the public surface changes
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: metadata permitted by the frozen SDK status type cannot make an
otherwise valid connected required server fail, but none of that metadata may
cross the safe projection. Smallest counterexample: accepting a connected row
only by copying its `config` or `serverInfo` into a receipt or diagnostic.

## Stop Conditions

- the exact frozen artifact does not settle the returned shape;
- repair requires projecting provider/configuration content or weakening
  required-server connectivity;
- a provider-neutral contract redesign is needed; or
- validation identifies a live attempt, cell movement, or release consequence.

Return the precise contradiction to Chatterbox. Do not request a live retry.

## Auto-Continuation

No. Stop at exact-head independent review. The queue owns merge and closeout.

## Result

Implemented and pushed for exact-head independent review. The queue owns
merge and canonical closeout; the closeout log records the accepted head and
merge identity.

**Root cause (confirmed).** The exact `0.3.259` `McpServerStatus` declaration
(`package/sdk.d.ts:1114-1158`, frozen in `mcp-status-rows.json`) permits
optional `serverInfo`, `error`, `config`, `scope`, and `tools` on every row,
and the shipped `0.3.259` implementation
(`sdk.mjs`: `mcpServerStatus()` returns `(await request({subtype:
"mcp_status"})).response.mcpServers`, verified against the frozen
`dist-inventory.json` digest `7fa7c212…`) passes native `mcp_status` rows
through unchanged. The sidecar projection rejected any row carrying `error`
or `config` (and the fake SDK emitted only `{name, status}`, hiding the
mismatch). Faithful fixtures that carry all five declared optional fields
deterministically reproduce the Card 145 bounded `mcp_status_invalid` on the
pre-repair sidecar and pass as bounded outcomes after repair — the
producer-vs-projection contradiction is confirmed, not speculative.

**Repair.** `projectMcpStatuses` now admits exactly the seven declared row
keys and discards the five optional metadata fields; every other key, an
unknown status, a wrong row count, a non-object entry, and a
missing/foreign/duplicate name stay fail-closed. Evidence still carries only
`name`, the canonical status kind, and the fixed local failure code. Rust
readiness validation and the projected wire are unchanged; no public surface
moved.

**Accepted row shapes.** `connected`, `pending`, `failed`, `needs-auth`, and
`disabled`, each with any subset or all of the declared optional metadata:
required connected is admitted; required `pending`/`failed`/`disabled` fail
bounded `mcp_server_failed`; required `needs-auth` fails bounded
`mcp_server_needs_auth`; optional rows are admitted with bounded evidence.
**Rejected shapes.** undeclared top-level `url`/`headers`, unknown status
strings, malformed rows, foreign names, duplicate/missing rows, and count
mismatches — all `mcp_status_invalid` or the existing typed failure, never
leaking metadata.

**Card 145 tuple.** Repaired for every declared row shape. The frozen
artifacts cannot settle whether a real native row ever carries an undeclared
top-level field; such a row remains fail-closed, and naming it would need one
separately authorized future open. No live attempt ran; both Contract 061
cells stay unqualified.

**Validation.** `cargo fmt` check, package-scoped focused validation (489
tests), affected-package verify, the card 116 mediated-stdio selector,
`qa:routes`, `qa:docs`, `qa:northstar`, and `git diff --check` all pass on
the final head. Earlier heads failed `effigy qa:routes` on the pre-existing
card-144 stale producer-gap cross classification (the `claude-agent.sdk`
matrix `cross_ref` named completed card 144 for `consumer_tool_exchange`);
origin/main `ccd58ca6` repointed that reference to card 146, so the final
merged head passes the routes gate, and the resolution is recorded in the
appended `PAPERCUTS.md` entry. Regression corpus:
`card-145-desktop-diagnostic-tuple.json` and `mcp-status-rows.json` with
guard tests, sidecar-asset tests for every status/metadata variant, and
non-leak assertions over all open responses.
