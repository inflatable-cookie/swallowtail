# 326 Kimi Code Local Server 0.43.0 Containment

Status: promoted
Owner: Tom
Date: 2026-09-15
Card: g05.078 (Research 308 containment campaign)
Authority: Contracts 017, 023, and 029; Research 270, 282, 308, and 325;
g05.017, g05.026, and g05.077; the Kimi Code local-server selection, prepared
route, drivers, protocol, activity, and frozen fixtures; and the official npm
and GitHub channels.

## Question

Does the separate `kimi-code.local-server` claim advance past `0.38.0`
through official stable `0.43.0`, and if the Research 282 authority break
persists, what is the maximal safe prefix and what posture makes every later
point fail closed?

## Remaining AllowUnverified rank

Named family only.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Kimi Code local server (`kimi-code.local-server`) | installed `0.34.0` | exact `0.28.1` plus `0.29.0..=0.38.0` | operator-authorized family; official npm and GitHub stable is `0.43.0` |

`kimi-code.acp` and `kimi-code.headless` share the npm package and stay
separate. Python `kimi-cli` and Kimi Platform Chat stay separate.

## Method

Re-probed npm `@moonshot-ai/kimi-code`, the GitHub release stream, and the
installed host `kimi --version` on 2026-09-15. npm `latest` is `0.43.0`,
published `2026-09-14T12:10:41.073Z`. GitHub latest is
`@moonshot-ai/kimi-code@0.43.0`, published `2026-09-14T12:03:30Z`. The
installed host remains `kimi 0.34.0` at
`sha256:9f4337e10da47843f6b550474012a53ba8b30dd665f83b176a5cd479c5f7e859`.

Re-downloaded the official npm tarballs for `0.42.0` and `0.43.0` into `/tmp`
and reproduced every registry `integrity` and `shasum`, the tarball SHA-256,
the file count (547 and 543), and the `dist/main.mjs` digest and size. Tagged
source for all eight points `0.38.0..=0.43.0` was read from the official
GitHub repository without checkout: every annotated tag resolves to the frozen
commit, and every tree hash reproduces Research 282/325. No downloaded
artifact was executed, installed, or authenticated; no prompt, catalogue call,
login, credential, host update, live session, or local-server start occurred.

Compared the exact source files feeding `kimi web --no-open --host
127.0.0.1`, REST/WS v2, authentication, session operations,
approval/question, Bash and terminal execution, and permission policy as git
blobs at the eight tagged commits: bearer middleware, REST model-catalog,
ws-control, REST session, REST approval, REST question, PTY terminal routes,
`kimi web` index/run/shared, `bashTool.ts`, `RuntimeWorkspaceView`, and the
session terminal service. Verified comment-only hops by comparing
comment-stripped whitespace-stripped bytes.

Official latest was `0.43.0` at probe and immediately before push.

## Identity

| Version | npm published | npm tarball SHA-256 | `dist/main.mjs` SHA-256 | GitHub annotated tag / commit / tree |
| --- | --- | --- | --- | --- |
| `0.38.0` | revalidated | `d5c047db…` | `16be7e50…` | `488fe6bb` / `0999454b` / `b0f988c1` |
| `0.39.0` | 2026-08-27T11:36:25.525Z | `b42ab693…` | — | `9076cf66` / `52e8d19d` / `5baec16c` |
| `0.39.1` | 2026-08-28T10:01:03.520Z | `22594a76…` | `ed24f532…` | `1c142e2b` / `5efca0c3` / `88491300` |
| `0.40.0` | 2026-09-02T05:58:56.966Z | `e947fa37…` | `c0efe89f…` | `042a03e4` / `e27ee608` / `4a9246c0` |
| `0.40.1` | 2026-09-02T09:20:41.955Z | `dd6dd058…` | `d0b563b9…` | `34afca12` / `0d45dddc` / `c00bba78` |
| `0.41.0` | 2026-09-04T11:01:04.740Z | `4421e127…` | `db8e0831…` | `d723a393` / `95478e8c` / `8d692694` |
| `0.42.0` | 2026-09-09T06:24:39.794Z | `686f888c…` | `3f632148…` | `62290bef` / `6954d2c8` / `c4f8f553` |
| `0.43.0` | 2026-09-14T12:10:41.073Z | `225bc17f…` | `5b300a57…` | `eb832931` / `ffa94fae` / `cfad0ffd` |

The full strings, file counts, unpacked sizes, and release-asset digests are
in `tests/fixtures/kimi-local-server-0.43.0/identity.json`. Published stables
after the previous ceiling `0.38.0` are exactly `0.39.0..=0.43.0`. npm has no
`0.38.1`, `0.39.2`, `0.40.2`, `0.41.1`, `0.42.1`, or `0.43.1`. Not a
major-line reset.

## Selected local-server ledger

`tests/fixtures/kimi-local-server-0.43.0/protocol.json` carries the full
`0.38.0..=0.43.0` git-blob ledger. Per-hop verdict:

