# 270 Kimi Code 0.39.1 Identity

Status: promoted
Owner: Tom
Date: 2026-09-01
Card: g05 batch 041

## Question

Is official `@moonshot-ai/kimi-code` `0.39.1` a compatible extension of the
`kimi-code.executable` installed-harness claims, a private milestone, or a
stop — independently on the ACP axis, the headless v1 axis, and the headless
v2 axis?

Two further questions were forced by the evidence and are answered here:

- At which exact published point does the default `kimi -p` engine change?
  The recorded answer was `0.38.0`. It is `0.33.0`, and the production
  headless claim has been wrong for `0.33.0..=0.37.2` since g04.064.
- Does the `0.39.0` ACP terminal-runner change alter process authority under
  the capabilities Swallowtail actually advertises? It does, and nothing in
  the adapter or runtime contains it.

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Kimi Code installed harness (`kimi-code.acp`; `kimi-code.headless`) | installed `0.34.0` | ACP exact `0.28.1` plus `0.29.0..=0.38.0`; headless `0.29.0..=0.37.2` v1 plus exact `0.38.0` v2 | operator-named family; Research 269 selected it alone; official npm and GitHub stable is `0.39.1` |

`kimi-code.local-server` shares the npm package and remains a separate
currentness family. Do not flatten onto it. Python `kimi-cli` and Kimi
Platform Chat stay separate axes. Gemini stays deferred. The g05.009
provider-operation observation gate and card 034 stay untouched.

## Method

Re-probed npm `@moonshot-ai/kimi-code` and the GitHub release stream on
2026-09-01. Downloaded the `0.38.0`, `0.39.0`, and `0.39.1` npm tarballs and
the `0.38.0`, `0.39.0`, `0.39.1`, and `0.34.0` GitHub platform archives to
disposable scratch space, verified each against the registry integrity fields
and the release `.sha256` sidecars and `manifest.json`, and extracted them
without executing anything.

Compared the executing surfaces three ways: git blobs at the annotated tags,
the packed `dist/main.mjs` bundle, and both extracted single-executable
archives.

After the first pass, the engine-routing premise inherited from Research 179
and 211 was re-tested rather than assumed. Every published point from `0.28.1`
to `0.39.1` was walked for `experimental-v2.ts`, `run-prompt.ts`,
`prompt-render.ts`, `v2/run-v2-print.ts`, `sub/acp.ts`, `sub/acp-native.ts`,
and `acp-server/src/server.ts`, and the `0.32.0`, `0.33.0`, `0.34.0`, and
`0.37.2` npm tarballs plus the `0.32.0` and `0.33.0` darwin-arm64 archives were
downloaded and verified so the routing boundary could be proved from shipped
artifacts and not only from source.

Read host `kimi --version` and digested the host binary. Did not install,
update, replace, or run the host, run a live probe, send a provider prompt,
authenticate, start a local server, or execute any downloaded binary.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | `0.34.0` | SHA-256 `9f4337e10da47843f6b550474012a53ba8b30dd665f83b176a5cd479c5f7e859`, size 176894272; byte-identical to the official `0.34.0` darwin-arm64 extracted artifact, whose archive `5b89d2298f05bbe100ae8b14dfca1df56bedc1ed83ff2a76f8efb94802f1692d` matches the official manifest |
| Official npm latest | `0.39.1` | published 2026-08-28T10:01:03.520Z; integrity `sha512-prxUZEhr4hFTnPtm3JLEpE3+1jFH9HvCpkAGFy4WErFHX0Ax7+7KdEgplOC+lm7IGv1cZLU6HdqT6con1IFbdQ==`; shasum `6ee2d2ce457b8fd4bdd110a87f013268979dce49`; tarball SHA-256 `22594a76d0aec0cdabd41050fdd354381c106c48a2f8f5edf98394b4b5e987f7`; 545 files; `bin.kimi` = `dist/main.mjs` |
| GitHub tag | `@moonshot-ai/kimi-code@0.39.1` | annotated tag `1c142e2b20378bfdc92629abfcc68499946bf96f`; commit `5efca0c3116743855c28426000073bfe34a4862f`; release published 2026-08-28T10:01:05Z |
| `0.39.1` artifacts | — | darwin-arm64 ZIP `d3a9cc0272caa68e89e747e68e1730ab86b29cdeee8d05a976f207d19020449a`, extracted `762ee3be8b67796657409b8d5074ab0beed6f42162035bd4a274055ef0c44cdd`; linux-x64 ZIP `9c301ac70fa5d1f7c73a3138bae1b5664ccc05159b10c93e5eb87b3beea04c21`, extracted `585547e082f2f3a32dd80825626a1c8dd4e82f55b4d6a8aa14e6397c00758eca`; npm `dist/main.mjs` `ed24f532d07e5d00777ae20b42ed18437e116b050bbcfc6ae0f3bc90affca337` |
| Adjacent stable | `0.39.0` | published 2026-08-27T11:36:25.525Z; integrity `sha512-T+8IwTc3etpNQhJXbB6wNppLvC4URWBR0SycDM2mb2ObXmqwn8xbOKzaItQQ63ZxjyV6zSHQDaogw0BjlHmtvA==`; tarball SHA-256 `b42ab69386d260c40f1397a6b319d05331554711815934054af815f04ca7ff48`; annotated tag `9076cf66a7ae1e0b2e28c9967099c4a6d6cfdb74`; commit `52e8d19dbd17efebc2e73f8e1a879bef7f23c2b1` |

