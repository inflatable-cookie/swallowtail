# 282 Kimi Code Local Server 0.41.0 Identity

Status: promoted
Owner: Tom
Date: 2026-09-04
Card: g05 batch 062

## Question

Is official `@moonshot-ai/kimi-code` `0.41.0` a compatible extension of the
`kimi-code.local-server` claim through `0.38.0`, a private milestone, a new
revision, or a stop?

The first run targeted `0.40.1`. Official latest moved to `0.41.0` during
that run. The operator retargeted this card to `0.41.0` on 2026-09-04;
collected `0.40.1` identity is published adjacency.

## Remaining AllowUnverified rank

Named family only.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Kimi Code local server (`kimi-code.local-server`) | installed `0.34.0` | exact `0.28.1` plus `0.29.0..=0.38.0` | operator-retargeted family; official npm and GitHub stable is `0.41.0` |

`kimi-code.acp` and `kimi-code.headless` share the npm package and stay
separate. Python `kimi-cli` and Kimi Platform Chat stay separate. Gemini
stays deferred. g05.009 card 034 stays untouched.

## Method

Re-probed npm `@moonshot-ai/kimi-code` and the GitHub release stream on
2026-09-04. Downloaded the `0.38.0`, `0.39.0`, `0.39.1`, `0.40.0`,
`0.40.1`, and `0.41.0` npm tarballs and the matching GitHub darwin-arm64
and linux-x64 zips plus `.sha256` sidecars and `manifest.json` to
`/tmp/kimi-card062`. Verified each tarball against the registry shasum,
each zip against its sidecar, and from `0.40.0` the extracted binary
against `manifest.json` (that release family checksums the extracted
payload, not the zip). Extracted without executing anything.

Compared selected local-server surfaces from git blobs at the annotated
tags: bearer middleware, REST model-catalog, ws-control, REST session,
approval, question, terminals, and `kimi web` index/run/shared. Traced
the `0.40.0` Bash `cwd` change in `RuntimeWorkspaceView.resolve` and
`bashTool` against Contracts 017 and 023 and the production
`AmbientHost` local-server launch.

Hashed host `~/.kimi-code/bin/kimi` and did not run, install, update, or
replace it. No provider prompt, authentication, catalogue, session, or
live server.

Official latest was `0.41.0` at probe and immediately before push.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | `0.34.0` | SHA-256 `9f4337e10da47843f6b550474012a53ba8b30dd665f83b176a5cd479c5f7e859`, size 176894272; not executed |
| Official npm latest | `0.41.0` | published 2026-09-04T11:01:04.740Z; integrity `sha512-9F89UvhJ…PfDyMAA==`; shasum `b0190679…`; tarball SHA-256 `4421e127…`; 547 files; `bin.kimi` = `dist/main.mjs` |
| GitHub tag | `@moonshot-ai/kimi-code@0.41.0` | annotated tag `d723a393…`; commit `95478e8c…`; release published 2026-09-04T11:01:07Z |
| `0.41.0` artifacts | — | darwin-arm64 ZIP `e7d32a5e…`, extracted `72b3cda4…`; linux-x64 ZIP `a51fbf04…`, extracted `5031a83b…`; npm `dist/main.mjs` `db8e0831…` |

Published stables after previous ceiling `0.38.0` are `0.39.0`, `0.39.1`,
`0.40.0`, `0.40.1`, and `0.41.0`. npm has no `0.38.1`, `0.39.2`, `0.40.2`,
or `0.41.1`. Not a major-line reset.

The `0.38.0` corpus was recomputed. Tarball `d5c047db…`, annotated tag
`488fe6bb…`, commit `0999454b…`, darwin-arm64 ZIP `48f534fc…`, extracted
`92bf3b4b…`, linux-x64 ZIP `2278e0c9…`, and extracted `7f18b701…` all
match the frozen `kimi-code-0.38.0` corpus. The three frozen local-server
protocol blobs also match: bearer middleware `9fedc57a…`, REST
model-catalog `f3f7105b…`, ws-control `12457745…`.

`0.40.1` identity collected in the first run is retained as adjacency:
tarball `dd6dd058…`, commit `0d45dddc…`.

## Selected protocol

