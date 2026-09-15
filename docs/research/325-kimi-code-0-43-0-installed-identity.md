# 325 Kimi Code 0.43.0 Installed Identity

Status: promoted
Owner: Tom
Date: 2026-09-15
Card: g05.077 (Research 308 useful-newer campaign)
Authority: Contracts 017, 023, and 029; Research 179, 210, 211, 270, 282, and
308; g05.016 and g05.017; the Kimi Code installed ACP/headless selection,
prepared profiles, commands, drivers, decoders, and frozen fixtures; and the
official npm and GitHub channels.

## Question

Do official npm and GitHub `@moonshot-ai/kimi-code` `0.40.0`, `0.40.1`,
`0.41.0`, `0.42.0`, and `0.43.0` independently extend the Kimi Code installed
ACP and headless claims from their current ceilings (`0.38.0` and `0.39.1`),
or does either selected route stop at its first exact incompatible hop?

## Remaining AllowUnverified rank

Kimi installed routes are family seventeen in Tom's 2026-09-14 Research 308
direction. The serial order still has the separate Kimi Code local-server
family and Oh My Pi after it. `kimi-code.local-server` is Research 282's
family; this record observes it and edits nothing it owns.

## Method

Re-probed npm `latest`, the GitHub latest stable release, and the installed
host `kimi --version` on 2026-09-15. npm `latest` is
`@moonshot-ai/kimi-code@0.43.0`, published `2026-09-14T12:10:41.073Z`. GitHub
latest is `@moonshot-ai/kimi-code@0.43.0`, published `2026-09-14T12:03:30Z`.
The installed host remains `kimi 0.34.0`.

Downloaded the official npm tarballs for `0.39.1`, `0.40.0`, `0.40.1`,
`0.41.0`, `0.42.0`, and `0.43.0` into `/tmp` and reproduced every registry
`integrity` and `shasum`, the tarball SHA-256, the file count, the unpacked
size, the `bin` entry, and the `dist/main.mjs` digest and size. Downloaded the
matching GitHub tagged source archives, resolved every annotated tag object to
its commit and tree, and hashed each source archive. Downloaded the `0.43.0`
npm bundle and the `0.41.0` bundle values reproduce the `kimi-local-server-0.41.0`
corpus exactly. No downloaded artifact was executed, installed, or
authenticated; no prompt, catalogue call, login, credential, host update, or
live session occurred.

Built one deterministic inventory per extracted tagged source tree: a sorted
`path -> SHA-256` map over the whole archive, so each hop's changed-path set is
the difference of two maps and no whole-tree rescan is needed. From that
inventory and the documented selected surfaces, froze
`tests/fixtures/kimi-code-0.43.0/` with an npm/GitHub identity table, a
mutation-sensitive selected-file blob ledger, per-hop ACP/headless
classification, and the A2 continuation result. Also extracted the executed
`AcpProcessService` class and the mapped headless emission surfaces from all
six npm bundles as an independent corpus oracle.

## Identity

Host `kimi 0.34.0` is unchanged: executable SHA-256
`9f4337e10da47843f6b550474012a53ba8b30dd665f83b176a5cd479c5f7e859`, size
176 894 272, observed only through `--version` and its digest.

