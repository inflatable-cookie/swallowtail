---
title: g05.035 Card 128 Desktop Grok ACP client-MCP probe packet
kind: desktop-probe-packet
status: ready
owner: Tom
created: 2026-09-07
updated: 2026-09-08
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
repaired the oracle so a later authorized rerun cannot collapse admission into
`ignores_client_mcp`.

The 2026-09-07 authorized rerun capsules (`21c31e50…`, `d4da8816…`) proved
more admission (`client_mcp_admitted`, `client_mcp_tools_listed`,
`echo_helper_live` on both segments) but bounded out with
`session_new_unanswered`. Card 137 diagnosed that as a harness gap: the probe
drained inbound frames without answering them during `session/new`, so any
client request Grok issued while establishing the session went unanswered.
The probe is now a conforming ACP client (below), and a future
`session_new_unanswered` names a harness defect, never a provider finding.

The 2026-09-08 decoupled rerun capsules (`51a2d094…` on `1.0.4`,
`02720b85…` on `1.0.5`) established the session and again recorded
`client_mcp_admitted`, `client_mcp_tools_listed`, and `echo_helper_live`
true on both segments — the sixth and seventh consecutive live observations
that Grok Build admits a client-declared MCP server, spawns it, connects,
and enumerates its tools. Both still ended `inconclusive`, and both causes
were harness bounds, not Grok: `1.0.5` returned `no_turn_result` because the
prompt exchange used the eight-second protocol bound, and `1.0.4` returned
`truncated` because capture stopped at 48 frames. Card 140 re-derived every
bound in the module for live use (below). No matrix cell moves and no claim
follows from those capsules.

## Bounds

Every bound is sized for a live model and justified in the module. Each
exchange holds one absolute deadline: answering an intervening request never
extends it, and the drain keeps reading past notification-only bursts, so a
request delivered late in an exchange is still answered.

- `initialize` and `authenticate`: `LIVE_PROTOCOL_WAIT`, 30 seconds. Pure
  protocol round trips with no inference behind them; the allowance covers a
  cold agent process still loading its runtime, not a model turn
- `session/new`: `LIVE_SESSION_NEW_WAIT`, 120 seconds. Establishment spawns
  every declared MCP server and enumerates its tools before answering
- `session/prompt`: `LIVE_PROMPT_WAIT`, 300 seconds. The only exchange that
  waits on a live model reasoning, calling the echo tool, and finishing the
  turn. A protocol-scale bound here is what produced the 2026-09-08 `1.0.5`
  `no_turn_result`
- child exit after stdin close: 10 seconds, so a Node agent flushing session
  state on shutdown is not killed and reported as unjoined cleanup
- echo helper `initialize` self-check: 5 seconds. Local and immediate, with
  margin so a cold first spawn cannot produce a false
  `echo_liveness_unproven`

Frame capture holds 512 frames. That capacity is spent on streaming chatter
only: at capacity the oldest non-decisive frame is evicted so the incoming
frame still lands. The outbound `session/new` and `session/prompt` requests,
every correlated response, every inbound agent request and the probe's
recorded answer, the `tool_call` and `tool_call_update` updates, and the turn
result are never evicted. `truncated` on a capsule therefore means
"uninteresting middle frames were elided" and is not a verdict; the
`truncated` inconclusive cause is reachable only if the whole capacity is
filled by decisive frames, which no ACP session shape reaches. A truncated
capsule still scores its real verdict.

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

The probe is a conforming ACP client during every exchange, including
`session/new`. Inbound client requests are answered from one small allowlist
(`grok_acp_client_request_reply`): `session/request_permission` selects the
agent's own `allow_once` option, and every other method — filesystem,
terminal, anything unknown — is refused with a recorded JSON-RPC `-32601`
error. The probe never reads or writes the filesystem, runs a shell, or
touches the network, and no answer is silent. When `session/new` has no
response, the cause separates an inbound request the probe failed to answer
(`session_new_unanswered`, harness-shaped) from the agent never responding
within the bound (`session_new_bound_exceeded`, provider-shaped).

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
  Other named causes cover missing `session/new`, a lost decisive frame,
  permission rejection, and an unattributed ACP echo title
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
- `truncated`: non-decisive middle frames were elided to stay under the
  512-frame capture bound. Decisive frames survive truncation, so this does
  not change the verdict; only a capacity filled entirely by decisive frames
  scores `inconclusive` with cause `truncated`
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