Mapped REST/WebSocket v2 protocol files change once at `0.39.0` and then
hold through `0.41.0`. The `0.38.0`→`0.39.0` hop in `auth.ts`,
`rest-modelCatalog.ts`, `ws-control.ts`, and `rest-session.ts` is
comment-only: whitespace-stripped bytes are identical. Approval, question,
and `terminals.ts` blobs are byte-identical from `0.38.0` through
`0.41.0`. Heartbeat `ping`/`pong` is unchanged.

`kimi web` lost `--allow-remote-terminals` at `0.39.0`; PTY terminal
routes stay on loopback binds. Swallowtail never passed that flag and
already binds `127.0.0.1`. Remote Control files are new and stay
unmapped. Those CLI deltas are not a mapped wire-shape change.

`0.40.1`→`0.41.0` selected local-server blobs are byte-identical.

## Bash cwd authority at 0.40.0

`packages/agent-core-v2/src/agent/tools/os/bash/bashTool.ts` computes
`effectiveCwd` as `view.resolve(args.cwd ?? view.workDir)` at every
point checked. Through `0.38.0`, `RuntimeWorkspaceView.resolve` mapped
the path and then called `assertAllowed`, throwing `FS_PATH_ESCAPES`
outside the workspace roots. From `0.40.0` (commit `b4ae7f8`, holds at
`0.41.0`) `resolve` is a pure path mapping. The shipped bash test
accepts `cwd: '/outside/workspace'`. Session PTY create still calls
`assertAllowed`; that is not the Bash tool path.

Swallowtail's owned local-server launch is
`kimi web --no-open --host 127.0.0.1 --port <port> --log-level info`
under `HarnessIsolation::AmbientHost`. `ProcessRequest` sets no cwd.

### Containment trace

- Contract 023: `AmbientHost` means the harness process and descendants
  execute with ambient host authority. Tool allowlists do not contain
  the process.
- Contract 017: under `AmbientHost` the working-resource lease is a
  location and callback scope only. Setting a working directory does
  not prove containment.
- No adapter control mediates Bash `cwd`.
- Loopback bind contains network exposure, not process cwd.
- After `0.40.0` no remaining provider assertion contains Bash cwd.

Containment is absent. The change widens local process authority for
this transport.

### A2 comparison

ACP A2 stopped `kimi-code.acp` at `0.38.0` because `acpTerminalRunner`
started spawning host processes when Swallowtail advertised
`terminal: false`. The mechanism here is different: a provider-internal
workspace assertion on `resolve()` used by the Bash tool, not a
Swallowtail-advertised capability that flipped. The conclusion is the
same: uncontained process-authority widening, so this family stops.

Wire-shape stability of the selected REST/WS v2 subset is real and is
not sufficient to qualify the authority change.

## Unmapped extras

Remote Control, the `0.41.0` auto-permission-mode drop of the
dangerous-command guard, web UI plugins/tower/rating, ACP
`KIMI_CODE_LEGACY_FLAG` ignored on `kimi acp`, and the secondary-model
pool default stay unmapped. ACP/headless observations are recorded only.

## Decision

**Stop.** Keep claim id `kimi.local-server.executable-window-2`,
baseline `0.28.1`, ceiling `0.38.0`, heartbeat-ping behavior, and
`AllowUnverified`. Do not admit a segment for card 063. Do not edit
`local_server/selection.rs`. Synthetic later-stable remains unpublished
`0.41.1` as observation only.

Reopen when a named Swallowtail control, `ProviderEnforced` /
`HostEnforced` isolation, or restored provider workspace assertion
contains Bash `cwd` for a local-server client.

No new public operation, behavior revision, or shared type. Decoder
specimens stay on `kimi-local-server-0.28.1-0.29.0`. ACP and headless
claims stay as they are.

This record edits no production claim.

## Sources

- npm `@moonshot-ai/kimi-code` `0.38.0` through `0.41.0`
- [GitHub `0.41.0`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.41.0)
- `apps/kimi-code/CHANGELOG.md` at `0.40.0`, `0.40.1`, and `0.41.0`
- git blobs at commits `0999454b`, `52e8d19d`, `5efca0c3`, `e27ee608`,
  `0d45dddc`, and `95478e8c`
- frozen `crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-0.38.0/`
  and `kimi-local-server-0.41.0/`
- [Contract 017](../contracts/017-provider-owned-session-load-replay-and-host-containment.md)
  and [Contract 023](../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Research 270](./270-kimi-code-0-39-1-identity.md),
  [Research 276](./276-all-route-version-currentness-checkpoint.md)