| Version | npm published | npm tarball SHA-256 | `dist/main.mjs` SHA-256 | GitHub annotated tag / commit / tree |
| --- | --- | --- | --- | --- |
| `0.39.1` | 2026-08-28T10:01:03.520Z | `22594a76d0aec0cdabd41050fdd354381c106c48a2f8f5edf98394b4b5e987f7` | `ed24f532d07e5d00777ae20b42ed18437e116b050bbcfc6ae0f3bc90affca337` | `1c142e2b` / `5efca0c3` / `88491300` |
| `0.40.0` | 2026-09-02T05:58:56.966Z | `e947fa378eb3f36b306ab1ebc39f74af5aadacba49e8968a5b98ec47f0cc2e83` | `c0efe89fda04f746a2ea73bd95edd6808e7cc93d257525fb379330b6adfa75a6` | `042a03e4` / `e27ee608` / `4a9246c0` |
| `0.40.1` | 2026-09-02T09:20:41.955Z | `dd6dd058384a500a08bc9d3982a8e04eb248c69403869dd16bd20353ef75e5c3` | `d0b563b97d70d86ba98c86b1511cee15053b2f7d3cc84998d961dc3d2ccc881c` | `34afca12` / `0d45dddc` / `c00bba78` |
| `0.41.0` | 2026-09-04T11:01:04.740Z | `4421e1277bbfa5e46a8e1a863fd9ba4d1a3db8dd890d928f571171ac62a80c1e` | `db8e083187241ce6683793ce366f1e100e732caa4f65bd28bbc3fac739762e95` | `d723a393` / `95478e8c` / `8d692694` |
| `0.42.0` | 2026-09-09T06:24:39.794Z | `686f888cfe7ef888159ffeb760bcdce9f4816e8b7fe9962529e59251b0ac3372` | `3f632148344f68c15633215244e1ca8c106116051cd0e744968906773230930a` | `62290bef` / `6954d2c8` / `c4f8f553` |
| `0.43.0` | 2026-09-14T12:10:41.073Z | `225bc17f06243edf6bcf0fc82bbe8838cab1cd426eb8e467e9ebab93d53ace90` | `5b300a573d306f879361eb7a2baec3a19448ea94eadca28001408b19c42eeb69` | `eb832931` / `ffa94fae` / `cfad0ffd` |

Every npm `integrity` and `shasum` reproduces the registry value; the full
strings and the per-point GitHub release-asset digests are in
`identity.json`. The published stable points after the previous ceilings are
exactly `0.39.1..=0.43.0`, `0.43.1` is the first unpublished later stable, and
there is no major-line reset.

The `0.39.1` npm tarball and bundle reproduce Research 270's frozen values
byte-for-byte, and the `0.41.0` values reproduce the
`kimi-local-server-0.41.0` corpus.

## Published selected-path classification per claim

| Hop | ACP | Headless |
| --- | --- | --- |
| `0.39.1→0.40.0` | the legacy `acp-adapter` package and `acp-native.ts` are removed and `sub/acp.ts` lazily imports `@moonshot-ai/acp-server` directly; `server.ts` adds a `restore()` requirement after `session/fork`; the other four changed files are comments | `experimental-v2.ts` doc comment, `run-v2-print.ts` agent-scoped goal/cron lookup and a `nonInteractive: true` request header; no emission change |
| `0.40.0→0.40.1` | no change at all | no change at all; only `apps/kimi-code/package.json` moves |
| `0.40.1→0.41.0` | no change at all | `run-v2-print.ts` adds the print-mode quiesce, wire-flush, and telemetry shutdown work (the lost-session-record fix) and honours `KIMI_DISABLE_TELEMETRY`; no emission change |
| `0.41.0→0.42.0` | image-format gating moves into the provider-bound engine; tool-event type imports move to agent-core-v2 re-exports; `interaction-bridge.ts` is a comment edit | the v1 print body and the `KIMI_CODE_LEGACY_FLAG` gate are deleted, `run-v2-print.ts` submits through `IAgentLoopService` and warns on stderr about trust-gated MCP servers; `prompt-render.ts`, `options.ts`, and the dispatch switch are byte-identical |
| `0.42.0→0.43.0` | `replay.ts` suppresses hidden thinking deltas | prompt submission moves to `IAgentLoopService.submit` / `promptHandle` and `kimi upgrade --yes` lands; no emission change |

### Headless

