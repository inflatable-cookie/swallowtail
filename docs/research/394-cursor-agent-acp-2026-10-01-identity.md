# Research 394: Cursor ACP 2026.10.01 Identity

Status: identity evidence for `cursor-agent.acp`; the separate claim commit in this PR extends the production claim through `2026.10.01-14929f9`. Scope authority is Tom's approved exact-pin/reset sweep ruling in [the version-currentness checkpoint procedure](../knowledge/operations/version-currentness-checkpoint.md#pre-v052-sweep-authority) and Queue task `swallowtail#127`.

## Official channel and ordered hops

Re-probed on 2026-10-08: the [official ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json) Cursor entry identifies `2026.10.01-14929f9` and links to its exact Cursor lab archive. Official registry manifest history for `cursor/agent.json` contains three releases after the qualified `2026.09.18-9a7762b`: `2026.09.26-dd393fe` (manifest commit `b62398f8201ca95926c16c93d0aaf7d7e8cf9d94`), `2026.09.28-64d2043` (`3a6bcd28d931a154338602f966418d00f8c71229`), and `2026.10.01-14929f9` (`310074e4c18c5ff39ec178e84d0f461f8ff73fe0`). Commit dates are frozen in the identity fixture. No omitted registry manifest hop is inferred.

The separately re-probed [general Cursor installer](https://cursor.com/install) script names `2026.10.01-e373342`. This is a separately documented CLI distribution channel; it does not change the registry manifest's exact ACP archive identity. This task selects only the ACP registry channel. The distinct build is recorded as unmapped and is not mixed into the ACP release-date claim.

`identity.json` freezes registry entries, manifest commits, exact archive URLs, archive sizes and SHA-256 digests for Darwin ARM64 and Linux x64, package runtime-index and selected ACP chunk digests, and the host observation. `dist-inventory.json` freezes the complete extracted Darwin `dist-package` file sets and every hop's added, removed, changed, and identical paths. Each sorted path set has a mutation-sensitive canonical SHA-256. For selected wire/lifecycle/tool/permission surfaces, the inventory names exact `index.js`, ACP command chunk, and `8096.index.js` hashes and classifies their per-hop deltas.

## Selected ACP surfaces

Static review of the exact archived chunks found the selected initialize subset unchanged: ACP protocol version constant, `cursor_login`, `loadSession`, list capability, HTTP/SSE MCP, and image-only among audio/image/embedded-context prompt capabilities; there is no `agentInfo`. Subagents remain client-negotiated/default-off and are not requested by Swallowtail. `session/new` still requires the same authentication before creating a session.

Selected tool call/update/result cases and literal values remain stable. The `2026.09.26` to `2026.09.28` ACP chunk uses compiler/minifier syntax rewrites, including optional-chain/generator changes and one changed internal minified key; selected ACP cases and emitted tool content mapping remain unchanged. The `2026.09.28` to `2026.10.01` selected tool bodies match; the permission method's only observed body difference is its minified imported-module identifier. `requestWebPermission` still maps one-shot allow/reject and provider cancellation/rejection as before; no persistent permission grant is qualified. Existing fixtures cover initialization, tool exchange, pending permission, cancellation, recovery failure, and cleanup.

The `2026.09.26` package adds `mcp_user_extension_stdio_unsandboxed_win32`; Swallowtail's selected route declares HTTP/SSE only. Worker, plugin, cloud, SEA, native dependency, general CLI, and other unselected files are bounded by the complete inventory and named reasons. This is static selected-surface evidence, not proof for provider internals, other route families, or unselected commands.

`loadSession` remains advertised without proven history/per-turn replay, so continuation recovery and provider session import/management stay blocked. No load/resume, new operation, lifecycle change, permission expansion, or consumer-visible narrowing is part of the extension. No catalogue/headless evidence transfers.

## Compatible extension decision

This is an ACP-only compatible extension: retain baseline `2026.07.01-41b2de7`, claim ID `cursor-agent.acp.release-window-3`, behavior revision `cursor-agent.acp-v1.interactive-v1`, all prior qualified points, `AllowUnverified`, and all exact-date holes. Add only the three frozen points through `2026.10.01-14929f9`. The catalogue claim remains at `2026.10.01-14929f9`; the headless claim remains at `2026.09.18-9a7762b`. No provider-session behavior was exercised. Public archives were downloaded and statically inspected; no archive was executed, no provider prompt or authenticated catalogue request was sent, and the host installation was not changed.
