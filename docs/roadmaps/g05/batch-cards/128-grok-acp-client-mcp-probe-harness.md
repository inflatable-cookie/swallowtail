# 128 Grok ACP Client-MCP Probe Harness

Status: complete; merged through PR 273 as `63e3464199d28fdae9f163c0caf567c5dfb1e39e`
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: Research 289 (card 118 evidence stop); Contract 063 Grok row; the operator's 2026-09-07 release scope (all three routes including MCP/tools/skills); Desktop's existing isolated-testing authorization

## Why

Grok registered tools and MCP block the release scope. The upstream
limitation, exactly: the official Grok Build ACP corpus and Swallowtail's own
route corpus show only an empty `mcpServers` list at session setup; ACP v1
permits a client-supplied list; Grok also has a native, configuration-owned
MCP subsystem. Whether Grok Build accepts, connects, discovers, and calls a
client-supplied ACP MCP server is unproven either way, and nothing in the
frozen `1.0.4`/`1.0.5` artifacts can prove it statically. Only an exact-route
probe can. Research 289 defines that probe; this card builds the harness so
Desktop, the sole integration/test owner, can run it under its existing
isolated-testing authorization without a Swallowtail live lane.

## Scope

1. A provider-free probe harness in `swallowtail-testkit` (or a script under
   `scripts/` driven by the testkit), taking an installed Grok executable path
   and version, and one disposable non-mutating stdio MCP server the harness
   itself ships (one tool, echo semantics, no filesystem or network): it runs
   ACP `initialize`, `session/new` with a non-empty `mcpServers` list naming
   that server, captures readiness and any tool discovery, issues one bounded
   prompt that would invoke the tool, captures tool-call, result, permission,
   cancellation, and disconnect behaviour, then proves stale-callback rejection
   and joined cleanup. Output: one redacted JSON capsule per (version) with
   the exact frames, and a verdict enum: `accepts_client_mcp`,
   `ignores_client_mcp`, `rejects_client_mcp`, `inconclusive`.
2. Proved against the fake ACP fixture for all four verdicts before any real
   run. No credentials in the harness; auth is whatever the installed Grok
   already has, under Desktop's authority.
3. `1.0.4` and `1.0.5` are separate evidence segments.
4. Hand-off packet for Desktop: command line, expected artefacts, what to
   send back. Desktop runs it; Swallowtail never runs it live.

## Decision Tree The Result Feeds (recorded here so no one re-derives it)

- `accepts_client_mcp`: card 118 proceeds under Research 289's adapter
  mapping; Grok registered tools ride the ACP `mcpServers` seam with the
  common kernel; no operator decision.
- `ignores`/`rejects`: the only evidence-backed path is a configuration-owned
  native Grok MCP mediation route (a new exact route id; the triage lead of
  2026-09-07). The smallest operator decision then is one of: (a) authorize
  that native route for the release scope; (b) accept Grok MCP/tools as an
  explicitly withheld cell in the release scope. Chatterbox presents that
  choice with the capsule attached.
- `inconclusive`: one authorized rerun with the named defect fixed; a second
  inconclusive is treated as `ignores`.

## Acceptance Criteria

- [x] harness proves all four verdicts on the fake ACP fixture
- [x] disposable stdio server has one echo tool and no filesystem/network authority
- [x] capsule output is redacted and bounded; frames carry no credentials or paths
- [x] hand-off packet written; Desktop can run it without Swallowtail present

## Validation

- `effigy validate:focused swallowtail-testkit swallowtail-adapter-grok`
- `effigy package:verify-affected swallowtail-testkit swallowtail-adapter-grok`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: the harness can only observe; it changes no adapter, claim, or
configuration, and its verdict is decided by frames, not by inference.
Smallest counterexample: a verdict emitted without the session/new frame.

## Stop Conditions

Grok ACP requires a field outside ACP v1 to name a server (record; return to Chatterbox).

## Auto-Continuation

No. Stop for exact-head review. The real run is Desktop's under its isolated-testing authorization.

## Result

PR 273 merged the provider-free harness at reviewed head
`dbcf4ed6c81c23b8573d3ceaf49cf7be28216235` as
`63e3464199d28fdae9f163c0caf567c5dfb1e39e`. The probe module and the
disposable echo stdio MCP server live in `swallowtail-testkit`; the fake ACP
fixture proves all four verdicts (`accepts_client_mcp`, `ignores_client_mcp`,
`rejects_client_mcp`, `inconclusive`) with no Grok process, and crate tests
check the redaction, frame-bound, and `session/new` oracle rules. The reviewed
head ships the probe and echo targets as cargo examples — package metadata
forbids extra bins — and repairs the MSRV clippy lint on the `none_or`
empty-list check; the runner script and hand-off packet invoke `--example`.
The hand-off packet is relayed to Acowtancy Desktop under its existing
isolated-testing authorization. The live installed-Grok probe was explicitly
not run: Swallowtail spawns no Grok process, and no adapter, claim, contract,
or matrix disposition changes with this harness.
