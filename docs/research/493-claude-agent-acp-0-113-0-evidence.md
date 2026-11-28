# 493 Claude Agent ACP 0.113.0 Qualification Evidence

Status: selected-route original qualification evidence through `0.113.0` on Linux
x64 and Darwin arm64. Preserves earlier segments (`0.53.0..=0.100.0`), the `0.58.0`
exclusion, exact `0.101.0` on v7, and exact `0.101.1..=0.106.0` on v8. Adds only
the 13 published stable hops `0.107.0` through `0.113.0` as exact guarded points on
`claude-agent.acp.guarded-permission-inheritance-v8`. Unpublished interior versions
remain rejected; `0.114.0` and later stay `UnverifiedNewer`. Does not move the
repo-local `package.json` pin (`0.106.0`), does not extend HTTP MCP live honouring
beyond exact `0.79.0`, `0.81.2`, and guarded `0.106.0` ([Research 492](492-claude-agent-acp-0-106-0-http-mcp-live-honouring.md)),
and does not qualify standalone Agent SDK, native executable, Node, shared ACP
schema, or Windows.

## Operator rulings (2026-11-28)

Tom issued three binding rulings for the 0.111.0–0.113.0 upstream changes:

1. **Reject the 0.111+ silent MCP drop (`startAroundRefusedMcpServers`) — fail closed.**
   If managed `disableSideloadFlags` refuses MCP startup, the session must fail
   rather than retry without the consumer's MCP servers. Implemented in the guard
   loader by replacing `startAroundRefusedMcpServers` with a single-attempt
   `start(false)` forwarder and poisoning `refusedMcpServers` to throw
   `silent MCP server drop refused`, freezing both.
2. **Veto 0.113 `RemoteControlAtStartup` in the guard loader.**
   Implemented on the same pattern as `IdleCompaction`: non-writable,
   non-configurable no-op `RemoteControlAtStartup.prototype.connect`, a subclass-
   refusing `Proxy` constructor forcing `asked = false` with
   `Object.preventExtensions`, frozen prototype and constructor,
   `startsWithRemoteControl` forced to return `false`, and `startRemoteControl`
   poisoned to throw `remote control at startup refused`.
3. **Deny 0.113 `plugin-command.js` `execFile` in the guard loader.**
   `node:child_process` stays restricted to the pinned native `claude` binary and
   bounded `/bin/ps` for `live-registry.js`. Any parent listed in
   `denied_child_process_parents` receives a separate builtin-free
   `swallowtail-child-denied:` facade where `spawn`, `spawnSync`, `execFile`,
   `execFileSync`, and `execFile[Symbol.for('nodejs.util.promisify.custom')]`
   unconditionally throw `child process invocation refused`. Capture fails closed
   if `acp/dist/plugin-command.js` is present in `files` without being listed in
   `denied_child_process_parents`.

Because all three rulings preserve existing v8 consumer-visible semantics without
changing the public API or adding a new permission posture, the 13 new hops stay
on `claude-agent.acp.guarded-permission-inheritance-v8`.

## Identity

On 2026-11-28, npm `latest` was `0.113.0` (`preview` `0.113.1-preview.2` is outside
the stable set). Between `0.106.0` and `0.113.0`, the complete published stable
set is 13 hops (extending [Research 490](490-claude-agent-acp-0-109-1-identity-stop.md)'s
eight-hop freeze through `0.109.1` with five further hops `0.110.0`–`0.113.0`).
Every tarball was downloaded and inspected statically with zero installation or
execution on Tom's Mac (decision `1ec44a26` v2). All npm SHA-512 integrity values,
computed SHA-256 digests, and GitHub tag-to-`gitHead` bindings match. The
[0.113.0 frozen fixture](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.113.0/README.md)
retains `registry.json`, `dependencies.json`, and `dist-inventory.json`.

