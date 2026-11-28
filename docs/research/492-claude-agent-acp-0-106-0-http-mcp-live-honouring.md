# 492 Claude Agent ACP 0.106.0 HTTP MCP Live Honouring

Status: accepted honouring evidence; exact guarded `0.106.0` point
Owner: Swallowtail worker
Created: 2026-11-28
Contracts: 029, 037, 063
Ruling: Tom authorized exactly one live Claude subscription attempt on
2026-11-28: one `claude-sonnet-4-6` turn through `http_mcp_live_gate` on exact
`claude-agent-acp` `0.106.0` under the guarded checked launch, subject to
decision `1ec44a26` v2.

## Question

Does `claude-agent.acp` on exact guarded `claude-agent-acp` `0.106.0` honour a
consumer-supplied streamable-HTTP MCP entry end to end: connect, list one
tool, complete one tool call, and end the turn `Completed` with `Clean`
cleanup?

## Authority check (decision 1ec44a26 v2)

Tom's decision `1ec44a26-3d20-45ab-84e9-1523c2dfbbe4` v2 (2026-11-17,
[admission procedure](../knowledge/operations/claude-agent-acp-admission-preparation.md))
governs unverified-original qualification tests on disposable GitHub runners
with isolated settings, dummy keys, loopback model peers, and no live calls or
host tool installs. Both that procedure and [Research 483](483-claude-agent-acp-0-106-0-evidence.md)
keep exact HTTP MCP live points separate.

This attempt falls outside that qualification-procedure ban:

- `0.106.0` is already qualified on Darwin arm64 through the 112-case disposable
  original proof in Research 483; the public claim already covers `0.106.0`.
- `effigy bootstrap:claude-agent-acp` (`npm ci --ignore-scripts`) populated only
  the scratchpad worktree's `node_modules/` with lifecycle scripts disabled, no
  global/host tool installation, and no mutation of the main checkout.
- Before execution, all 287 frozen graph files and the Darwin arm64 `2.1.322`
  native binary (`323,702,464` bytes) matched `0.106.0.json` SHA-256 digests.
- The live gate launched exclusively through `claude_agent_acp_guarded_v1_frozen`,
  `swallowtail_host_local::capture_frozen_guarded_recipe`,
  `LocalCheckedNodeLaunch`, and `ClaudeAgentPreparationInput::with_checked_process`.
- Tom explicitly authorized this single live Claude subscription attempt on
  2026-11-28.

## Answer

Yes. The single authorized attempt accepted the full honouring tuple with
`Clean` cleanup. The persisted gate record (`attempt-9884.json`, also printed
under `--nocapture`) reports `declaration_sent: true`, authenticated MCP
connect, tool listing, one completed tool call returning `pong`, terminal
status `completed` with no terminal code, and cleanup class `clean` with no
cleanup code or stage. The reported model was `claude-sonnet-4-6`; model is
provenance.

Before the live attempt, all non-live checks passed on the rewritten guarded
gate and `0.106.0` lockfile:

- 15/15 `http_mcp_live_harness` and `http_mcp_live_gate` unit tests (including
  0.106.0 manifest-to-`node_modules` mapping and `allow_once` option selection)
- `effigy format:current-claude-agent-acp`, `effigy lint:acp-guard`,
  `effigy check:acp-guard-msrv`
- `effigy validate:current-claude-agent-acp`,
  `effigy validate:acp-guard-captured-child`,
  `effigy validate:acp-checked-prepared`,
  `effigy validate:claude-agent-acp-trusted-native-self-test`
- `effigy security:dependencies` (`check-chrono-maintenance.py` self-test and
  check, `cargo deny check advisories licenses sources`)
- `effigy qa:docs`, `effigy qa:routes`, and `effigy package:release-floor`
  (Rust `1.97.1` workspace clippy and tests)

The existing thirty-second cleanup deadline and production cleanup path were
unchanged. No host update, login, or auth change occurred.

## Observed gate

The persisted typed record (also printed by the gate) contains:

- schema version: `1`
- accepted: `true`
- typed stop: none (`null`)
- model: `claude-sonnet-4-6`
- declaration sent: `true`
- MCP connected: `true`
- tool listed: `true`
- tool called: `true`
- tool result: `true`
- terminal status: `completed`; terminal code: none (`null`)
- cleanup class: `clean`; cleanup code: none (`null`); cleanup stage: none (`null`)

No raw provider stream, bearer, account identifier, session id, or private path
is retained.

## Relation to prior evidence

- [Research 361](361-claude-agent-acp-http-mcp-live-honouring-accepted.md)
  remains accepted prior evidence on exact `0.79.0`.
- [Research 364](364-claude-agent-acp-0-81-2-http-mcp-live-honouring.md)
  remains accepted prior evidence on exact `0.81.2`.
- [Research 483](483-claude-agent-acp-0-106-0-evidence.md) remains the
  selected-route guarded qualification evidence through `0.106.0`.
- This single `0.106.0` attempt is spent; no further attempt is permitted by
  this authorization.

## Disposition

- `claude-agent.acp` `client_mcp_servers` is `Yes` on exact `0.79.0`, `0.81.2`,
  and guarded `0.106.0`. Other points in the window stay unqualified for
  honouring. Research 351 emission stands; `sse` remains modelled and
  unemitted.
- Root `package.json` and `package-lock.json` pin
  `@agentclientprotocol/claude-agent-acp` to exact `0.106.0` (resolving
  `@agentclientprotocol/sdk@2.6.0`, `@anthropic-ai/claude-agent-sdk@0.3.322`,
  `diff@9.0.0`, `zod@4.6.7`, and the matching `0.3.322` platform binaries).

## Next move

A different exact version point needs its own authorization and live gate.
Stable versions above `0.106.0` remain `UnverifiedNewer`
([Research 490](490-claude-agent-acp-0-109-1-identity-stop.md)).