Published stables after the previous ceiling `0.38.0` are exactly `0.39.0`
and `0.39.1`. npm has no `0.38.1`, no `0.39.2`, and no `0.40.0`; GitHub has no
matching tags. Not a major-line reset.

The `0.38.0` corpus was recomputed rather than trusted. Its tarball
`d5c047db…`, annotated tag `488fe6bb…`, commit `0999454b…`, darwin-arm64
ZIP `48f534fc…`, extracted `92bf3b4b…`, linux-x64 ZIP `2278e0c9…`, and
extracted `7f18b701…` all match the frozen `kimi-code-0.38.0` corpus.

## Engine routing boundary

Swallowtail launches `kimi acp` and
`kimi --model <model> --prompt <content> --output-format stream-json`, and
never sets `KIMI_CODE_LEGACY_FLAG`.

`apps/kimi-code/src/cli/experimental-v2.ts` defines the engine gate. It has
two shapes across this family:

| Span | `isKimiV2Enabled()` | Default `-p` engine | Default `acp` |
| --- | --- | --- | --- |
| `0.28.1..=0.32.0` | `KIMI_CODE_EXPERIMENTAL_FLAG` truthy | agent-core v1 print | `acp-adapter` |
| `0.33.0..=0.39.1` | `!isLegacyEnabled()`, keyed on `KIMI_CODE_LEGACY_FLAG` | agent-core-v2 `runV2Print` | `acp-server` |

The blob is `de40d76c` at `0.32.0` and `09deacc9` from `0.33.0` through
`0.39.1`. At `0.32.0` the string `KIMI_CODE_LEGACY_FLAG` does not appear in
the shipped bundle at all. The bundled `isKimiV2Enabled` digest is
`7230b195…` through `0.32.0` and `944f4317…` from `0.33.0` onward, identical
in the npm bundle and the extracted darwin-arm64 archive at every point
checked. `run-prompt.ts` delegates to `runV2Print` under that gate and is
unchanged in substance across the flip; only the gate's meaning moved.

`sub/acp-native.ts` first ships at `0.33.0`, and `sub/acp.ts` gains the
`if (!isLegacyEnabled()) registerNativeAcpCommand(...)` branch in the same
release.

So the default flip — for both the headless and the ACP route — is `0.33.0`.

### What this corrects

Research 179 recorded the ACP evidence surface as `packages/acp-adapter`.
Research 179's errata and Research 211 recorded the headless v1→v2 default
flip as happening at `0.38.0`. Both were drawn from a single sampled point.
The flip is `0.33.0` for both routes.

The ACP correction moves no qualified point: the mapped `acp-server` blobs are
byte-identical from `0.33.0` through `0.39.1` except `server.ts`, which
changes once at `0.37.0` and holds through `0.39.1`, and Research 179's
prompt-free initialize observation was already taken on the native path.