The selected headless route is `kimi --model <model> --prompt <content>
--output-format stream-json`, which has run agent-core-v2 `runV2Print` by
default since `0.33.0`. Across all five hops the `dispatchNativeEvent` switch
is byte-identical — the same ten case labels, the same `writer` calls, the same
`tool.progress` stderr branch — and `prompt-render.ts` (the `PromptJsonWriter`,
`PromptTranscriptWriter`, `system.version` preamble, `session.resume_hint`, and
`stringifyToolOutput` writers) and `options.ts` are byte-identical git blobs.
The `TurnStepRetryingPayload` field set is identical from `0.39.1` through
`0.43.0` even though the file moves from `agent/stepRetry/stepRetryService.ts`
to `agent/loop/turnEvents.ts` at `0.42.0`. The npm bundle carries one distinct
digest for each of the seven mapped emission surfaces across all six points.

The hops do change the runner internals: `0.40.0` moves goal/cron service
resolution to agent-scoped services, `0.41.0` reworks shutdown quiescence and
wire flushing, `0.42.0` deletes the legacy v1 body and the `experimental-v2`
gate, and `0.43.0` moves prompt submission and cancellation onto
`IAgentLoopService`. None of those changes the argv, the JSONL roles, the meta
types, the retry record, the tool record, the terminal outcome, the retention
source, the process-level cancellation surface, or the cleanup contract.

Two deltas stay unmapped and are recorded rather than widened into the claim:

- `0.41.0` drops the auto-permission-mode deny guard for dangerous and
  unanalyzable Bash commands. It is provider tool policy behind the mode the
  print route forces for itself, Research 282 already recorded it as unmapped,
  and the route's ambient-host process authority is unchanged — the harness
  always owned the commands it runs.
- `0.42.0` writes a trust-gated project-level MCP warning to stderr. It is a
  stderr diagnostic, not a `stream-json` stdout record.

Decision: compatible extension. Keep `kimi.headless.executable-window-2`,
`kimi.headless.stream-json.v2`, baseline `0.33.0`, and `AllowUnverified`;
qualify `0.40.0`, `0.40.1`, `0.41.0`, `0.42.0`, and `0.43.0`, raising
`KIMI_HEADLESS_LATEST_QUALIFIED_VERSION` to `0.43.0`. Unpublished `0.43.1`
stays the visible `UnverifiedNewer` point.

### ACP

The g05.017 A2 ruling is binding: a newer ACP point is admitted only if exact
shipped source proves the host-process authority is bounded or absent. It is
not.

`packages/acp-server/src/acp-terminal/acpTerminalRunner.ts` is git blob
`9016d48b643f35b263449d98dee25597a9a24d30` at all six points, and the
`AcpProcessService` class extracted from the executed npm `dist/main.mjs` is
SHA-256 `7c58e045273d9dbcea96e38d0792d6122193810cd3b9a7c93bb02d40d21284e4` at
all six — the exact digest the frozen `kimi-code-0.39.0-acp-authority` corpus
records for `0.39.0` and `0.39.1`. The branch

```
if (!this.connection.terminalEnabled || !isBashToolInvocation(args, options)) {
  return this.local.spawn(command, args, { ...options, cwd: options?.cwd ?? this.cwd });
}
```

still delegates to a direct host-process spawn. Swallowtail always advertises
`clientCapabilities.terminal: false` and `auth.terminal: false`, so
`terminalEnabled` is always false and that branch is always taken. The
containment trace is unchanged: the route declares `HarnessIsolation::AmbientHost`,
Contract 015 holds that process ownership implies neither callback authority
nor filesystem containment, and no adapter-side or runtime-side control
mediates a process the harness spawns for itself. Containment is absent.

The other ACP changes in the window are session-fork restoration, image-format
gating moved to the engine, type-import reorganisation, and a hidden-thinking
filter in replay. None adds or removes an authority path, and none contains the
one that exists.

Decision: stop unchanged. Keep `kimi.acp.executable-window-5`, baseline
`0.28.1`, ceiling `0.38.0`, `QualifiedOnly`, and the exact `0.39.0` / `0.39.1`
exclusions. The published gap `0.40.0..=0.43.0` is posture-rejected
`Incompatible`; the deny-list does not grow. No new ACP behavior revision,
public operation, or shared type.

## Unmapped extras

