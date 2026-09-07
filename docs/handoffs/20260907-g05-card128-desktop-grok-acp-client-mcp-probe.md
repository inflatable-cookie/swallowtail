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

The 2026-09-07 live capsules (`1a1f263d…`, `a0eaafb6…`) stay evidence. Both
are `inconclusive`. They proved protocol admission (`initialize` and
`tools/list` on the echo server) and did not prove invocation (`tools/call`
absent). Do not reclassify the four matrix cells from those runs. Card 133
repairs the oracle so a later authorized rerun cannot collapse admission into
`ignores_client_mcp`.

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
non-empty `mcpServers` list naming that server, then one directive prompt
that names the echo tool and the fixed argument `text=ping`. The exact prompt
text is recorded on the capsule. The echo server appends received method
names (`initialize`, `tools/list`, `tools/call`) to a unique per-run
transcript file under `GROK_HOME`. The runner exclusive-creates that empty
file and refuses if the path already exists, so `1.0.4` then `1.0.5` (or an
authorized rerun) on the same authorized home cannot inherit a prior
`tools/call`. The transcript path is passed both as MCP `env`
(`SWALLOWTAIL_ECHO_MCP_TRANSCRIPT`) and as `--transcript` on
`mcpServers.args`, so a Grok that drops `env` is not silent.

Admission and invocation are separate capsule fields. `client_mcp_admitted`
is true when the echo transcript contains `initialize`.
`client_mcp_tools_listed` is true when it contains `tools/list`.
`echo_helper_live` is true when this run proved the copied echo helper
spawnable with a provider-free `initialize` self-check. That check does not
write the probe transcript and does not spawn Grok.
`accepts_client_mcp` still requires a `tools/call` line.
`ignores_client_mcp` requires a completed prompt, no echo `initialize`, no
`tools/call`, and `echo_helper_live`. A run that admits the server but does
not call it is `inconclusive` and must name `turn_completed_without_tool_call`
or `no_turn_result`. An empty transcript cannot score `ignores_client_mcp`
unless the helper was proven live, the prompt turn completed, and ACP did
not show an unattributed echo tool call. An unproven or unspawnable helper
is `inconclusive` with `echo_liveness_unproven`.

If Desktop already isolated `GROK_HOME`, the script leaves it in place and
does not copy host credentials or config. If `GROK_HOME` is unset, the script
creates an empty isolated directory and exports it as `GROK_HOME`; the Grok
child uses that directory as `HOME` and as the ACP `session/new` `cwd` so a
stray run cannot mutate host Grok state or treat the operator home as the
workspace root. Cargo keeps the host `HOME`. Auth is whatever that isolated
environment already has.

The live example also refuses unless
`SWALLOWTAIL_DESKTOP_GROK_ACP_CLIENT_MCP_PROBE=1`.

Provider-free four-verdict proof and the four oracle shapes (no Grok
process):

```sh
cargo run --offline --locked -p swallowtail-testkit --example grok-acp-client-mcp-probe -- \
  --fixture accepts_client_mcp --version 1.0.5
```

`--fixture` accepts `accepts_client_mcp`, `ignores_client_mcp`,
`rejects_client_mcp`, or `inconclusive`. Crate tests also drive
admitted-and-called, admitted-listed-not-called-with-completed-turn,
admitted-not-listed, no-admission, and helper-unspawnable.

## Expected artefacts

One redacted JSON capsule per exact version. Fields:

- `route`: `grok-build.acp`
- `version`: `1.0.4` or `1.0.5`
- `verdict`: exactly one of `accepts_client_mcp`, `ignores_client_mcp`,
  `rejects_client_mcp`, `inconclusive`
- `inconclusive_cause`: named cause when `verdict` is `inconclusive`;
  otherwise null. `no_turn_result` means the prompt had no result.
  `turn_completed_without_tool_call` means the turn finished after admission
  without echo `tools/call`. `echo_liveness_unproven` means the echo helper
  was not proven spawnable, so an empty transcript cannot score ignore.
  Other named causes cover missing `session/new`, truncation, permission
  rejection, and an unattributed ACP echo title
- `prompt`: the exact directive text sent on `session/prompt`
- `prompt_turn_completed`
- `stop_reason`: ACP prompt `stopReason` when the turn returned a result
- `client_mcp_admitted`: echo-server `initialize` observed
- `client_mcp_tools_listed`: echo-server `tools/list` observed
- `echo_helper_live`: this run proved the echo helper spawnable
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
stdio transcript to contain `tools/call`. `ignores_client_mcp` requires a
completed `session/prompt` turn, no echo `initialize`, and
`echo_helper_live`. A timeout or missing prompt result is `inconclusive`
with `no_turn_result`. Admission without a call on a completed turn is
`inconclusive` with `turn_completed_without_tool_call`. A `title: "echo"`
tool call without echo MCP `initialize` is `inconclusive`. A helper that
fails the per-run spawn self-check is `inconclusive` with
`echo_liveness_unproven`; that is not a provider finding. Live spawn refuses
unless `GROK_HOME` is an existing directory; session `cwd` and the Grok child
`HOME` are that directory. The echo MCP transcript is a unique
exclusive-created file in that directory; a path that already exists is
refused. `stale_callback_rejected` is true only when Grok actually sent a
post-close callback.

Frames must contain no credentials, tokens, or host paths. `1.0.4` and `1.0.5`
are separate evidence segments; do not merge them.

## Evidence gate scope

Desktop (Acowtancy) is the owner who runs this gate under its existing
isolated-testing authorization. While this packet is live, the following
`grok-build.acp` cells are `evidence_pending` under the Feature Matrix Rule;
no producer card is named because the probe outcome is outstanding. The
2026-09-07 live capsules do not reclassify them:

- grok-build.acp client_mcp_servers
- grok-build.acp consumer_tool_exchange
- grok-build.acp registered_tools
- grok-build.acp selected_skill_bundle

## What to send back

Return both capsules (or the named defect that blocked a rerun). Chatterbox
applies the card 128 decision tree, converting each verdict into the feature
matrix cross kind. Admission fields constrain the tree:

- `accepts_client_mcp`: `producer_gap` with card 118 under Research 289
- `ignores_client_mcp` or `rejects_client_mcp`: `provider_limitation` with
  the returned capsule as frozen evidence, or the operator's native Grok MCP
  mediation or explicitly withheld-cell decision as a `producer_gap` with the
  card that builds it. `ignores_client_mcp` is forbidden when
  `client_mcp_admitted` is true or `echo_helper_live` is false; an
  `initialize` line on the echo transcript refutes ignore, and a helper spawn
  failure must not freeze `provider_limitation`
- `inconclusive`: one authorized rerun with the named `inconclusive_cause`
  fixed. `echo_liveness_unproven` is a harness defect, not a provider
  finding. A second inconclusive is treated as `ignores_client_mcp` only when
  `client_mcp_admitted` is false and `echo_helper_live` is true. If admission
  is already proven, or helper liveness is unproven, that branch must not
  fire; keep the cells `evidence_pending` until invocation is proven or the
  operator names a different cell kind

No Swallowtail adapter, claim, contract, tag, or release action follows from
this packet alone.