The headless correction does move production. Since g04.064, the claim has
carried `0.29.0..=0.37.2` as qualified `kimi.headless.stream-json.v1`. From
`0.33.0` the default `-p` path emits a
`{"role":"meta","type":"system.version",…}` preamble, and the adapter's v1
decoder answers `system.version` with `Err(malformed_stream())`. Every point
in `0.33.0..=0.37.2` was therefore claimed as qualified while being unable to
complete a run. Recorded host `0.34.0` sits inside that span. This is a
pre-existing defect surfaced by this run, not a `0.39.x` regression.

## Selected protocol

Mapped ACP source is byte-identical from `0.37.2` through `0.39.1` in both
copies: `acp-server` `server.ts` `6e4ee878…`, `events-map.ts` `cb549b73…`,
`auth-methods.ts` `7b537995…`, `modes.ts` `a0ecdb7c…`, `config-options.ts`
`beeb4300…`, `approval.ts` `2c14a18e…`, `model-catalog.ts` `3fad223b…`;
and `acp-adapter` `events-map.ts` `0448f2eb…`, `server.ts` `6707fd4c…`.
Both ACP dispatch files are unchanged from `0.38.0`.

Mapped headless source is byte-identical from `0.37.2` through `0.39.1`:
`prompt-render.ts` `0e2f3523…`, `options.ts` `004fd7ca…`, `run-prompt.ts`
`cd519b22…`, `experimental-v2.ts` `09deacc9…`.

Five selected-adjacent files changed at `0.39.0` and held at `0.39.1`:

- `acp-server/src/session.ts` — doc-comment wording only.
- `acp-server/src/start.ts` — agent-core-v2 DI plumbing; `ensureMainAgent`
  now returns an agent context resolved through
  `IAgentLifecycleService.handleOf`, and `IHostProcessService` is threaded
  into the ACP runtime provider. No mapped method changes.
- `cli/v2/run-v2-print.ts` — the same DI refactor for `AgentGoal` and
  `AgentCron` resolution and for `IAgentLifecycleService.list()`. Every
  writer call site is unchanged.
- `acp-server/src/convert.ts` — stdio MCP servers without a declared runtime
  identity now default to `transport: stdio`, `runtime_id: local` instead of
  throwing. Inert for Swallowtail, which sends `mcpServers: []` on both
  `session/new` and `session/load`.
- `acp-server/src/acp-terminal/acpTerminalRunner.ts` — process spawn falls
  back to a local host process when the client advertises no terminal
  capability or the invocation is not the interactive Bash tool, replacing
  the `ACP terminal capability is unavailable` and
  `ACP runtime only supports interactive Bash tool processes` errors.

Only the last is both material and reachable under Swallowtail, and it is not
a compatible extension. It is an authority change, treated below.

## ACP process authority at 0.39.0

`acpTerminalRunner.ts` previously failed closed twice — once when the client
advertised no terminal capability, once when the invocation was not the
interactive Bash tool. At `0.39.0` both errors become
`this.local.spawn(command, args, { ...options, cwd: options?.cwd ?? this.cwd })`
— a direct host process spawn.

Swallowtail's ACP initialize always sends `clientCapabilities.terminal: false`
and `auth.terminal: false`, so `connection.terminalEnabled` is always false
and the new local-spawn branch is always the one taken. Under a `Read`
resource lease the adapter passes `resource_io: None` and refuses
`fs/write_text_file`.

The observable delta is therefore: at `0.38.0` an ACP session under
Swallowtail's advertised capabilities could not execute a host process at all;
at `0.39.0` the agent's Bash, Grep, and Glob tools execute host processes in
the leased cwd with no ACP terminal negotiation and outside the filesystem
callback that the read-only lease governs.

### Containment trace

The question was taken as an authority-invariant question first, not as a
revision-label question. The trace looked for any adapter or runtime control
that constrains that spawn:

- the route declares `HarnessIsolation::AmbientHost`, defined in core as
  "harness inherits the ambient host environment without an isolation claim"
- Contract 015: "Process ownership implies neither callback authority nor
  filesystem containment."
