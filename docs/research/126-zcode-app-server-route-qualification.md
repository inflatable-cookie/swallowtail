# 126 ZCode App-Server Route Qualification

Status: promoted
Owner: Tom
Date: 2026-08-17

## Question

Does Z.AI ZCode expose a stable enough machine boundary for a Swallowtail
adapter family, which surface should be first, and how much of Swallowtail's
portable feature set can that surface honestly claim?

## Method

Sources:

- official docs: welcome, install (desktop `v3.7.7`), FAQ, Connect Models,
  MCP, plugins
- unofficial npm `zcode-app-cli@3.7.7-13` and its README / CONFIGURATION /
  DEVELOPMENT notes
- community ACP bridge `william0wang/zcode-acp` PROTOCOL and backend client
  (wire evidence only; not a Swallowtail pin)
- isolated probe checkout `/Users/tom/Dev/scratch/zcode-probe` outside the
  Swallowtail tree
- `--version`, root help, `doctor --json`, payload hashes
- stdio handshake of `node vendor/zcode.cjs app-server` with isolated `HOME`
  and no provider key

No prompt was sent. No Z.AI account, Coding Plan key, or live model was used.
Session ids, workspace paths, and raw JSONL remain private capture data and
are not copied here.

This host does not have `ZCode.app` installed. Qualification used the npm
package that vendors `resources/glm/zcode.cjs` extracted from the
`3.7.7` Linux x64 deb. That is a packaging source, not a second product.

## Installed Artifact

Official product is a desktop ADE. The CLI runtime ships inside the app as
`resources/glm/zcode.cjs` and is not added to `PATH`. The probed npm package
vendors that runtime and injects a local TUI.

Two version numbers exist. Do not flatten them.

| Fact | Value |
| --- | --- |
| Product | ZCode (Z.AI ADE) |
| Desktop About | `3.7.7` |
| Probe package | `zcode-app-cli@3.7.7-13` |
| Runtime identity | `zcode-runtime 0.16.3` (`zcode --version` / `doctor --json`) |
| Executable | `vendor/zcode.cjs` |
| Executable size | 13,135,488 bytes |
| Executable SHA-256 | `3e3433d90fa502e5d02498dfde6c2090df898331359bcfe5f3dbc9a1d00b685f` |
| Launcher | `bin/zcode.js` (`#!/usr/bin/env node`) |
| Launcher SHA-256 | `36b9cb48bb79eab0c568909fb9830750f68f701a5aab16cb181c735909555362` |
| `package.json` SHA-256 | `3503bcbfe812aa8a6d663db649ec4b2e4d45571e049ad5d40723798c5200ee8f` |
| Extraction source | `ZCode-3.7.7-linux-x64.deb` (`extraction.json` `appVersion` `3.7.7`) |
| Probe Node | `v22.23.2` (`engines`: Node `>=22.19`) |
| Packaging | `node-bundle`; `sea` optional / false on this host |

`zcode -v` prints both launcher and runtime lines. `doctor --json`
`cli.version` is `0.16.3`. Desktop About `3.7.7` is not the compatibility
axis.

A host-approved desktop copy at
`/Applications/ZCode.app/Contents/Resources/glm/zcode.cjs` is the same
runtime class if the payload digest matches. Do not treat an unverified
desktop build as this pin.

The npm launcher is a Node shebang. Swallowtail must spawn
`LocalExecutableLaunch::interpreted_script(node, zcode.cjs)` with
`app-server`. Do not wrap Python, the TUI, or a community ACP bridge.

## Surfaces

Do not flatten these into one Swallowtail route.

| Surface | Machine boundary | Swallowtail role |
| --- | --- | --- |
| App-server stdio (`zcode app-server`) | Owned process; line-delimited JSON, JSON-RPC-shaped, no `jsonrpc` field; bidirectional requests | Primary installed-harness route |
| Headless `--prompt` / `--print` | One-shot stdout; `--prompt` help default mode `yolo` | Weaker one-shot; defer |
| Interactive TUI | Human terminal / slash commands | Not a driver |
| Desktop ADE | GUI | Not a driver |
| Community ACP (`zcode-acp`) | Translates app-server into ACP | Later distinct route only if Swallowtail speaks ACP to a native ZCode ACP; do not wrap the bridge |
| Custom / Z.AI HTTP model endpoints | Hosted Anthropic or OpenAI-compatible inference | Separate identity; not this harness |
| OpenCode HTTP | Existing `opencode.http` | Different product; do not merge |

App-server is the documented “ZCode Protocol” stdio server. Community
clients (ACP bridge, Pi `zcode-provider`) already speak it. That is
evidence the wire exists. It is not permission to pin those clients.

## Wire

Framing: one JSON object per line. Classification:

- `id` + `method` — request (client→server or server→client)
- `id` + `result`/`error` — response
- `method` only — notification

Handshake proven without a provider key:

1. Client sends `session/create` with workspace path/key and mode `plan`.
2. Server blocks on a server→client request
   `session/requestRuntimePreferences` (`scope`: `runtime-materialization`,
   string id). Create hangs until answered.
