# 320 Kiro ACP 2.21.4 Identity

Status: promoted; identity evidence. The claim rebind lands in g05.072 with
this record.

Owner: Tom
Date: 2026-09-14
Card: g05.072 (Research 308 useful-newer campaign)
Authority: Contract 029; Research 156, 251, 254, 308, and 319; the Kiro
prepared-integration guide; and the official Kiro stable installer channel.

## Answer

Official stable-manifest latest is `2.21.4`. The exact `kiro-cli.release`
point can move from `2.18.1` to `2.21.4` as a compatible extension of the
selected ACP surface: every selected wire, lifecycle, permission,
authentication, process-authority, and cleanup input is byte-stable across
all eleven published stable hops. This is one exact `QualifiedOnly` point,
not a range. The `kiro.acp.stdio-v1` behavior revision is unchanged.

## Method and channel boundary

The official stable manifest, installer script, ACP docs, and changelog were
rechecked at the start of the run and again immediately before the identity
commit. The manifest tip is `2.21.4` (universal macOS DMG
`954a48e8…f95b7`, Linux aarch64 headless tar.xz
`f582eac0…55df6f`, both matching the g05.072 planning digests). Per-version
manifest endpoints under `/stable/<version>/` return HTTP 403 AccessDenied,
so the version ledger comes from the official changelog entries (2.19.0,
2.20.0, 2.21.0 headlines with patch entries 2.19.1, 2.19.2, 2.20.1, 2.20.2,
2.21.1, 2.21.2, 2.21.3, 2.21.4) plus per-version artifact downloads.

All twelve points (baseline `2.18.1` through `2.21.4`) were retrieved as
exact Linux aarch64 headless tar.xz archives into `/tmp` (7.3 GB total),
hash-verified, and extracted without installation. The baseline archive
reproduces Research 156's recorded digest byte-for-byte. The selected
baseline `2.18.1` and official `2.21.4` universal macOS DMGs were also
retrieved, hash-verified, and read-only mounted for static inspection of the
mac bundle: same four-binary layout, matching `CFBundleShortVersionString`
values, and per-executable digests recorded. No artifact was executed, no ACP
message was sent, no login, credential, installation, host update, or
provider credit was used. `kiro-cli` remains absent from this host.

## Complete artifact result

The shipped Linux tree has the same eight paths at every hop
(`BUILD-INFO`, `README`, `install.sh`, and `bin/{q,qchat,kiro-cli,
kiro-cli-chat,kiro-cli-term}`). `install.sh`, `README`, `q`, and `qchat`
are byte-identical across all twelve points. `BUILD-INFO` and the three
Kiro binaries change at every hop; `BUILD-INFO` carries the exact release,
an aarch64-gnu target triple, and a per-hop build hash. The chat binary
grows from 668 MB to 829 MB across the chain.

## Selected-surface classification

The selected entrypoint is `kiro-cli acp`: the `kiro-cli` router owns the
`acp` subcommand (`AcpArgs`, help `Agent Client Protocol (ACP)`) and
launches the chat binary, whose `chat-cli-v2` agent implements the stdio
serve over `sacp-11.0.0` plus `agent-client-protocol-0.10.4`. Both library
pins and their ten-file `sacp` source set are identical at every hop, and
the ACP serving module closure (`src/agent/acp/`, 43 baseline files)
changes only by the additive `extension_request.rs` at `2.19.2`.

Stable at every hop unless named:

- `initialize`: the ACP capability struct literals
  (`AgentCapabilities` with 8 elements, `InitializeResponse` with 5,
  `PromptCapabilities` with 4, `NewSessionResponse` with 5) are present
  exactly once per hop; `loadSession`/`promptCapabilities`/`agentInfo`
  shape unchanged.
- `session/new`: `cwd`+`mcpServers` request literal and the
  `sessionId`-only parse unchanged; the frozen bridge
  `newSession({cwd:process.cwd(),mcpServers:[]})` is present at every hop.