The `acp-server` image-format gating move, the `replay.ts` hidden-thinking
filter, the print-mode trust-gated MCP stderr warning, the auto-permission-mode
dangerous-command guard drop, the agent-core-v2 prompt/loop submission and
cancellation rework, the retry/requester and compaction internals, dynamic
`deferred` MCP tool loading, `kimi upgrade --yes`, `kimi migrate --run`, the
subagent model pool, always-on Remote Control, minidb read model and search
worker, AI session titles, Tower, the web UI, and the session picker all stay
unmapped or with their own family.

## Separate family observations

Recorded for `kimi-code.local-server` only; this record edits none of its
claim, fixture, route, guide, matrix cell, or conclusions. `0.40.0` removed the
Bash `cwd` workspace assertion that stopped that family, and `0.41.0` dropped
the auto-mode dangerous-command deny guard. Research 282 already owns both and
records the family as stopped at `0.38.0`. `kimi-code.headless` and
`kimi-code.acp` observations there are not carried back the other way. Python
`kimi-cli` and Kimi Platform Chat stay separate axes.

## Decision

A split outcome.

**ACP — stop unchanged.**

- Keep exact `0.28.1` Deprecated on `kimi.acp.reasoning.legacy-select-v1`.
- Keep maintained `0.29.0..=0.38.0` on
  `kimi.acp.reasoning.declared-effort-v2` and
  `KIMI_CODE_LATEST_QUALIFIED_VERSION` at `0.38.0`.
- Keep exclusions exactly `0.39.0` and `0.39.1`; `0.40.0..=0.43.0` is the
  posture-rejected published gap, not a new deny-list entry.
- No new behavior revision. Requalification still needs provider-side
  containment or a Swallowtail-side control that does not exist.

**Headless — extend.**

- Keep `0.29.0..=0.32.0` Deprecated on `kimi.headless.stream-json.v1`.
- Extend maintained `0.33.0..=0.39.1` to `0.33.0..=0.43.0` on
  `kimi.headless.stream-json.v2`.
- Raise `KIMI_HEADLESS_LATEST_QUALIFIED_VERSION` to `0.43.0`.
- Host `0.34.0` stays qualified Maintained v2.

Shared:

- No new behavior revision, public operation, shared type, or public API
  change. Decoder specimens stay on the existing corpora.
- Do not widen or edit `kimi-code.local-server`. Do not touch g05.009 or
  card 034.

Card g05.077 owns the claim change.

## Sources

- [npm `@moonshot-ai/kimi-code@0.43.0`](https://registry.npmjs.org/@moonshot-ai%2Fkimi-code/0.43.0)
- [GitHub `0.43.0` release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.43.0)
- npm tarballs and GitHub tagged source archives for `0.39.1`, `0.40.0`,
  `0.40.1`, `0.41.0`, `0.42.0`, and `0.43.0`
- `apps/kimi-code/CHANGELOG.md` at `0.40.0` through `0.43.0`
- tagged-source commits `5efca0c3`, `e27ee608`, `0d45dddc`, `95478e8c`,
  `6954d2c8`, and `ffa94fae`
- frozen `crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-0.43.0/`,
  plus the historical `kimi-code-0.38.0/`, `kimi-code-0.38.0-headless-v2/`,
  `kimi-code-0.39.0-acp-authority/`, and `kimi-code-0.39.1/` corpora
- [Contract 015](../contracts/015-acp-v1-negotiation-and-client-callbacks.md),
  [Contract 017](../contracts/017-provider-owned-session-load-replay-and-host-containment.md),
  [Contract 023](../contracts/023-harness-operation-isolation-and-native-boundary.md),
  and [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md)
- [Research 179](./179-kimi-code-0-38-0-identity.md),
  [Research 210](./210-kimi-code-headless-reasoning-effort-evidence.md),
  [Research 211](./211-kimi-code-0-38-0-headless-v2-identity.md),
  [Research 270](./270-kimi-code-0-39-1-identity.md),
  [Research 282](./282-kimi-code-local-server-0-41-0-identity.md), and
  [Research 308](./308-all-route-version-currentness-checkpoint.md)