3. Client replies with defaults:
   `nativeSearchEnhancementsEnabled=false`,
   `memoryEnabled=false`,
   `askUserQuestionAutoResolutionEnabled=false`.
4. Server returns the `session/create` result. Observed top-level keys:
   `session`, `settings`, `projection`, `protocol`, `runtime`, `messages`,
   `slashCommands`, `todos`, `todoGroups`. Observed `session` fields:
   `sessionId`, `sessionKind`, `status`, `mode`, `model`, `title`,
   `traceId`, `target`, `workspace`, `createdAt`, `updatedAt`.

Do not skip the preferences reply. Community ACP auto-answers it for the
same reason.

Documented follow-on methods (community PROTOCOL; not live-prompted here):

- `session/subscribe` then `session/event` (`turn.started`,
  `model.streaming` `text_delta` / `reasoning_delta` / `tool_call`,
  `tool.updated`, `turn.completed` / `turn.failed`, `session.updated`,
  `turn.terminal`)
- `session/send` (enqueue; `{ accepted: true }`)
- `session/stop` (native cancel; fire-and-forget with a known startup race)
- `session/list`, `session/resume`, `session/read`, `session/messages`
- `session/setMode`, `session/setThoughtLevel`, `session/compact`,
  `session/steer`, `session/fork`, `session/rewind`, `session/goal`
- server→client `interaction/requestPermission` and
  `interaction/requestUserInput`

Unknown event types must be ignored. Do not project reasoning text, tool
argument or result bodies, prompts, or raw JSONL into diagnostics.

`--stdio` is implied by `app-server`. The probed spawn did not pass that
flag.

## Config And Trust

Isolated first launch writes credential-free `~/.zcode/cli/config.json`.
Observed default `permission.mode` is `build`. Provider map key `zai` is
required for the upstream no-login gate even when the endpoint is custom.
Custom `openai-compatible` / `anthropic` providers are documented. An
inline `options.apiKey` is required; env-only keys do not satisfy the
login gate.

`--prompt` help default is `yolo`. That is a CLI one-shot default, not a
Swallowtail default. Host supplies mode. Swallowtail does not ship `yolo`.

Host-approved config path, cwd, Node, and `zcode.cjs` are prepared
evidence. Swallowtail does not mint Z.AI credentials.

## Route Decision

ZCode qualifies for a dedicated installed-harness package and route,
distinct from OpenCode and from hosted GLM HTTP:

- package: `swallowtail-adapter-zcode`
- family: `zcode`
- route: `zcode.app-server`
- driver: `swallowtail.zcode.app-server`
- transport: owned process; line-delimited JSON over stdio
- version axis: `zcode.runtime`
- first qualified point: exact runtime `0.16.3` with the `zcode.cjs`
  digest above
- unverified-newer: no, until a second exact payload is qualified

Do not start with `--print`, the TUI, desktop GUI, community ACP, or an
extension of `swallowtail-adapter-opencode`.

## First Production Subset

One bounded structured run:

- host-approved Node and `zcode.cjs` at exact `0.16.3` / digest above
- spawn `app-server`; stdout stays protocol-only
- host-approved settings/config, cwd, mode, provider, and model
- answer `session/requestRuntimePreferences` before treating create as
  complete
- Swallowtail-owned idle from `session/send` through turn terminal /
  failed
- project turn lifecycle, assistant text, content-free reasoning
  progress, tool id/name/lifecycle, usage, terminal completed / error
- ignore unknown event types
- cancellation is force-stop of the owned process; do not advertise
  `session/stop` until a live race-safe fixture exists
- joined process cleanup on kill

Live proof may use a host-local OpenAI-compatible endpoint through the
custom-provider path (provider id `zai`). That qualifies the app-server
loop. It does not qualify Z.AI Coding Plan, OAuth, or official GLM.

## Deferred

- native `session/stop` cancel
- `--prompt` / `--print` one-shot
- session resume, list, fork, rewind, goal, compact, steer
- `session/messages` / `session/read` history (Contract 054 later)
- permission / elicitation (`interaction/*`)
- subagent / background-task topology
- model catalogue
- community or native ACP
- Z.AI official Coding Plan / OAuth live
- OpenCode flattening

## Contract Fit

No new provider-neutral contract is required for the first structured-run
subset. Contracts 005-006, 009-010, 023, 029, 032-033, 037, 039-041,
044-045, 051, and 052 already govern owned-process harness runs, prepared
evidence, activity, usage, interpreted-script launch, and fail-closed
diagnostics.

The codec can live in the new adapter until a second consumer needs a
shared crate.

Contract 036 still requires architecture/package review before the package
enters the workspace release set. Immutable `v0.3.2` must not be described
as containing this route.

## Recommendation

Promote ZCode into g03 as an installed-harness foundation:
`zcode.app-server` at exact runtime `0.16.3`. Evidence is sufficient for a
bounded structured-run driver after live prompt proof. Keep `--print`,
ACP, history, and native stop as later distinct work. Keep OpenCode and
hosted GLM HTTP unchanged.

Next planning move: spec the selected app-server subset, then compile a
g03 tranche. Freeze redacted fixtures from handshake captures during that
tranche; do not commit private transcripts into research.