- `session/prompt`: the field is `prompt` at every hop
  (`this.connection.prompt({prompt:e,sessionId:…})` exactly once per hop).
  The official docs page (unchanged digest since Research 254) still shows
  the stale `content` example at `2.21.4`; sending `content` stays a named
  negative.
- `session/cancel`: notification lookup unchanged.
- `session/update` kinds: the
  `user_message_chunk|agent_message_chunk|agent_thought_chunk|tool_call|
  tool_call_update` discriminator literal set is present at every hop.
- Permission kinds `allow_once|allow_always|reject_once|reject_always`
  present at every hop; Swallowtail still never selects `allow_always` and
  never maps `--trust-all-tools`.
- Stop reasons (`end_turn`, `cancelled`, `refusal`, `max_tokens`,
  `max_turn_requests`) unchanged; no typed `auth_required` failure code
  appears on the selected paths (contrast Research 319's Goose stop).
- `KIRO_API_KEY`, `KIRO_HOME`, and the session-root env names are
  unchanged; the host-auth and working-resource authorities are unchanged.

Bounded unmapped deltas:

- `2.19.1..2.19.2`: the ACP extension-request handler appears
  (`extension_request.rs`) and begins dispatching
  `_kiro.dev/commands/execute`, `_kiro.dev/commands/options`,
  `_kiro.dev/session/list`, `_kiro.dev/session/terminate`,
  `_kiro.dev/settings/list`, `_kiro.dev/settings/set`,
  `_kiro.dev/session/steer` (+`clear`), `_kiro.dev/session/spawn`, and
  `_kiro.dev/message/send`. Swallowtail does not map `_kiro.dev/*`
  extensions; these are advertised-only on the selected route.
- `2.20.1..2.20.2`: the serve method-literal region gains `session/load`
  (`initializesession/newsession/load…`), and `_kiro.dev/mcp/
  startup_status` joins the extension set. `session/load` stays
  advertised-only (`loadSession: true` was already advertised at the
  baseline) and unmapped.
- `2.21.3..2.21.4`: the chat binary's JS chunk is rebuilt. The old sorted
  bridge method map (which still named `session_set_model`) is replaced by
  a new KAS client chunk (`initialize`, `authenticate`, `providers_list`,
  …). `session/set_model` stays unsupported either way. The router
  subcommand soup gains the chat-only `--v2` harness flag (changelog:
  "run a single session on the V2 harness"); it is a chat flag and is never
  part of the selected ACP argv, which is byte-identical across all hops.

## Effect on Research 251 and 254

Research 251 (effort) and 254 (agent profiles) recorded empty deliver-now
sets partly because the `2.18.1` platform archives returned HTTP 403 on
2026-08-28. This run retrieves the exact artifacts, so their package/source
gates are now closed with per-hop binary evidence: the chat CLI effort
parsing (`--effort` under `kiro-cli chat`) and the `--agent` placeholder
remain outside the ACP entrypoint docs and outside the selected argv at
every hop through `2.21.4`. Their empty deliver-now sets stand unchanged.

## Decision and disposition

The identity-first decision is a compatible exact-point rebind: the
`kiro.acp.release-window-1` claim moves to one maintained `2.21.4` point
with the unchanged `kiro.acp.stdio-v1` behavior revision and `QualifiedOnly`
posture. No range, second exact point, exclusion, or unverified-newer
posture is added. The frozen `kiro-acp-2.18.1` decoder corpus still covers
the selected wire because every selected-surface literal is unchanged, so
the corpus and its fixtures stand unchanged. The new evidence corpus is
[`kiro-acp-2.21.4`](../../crates/swallowtail-adapter-kiro/tests/fixtures/kiro-acp-2.21.4/)
with the mutation-sensitive assertions in
[`kiro_2_21_4_delta_ledger.rs`](../../crates/swallowtail-adapter-kiro/tests/kiro_2_21_4_delta_ledger.rs).

## Non-goals

- promoting `_kiro.dev/*` extensions, `session/load`, `session/set_model`,
  `--agent`, `--effort`, `--v2`/`--v3`, or `--trust-all-tools` onto the
  route
- live initialize, prompt, login, or API-key use
- installing Kiro or updating the host