| Hop | Verdict |
| --- | --- |
| `0.38.0→0.39.0` | safe: the four mapped wire schemas are comment-only (doc-comment removal, code identical); approval, question, and terminal routes byte-identical; heartbeat `ping`/`pong` unchanged |
| `0.39.0→0.39.1` | safe: every selected blob byte-identical |
| `0.39.1→0.40.0` | **stop**: `RuntimeWorkspaceView.resolve` drops the `assertAllowed` call and becomes a pure path mapping; `bashTool` still computes `effectiveCwd` as `view.resolve(args.cwd ?? view.workDir)`, so Bash `cwd` escapes the workspace roots |
| `0.40.0→0.40.1` | still stopped: every selected blob byte-identical |
| `0.40.1→0.41.0` | still stopped: every selected blob byte-identical; the auto-mode dangerous-command guard drop stays unmapped provider tool policy |
| `0.41.0→0.42.0` | still stopped: model-catalog import-source move, `watch_fs` removal, delete-response reshape, terminal compat-schema removal, and Remote Control de-experimentalization are all unmapped or inert on the selected route; both Bash authority blobs unchanged |
| `0.42.0→0.43.0` | still stopped: one added `localServerToken` field in the unmapped Remote Control QR output; every other selected blob byte-identical; both Bash authority blobs unchanged |

The safe prefix is therefore `0.39.0..=0.39.1`: at both points `resolve`
still maps then calls `assertAllowed`, throwing `FS_PATH_ESCAPES` outside the
workspace roots. The `0.39.0` `RuntimeWorkspaceView` delta is a win32
shell-path bridge only, and the `bashTool` delta is shell-path bridging,
output-accumulator renaming, `toolCallId` threading, and description text —
none touches the `cwd` authority path.

The `0.40.0` uncontained `resolve` (`01db1bbf…`) and the `0.39.0`-shaped
`bashTool` (`41090010…`) are byte-identical at every point through `0.43.0`.
No later release restores containment.

## Containment trace

- Contract 023: `AmbientHost` means the harness process and descendants
  execute with ambient host authority. Tool allowlists do not contain
  the process.
- Contract 017: under `AmbientHost` the working-resource lease is a
  location and callback scope only. Setting a working directory does
  not prove containment.
- No adapter control mediates Bash `cwd`. `ProcessRequest` sets no cwd.
- Loopback bind contains network exposure, not process cwd.
- Session PTY create still calls `assertAllowed`; that is not the Bash tool
  path.
- After `0.40.0` no remaining provider assertion contains Bash cwd, and no
  later hop adds one.

Containment is absent from `0.40.0` through `0.43.0`. The change widens local
process authority for this transport. Same risk class as the ACP A2 stop,
different mechanism: a provider-internal workspace assertion on `resolve()`
used by the Bash tool, not a Swallowtail-advertised capability that flipped.

## Unmapped extras

Remote Control routes, QR output, and experimental gating; `watch_fs`
subscription and messages; the session-delete response reshape; the
terminal-route `runtime_id` compat schema; the model-catalog import-source
move; the `0.41.0` auto-permission-mode dangerous-command guard drop;
agent-core-v2 prompt/loop submission, DI, telemetry, and
shutdown-quiescence internals; web UI plugins/tower/rating. The adapter maps
none of these: it never passes `--remote-control`, never subscribes
`watch_fs`, returns unsupported for session deletion, sends no `runtime_id`
to the PTY terminals endpoint, and runs no print-route permission mode.
ACP/headless observations are recorded only.

## Decision

**Extend and fail closed.** Keep claim id
`kimi.local-server.executable-window-2` and baseline `0.28.1`. Extend the
maintained heartbeat-ping segment from `0.35.0..=0.38.0` to
`0.35.0..=0.39.1` on the unchanged
`kimi.local-server.rest-ws-v2-heartbeat-ping` behavior revision, and change
the newer-version posture from `AllowUnverified` to `QualifiedOnly` so every
point above `0.39.1` fails closed — including the published `0.40.0..=0.43.0`
gap. No new behavior revision, public operation, or shared type. Decoder
specimens stay on `kimi-local-server-0.28.1-0.29.0`. ACP and headless claims
stay as they are.

A later segment may reopen only on exact restored containment: a provider
workspace assertion, explicit Swallowtail mediation, or
`ProviderEnforced`/`HostEnforced` isolation containing Bash `cwd` for a
local-server client.

Card g05.078 owns the claim change.

This record edits no production claim.

## Sources

- npm `@moonshot-ai/kimi-code` `0.38.0` through `0.43.0`
- [GitHub `0.43.0`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.43.0)
- `apps/kimi-code/CHANGELOG.md` at `0.40.0` through `0.43.0`
- git blobs at commits `0999454b`, `52e8d19d`, `5efca0c3`, `e27ee608`,
  `0d45dddc`, `95478e8c`, `6954d2c8`, and `ffa94fae`
- frozen `crates/swallowtail-adapter-kimi/tests/fixtures/kimi-local-server-0.43.0/`
  and the historical `kimi-code-0.38.0/` and `kimi-local-server-0.41.0/`
  corpora
- [Contract 017](../contracts/017-provider-owned-session-load-replay-and-host-containment.md),
  [Contract 023](../contracts/023-harness-operation-isolation-and-native-boundary.md),
  and [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md)
- [Research 270](./270-kimi-code-0-39-1-identity.md),
  [Research 282](./282-kimi-code-local-server-0-41-0-identity.md),
  [Research 308](./308-all-route-version-currentness-checkpoint.md), and
  [Research 325](./325-kimi-code-0-43-0-installed-identity.md)