- Contract 015 also holds that the client advertising filesystem write false
  and omitting terminal capability makes "any write or terminal request
  unsupported", stopping the scope
- Contract 023 keeps `AmbientHost` a valid explicit posture with no container,
  VM, App Sandbox, or Landlock prerequisite
- no adapter-side or runtime-side control mediates a process the harness
  spawns for itself

Containment is absent. The `Read` lease governs the ACP callback channel, not
the harness process.

### Disposition

Contract 029 lists capability and failure behavior as milestone triggers, but a
new behavior revision would assert that the changed behavior is qualified.
Wire-shape stability across `0.38.0`→`0.39.1` is real and recorded below, and
it is not sufficient to qualify an authority change that Contract 015 treats as
scope-stopping.

`kimi-code.acp` therefore stops at `0.38.0`. Exact `0.39.0` and `0.39.1` join
the ACP claim's exclusion set.
`InterfaceCompatibilityClaim::assess` tests exclusions before the
`AllowUnverified` newer path, so both classify `Incompatible` instead of being
silently admitted. No new public type was required and none was invented.

The headless route is unaffected: `kimi -p` bootstraps its own scope and never
constructs `AcpRuntimeProviderFactory` or `AcpProcessService`, so its agent's
process authority is unchanged across `0.38.0`→`0.39.1`.

## Cross-corpus oracle

Each mapped surface was extracted by brace matching from independent corpora —
the npm `dist/main.mjs` bundle and the extracted single-executable archives —
with bundler alias suffixes normalized. The span checked is `0.32.0`,
`0.33.0`, `0.34.0`, `0.37.2`, `0.38.0`, `0.39.0`, `0.39.1`, which brackets the
routing boundary on both sides.

Six headless surfaces produce one digest each across every corpus-version
pair: `PromptJsonWriter` `aca3e562…`, `PromptTranscriptWriter` `7e7f5ef7…`,
`writeExperimentalVersion` `0c6ba818…`, `writeResumeHint` `f4e14f24…`,
`stringifyToolOutput` `8aacfc7c…`, and the `runPrompt` v1/v2 dispatch
`d6d2b7ce…`.

Two surfaces move, and both are recorded with their two digests rather than
smoothed over:

- `isKimiV2Enabled` — `7230b195…` through `0.32.0`, `944f4317…` from
  `0.33.0`. This is the routing boundary itself.
- `dispatchNativeEvent` — `216fdc31…` for `0.32.0..=0.36.1` and
  `4e0102b3…` for `0.37.0..=0.39.1`. The `0.37.0` change is a TypeScript
  retype (`DomainEvent` → `Event2<any>` plus casts). All ten case labels,
  every writer call, every argument, and the stderr branch are identical, so
  the
  emitted JSONL does not change.

`AcpProcessService` is the single differing selected ACP surface,
`8f8a2726…` at `0.38.0` and `7c58e045…` from `0.39.0`, so the corpus cannot
be read as claiming a clean ACP no-op. Both bundled `AcpServer` copies and
both bundled `assistantDeltaToSessionUpdate` copies are stable.

The `0.34.0` v2 runner also swaps `setTimeout` for `setClampedTimeout` in the
print background-policy wait loop. The adapter selects no goal, no session
resume, and no print background configuration, so that path is unreachable.

## Unmapped extras

Remote Control behind `KIMI_CODE_EXPERIMENTAL_REMOTE_CONTROL`, subagent and
swarm `fork` behind `KIMI_CODE_EXPERIMENTAL_SUBAGENT_FORK`, tower mode behind
`KIMI_CODE_EXPERIMENTAL_TOWER`, `[swarm] timeout_ms` and
`KIMI_CODE_SWARM_TIMEOUT_MS`, the `kimi update` request timeout, sign-in
presentation fixes, TUI and web client changes, the MCP `structuredContent`
forwarding rule, the skill-instruction transcript rebuild fix, and the
resume-time background-task warning all stay unmapped.

## Separate family observations