| ACP hop | Published UTC | ACP SDK | Agent SDK | Bundled native manifest | Tag matches `gitHead` |
| --- | --- | --- | --- | --- | --- |
| 0.107.0 | 2026-11-13 13:39:38.999 | 2.6.0 | 0.3.323 | 2.1.323 | yes |
| 0.107.1 | 2026-11-16 09:52:58.953 | 2.7.0 | 0.3.323 | 2.1.323 | yes |
| 0.107.2 | 2026-11-16 11:41:46.529 | 2.7.0 | 0.3.325 | 2.1.325 | yes |
| 0.107.3 | 2026-11-16 14:39:18.879 | 2.7.0 | 0.3.325 | 2.1.325 | yes |
| 0.108.0 | 2026-11-17 13:43:51.377 | 2.7.0 | 0.3.326 | 2.1.326 | yes |
| 0.108.1 | 2026-11-18 17:21:24.324 | 2.8.0 | 0.3.326 | 2.1.326 | yes |
| 0.109.0 | 2026-11-19 13:36:23.668 | 2.8.0 | 0.3.328 | 2.1.328 | yes |
| 0.109.1 | 2026-11-21 15:37:49.660 | 2.9.1 | 0.3.330 | 2.1.330 | yes |
| 0.110.0 | 2026-11-23 09:52:52.982 | 2.9.1 | 0.3.330 | 2.1.330 | yes |
| 0.110.1 | 2026-11-23 23:34:48.855 | 2.9.1 | 0.3.330 | 2.1.330 | yes |
| 0.111.0 | 2026-11-24 15:27:48.687 | 2.11.0 | 0.3.331 | 2.1.331 | yes |
| 0.112.0 | 2026-11-24 18:16:52.662 | 2.12.0 | 0.3.331 | 2.1.331 | yes |
| 0.113.0 | 2026-11-26 13:38:41.316 | 2.12.0 | 0.3.331 | 2.1.331 | yes |

`zod@4.6.7` and `diff@9.0.0` are unchanged across all 13 hops.
`dist/idle-compaction.js` is byte-identical through `0.113.0`
(`3128c2566a2f8c3b69be35e63780f8c87faf7f3184cd723d7a095a44bde7f28d`).

## Static security-edge audit

AST inspection of all 13 hops (`0.107.0.json`–`0.113.0.json`, verified at 100%
byte-for-byte parity on `0.106.0.json`) found zero uncovered authority edges
beyond the three ruled surfaces:

- `0.108.0+` `acpsdk/dist/mcp/http.js` (`node:http`, `node:crypto`): loopback
  `127.0.0.1` MCP-over-ACP bridge; already noted in Research 490 and unreachable
  unless a client declares `"type": "acp"` (Swallowtail only admits HTTP MCP).
- `0.109.0+` `acp/dist/v2/prompt-keys.js`, `0.113.0` `acp/dist/v1/adapter-commands.js`
  and `acp/dist/v2/adapter-commands.js`: `node:crypto` only.
- `0.112.0+` `acp/dist/session-rewind.js` (`node:fs/promises`, `node:path`,
  `node:crypto`): operates only on the existing Claude transcript directory
  already read by `resumed-session.js`; writes occur only on `_session/rewind`
  (never sent by Swallowtail); load/resume takes a read-only `ENOENT` fast path.
- `agentsdk/sdk.mjs`: keeps the exact two `process.getBuiltinModule("async_hooks")`
  calls through `0.3.331` and zero new `denied_dynamic` expressions.

## Guard-loader and host changes

- `crates/swallowtail-host-local/src/captured_manifest.rs`: adds optional
  `mcp_refusal_fallback` (`0.111.0`–`0.113.0`), `remote_control` (`0.113.0`), and
  `denied_child_process_parents` (`0.113.0`) fields with fail-closed validation
  and `plugin-command.js` enforcement. Existing `0.101.0`–`0.106.0` graph configs
  remain unchanged.
- `crates/swallowtail-host-local/src/captured_source.rs`: encodes loader preload
  arguments as `data:text/javascript;base64,` instead of double percent-encoding,
  keeping `--import` argv (~28 KiB) well below Linux `MAX_ARG_STRLEN` (128 KiB).
- `crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-acp-guard-loader.mjs`
  (SHA-256 `7259262fafb71b8660ce6d80adb8748feafba459c1a1c425b9538ad34415f3c8`):
  enforces all three rulings and the bounded SDK `0.3.331` env repair below.
- Focused non-live proofs in `tests/acp_guard_capture.rs`:
  - `mcp_refusal_fallback_fails_closed_without_retry_or_silent_drop`
  - `remote_control_at_startup_is_vetoed_like_idle_compaction`
  - `denied_plugin_command_child_process_edge_refuses_even_for_native_and_ps`
  - `native_child_allows_only_crash_reporting_disable_zero`

## Trusted disposable-runner proof and repairs

Qualification ran exclusively on disposable GitHub-hosted `ubuntu-24.04` (offline
container) and `macos-15` runners via `.github/workflows/acp239-trusted-native.yml`
through the real prepared Rust/`LocalProcessHost` route, owned loopback model peer,
dummy keys, and isolated settings ([procedure](../knowledge/operations/claude-agent-acp-admission-preparation.md)).

Two non-authority-widening repairs were surfaced by the originals and fixed with
focused unit tests:

