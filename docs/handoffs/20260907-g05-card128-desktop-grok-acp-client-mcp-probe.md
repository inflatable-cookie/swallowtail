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
The echo server appends received method names (`initialize`, `tools/list`,
`tools/call`) to a unique per-run transcript file under `GROK_HOME`. The
runner exclusive-creates that empty file and refuses if the path already
exists, so `1.0.4` then `1.0.5` (or an authorized rerun) on the same
authorized home cannot inherit a prior `tools/call`. The transcript path is
passed both as MCP `env` (`SWALLOWTAIL_ECHO_MCP_TRANSCRIPT`) and as
`--transcript` on `mcpServers.args`, so a Grok that drops `env` is not
silent. `accepts_client_mcp`
requires a `tools/call` line. `ignores_client_mcp` requires an `initialize`
line (the echo process was actually reached) plus a completed prompt and no
`tools/call`. An empty or missing transcript cannot separate "Grok ignored
`mcpServers`" from a harness fault, and is `inconclusive`.

If Desktop already isolated `GROK_HOME`, the script leaves it in place and
does not copy host credentials or config. If `GROK_HOME` is unset, the script
creates an empty isolated directory and exports it as `GROK_HOME`; the Grok
child uses that directory as `HOME` and as the ACP `session/new` `cwd` so a
stray run cannot mutate host Grok state or treat the operator home as the
workspace root. Cargo keeps the host `HOME`. Auth is whatever that isolated
environment already has.

The live example also refuses unless
`SWALLOWTAIL_DESKTOP_GROK_ACP_CLIENT_MCP_PROBE=1`.

Provider-free four-verdict proof (no Grok process):

```sh
cargo run --offline --locked -p swallowtail-testkit --example grok-acp-client-mcp-probe -- \
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
- `echo_mcp_methods`: method names observed on the disposable echo server stdio

Auth that fails before `session/new`, or a `session/new` JSON-RPC error that
does not mention `mcpServers` or the echo server name, is `inconclusive`.
Only an explicit client-MCP rejection is `rejects_client_mcp`. Overflow does
not abort without a capsule. `accepts_client_mcp` requires the echo MCP
stdio transcript to contain `tools/call`. `ignores_client_mcp` requires
`initialize` on that transcript, a completed `session/prompt` turn, and no
`tools/call`. A timeout, missing prompt result, missing `initialize`, or a
`title: "echo"` tool call without echo MCP `initialize` is `inconclusive`.
Live spawn refuses unless `GROK_HOME` is an existing directory; session
`cwd` and the Grok child `HOME` are that directory. The echo MCP transcript
is a unique exclusive-created file in that directory; a path that already
exists is refused. `stale_callback_rejected` is true only when Grok actually
sent a post-close callback.

Frames must contain no credentials, tokens, or host paths. `1.0.4` and `1.0.5`
are separate evidence segments; do not merge them.

## Evidence gate scope

Desktop (Acowtancy) is the owner who runs this gate under its existing
isolated-testing authorization. While this packet is live, the following
`grok-build.acp` cells are `evidence_pending` under the Feature Matrix Rule;
no producer card is named because the probe outcome is outstanding:

- grok-build.acp client_mcp_servers
- grok-build.acp consumer_tool_exchange
- grok-build.acp registered_tools
- grok-build.acp selected_skill_bundle

## What to send back

Return both capsules (or the named defect that blocked a rerun). Chatterbox
applies the card 128 decision tree, converting each verdict into the feature
matrix cross kind:

- `accepts_client_mcp`: `producer_gap` with card 118 under Research 289
- `ignores_client_mcp` or `rejects_client_mcp`: `provider_limitation` with
  the returned capsule as frozen evidence, or the operator's native Grok MCP
  mediation or explicitly withheld-cell decision as a `producer_gap` with the
  card that builds it
- `inconclusive`: one authorized rerun with the named defect fixed; a second
  inconclusive is treated as `ignores_client_mcp` and follows that outcome

No Swallowtail adapter, claim, contract, tag, or release action follows from
this packet alone.
