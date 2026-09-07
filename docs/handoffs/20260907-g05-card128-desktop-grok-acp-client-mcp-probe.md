---
title: g05.035 Card 128 Desktop Grok ACP client-MCP probe packet
kind: desktop-probe-packet
status: ready
owner: Tom
created: 2026-09-07
updated: 2026-09-07
card: docs/roadmaps/g05/batch-cards/128-grok-acp-client-mcp-probe-harness.md
---

# Card 128 Desktop Grok ACP Client-MCP Probe Packet

Swallowtail ships the provider-free harness. Desktop, the sole
integration/test owner, runs the live installed-Grok path under its existing
isolated-testing authorization. Swallowtail crate tests never spawn Grok.

Keep two counts distinct: producer seams merged on `main` are 14/18; Desktop
live acceptance remains 0/18 until Desktop returns capsules. This packet does
not change either count, authorize a tag, or claim Grok MCP support.

## Command

From a Swallowtail checkout, one exact version at a time (`1.0.4` then
`1.0.5`):

```sh
SWALLOWTAIL_DESKTOP_GROK_ACP_CLIENT_MCP_PROBE=1 \
  scripts/run-grok-acp-client-mcp-probe.sh \
    --version 1.0.5 \
    --grok-executable /path/to/installed/grok \
    --output /path/to/grok-1.0.5-client-mcp.capsule.json
```

The runner builds the disposable `grok-acp-echo-mcp` stdio server (one `echo`
tool, no filesystem or network) and drives ACP `initialize`, `authenticate`
with Grok's already-cached token method id only, and `session/new` with a
non-empty `mcpServers` list naming that server, then one bounded echo prompt.

If Desktop already isolated `GROK_HOME`, the script leaves it in place and
does not copy host credentials or config. If `GROK_HOME` is unset, the script
creates an empty isolated directory and exports it as `GROK_HOME`; the Grok
child uses that directory as `HOME` and as the ACP `session/new` `cwd` so a
stray run cannot mutate host Grok state or treat the operator home as the
workspace root. Cargo keeps the host `HOME`. Auth is whatever that isolated
environment already has.

The live binary also refuses unless
`SWALLOWTAIL_DESKTOP_GROK_ACP_CLIENT_MCP_PROBE=1`.

Provider-free four-verdict proof (no Grok process):

```sh
cargo run --offline --locked -p swallowtail-testkit --bin grok-acp-client-mcp-probe -- \
  --fixture accepts_client_mcp --version 1.0.5
```

`--fixture` accepts `accepts_client_mcp`, `ignores_client_mcp`,
`rejects_client_mcp`, or `inconclusive`.

## Expected artefacts

One redacted JSON capsule per exact version. Fields:

- `route`: `grok-build.acp`
- `version`: `1.0.4` or `1.0.5`
- `verdict`: exactly one of `accepts_client_mcp`, `ignores_client_mcp`,
  `rejects_client_mcp`, `inconclusive`
- `frames`: bounded redacted ACP JSON-RPC objects, including the outbound
  `session/new` with non-empty `mcpServers` when sent
- `stale_callback_rejected`
- `cleanup_joined`
- `truncated`: capture stopped at the frame bound; verdict is then `inconclusive`

Auth that fails before `session/new`, or a `session/new` JSON-RPC error that
does not mention `mcpServers` or the echo server name, is `inconclusive`.
Only an explicit client-MCP rejection is `rejects_client_mcp`. Overflow does
not abort without a capsule. `ignores_client_mcp` requires a completed
`session/prompt` turn with no echo-named tool call. A timeout, missing prompt
result, or a `title: "echo"` tool call without the client MCP server name is
`inconclusive` — ACP v1 does not attribute a tool call to a client-supplied
server, so that shape must not be published as a Grok ignore.
Permission requests are answered with an `allow_once` `optionId` taken from
the request's `options`, not invented by the harness.

Frames must contain no credentials, tokens, or host paths. `1.0.4` and `1.0.5`
are separate evidence segments; do not merge them.

## What to send back

Return both capsules (or the named defect that blocked a rerun). Chatterbox
applies the card 128 decision tree:

- `accepts_client_mcp`: card 118 proceeds under Research 289
- `ignores_client_mcp` or `rejects_client_mcp`: operator chooses native Grok
  MCP mediation or an explicitly withheld Grok MCP/tools cell
- `inconclusive`: one authorized rerun with the named defect fixed; a second
  inconclusive is treated as `ignores_client_mcp`

No Swallowtail adapter, claim, contract, tag, or release action follows from
this packet alone.