Recorded for the `kimi-code.local-server` family only; this record edits none
of its claim, fixtures, route, guide, matrix cell, or conclusions. `kimi web`
lost `--allow-remote-terminals` at `0.39.0` and PTY terminal routes now stay
on loopback binds only. `web/index.ts`, `web/run.ts`, and `web/shared.ts`
changed; `web/remote-control.ts` and `web/remote-control-lock.ts` are new.
`kap-server` `middleware/auth.ts`, `protocol/rest-modelCatalog.ts`, and
`protocol/ws-control.ts` changed. That family needs its own identity run
before any of it becomes a claim.

## Decision

A split outcome. The headless axes are corrected and extended; the ACP axis
stops.

Keep claim ids `kimi.acp.executable-window-2` and
`kimi.headless.executable-window-2`, both baselines, and `AllowUnverified`.

**ACP — stop.**

- Keep exact `0.28.1` Deprecated on `kimi.acp.reasoning.legacy-select-v1`.
- Keep Maintained `0.29.0..=0.38.0` on
  `kimi.acp.reasoning.declared-effort-v2`. `KIMI_CODE_LATEST_QUALIFIED_VERSION`
  stays `0.38.0`.
- Add exact `0.39.0` and `0.39.1` to the claim's exclusions so
  `AllowUnverified` cannot admit them. Both classify `Incompatible`.
- No new ACP behavior revision. Requalification needs either provider-side
  containment or a Swallowtail-side control that does not exist today.

**Headless v1 — correct down.**

- `0.29.0..=0.37.2` becomes `0.29.0..=0.32.0`, still Deprecated on
  `kimi.headless.stream-json.v1`. `0.33.0..=0.37.2` was never a working v1
  span.

**Headless v2 — correct down and extend.**

- Exact `0.38.0` becomes Maintained `0.33.0..=0.39.1` on
  `kimi.headless.stream-json.v2`.
- `KIMI_HEADLESS_LATEST_QUALIFIED_VERSION` becomes `0.39.1`.
- Host `0.34.0` reclassifies from broken-v1 to qualified Maintained v2.
- The two segments remain adjacent with no gap: `0.32.0` and `0.33.0` are
  consecutive published points.

Shared:

- Preserve every existing negative point: `0.28.0` and `0.28.2` stay outside
  the ACP claim, and `0.28.1` stays outside the headless claim.
- Synthetic later-stable `UnverifiedNewer` is unpublished `0.39.2` on the
  headless axis, whose ceiling is `0.39.1`.
- On the ACP axis the ceiling stays `0.38.0`, so the first admissible
  unverified-newer point is unpublished `0.38.1`. Exact published `0.39.0` and
  `0.39.1` are excluded and classify `Incompatible`. Unpublished `0.39.2` is a
  later admissible unverified-newer point, not the first one.
- `0.38.1` is a line-terminal absence inside the extended headless window, the
  same disposition the family already gives unpublished `0.37.3`; it is not a
  skipped interior hole and is not added as an exclusion.
- No new behavior revision, public operation, shared type, or public API
  change. Decoder specimens stay on the existing corpora.
- Do not widen or edit `kimi-code.local-server`. Do not touch g05.009 or
  card 034.

Card 042 owns the claim change. This record edits no production claim.

## Sources

- npm `@moonshot-ai/kimi-code` `0.38.0`, `0.39.0`, `0.39.1`
- [GitHub `0.39.1`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.39.1)
- [GitHub `0.39.0`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.39.0)
- `apps/kimi-code/CHANGELOG.md` at `0.39.0` and `0.39.1`
- git blobs at commits `0999454b`, `52e8d19d`, and `5efca0c3`, plus `0.37.2`
  commit `c41fadf0`
- frozen `crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-0.38.0/`,
  `kimi-code-0.38.0-headless-v2/`, `kimi-code-0.33.0-headless-routing/`, and
  `kimi-code-0.39.0-acp-authority/`
- [Contract 015](../contracts/015-acp-v1-negotiation-and-client-callbacks.md)
  and [Contract 023](../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Research 179](./179-kimi-code-0-38-0-identity.md),
  [Research 210](./210-kimi-code-headless-reasoning-effort-evidence.md),
  [Research 211](./211-kimi-code-0-38-0-headless-v2-identity.md),
  [Research 269](./269-all-route-version-currentness-checkpoint.md)