1. **SDK `0.3.331` `BUN_ENABLE_CRASH_REPORTING="0"`:** Run
   [`46164358810`](https://github.com/inflatable-cookie/swallowtail/actions/runs/46164358810)
   at `d0a1bd84b` passed `0.101.0`–`0.110.1` and failed on `0.111.0`–`0.113.0`
   because Agent SDK `0.3.331` sets `BUN_ENABLE_CRASH_REPORTING="0"` in the child
   env before spawning the Bun-compiled native `claude` binary to disable Bun
   crash telemetry. The guard loader now permits only exact
   `BUN_ENABLE_CRASH_REPORTING === "0"` for the pinned native binary while keeping
   every other `BUN_*` key, non-zero values, `/bin/ps` invocations, and the host
   Node startup env filter strictly refused.
2. **Passive `session_info_update` between turns:** Run
   [`46165964675`](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675)
   at `d3cf9d8de` passed 22/23 jobs (293/294 behavior cases: 11/11 Linux x64 jobs
   through `0.113.0`, 11/12 Darwin arm64 jobs including `0.112.0` and `0.113.0`).
   One single case (`macos-15`, `0.111.0/user-deny`, `resume/1`) failed because
   `SessionTitles` runs background title generation asynchronously and emitted
   `session_info_update` in the gap between `resume/0` completing and `resume/1`
   starting. `ClaudeAgentHooks::on_session_update` now accepts `session_info_update`
   as passive metadata only when `sessionId` matches the bound session and
   `decode_session_update` validates the payload as `AcpSessionUpdate::SessionInfo`;
   mismatched sessions, malformed payloads, and all non-metadata idle updates
   still fail closed (`passive_session_info_update_validates_session_and_payload_without_admitting_other_idle_updates`).

The pre-claim `ADMITTED` override has been removed and the public-claim run on
this PR's final head verifies all 23 matrix jobs (21 exact guarded hops × 7
scenarios × 2 platforms = 294 behavior cases) with zero private overrides.

| Platform | Run `46165964675` jobs (0.107.0–0.113.0) | Result | Job link |
| --- | --- | --- | --- |
| Linux x64 | 0.107.0 / 0.107.1 | 14/14 | [job 141043428937](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675/job/141043428937) |
| Linux x64 | 0.107.2 / 0.107.3 | 14/14 | [job 141043428855](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675/job/141043428855) |
| Linux x64 | 0.108.0 / 0.108.1 | 14/14 | [job 141043428849](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675/job/141043428849) |
| Linux x64 | 0.109.0 / 0.109.1 | 14/14 | [job 141043428903](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675/job/141043428903) |
| Linux x64 | 0.110.0 / 0.110.1 | 14/14 | [job 141043428538](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675/job/141043428538) |
| Linux x64 | 0.111.0 / 0.112.0 | 14/14 | [job 141043428839](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675/job/141043428839) |
| Linux x64 | 0.113.0 | 7/7 | [job 141043428889](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675/job/141043428889) |
| Darwin arm64 | 0.107.0 / 0.107.1 | 14/14 | [job 141043428738](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675/job/141043428738) |
| Darwin arm64 | 0.107.2 / 0.107.3 | 14/14 | [job 141043428902](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675/job/141043428902) |
| Darwin arm64 | 0.108.0 / 0.108.1 | 14/14 | [job 141043428821](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675/job/141043428821) |
| Darwin arm64 | 0.109.0 / 0.109.1 | 14/14 | [job 141043428883](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675/job/141043428883) |
| Darwin arm64 | 0.110.0 / 0.110.1 | 14/14 | [job 141043428787](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675/job/141043428787) |
| Darwin arm64 | 0.111.0 / 0.112.0 | 13/14 (exposed `session_info_update` race on `0.111.0/user-deny`) | [job 141043428825](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675/job/141043428825) |
| Darwin arm64 | 0.113.0 | 7/7 | [job 141043428816](https://github.com/inflatable-cookie/swallowtail/actions/runs/46165964675/job/141043428816) |

Retained `0.101.0`–`0.106.0` jobs on both platforms also passed 100% in run `46165964675`.

## Disposition

- `CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION` moves from `0.106.0` to `0.113.0`.
- `claude_agent_acp_claim()` adds 13 exact singleton `Maintained` segments
  (`0.107.0`–`0.113.0`) on `claude-agent.acp.guarded-permission-inheritance-v8`
  (28 total milestones; 21 exact guarded points `0.101.0`–`0.113.0`).
- Unpublished interior gaps (`0.106.1`, `0.107.4`, `0.108.2`, `0.109.2`,
  `0.110.2`, `0.111.1`, `0.112.1`) stay rejected; `0.114.0` and later stay
  visible `UnverifiedNewer`.
- Root `package.json` and `package-lock.json` stay pinned at `0.106.0`; moving
  the pin and extending HTTP MCP live honouring to `0.113.0` requires a
  separately authorized live Claude subscription turn through `http_mcp_live_gate`.
