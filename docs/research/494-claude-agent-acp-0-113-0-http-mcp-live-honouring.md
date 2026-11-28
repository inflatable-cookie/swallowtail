# Research 494 — Claude Agent ACP `0.113.0` Pin and HTTP MCP Live Honouring

Status: Accepted
Owner: Tom Wright
Created: 2026-11-28
Contracts: 029, 037, 063
Ruling: Move the repo-local `@agentclientprotocol/claude-agent-acp` sidecar pin from `0.106.0` to `0.113.0` and extend `client_mcp_servers` HTTP MCP live honouring to exact guarded `0.113.0`

## Question

Can Swallowtail move its repo-local `@agentclientprotocol/claude-agent-acp`
development and live-probe pin from `0.106.0` to `0.113.0` and extend the live
HTTP MCP honouring claim to exact guarded `0.113.0` after one
operator-authorized `claude-sonnet-4-6` attempt?

## Authority check

Yes. The same authority chain used for `0.106.0` in [Research 492](./492-claude-agent-acp-0-106-0-http-mcp-live-honouring.md)
holds for `0.113.0`:

- **Upstream qualification and guard rulings.** [Research 493](./493-claude-agent-acp-0-113-0-evidence.md)
  qualified all 13 stable hops `0.107.0` through `0.113.0` on
  `claude-agent.acp.guarded-permission-inheritance-v8` via trusted disposable
  Linux x64 and Darwin arm64 original proofs (294 cases), raising
  `CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION` to `"0.113.0"` while keeping the
  three 2026-11-28 guard-loader rulings in force: fail-closed
  `startAroundRefusedMcpServers` (`mcp_refusal_fallback: "acp/dist/mcp-servers.js"`),
  vetoed `RemoteControlAtStartup` (`remote_control: "acp/dist/remote-control.js"`),
  and denied `plugin-command.js` `child_process`
  (`denied_child_process_parents: ["acp/dist/plugin-command.js"]`), plus
  `BUN_ENABLE_CRASH_REPORTING="0"`.
- **Preparation decision `1ec44a26` v2.** Per
  [claude-agent-acp-admission-preparation.md](../knowledge/operations/claude-agent-acp-admission-preparation.md),
  `0.113.0` is one of the 21 frozen guarded points, package installation in
  the isolated worktree ran strictly with `npm install --ignore-scripts`, and
  every graph file and native binary digest matched the frozen `0.113.0.json`
  manifest before any launch.
- **Explicit single-attempt authorization.** Operator Tom authorized exactly
  one live Claude subscription `claude-sonnet-4-6` turn through
  `http_mcp_live_gate` on guarded `0.113.0` dated `2026-11-28`.

## Answer

Yes. Both the repo-local pin move to `0.113.0` and the live HTTP MCP honouring
claim on exact guarded `0.113.0` are accepted.

1. **Pin and lockfile update.** Root `package.json` now pins
   `@agentclientprotocol/claude-agent-acp` to exact `0.113.0`, and
   `package-lock.json` resolves `@agentclientprotocol/claude-agent-acp@0.113.0`
   (tarball SHA-256
   `9d46d6327f2d6f17bcdd1c12a724cdb898bf56bc19946c3ac38b2a36d241fad1`),
   `@agentclientprotocol/sdk@2.12.0`,
   `@anthropic-ai/claude-agent-sdk@0.3.331` plus its eight platform optional
   dependencies, `diff@9.0.0`, and `zod@4.6.7`.
2. **Pre-launch integrity verification.** All 310 JS/manifest files in
   `crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-acp-guard-graphs/0.113.0.json`
   and the `darwin-arm64` native `claude` binary (`381,464,176` bytes, SHA-256
   `4874591dc39bff1188921b82cbeb8a44e3a33b0f92c888b3d43639b9344101f7`) in
   worktree `node_modules/` matched the frozen manifest digests before any
   process launched.
3. **Gate staging and non-live validation.**
   `crates/swallowtail-adapter-claude-agent/tests/http_mcp_live_gate.rs`
   advances `HTTP_MCP_HONOURING_VERSION` to `"0.113.0"`. All 310 entries in
   `0.113.0.json` fit the existing five `PACKAGE_PREFIXES`, so staging needed no
   layout change, and `claude_agent_acp_guarded_v1_frozen("0.113.0", ...)`
   with `capture_frozen_guarded_recipe` and `LocalCheckedNodeLaunch` +
   `with_checked_process` enforced the exact #623 guard-loader rulings. Every
   non-live check ran and passed before the live attempt:
   - `http_mcp_live_*` unit tests (`15 passed; 224 skipped`)
   - `effigy validate:current-claude-agent-acp`
   - `effigy lint:acp-guard`
   - `effigy check:acp-guard-msrv`
   - `effigy validate:acp-guard-captured-child`
   - `effigy validate:acp-checked-prepared`
   - `effigy validate:claude-agent-acp-trusted-native-self-test`
   - `effigy validate:claude-agent-sdk-native-proof-harness` (`226 passed`)
   - `effigy security:dependencies`
   - `effigy qa:docs` and `effigy qa:routes`
   - `effigy check:acp-guard-api` (only the pre-existing `swallowtail-runtime`
     baseline diff on `cancel_sign_in` and `complete_sign_in`)
   - `effigy package:release-floor` (full Rust `1.97.1` Clippy and workspace
     `--all-features --locked` test suite passed; an initial run hit a
     parallel-contention timeout in
     `claude_agent_sdk_native_proof_harness::a_pre_callback_denial_fabricates_no_permission_request`,
     which passed under both `validate:claude-agent-sdk-native-proof-harness` and
     the full `package:release-floor` rerun)

## Observed gate

On `2026-11-28` (operator Tom), the single authorized
`SWALLOWTAIL_LIVE_CLAUDE_AGENT_ACP_HTTP_MCP=1` run of
`http_mcp_live_gate::one_authorized_claude_agent_acp_http_mcp_live_attempt`
under the checked guarded launch passed in `10.75s` (exit status `0`) and
recorded:

```json
{"schema_version":1,"accepted":true,"stop":null,"model":"claude-sonnet-4-6","declaration_sent":true,"connected":true,"tools_listed":true,"tool_called":true,"tool_result":true,"terminal":{"status":"completed","code":null},"cleanup":{"class":"clean","code":null,"stage":null}}
```

The persisted record confirms `declaration_sent`, `connected`, `tools_listed`,
`tool_called`, `tool_result`, terminal `completed` without an error code, and
`clean` cleanup (`code: null`, `stage: null`). Recorded model
`claude-sonnet-4-6` is provenance only. No raw provider streams, tokens,
account identifiers, session identifiers, or private host paths are retained.

## Relation to prior evidence

Live HTTP MCP honouring (`client_mcp_servers: Yes`) for `claude-agent.acp` now
covers four exact points: `0.79.0` (Research 361), `0.81.2` (Research 364),
guarded `0.106.0` (Research 492), and guarded `0.113.0` (this record). Other
qualified points remain unqualified for live HTTP MCP honouring, and stdio MCP
live honouring remains separate.

## Disposition

Accept exact guarded `0.113.0` for `client_mcp_servers` live HTTP MCP honouring
and move the repo-local `@agentclientprotocol/claude-agent-acp` pin to `0.113.0`.

## Next move

- Keep root `package.json` pinned to exact `0.113.0` and
  `HTTP_MCP_HONOURING_VERSION` set to `"0.113.0"`.
- Do not advance the local pin or HTTP MCP live honouring claim past `0.113.0`
  without separate upstream qualification and a separate operator-authorized
  live attempt.
