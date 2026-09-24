# Changelog

All notable Swallowtail changes are recorded here. Releases are source-only
annotated Git tags from the canonical repository.

## [Unreleased]

### Notes
- raise the Ollama attached-runtime qualified ceiling through official
  `0.34.4`: Research 350 freezes both published hops after `0.34.2` with
  tag/commit/tree/tarball and selected-file hashes, and reproduces Research
  342's `0.34.2` hashes. `ShowResponse.thinking` is additive and ignored by
  the catalog decoder; remaining ChatHandler and leftover routes.go deltas
  stay bounded unmapped. Qualify `0.34.3` and `0.34.4` as a compatible
  extension of `ollama.native-text-v1`. Keep the baseline, claim identity,
  behavior revision, historical exclusions, decoder specimen, and
  `AllowUnverified`. Synthetic later-stable `0.34.5` stays visibly
  unverified. No provider operation or host mutation occurred.
- raise Claude Code headless through official `2.1.281` as a compatible
  extension of `claude-code.headless.stream-json.v1`, and split response-only
  into v1 `2.1.227..=2.1.278` plus v2 `2.1.280..=2.1.281` under the Contract
  039 narrowed built-in-hook guarantee. Unpublished `2.1.279` is an explicit
  exclusion. v2 without a project location launches in an adapter-owned empty
  directory. The nine `@builtin` plugins that `--safe-mode` keeps from
  `2.1.280` are disclosed in the Claude prepared guide. Watcher stays exact
  `2.1.251`. Research 348.
- raise the Grok Build ACP executable window through the current official
  npm `@xai-official/grok` `1.0.41`: Research 340 freezes all eleven
  published stables `1.0.31..=1.0.41` after the `1.0.30` ceiling with
  verified linux-x64 identity, a darwin-arm64 cross-check that reproduces
  Research 314, a byte-identical selected-literal presence map, an
  unchanged `grok-4.6` model document, the 62 mapped-core ACP modules, and
  a complete shipped-file inventory. The `1.0.41` hop adds one unmapped
  `subagent_handoff` module and moves no mapped surface. The claim keeps
  baseline `0.2.114`, claim id `grok-build.acp.executable-window-2`,
  behavior `grok-build.acp-v1.cached-token-model-4-6-v3`, and
  `AllowUnverified`, and extends the maintained window `1.0.4..=1.0.41`.
  The exact `1.0.30` catalogue claim and the `1.0.4`/`1.0.5`
  registered-tool courier stay independently bounded.
- freeze Claude Code `2.1.280` and `2.1.281` official artifacts and built-in
  hooks in Research 341. The `2.1.281` default-enabled `agents-md` hook can
  add project instructions despite `--safe-mode`; no selected writable
  setting disables all built-ins. g06.018 takes the narrowed-claim escalation
  branch, leaving both qualified ceilings at `2.1.278` pending a ruling.
- rebind the Goose `goose.acp` claim to exact `1.50.1` under the new
  `goose.acp.stdio-v2.auth-required` behavior revision. Research 328
  re-probed official GitHub `v1.50.1`, classified the `1.50.0..1.50.1` MCP
  protocol-version default change as unmapped on the selected route, and
  carried the typed provider-authentication binding on both `session/new` and
  `session/prompt` into the adapter diagnostic
  `swallowtail.goose.acp.auth_required`. Research 319's historical stop and
  the independently gated builtin, mode, lifecycle, and effort surfaces stand.
- advance the Qoder `qoder.headless` claim to one exact `1.1.54`
  `QualifiedOnly` point on the new adapter-private
  `qoder.headless.stdio-stream-json-v2` behavior revision. Research 328 froze
  official npm `1.1.54` after the `1.1.52` stop and recorded the deliberate
  adapter-owned `--max-turns 8` AgentLoop ceiling plus its
  `error_max_turns` provider-failure shape. The historical `1.1.25` decoder
  specimens and Research 256's independent empty skill-visibility disposition
  remain unchanged.

### Added
- prove `opencode.acp` honours a consumer-supplied streamable-HTTP MCP entry
  on installed exact `1.18.18`. One live attempt accepted: connect,
  `tools/list`, one `tools/call`, completed turn, clean cleanup.
  `client_mcp_servers` is Yes on that exact point only; later window points
  stay unqualified for honouring. The host default agent model is unverified;
  the claim does not depend on it. The live gate records the selected default
  agent's model override, else the root `model`, only after that definition
  and existing host auth are present, and stops with `no_usable_model` when
  those are missing. Harness proof is a default-feature test; the live binary
  stays behind `live-probes` and an env gate. The disposable loopback MCP
  server is test-only. Research 349, g06.028.
- add the OpenCode ACP production route `opencode.acp` in
  `swallowtail-adapter-opencode`, unflattened from `opencode.http`. Axis
  `opencode.executable`, claim `opencode.acp.executable-window-1`, compiled
  behavior `opencode.acp-v1.client-mcp-servers-v2` for `1.18.31..=1.18.32`,
  with accepted-but-older `opencode.acp-v1.client-mcp-servers-v1` covering
  deprecated `1.18.18..=1.18.30`. The driver pins `opencode acp --pure`,
  records provider-fixed `protocolVersion` `1` without inferring acceptance,
  admits one stdio MCP entry named `swallowtail-opencode-acp`, carries that
  declaration onto working-state restoration, and models the URL-plus-header
  `http`/`sse` placement behind the contract gate. Feature matrix MCP cells
  stay No: Research 337 proved representability, not live honouring. No live
  ACP session, login, install, or host update. Research 337, g06.016.
- wire Contract 063's consumer-supplied streamable-HTTP MCP placement into
  `opencode.acp` production `session/new`. A session declares at most one
  consumer entry, stdio or HTTP, under `swallowtail-opencode-acp`. HTTP
  encoding emits ACP `type: "http"` with the consumer URL and headers
  verbatim; `sse` stays modelled and unemitted. The public encoder returns
  `OpenCodeAcpEncodedMcpServers`, whose `Debug` form redacts URL and header
  values; the wire JSON stays crate-private. URL and header values stay
  out of failures, diagnostics, activity, receipts, `Debug`, and plan
  fingerprints. The Contract 061 placement projection names
  `consumer-supplied-http`. Feature matrix MCP cells stay No: emission is
  proven, live honouring of a remote tool call is not. Research 337, g06.019.

### Changed
- accept the exact `command-code.headless` `1.65.0` live surface on
  `deepseek/deepseek-v4-flash`. Research 347 records one authorized gate:
  structured one-turn completion and the Contract 043 two-turn private
  exact-id continuation both `Completed` with `Clean` cleanup. Research 330
  stays the exact-`1.54.0` paid-model record; Research 116/118 stay bound to
  `1.15.1`. g06.029.
- rebind the exact `command-code.npm` `QualifiedOnly` point from `1.54.0` to
  official npm `command-code` `1.65.0` with claim
  `command-code.headless-window-1` and behavior
  `command-code.agent-event-ndjson-v1` unchanged. Research 339 froze all 19
  published stables after `1.54.0`; selected invocation, AgentEvent, result,
  usage, plan mode, retention, and local lifecycle stay compatible. Research
  330 live acceptance stays bound to exact `1.54.0` and is gated at `1.65.0`;
  Research 116/118 stay bound to `1.15.1`. Unpublished `1.66.0` stays
  incompatible. No prompt, login, install, host update, or downloaded-artifact
  execution. g06.021.
- raise the qualified OpenCode HTTP ceiling from `1.18.30` to official npm
  `opencode-ai` `1.18.31` on the `opencode.server` axis. Compatible extension
  of `opencode.http-sse.surface-19`: selected HTTP/SSE route files and OpenAPI
  stay byte-identical, and the published GitHub tags diverge, so the hop is
  proven from complete source-tree inventories rather than a commit range.
  Unmapped remote-config auth defect-to-400 mapping, unmapped OpenCode ACP
  restore/config-option work, the unmapped TUI exit-status change, and
  unmapped GitHub Copilot summarized-thinking request shaping are the only
  shipped server source changes; provider SDK bumps stay provider-facing.
  Baseline `1.14.48`, claim id, historical gaps, and `AllowUnverified` stay;
  unpublished `1.18.32` remains visible `UnverifiedNewer`. Research 332, g06.010.
- raise both qualified Claude Code ceilings from `2.1.270` to official npm
  `@anthropic-ai/claude-code` `2.1.278`: headless `2.1.220..=2.1.278` and
  response-only `2.1.227..=2.1.278`. Compatible extension of
  `claude-code.headless.stream-json.v1` and
  `claude-code.response-only.stream-json.v1`. All eight published hops after
  the ceiling were retrieved and classified on both platform builds; every
  selected flag keeps an identical constructor-site fingerprint intersection,
  the format, effort, and permission enumerations are unchanged, and the
  required `init` keys plus `stream_event`, `hook_started`, `result`, and
  `thinking_tokens` presence hold. Wrapper files except `package.json` and
  `sdk-tools.d.ts` are byte-identical across all nine compared versions, and
  every `sdk-tools.d.ts` delta is SDK tool declaration content the routes do
  not consume. Host `claude` was not installed. Unpublished `2.1.244`,
  `2.1.249`, `2.1.253` through `2.1.256`, `2.1.262`, and `2.1.264` stay
  incompatible; unpublished `2.1.279` remains visible `UnverifiedNewer`.
  Baselines, claim ids, `AllowUnverified`, watcher exact `2.1.251`, and every
  feature-specific exact-version set stay. Research 331, g06.009.
- raise the shared `codex.cli` ceiling for Codex exec and app-server from
  qualified `0.154.0` through official npm `@openai/codex` `0.155.1`.
  Research 333 freezes both published stables `0.155.0` then `0.155.1` as a
  compatible extension of `codex.exec.jsonl-v1`,
  `codex.app-server.v2.workspace-roots`, and
  `codex.app-server.lifecycle.v1.strict-descendant-hard-delete`. Keep
  baseline `0.80.0`, both claim ids, and `AllowUnverified`. Pin newly
  interior unpublished `0.154.1` incompatible alongside `0.149.2`,
  `0.150.2`, `0.151.1`, and `0.152.2`. After qualification, unpublished
  `0.155.2` is the first visible `UnverifiedNewer` point. Feature-specific
  exact pins stay on the `0.147.0..=0.149.1` probed points. Voice,
  code-mode-host, attachments, managed-daemon, and TUI stay unmapped.
  Research 333.
- raise the Qwen Code headless qualified ceiling through official npm
  `@qwen-code/qwen-code` `0.24.2`: Research 334 freezes every published hop
  after `0.23.3` (`0.23.4`, `0.24.0`, `0.24.1`, `0.24.2`) and classifies the
  selected stream, controls, help, session, and Plan surfaces as a compatible
  extension. Extend the maintained `0.22.0..=0.24.2` segment with unpublished
  interior `0.22.4` and `0.23.5` kept incompatible, keep the exact
  Plan set bounded at `0.22.3`, keep exact `0.21.15` reasoning and budgets,
  preserve historical gaps and `AllowUnverified`, and leave board/sandbox,
  goal workflow, Web Shell, daemon, ACP, Model Studio, and other adjacent
  deltas unmapped. No provider operation or host mutation occurred.
- raise the Claude Agent ACP qualified ceiling from `0.76.0` to official npm
  `@agentclientprotocol/claude-agent-acp` `0.79.0`. Compatible extension of
  `claude-agent.acp.initialize-meta-extensions-v7`: selected mapped ACP routes
  stay, the ACP SDK pin stays `1.4.0`, and `dist/index.js`, `dist/settings.js`,
  `dist/utils.js`, and `dist/lib.js` are byte-identical across every hop, so
  mode ids/categories, `plan`/`acceptEdits`, permission option kinds, and the
  effort config id stay. `0.77.0` removes the unmapped `agent` config option
  and changes the multi-select Other description; `0.78.0` adds
  capability-gated `compaction_update` that Swallowtail does not advertise and
  changes the single-select Other description; `0.79.0` bumps the unmapped
  Agent SDK pin to `0.3.274` and keeps command text in shell permission
  titles. Already-mapped form elicitation accepts the two new Other
  description strings. Unpublished `0.58.0`, `0.73.1`, `0.74.1`, `0.75.2`,
  `0.76.1`, `0.77.1`, and `0.78.1` stay incompatible; synthetic unpublished
  `0.80.0` remains visible `UnverifiedNewer`. Claude Code stream-JSON and the
  Claude Agent SDK sidecar stay separate families. Research 335.
- extend the qualified Oh My Pi RPC `18.x` package segment from
  `18.0.0..=18.1.22` to `18.0.0..=18.2.7` on the unchanged
  `oh-my-pi.rpc-v2-v18.0.0` behavior revision. Official npm and GitHub
  latest is `18.2.7`. Research 345 froze the previous ceiling `18.1.22`
  and every published hop `18.2.0` through `18.2.7`: selected JSONL wire
  files `rpc-frame.ts`, `rpc-input.ts`, `rpc-messages.ts`, and
  `host-uris.ts` are byte-identical; remaining mapped hops are unmapped
  skill, login-secret, backpressure-spool, LSP/DAP framing, import-path,
  or append-only argv changes. Claim id `oh-my-pi.rpc.package-window-2`,
  retained deprecated `17.2.9..=17.4.2`, exclusions `18.0.2`/`18.1.7`,
  `AllowUnverified`, and the `oh-my-pi-rpc-17.2.9` decoder corpus stay.
  Unpublished `18.2.8` stays permitted `UnverifiedNewer`. `pi.package`
  stays a separate axis. Not a major-line reset, new public operation,
  or new driver/facade.
- raise the Cursor Agent catalogue, ACP, and headless qualified ceiling
  through official `2026.09.18-9a7762b`: Research 343 freezes both
  published hops after `2026.09.10-fd3934a` with a complete tree inventory
  and classifies the selected CLI definitions, ACP initialize subset, and
  stream-json event keys as unchanged on Swallowtail's selected client.
  Add exact milestones `2026.09.15-d2fe57e` and `2026.09.18-9a7762b` with
  no inferred gap, keep the three distinct route claims, historical
  milestones, exact feature-specific sets, and `AllowUnverified`, and leave
  negotiated `sessionCapabilities.subagents`, worker/persist flags,
  SEA/native packaging, ACP load/replay, and continuation recovery
  unmapped or blocked. Older published `2026.08.25-3e8eec8` and
  `2026.09.08-6caf4ff` stay independently unqualified gaps. No provider
  operation, downloaded-artifact execution, or host mutation occurred.
- raise the Pi RPC qualified ceiling from `0.85.1` to official npm
  `@earendil-works/pi-coding-agent` `0.86.1`. Compatible extension of
  `pi.rpc.strict-lf-v0.84.0-message-update-delta`: selected mapped commands
  and argv stay; published `0.86.0` is also qualified. `0.86.0`
  `{ source: "rpc" }` on already-mapped `steer` / `follow_up` covers
  unmapped extension input handlers and is a no-op under selected
  `--no-extensions`. `0.86.1` `META_API_KEY` help, Meta Muse, compile
  cache, cache warming, `/bug`, Radius, compaction budgets, custom-provider
  `TranscriptContext`, and `user_bash` fail-closed stay unmapped.
  Unpublished `0.83.1`, `0.84.5`, and `0.85.2` stay incompatible;
  unpublished `0.86.2` remains visible `UnverifiedNewer`. `pi.sdk-sidecar`
  stays exact `0.84.2`. Research 344.
- extend the Antigravity catalogue claim from maintained `1.1.9..=1.2.2`
  to maintained `1.1.9..=1.2.7` on `antigravity-cli.release` with the
  unchanged `antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1` behavior
  revision, baseline `1.1.9`, and `AllowUnverified` posture. Research 346
  froze official GitHub `1.2.3` through `1.2.7` with both platform
  digests; every public hop changes only `CHANGELOG.md`, and the release
  notes name no selected-path change to `agy models`. Headless stays
  maintained `1.1.9..=1.1.17` at the Research 283 `1.1.22` provider-managed
  retry stop; `1.2.6` and `1.2.7` deepen that gap, so `1.1.18..=1.2.7`
  stay unqualified. Unpublished `1.2.8` remains visible
  `UnverifiedNewer`. Gemini CLI and `antigravity-acp` were not touched.
  Research 346.
- raise the Ollama attached-runtime qualified ceiling through official
  `0.34.2`: Research 342 freezes all four published hops after `0.33.2`
  with tag/commit/tree/tarball and selected-file hashes. The native decoder
  now accepts and ignores the additive `0.33.3` `prompt_eval_cached_count`
  metrics key named by Research 313, then qualifies `0.33.3`, `0.34.0`,
  `0.34.1`, and `0.34.2` as a compatible extension of
  `ollama.native-text-v1`. Keep the baseline, claim identity, behavior
  revision, historical exclusions, decoder specimen, and `AllowUnverified`.
  Synthetic later-stable `0.34.3` stays visibly unverified. No provider
  operation or host mutation occurred.
- extend the qualified Oh My Pi RPC package window from
  `17.2.9..=17.4.0` to two segments on `oh-my-pi.package`: retained
  `17.2.9..=17.4.2` on the unchanged `oh-my-pi.rpc-v2-v17.2.9` behavior
  revision, and a distinct adapter-private `18.0.0..=18.1.22` segment on the
  new `oh-my-pi.rpc-v2-v18.0.0` revision. Contract 029 gives a claim its own
  revision, so the claim id moves from `oh-my-pi.rpc.package-window-1` to
  `oh-my-pi.rpc.package-window-2`; because the `18.x` segment carries the
  newest behavior revision, the retained `17.x` segment reports `Deprecated`
  while staying executable. Research 327 froze all 36 published npm stables
  from `17.4.0` through official `18.1.22` with reproduced registry
  integrity, shasum, tarball, `dist/cli.js`, shipped file-count, and GitHub
  tag-commit identity plus one deterministic per-hop shipped-tree inventory
  before any claim changed: `rpc-messages.ts`, `rpc-input.ts`, `host-uris.ts`,
  and `message-framing.ts` are byte-identical across all 36 versions,
  `rpc-types.ts` from `17.4.2`, and `docs/rpc.md` is blob `310b4470` from
  `v17.4.2` through `v18.1.22`. The later `17.x` hops are additive or
  advisory (an unmapped background-command callback, an accurate
  `agentInvoked` field, optional select `optionDetails`), and every mapped
  `18.x` hop is byte-identical, unmapped, additive, client-library, or a
  resolved session-scoped persistence change with the adapter's asserted
  `get_state` values unchanged. `18.0.2` and `18.1.7` are explicit
  exclusions; the npm-unpublished GitHub tags `17.4.3` and `17.4.4` stay
  incompatible between the segments; `18.1.23` stays permitted
  `UnverifiedNewer` on the `18.x` revision. Baseline `17.2.9`,
  `AllowUnverified`, the frozen `oh-my-pi-rpc-17.2.9` decoder corpus, and
  every historical specimen stay, `pi.package` is untouched, and no new
  public operation, shared type, or shared behavior revision is required.
  Research 327, g05.079.
- extend the Kimi Code local-server ceiling from maintained
  `0.35.0..=0.38.0` to maintained `0.35.0..=0.39.1` with the
  `kimi.local-server.rest-ws-v2-heartbeat-ping` behavior revision and
  baseline `0.28.1`, and change the newer-version posture from
  `AllowUnverified` to `QualifiedOnly` so every point above `0.39.1` fails
  closed. Contract 029 gives the claim its own revision, so the posture
  change revises the claim id from `kimi.local-server.executable-window-2`
  to `kimi.local-server.executable-window-5` (mirroring the ACP A2 revision
  and skipping the frozen `window-3`/`window-4` reservations). Research 326
  froze official npm and GitHub `0.38.0..=0.43.0` and a mutation-sensitive
  thirteen-file
  local-server blob ledger before any claim changed: `0.39.0` and `0.39.1`
  preserve the Bash workspace assertion (`RuntimeWorkspaceView.resolve`
  still maps then calls `assertAllowed`, and `bashTool` still computes
  `effectiveCwd` through `view.resolve`), while `0.40.0` removes it and the
  uncontained pure-mapping `resolve` persists byte-identical through
  `0.43.0` with no restored containment. The two newly classified hops add
  none: the `0.41.0→0.42.0` wire deltas (model-catalog import-source move,
  `watch_fs` removal, delete-response reshape, terminal compat-schema
  removal, Remote Control de-experimentalization) are all unmapped or inert
  on the selected route, and `0.42.0→0.43.0` is one added field in the
  unmapped Remote Control QR output. The published `0.40.0..=0.43.0` gap is
  posture-rejected; no new exclusion is added, and no new behavior revision,
  public operation, or shared type is required. `kimi-code.acp` and
  `kimi-code.headless` are untouched. Research 326, g05.078.
- raise the Kimi Code installed headless ceiling from maintained
  `0.33.0..=0.39.1` to maintained `0.33.0..=0.43.0` on
  `kimi.headless.executable-window-2` with the `kimi.headless.stream-json.v2`
  behavior revision, baseline `0.33.0`, and `AllowUnverified` unchanged.
  Research 325 froze official npm and GitHub `0.39.1`, `0.40.0`, `0.40.1`,
  `0.41.0`, `0.42.0`, and `0.43.0` and one deterministic tagged-source
  inventory per point before any claim changed: the `dispatchNativeEvent`
  switch is byte-identical across all six points, `prompt-render.ts` and
  `options.ts` are byte-identical git blobs, and the JSONL roles, meta types,
  `system.version` preamble, `session.resume_hint`, retry record, tool record,
  terminal shape, retention, cancellation, and cleanup do not move. Only
  internal dependency-injection, telemetry, shutdown-quiescence,
  prompt-submission, and cancellation mechanics change, plus the `0.42.0`
  deletion of the v1 legacy body and the `KIMI_CODE_LEGACY_FLAG` gate. The
  `0.41.0` auto-permission-mode dangerous-command guard drop and the `0.42.0`
  trust-gated MCP stderr warning stay unmapped. `kimi-code.acp` keeps its
  `QualifiedOnly` `0.38.0` cap: Research 325 proved `acpTerminalRunner.ts` and
  the bundled `AcpProcessService` byte-identical at every point through
  `0.43.0`, so the A2 `terminal: false` local host-process spawn is unchanged,
  the exact `0.39.0`/`0.39.1` exclusions do not grow, and
  `0.40.0..=0.43.0` is the posture-rejected published gap. Unpublished
  `0.43.1` stays the visible `UnverifiedNewer` point, and the separate
  `kimi-code.local-server` family did not move. Research 325, g05.077.
- raise the qualified Gemini CLI ACP and headless ceilings from maintained
  `0.51.0..=0.56.0` to maintained `0.51.0..=0.59.0` on the separate
  `gemini-cli.acp-agent` and `gemini-cli.headless-stream-json` axes with the
  `gemini-cli.acp.v0.51.0` and `gemini-cli.headless.stream-json.v1` behavior
  revisions, baseline `0.51.0`, and `AllowUnverified` unchanged. Research 324
  froze official npm and GitHub `0.56.0`, `0.57.0`, `0.58.0`, and `0.59.0`
  and one deterministic tagged-source inventory per point: no changed path
  lies under `packages/cli/src/acp/**`, every selected ACP source is
  byte-identical with the `@agentclientprotocol/sdk@0.16.1` pin, and every
  selected stream-json event, terminal shape, native exit code, option, and
  retention source is unchanged. Only `geminiChat.ts` provider-request retry,
  empty-part, and abort-rollback internals and the `resolveWorktreeBaseSha`
  git helper move. Workspace-trust fail-closing, the policy safety-checker
  declaration, macOS Seatbelt sandboxing, and the MCP OAuth SSRF repair stay
  unmapped; unpublished `0.59.1` remains visible `UnverifiedNewer`. Research
  324, g05.076.
- split the Antigravity claims under official GitHub
  `google-antigravity/antigravity-cli` `1.2.2`: the catalogue claim advances
  from maintained `1.1.9..=1.1.17` to maintained `1.1.9..=1.2.2` on the
  `antigravity-cli.release` axis with the
  `antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1` behavior revision,
  baseline `1.1.9`, and `AllowUnverified` unchanged, because Research 323
  classified every published hop `1.1.27..=1.2.2` from the official release
  notes under Tom's 2026-09-15 authority ruling and found no selected-path
  change to `agy models`; the headless claim keeps maintained
  `1.1.9..=1.1.17` because the Research 283 `1.1.22` provider-managed HTTP
  502 retry stands and `1.1.28` plus `1.2.1` broaden provider-managed retry
  with no published finite bound or disable control, so `1.1.18..=1.2.2`
  stay the named unqualified gap with `1.1.28`'s partial-output
  `--print-timeout` expiry and `1.2.0`'s content-filter stop reason inside
  it. Official identity for both platform assets and the reproduced
  `1.1.26` boundary are frozen in
  `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.2.2/`;
  collected pre-ruling hashes are retained, exhaustive binary scanning
  stopped at the ruling, downloaded binaries were never executed, and no
  provider, prompt, login, credential, installation, or host-mutation
  operation occurred. Research 323, g05.075.
- advance the Deep Agents ACP exact point from `0.1.25` to official npm
  `0.1.30`: Research 322 freezes all five published stable successors and
  the exact-pinned `deepagents` runtime dependency chain `1.12.4..=1.13.4`
  as registry-verified tarballs (both baselines reproduce earlier
  research), keeps the ACP registry's discovery-only `0.1.7` entry and the
  CLI constructor-default `agentInfo.version` `0.0.1` outside the claim,
  and finds the selected no-extra-argv stdio wire byte-stable across the
  whole window: initialize shape, `session/new` cwd authority, the `prompt`
  field, stop reasons, cancel, permission options with the unchanged
  catch-returns-allow fallback, slash-command interception, and joined
  cleanup. The sole CLI body change (`0.1.27..0.1.28`) rewrites
  `ACPFilesystemBackend.read` pagination and is unreachable on the selected
  route because the CLI passes an explicit `FilesystemBackend` config
  backend before any client-capability branch and the selected initialize
  advertises host `fs` false. The selected `deepagents` deltas are additive
  and decoder-invisible: a builtin `delete` tool through unchanged generic
  tool/permission paths with kind `other` (the child's already-unbounded
  local-file authority now also covers recursive deletion, still outside
  the Swallowtail bounded-write disposition), `write_file` `content`
  becoming required, and `read_file` pagination footer text inside opaque
  tool-result content; the subagent fork/isolated machinery stays
  unselected. Keep the `deepagents.acp.package-window-1` claim,
  `deepagents.acp.stdio-v1` behavior revision, empty selected argv, exact
  one-point `QualifiedOnly` posture, and no unverified-newer execution.
  No provider operation, host mutation, or downloaded-artifact execution
  occurred.
- advance the Mistral Vibe headless exact point from `2.24.2` to official
  GitHub/PyPI `2.25.4`: Research 321 freezes all eight published stable
  successors as exact source tarballs and registry-matched sdist/wheel
  pairs (baseline reproducing Research 150), keeps GitHub-only `2.24.4` a
  named PyPI packaging gap, and finds the selected surface byte-stable
  across every hop: `vibe/cli/programmatic.py` streaming print wire,
  `vibe/core/middleware.py` turn-limit middleware, upstream
  `tests/cli/test_programmatic.py`, public-history union and generation
  status, `LIMIT`-only stop reason, `SessionOptions` shape, the builtin
  `plan` profile, headless callback denial, trust and workdir authority,
  missing-API-key failure, and the console script. The selected argv adds
  adapter-private `--legacy-harness`: upstream `2.25.1` resolves the
  session harness from flags, the ambient GrowthBook rollout cache, and
  native-module availability (the internal Unified Harness module is
  bundled in GitHub release zips from `2.25.1` and absent from every PyPI
  distribution), and the flag has first upstream precedence so the legacy
  Python harness the corpus covers runs deterministically on both
  channels. The Unified Harness backend and `--smart-approve` stay
  unmapped, and Research 199's caller-decreasing `--max-turns` `1..=8`
  binding plus Research 252's Plan-only profile boundary carry over
  unchanged. Keep the `mistral-vibe.headless.release-window-1` claim,
  `mistral-vibe.headless.stdio-streaming-v1` behavior revision, exact
  one-point `QualifiedOnly` posture, and no unverified-newer execution. No
  provider operation, host mutation, or downloaded-artifact execution
  occurred.
- advance the Kiro ACP exact point from `2.18.1` to official stable-manifest
  `2.21.4`: Research 320 freezes all eleven published stable successors as
  exact official archives with byte-identical `install.sh`/`README`/`q`/
  `qchat`, per-hop BUILD-INFO and executable digests, read-only-mounted DMG
  identity, and a byte-stable selected ACP surface: initialize capability
  shape, `session/new` cwd authority, the `prompt` field (the official docs
  `content` example stays stale at `2.21.4`), cancel, session-update kinds,
  permission kinds with `allow_always` unselected, stop reasons, and joined
  cleanup. Keep the `kiro.acp.release-window-1` claim,
  `kiro.acp.stdio-v1` behavior revision, exact one-point `QualifiedOnly`
  posture, and no unverified-newer execution. The `_kiro.dev/*` extension
  set (from `2.19.2`) and the advertised `session/load` (from `2.20.2`)
  stay unmapped additions, and the chat-only `--v2` harness flag at
  `2.21.4` never enters the selected argv. Research 251 and 254 keep their
  empty deliver-now sets with their package/source gates now closed by
  per-hop binary evidence. No provider operation, host mutation, or
  downloaded-artifact execution occurred.
- advance the Command Code exact point from `1.15.1` to official npm
  `1.54.0`: Research 317 freezes all 67 published stable successors with a
  complete package-tree inventory, no removals, eight additive bundled
  references, a byte-identical `dist/index.mjs`, and all selected invocation,
  AgentEvent, result, usage, failure, stdin, and local lifecycle inputs
  classified without executing downloaded artifacts. Keep the
  `command-code.headless-window-1` claim, `command-code.agent-event-ndjson-v1`
  behavior revision, exact one-point `QualifiedOnly` posture, private exact
  `--resume` boundary, and no public catalogue/export/Provider API or write
  authority. Research 116 and 118 remain bound to `1.15.1`: authenticated
  completion/tool/usage/credit and two-turn private continuation do not migrate
  and require a separately authorized live `1.54.0` requalification. No
  provider operation, host mutation, or downloaded-artifact execution occurred.
- advance the Grok Build catalogue exact point from `1.0.25` to official and
  installed `1.0.30`: Research 316 reproduces the Research 314 platform
  tarball, brotli, and decompressed executable digests for `1.0.25` and every
  later published stable through `1.0.30` without executing a downloaded
  artifact, and proves the exact root flag `--no-auto-update`, the `models`
  subcommand grammar with no `PROMPT` argument, the authentication preamble,
  the shipped `*`/`-` bullet format literals with byte-identical counts, the
  `xai-grok-pager/src/models.rs` module path, and the embedded
  `default_models.json` document identical at every hop. Re-identify the exact
  one-point claim as `grok-build.catalogue.executable-1-0-30`, keep the
  `QualifiedOnly` posture and the `grok-build.catalogue.models-text-v1`
  behavior revision, and reject every older and newer point. One prompt-free
  authenticated `--no-auto-update models` observation of the installed exact
  `1.0.30` executable returned the ordered `grok-4.6` default then `grok-4.5`
  with zero stderr, no prompt, no model session, no inference, no tool
  dispatch, and joined cleanup; the redacted capsule is frozen. The Grok ACP
  window and the `1.0.4`/`1.0.5` registered-tool courier stay independent.
- rebind the Claude Agent SDK exact one-point package/native tuple to official
  npm `@anthropic-ai/claude-agent-sdk` `0.3.270` carrying native `2.1.270`:
  Research 315 freezes all nine published hops `0.3.260..=0.3.270` after the
  `0.3.259` ceiling (gaps `0.3.262`, `0.3.264`) with wrapper/native coupling,
  tarball digests, native commits, platform payloads, and a complete 15-file
  tree inventory; 42 declaration surfaces are classified unmapped with the
  mapped query/session/permission/model/tool-admission subset, the
  sidecar/native lifecycle, and the credential non-custody posture unchanged.
  Keep the behavior revision, wire, Node `22.23.2`, sidecar source-tag axes,
  claim ids, and `QualifiedOnly` posture with no unverified-newer. Research
  301 live registered-tool acceptance stays bound to `0.3.259`/`2.1.259` and
  does not transfer: the compiled tuple projects `registered_tools` and
  `consumer_tool_exchange` unqualified (`live_tuple_not_compiled`) until a
  separately authorized live requalification runs on the new tuple. No
  provider operation, downloaded-artifact execution, or host mutation
  occurred.
- raise the Grok Build ACP executable window through official npm
  `@xai-official/grok` `1.0.30`: Research 314 freezes all 25 published
  stables `1.0.6..=1.0.30` after the `1.0.5` ceiling with verified
  wrapper/platform integrity, tarball and decompressed executable digests,
  and a complete shipped-file inventory; the mapped ACP method, callback,
  key, permission, auth, and model literal presence map is byte-identical at
  every hop, the embedded model document changes once only by dropping the
  unread `show_model_fingerprint` key, and the 62 mapped-core ACP modules
  persist while added modules and internal renames stay unmapped. Extend the
  maintained window `1.0.4..=1.0.30` on
  `grok-build.acp-v1.cached-token-model-4-6-v3`, keep the baseline, claim
  identity, deprecated `0.2.114..=0.2.117` segments, `0.2.118..=0.2.121` and
  `1.0.0..=1.0.3` gaps, `grok-4.6` binding, and `AllowUnverified`, and leave
  published alpha `1.0.31` and the next unpublished stable `1.0.32` visible
  unverified. The exact `1.0.25` `QualifiedOnly` catalogue claim and the
  registered-tool courier on the accepted live versions `1.0.4` and `1.0.5`
  stay independently bounded and do not move. No provider operation,
  downloaded-artifact execution, or host mutation occurred.
- raise the Ollama attached-runtime qualified ceiling through official
  `0.33.2`: Research 313 freezes all five published hops after `0.32.15`
  with tag/commit/tree and selected-file hashes and classifies the eight
  selected structs as byte-identical through `0.34.0` with routes identical
  through `0.33.2`. Qualify `0.33.0`, `0.33.1`, and `0.33.2` with no
  inferred gap, keep the baseline, claim identity, behavior revision,
  historical exclusions, decoder specimen, and `AllowUnverified`, and leave
  `0.33.3` and `0.34.0` visibly unverified: the strict native decoder
  fail-closes on the additive `0.33.3` `prompt_eval_cached_count` metrics
  key, with a decoder-tolerance follow-up named. No provider operation or
  host mutation occurred.
- raise the Cursor Agent catalogue, ACP, and headless qualified ceiling
  through official `2026.09.10-fd3934a`: Research 312 freezes all three
  published hops after `2026.08.11-e8db854` with a complete tree inventory
  and classifies the selected CLI definitions, ACP initialize construction,
  and stream-json event keys as text-identical modulo minifier renames. Add
  exact milestones `2026.08.31-4057e58`, `2026.09.02-c22c1a3`, and
  `2026.09.10-fd3934a` with no inferred gap, keep the three distinct route
  claims, historical milestones, exact feature-specific sets, and
  `AllowUnverified`, and leave worker/controller flags, SEA/native
  packaging, ACP load/replay, and continuation recovery unmapped or blocked.
  No provider operation or host mutation occurred.
- raise the Codex exec and app-server qualified ceiling through official npm
  `@openai/codex` `0.154.0`: Research 311 freezes all six published stable
  hops after `0.152.1` with complete wrapper/platform/source-tree
  inventories and classifies the selected exec JSONL, app-server RPC/schema,
  catalogue, session, lifecycle, and process surfaces as a compatible
  extension. Extend the maintained `codex.cli` window through `0.154.0`, pin
  newly interior unpublished `0.152.2` incompatible alongside `0.149.2`,
  `0.150.2`, and `0.151.1`, keep feature-specific exact pins on the
  `0.147.0..=0.149.1` probed points, and preserve historical windows and
  `AllowUnverified`. The default-off `exec --worktree` opt-in, plugin
  reconcile, user verification, and other adjacent deltas stay unmapped. No
  provider operation or host mutation occurred.
- raise the Qwen Code headless qualified ceiling through official npm
  `@qwen-code/qwen-code` `0.23.3`: Research 310 freezes every published
  `0.23.x` hop and classifies the selected stream, controls, help, session,
  and Plan surfaces as a compatible extension. Extend the maintained
  `0.22.0..=0.23.3` segment, keep the exact Plan set bounded at `0.22.3`, and keep exact `0.21.15`
  reasoning and budgets, preserve historical gaps and `AllowUnverified`, and
  leave Web Shell, daemon, ACP, Model Studio, and other adjacent deltas
  unmapped. No provider operation or host mutation occurred.
- raise the Claude Agent ACP qualified ceiling from `0.73.0` to official npm
  `@agentclientprotocol/claude-agent-acp` `0.76.0`. Compatible extension of
  `claude-agent.acp.initialize-meta-extensions-v7`: the selected mapped ACP
  routes stay, and the mode module, config-id module, elicitation, tools,
  settings, utils, and complete `dist/permissions/**` tree are byte-identical
  across every hop, so mode ids/categories, `plan`/`acceptEdits`, permission
  option kinds, and the effort config id are unchanged. The ACP SDK pin stays
  `1.4.0` and the Agent SDK pin stays `0.3.257`. `0.74.0` adds the
  `--hide-claude-auth` `argv` guard; `0.75.0` adds the `authStatus` push
  extension, context compaction as an ordinary `tool_call` lifecycle with an
  extra `_meta.contextCompaction` key, and usage Markdown; `0.75.1` reads the
  resumed model from the local transcript and tightens AIR fork metadata;
  `0.76.0` adds capability-gated recommended config values, clear-context
  coordination, and a `vitest` dev-dependency bump. Every one of those stays
  unmapped with a reason. Published hops `0.74.0`, `0.75.0`, `0.75.1`, and
  `0.76.0` are qualified; unpublished `0.58.0`, `0.73.1`, `0.74.1`, `0.75.2`,
  and `0.76.1` stay incompatible; synthetic unpublished `0.77.0` remains
  visible `UnverifiedNewer`. Host `0.63.0` stays observation-only. Claude Code,
  the Claude Agent SDK sidecar, and the watcher stay untouched. Research 309,
  g05.059.
- raise both qualified Claude Code ceilings from `2.1.257` to official npm
  `@anthropic-ai/claude-code` `2.1.270`: headless `2.1.220..=2.1.270` and
  response-only `2.1.227..=2.1.270`. Compatible extension of
  `claude-code.headless.stream-json.v1` and
  `claude-code.response-only.stream-json.v1`. All eleven published hops after
  the ceiling were retrieved and classified on both platform builds; every
  selected flag keeps an identical normalized option-definition shape, the
  format, effort, and permission enumerations are unchanged, and the
  normalized `init`, `stream_event`, `hook_started`, `result`, and
  `thinking_tokens` constructions hold. Wrapper files except `package.json`
  and `sdk-tools.d.ts` are byte-identical across all twelve compared
  versions, and every `sdk-tools.d.ts` delta is SDK tool declaration content
  the routes do not consume. Host `2.1.258` help equals the frozen `2.1.257`
  digest. Unpublished `2.1.244`, `2.1.249`, `2.1.253` through `2.1.256`,
  `2.1.262`, and `2.1.264` stay incompatible; unpublished `2.1.271` remains
  visible `UnverifiedNewer`. Baselines, claim ids, `AllowUnverified`, watcher
  exact `2.1.251`, and every feature-specific exact-version set stay.
  Research 307, g05.058.

## [0.5.1] - 2026-09-13

### Added
- add the Grok Build pre-session model catalogue as a separately prepared
  `ModelCatalog` operation on exact installed `1.0.25` only. One bounded
  authenticated non-inference `--no-auto-update models` process with stdin
  closed unwritten reports ordered exact model ids and the source default under
  `HarnessConfigurationPosture::Ambient`. The operation may refresh
  authentication or bounded catalogue metadata, but it sends no prompt, opens
  no model session, dispatches no tool, invokes no inference, updates nothing,
  retries nothing, and retains no provider state beyond the bounded operation;
  no provider-suppression file or enterprise-precedence gate is required.
  The shipped exact-`1.0.25` output carries an authentication preamble and
  `*`/`-` bullet rows, which the parser requires. Display name, description,
  input token limit, and reasoning modes project only from the frozen
  exact-`1.0.25` default-model document by exact id equality, unknown ids pass
  through, malformed, duplicate, empty, over-limit, and ambiguous-default
  catalogues fail closed with joined cleanup, and `list_models_recorded`
  returns only redacted stdout/stderr byte counts and digests captured before
  parsing. The new `QualifiedOnly` claim
  `grok-build.catalogue.executable-1-0-25`
  (`grok-build.catalogue.models-text-v1`) admits no older or newer point, and
  the ACP execution claim, `grok_build_model_for_version`, and released
  `v0.5.0` are unchanged. The disproved suppression attempts are recorded as
  evidence. Research 306, g05.053.

### Changed
- raise the qualified OpenCode HTTP ceiling from `1.18.29` to official npm
  `opencode-ai` `1.18.30` on the `opencode.server` axis. Compatible extension
  of `opencode.http-sse.surface-19`: selected HTTP/SSE route files and OpenAPI
  stay byte-identical, and the published GitHub tags diverge, so the hop is
  proven from complete source-tree inventories rather than a commit range.
  Unmapped Bedrock model-id resolution, unmapped GitLab reasoning-option
  shaping, and the unmapped GPT-6 Astra system prompt are the only shipped
  server source changes; provider SDK bumps and the explicit-service-tier
  patch stay provider-facing. Baseline `1.14.48`, claim id, historical gaps,
  and `AllowUnverified` stay; unpublished `1.18.31` remains visible
  `UnverifiedNewer`. Research 304, g05.051.

## [0.5.0] - 2026-09-10

### Added
- raise the Pi RPC qualified ceiling from `0.84.4` to official npm
  `@earendil-works/pi-coding-agent` `0.85.1`. Compatible extension of
  `pi.rpc.strict-lf-v0.84.0-message-update-delta`: selected mapped commands
  and argv stay; published `0.85.0` is also qualified. `0.85.0` experimental
  server/client packaging, `PI_SERVER_*` help, persistent Claude thinking
  effort, in-memory session restore, and GPT-6 Astra stay unmapped. Abort
  still waits for idle; compaction cancel is a bugfix of already-mapped
  abort covering unmapped `compact`. Unpublished `0.83.1` and `0.84.5` stay
  incompatible; unpublished `0.85.2` remains visible `UnverifiedNewer`.
  `pi.sdk-sidecar` stays exact `0.84.2`. Research 302, g05.044.
- add the Claude SDK registered-only session profile:
  `ClaudeAgentSdkRegisteredOnlyBinding` binds one non-empty qualified
  registered-tool selection, zero admitted native SDK tools, and explicit
  `Read` or `ReadWrite` working-resource access in one structural constructor;
  preparation `with_registered_only` / `with_registered_only_binding` and
  driver `with_registered_only` propagate it. Open `tools[]` carries only the
  selected carrier spellings with `allowedTools` unset, and `Read`, `Glob`,
  `Grep`, `Edit`, `Write`, `MultiEdit`, and `Bash` are structurally
  disallowed. Ordinary native constructors and additive
  `with_registered_tools` stay byte-compatible; empty native admission
  without the binding keeps the typed early failure, and an extracted
  registered-only profile without its binding fails
  `profile.registered_only_unbound` at prepare before any lease, process, or
  provider contact. Provider-free fixtures only. g05.046.

### Changed
- corrected the `claude-agent.sdk` native-mediation guarantee: on the frozen
  qualified tuple one in-workspace native `Read` under `default` mode
  completed without a `canUseTool` callback, so the documentation no longer
  claims every admitted native tool is offered through the permission
  exchange. `permission_exchange=Yes` stays scoped to the callbacks the SDK
  emits, registered MCP keeps the Contract 063 before-dispatch bridge, and
  matrix values are unchanged. Research 303 binds Desktop g02.049's three
  merged capsules provider-free. g05.045.

## [0.4.4] - 2026-09-09

### Added
- add an additive Claude Agent SDK prepared-route open surface,
  `ClaudeAgentSdkPreparedSession::open_route_session_with_receipt`, that
  returns the unchanged stable open failure plus one structured failed-open
  receipt: the underlying route code, the observation-derived open stage, the
  exact bounded sidecar subcode when the sidecar rejected the open command,
  whether provider readiness was reached, and the observed cleanup
  disposition — resource and credential release, the owned-tree survivor
  posture, and, for registered opens, the bridge lease close (admission
  frozen, joined calls, listener and registry release). Provider-free
  fixtures cover construction, initialization, account, and MCP-status
  rejections, readiness validation, deadline, joined cleanup, and unconfirmed
  cleanup, and reproduce the exact Card 132 registered-open request against
  the frozen fake `0.3.259` sidecar; the deterministic route identifies no
  producer defect, so the live rejection cause stays unresolved with the
  narrowed boundary recorded, both Contract 061 registered-tool cells remain
  unqualified, and no live retry was run. Research 296. g05.035 card 144.
- let `claude-code.response-only` bind one optional host-approved `Read`
  working resource as the native child's project location through
  `ClaudeCodeResponseProfileInput::with_working_resource`; the bound resource
  reaches the child process request and the host applies it as the child
  working directory, absence keeps today's inherited-directory behaviour, a
  scratch-directory fixture freezes the child-side view — the child runs in
  the leased directory, tools stay suppressed from its own argv, and
  cwd-relative resolution anchors there — while the native CLI's recorded
  cwd-relative ambient behaviour (project settings, upward `CLAUDE.md`
  discovery, git context) is recorded in the guide as ambient provider
  evidence, and no isolation or boundary claim is added; the route remains
  `AmbientHost`. g05.029 card 108.
- bind one Contract 063 `RegisteredToolPreparation` into `grok-build.acp` open
  through `GrokRegisteredToolBinding::qualify` plus
  `GrokPreparedSession::open_registered_session` and
  `GrokAcpDriver::with_registered_tools`. The Swallowtail-owned mediated-stdio
  courier is declared as one reserved entry in the ACP `session/new`
  `mcpServers` list, with the host-resolved courier path, the fixed wire tag
  and its one-shot rendezvous, and the allowlisted environment as ACP's
  `{name, value}` list; Grok spawns that child and Swallowtail never holds its
  `ProcessHandle`. The lease is bound to one exact ACP turn attempt named by
  `with_turn`, which is Contract 063's one active provider turn per server
  lease: while that lease is live no other turn may start. Cancellation settles
  the lease rather than only observing it, so an outstanding call is cancelled
  instead of being left free to dispatch, and turn terminal, cancellation, or
  deadline settles with the exact cause before the consumer sees the terminal
  outcome — including the transport-failure path, where the protocol pump is
  the terminal publisher. A lease the host could not join fails its own turn,
  is never masked by a normal completion, and refuses every later turn while it
  stays retained, and a turn whose start fails before any task exists settles
  before it reports the failed attempt. Settlement is serialized: one caller
  closes and every concurrent caller awaits that same completion, so a
  cancellation still joining can never be read as a clean result by the turn
  or by session close. Opening is bounded by the lesser of the caller's
  `with_open_deadline` and Contract 063's ten-second opening ceiling, across
  every step of the minted-open lifecycle including credential acquisition,
  resource resolution, process startup, and the courier ready barrier, which
  runs on a scoped task under that same bound and is joined on every path.
  Each step is raced individually so a step that owns partial resources still
  runs its own cleanup, and the success boundary is checked against the ceiling
  too, so opening can never succeed past it. A cleanup that fails while the
  barrier is being abandoned keeps its truth through abandonment, so the
  working resource and credential stay held there as well. Every failure after the lease is minted
  closes it explicitly and reports its cleanup truth: a failed registered close
  is never a clean session close, and both the session-close and the
  open-abort paths retain the working resource and credential rather than
  returning them for reuse beside work that may still execute. Omission is byte-identical: an open without a
  binding still sends `mcpServers: []`. Contract 061 stays
  `Unqualified / real_route_gate_pending` with the reason "callable seam
  present; live gate pending", and no feature-matrix cell moves. Provider-free
  fixtures only, over the real courier, kernel, lease, and dispatcher: courier
  declaration, namespaced `tools/list` identity and schema, one mediated
  round-trip inside a mounted ACP turn, revoked-before-dispatch, unknown tool
  name, cancellation of an in-flight call, settlement completed before terminal
  on both the prompt and the transport-failure paths, post-terminal and
  post-cancel refusal, refusal after a failed turn start, one shared cleanup
  truth across concurrent settlement, retained leases when cleanup fails at the
  ready barrier, the
  ten-second ceiling capping a generous caller budget and bounding the ready
  barrier, unbound-turn and retained-lease refusal, an unanswered
  open that reaches `session/new` and expires on its deadline, retained
  resource and credential on unjoined cleanup at both session close and open
  abort, close, and fail-closed transport, kind, identity, host, turn,
  deadline, and unspawnable-command paths. No live Grok, credential, provider call, or
  support claim. g05.035 card 118.
- ship the executable `admitted_claude_agent_sdk` worked example for
  `claude-agent.sdk`: admit the addable route with the three opaque
  host-owned references (interpreted-script launch recipe, sidecar
  environment, delegated subscription credential), lift the
  `AdmittedInstanceRecord` with `ClaudeAgentSdkSessionPreparation::from_admitted`,
  build the mediated-stdio registered-tool preparation over the
  private-loopback carrier with the courier proxy recipe, bind it through
  `with_registered_tools`, prepare, open, run one admitted turn, and close
  with the cleanup outcome. The guide's Explicit Inputs and Normal Path
  sections and the card 132 Desktop packet now point at it; nothing runs a
  live route. g05.035 card 142.
- bind one Contract 063 `RegisteredToolPreparation` into `claude-agent.sdk`
  open through `ClaudeAgentSdkSessionPreparation::with_registered_tools`
  (host-resolved courier path and Card 084 env) and
  `ClaudeAgentSdkSessionProfile::qualify`. The provider spawns the declared
  courier; Swallowtail never holds that `ProcessHandle`. Ready is the
  kernel-observed authenticated connect. Contract 061 stays
  `Unqualified / real_route_gate_pending` with the reason "callable seam
  present; live gate pending". Provider-free fake-SDK proofs cover
  declaration, one mediated round-trip, deny, cancel, close, omission,
  second-read, ready-after-connect, and unspawnable command. g05.035 card 125.
- bind one resolved Contract 063 selected-skill bundle to a fresh Claude Agent
  SDK session through the additive `ClaudeAgentSdkSessionProfile` binding,
  preparation, and driver surfaces. The bundle crosses as one labelled
  `selectedSkillBundle` input through the pinned sidecar's explicit plain-string
  `systemPrompt` surface, while fixed instructions, per-turn
  user text, workspace writes, ambient skills, and raw reference paths stay
  separate. Digest, bounds, reference, and text-encoding failures fail closed
  before SDK construction; resume and listing refuse redeclaration. The
  Contract 061 selected-skill row is available only for this prepared route;
  registered-tool mediation stays `Unqualified / real_route_gate_pending` and
  Card 084 consumer MCP remains separate. Provider-free fixtures only; no
  live provider pass is claimed. g05.035 card 126.
- bind a resolved selected-skill bundle to a prepared Codex app-server session
  through the additive `CodexSessionProfileInput::with_selected_skill_bundle`.
  The bundle crosses as one distinct labelled `selectedSkillBundle` input on
  the `thread/start` parameter surface that already carries the labelled
  `developerInstructions` input — never merged into instructions or user text,
  never a working-resource write. Digest, bounds, reference, and text-encoding
  failures stay typed before provider work; resumed or loaded threads refuse
  bundle redeclaration; provider-direct MCP remains withheld. The Contract 061
  `registered-tool.selected-skill-bundle` row becomes available for this route
  only. Provider-free fixtures only; no live provider pass is claimed. g05.035
  card 127.

### Added
- add the provider-neutral Contract 063 registered-tool and operation-bridge
  vocabulary with bounded selected skill/reference transport and provider-free
  lifecycle checks. Claude's route-local mediated stdio proxy and Codex's
  dynamic native-tool binding remain exact, while Grok consumer-MCP support is
  unqualified; these surfaces do not constitute a live provider pass. g05.035
  cards 114-118.
- add bounded first-turn model-qualification evidence to the opt-in debug
  observer for `supported_model_rejected`: requested/effective ids, catalogue
  size and digest, membership, phase/source, and declared/loaded identity
  labels. Failure predicates, codes, messages, order, and response shape stay
  unchanged; provider-free fixtures cover alias-only, canonical-only, both-id,
  and neither-id catalogues. g05.029 card 119.
- verify the loaded `@anthropic-ai/claude-agent-sdk` package manifest at open
  before `sdk.query` is constructed. Open evidence carries the loaded package
  version; mismatches fail typed `sdk_version_mismatch` with bounded declared
  and loaded identity evidence, while missing or unreadable identity fails
  typed `sdk_identity_unverifiable`. Provider-free matching, mismatching, and
  missing-manifest fixtures cover the gate. g05.029 card 120.
- make sessions terminal after any first-turn rejection. A retry fails typed
  `session_rejected_terminal` with the bounded original sidecar code, without
  replaying SDK input or provider work; explicit close remains available, and
  genuine first-message shape failures retain `init_missing`. Provider-free
  rejection-then-retry and rejection-then-close fixtures cover cleanup and
  no-replay evidence. g05.029 card 121.
- bound loaded SDK package identity lookup to the imported module's own package
  boundary, so unrelated ancestor manifests remain
  `sdk_identity_unverifiable` before SDK construction. Align sidecar model
  evidence with Rust's C0/C1 control-character predicate while preserving the
  existing byte, catalogue, key, digest, code, and observer bounds. Provider-
  free nested-package and shared boundary/control fixtures cover both paths.
  g05.029 card 124.
- add consumer-declared stdio MCP servers on `claude-agent.sdk` open as
  additive prepared input beside the Copy session profile. `strictMcpConfig`
  stays true, server env is an explicit child-allowlist object, every
  `mcp__` tool call goes through `canUseTool`, and required connect failure
  fails open typed. SSE/HTTP, in-process SDK servers, and managed MCP stay
  out. Provider-free fake-SDK proofs cover connect, mediation, deny-never-
  reaches-server, undeclared-before-construct, and required-server failure.
  g05.029 card 084.

### Added
- add opt-in provider-owned Claude Agent SDK session persistence with
  replay-free `resume` and `resumeSessionAt` attachment. Resume rebinds the
  host-leased cwd, first-party account, and exact `SessionResumeBinding`, while
  a separate bounded `listSessions` projection returns only provider session
  ids, leased cwd, timestamps, and titles; persistence remains disabled by
  default and provider-free proofs cover attach, mismatch, unknown-session,
  boundary, listing, and persistence-off paths. g05.029 card 083.

### Added
- add provider-sourced install guidance to absent discovery outcomes for Claude
  Code, Codex CLI, and Grok Build. Guidance carries only the harness name,
  vendor command, source URL, and freeze date; present outcomes carry none and
  discovery never executes the command. g05.029 card 088.

### Changed
- qualified `claude-agent.sdk` registered tools on the exact accepted Card 318
  live tuple (Research 301): the Contract 061 registered capability projects
  `Qualified` with `ExactOneShot` one-shot permission (one Allow dispatched
  `desktop/reconcile` exactly once with unchanged `{}` and correlated its fixed
  `{"ok":true}` result; Deny, cancellation, and stale/foreign controls
  dispatched zero times as pass evidence), `NoProgress`, and `NotCarried`
  selected-skill delivery, and the mediation-kind row moved from
  `real_route_gate_pending` to route-validation support. The route pins SDK
  `0.3.259`, native `2.1.259`, Node `22.23.2`, and the `0.4.4` sidecar tag
  exactly, so the compiled platform is the one variable axis: only the Darwin
  arm64 target the capsules ran on projects the qualified truth, and every
  other target publishes the unqualified truth with `platform_not_admitted`.
  The capsule's cleanup stays the accepted Contract 019 route-qualified
  degraded macOS posture, never renamed `Clean`. Route tests freeze the
  source, task, capsule, Desktop PR, review, merge, closeout, courier,
  attempt, dispatch, and correlation identities, and the frozen transcript
  gains the exact live Allow shape. The retired
  `CLAUDE_AGENT_SDK_REAL_ROUTE_GATE_PENDING_CODE` stays on the immutable 0.4.3
  surface but is published by no row; the unreleased 0.4.4 API surface adds
  `CLAUDE_AGENT_SDK_REGISTERED_TOOL_ROUTE` and
  `CLAUDE_AGENT_SDK_REGISTERED_TOOL_PLATFORM_NOT_ADMITTED_CODE`. Registered
  tool omission stays byte-identical. g05 batch card 153.
- retained the Claude Agent SDK's structured provider-failure facts end to end
  instead of collapsing them to generic `provider_failed`: the sidecar now
  projects the validated numeric `api_error_status`, the bounded
  `terminal_reason`, and the latest active-turn `rateLimitStatus` through the
  strict wire into the failed terminal diagnostic, rejecting malformed present
  values and resetting rate state at each turn boundary so idle or prior-turn
  notices never contaminate the next result. Only the documented `402`
  classifies billing-specific (`provider_billing_unavailable`,
  `EntitlementUnavailable`, configuration change required); `400` and `429`
  keep distinct explicitly mixed route codes without ever becoming
  `QuotaExhausted`, and absent or unlisted statuses stay generic. No status or
  rate notice authorizes retry, fallback, replay, or account mutation, and no
  provider prose, quota payload, or credential ever crosses the wire. No
  qualification, matrix, candidate, tag, or release consequence follows.
  g05 batch card 152.
- repaired the Claude Agent SDK sidecar's strict MCP-status projection to
  admit and discard every optional field the exact `0.3.259`
  `McpServerStatus` declaration (`package/sdk.d.ts:1114-1158`) permits —
  `serverInfo`, `error`, `config`, `scope`, and `tools` — instead of
  rejecting rows that carry them: a required connected server stays admitted
  however much declared metadata its row carries, and `failed`, `needs-auth`,
  and `disabled` rows reach their bounded failure codes rather than
  collapsing to `mcp_status_invalid`. Rows with an undeclared top-level key,
  unknown status, missing/foreign/duplicate names, a non-object entry, or a
  count mismatch stay fail-closed, and no raw error text, configuration,
  URL, header, path, or tool description ever crosses the projection. The
  fake SDK's status rows now carry all five declared optional fields with
  fixture-only markers, and the adapter's bounded regression corpus freezes
  the row-shape table plus the Card 145 Desktop diagnostic tuple (Research
  297), whose one authorized open stopped at the pre-repair bounded
  `mcp_status_invalid` before provider readiness with cleanup confirmed. No
  qualification, matrix, candidate, tag, or release consequence follows and
  both Contract 061 `claude-agent.sdk` cells stay unqualified. g05.035 card
  146.
- Raised qualified OpenCode HTTP ceiling from `1.18.28` to official npm
  `opencode-ai` `1.18.29` on the `opencode.server` axis. Compatible-extension:
  selected HTTP/SSE route files and OpenAPI byte-identical; only unmapped
  Codex OAuth model-id filtering changed. Research 292. g05.037 cards 135-136.
- qualified `grok-build.acp` registered tools on the exact maintained Grok
  Build `1.0.4..=1.0.5` segments from the accepted Card 128 live capsules
  (Research 295), version-scoped: `grok_build_acp_registered_tool_qualification`
  now takes the executable version and returns `Qualified` with
  `NotRepresented` one-shot Deny strength, `NoProgress`, and `NotCarried`
  selected-skill delivery only inside that maintained segment — deprecated
  `0.2.x`, the unprobed gap, and unverified-newer points project the
  unqualified truth and a registered open refuses with `version_not_admitted`
  before any host, lease, or provider work. Frozen ACP v1 session setup and
  the exact Grok artifacts expose no session-scoped selected-skill input, so
  that cell records the exact route limitation. The mediation-kind row moves
  from `real_route_gate_pending` to route-validation support on admitted
  versions, route tests bind the qualification to the capsule, receipt,
  Desktop PR, review, merge, and closeout identities, and
  `GROK_ACP_REAL_ROUTE_GATE_PENDING_CODE` is replaced by
  `GROK_ACP_REGISTERED_TOOL_ROUTE` and the version-scoped signatures in the
  unreleased 0.4.4 API surface. Registered-tool omission stays byte-identical.
  g05.035 card 143.

## [0.4.3] - 2026-09-06

### Added
- propagate bounded Claude Agent SDK termination causes, sanitized `turn_ended`
  result fields, close evidence, and redacted stderr tails into safe consumer
  diagnostics. Qualify SDK 0.3.259 rate-limit information updates as payload-free
  progress while preserving provider error results. Malformed and unmapped SDK
  messages remain terminal as `unknown_message`; provider-free sequence proofs
  cover advisory success, rejection, interrupt and cleanup. g05.029 card 105.
- add an explicit Grok Build ACP interactive-session permission exchange. The
  default reject-and-cancel path remains unchanged; the opt-in prepared session
  exposes bounded one-shot `allow_once` and `reject_once` choices through the
  exact runtime callback exchange, with turn-deadline and abandonment handling.
  Provider-free ACP fixtures cover both choices, timeout, cancellation
  abandonment, malformed requests, bounds, and no active turn. g05.029 card
  085.
- add bounded `Query.supportedModels` evidence and route-local `set_model` to
  the Claude Agent SDK sidecar. Unsupported models fail before `Query.setModel`;
  confirmed values update the effective model, while the pinned SDK's
  confirmation-free result returns typed `model_change_unconfirmed` and keeps
  the prior model. Add optional open-time `low`, `medium`, `high`, `xhigh`, and
  `max` effort with init-confirmed or requested-only evidence; no mid-session
  effort setter is invented. Provider-free fake-SDK proofs cover all paths.
  g05.029 card 082.

## [0.4.2] - 2026-09-06

### Added
- publish Contract 061 candidate K across the Mistral Vibe, Muse Code, Oh
  My Pi, and Qwen adapters. Prepared facades emit the exact 52-row tranche;
  the single interactive Oh My Pi attachment row is consumer-mediated per
  turn from its bounded plan capability, with provider-free ledgers and
  negative assembly coverage. g05.009 card 099.
- complete Contract 061 candidate B across Alibaba Conversations,
  Anthropic Managed Agents and Messages, and xAI Responses WebSocket. The
  prepared facades publish 74 exact rows and withhold only the two named
  Alibaba matrix-descriptor-only rows; adapter-local ledgers cover all 76
  rows, including the single consumer-mediated Managed Agents per-turn
  exchange. g05.009 card 098.
- admit explicit Claude Agent SDK `Bash` profiles under a read-write working
  resource lease. Every Bash call remains consumer-mediated in `default`,
  `plan`, and `acceptEdits`; the callback carries bounded command and
  description views with truncation, while the sidecar retains and returns the
  full input unchanged on allow. Provider-free fake-SDK proofs cover denial,
  execution, truncation, and no live credentials or provider calls. g05.029
  card 081.
- complete Contract 061 candidate L across OpenCode HTTP and Pi RPC/SDK-sidecar
  prepared facades: 64 emitted rows and 5 construction-time withheld rows
  across 69. The Pi sidecar keeps catalogue, usage, and activity rows withheld
  because its exact session plan has no corresponding role or requirement;
  adapter-local ledgers are bound bidirectionally to provider-free prepared
  facade fixtures, with retained-plan per-turn authority. g05.009 card 097.

### Fixed
- repair the `claude-agent.sdk` open path for SDK `0.3.259`: the spawn hook
  now accepts the frozen object-form `SpawnOptions`, first-party readiness
  gates on `accountInfo().apiProvider` while `subscriptionType`, `tokenSource`,
  and `apiKeySource` are labelled observations, sidecar rejection codes reach safe
  route diagnostics, bounded initialize and
  control evidence reports requested-with-supported-list readiness, and the
  first query requires `system/init` before publishing effective model and
  capabilities. Canonical effective models remain distinct from requested
  aliases, and floor-passing newer Node runtimes are recorded as
  `UnverifiedNewer`. Provider-free fake-SDK fixtures cover each path. The live
  editing turn remains unresolved: after readiness, `system/init`, canonical
  cwd, and effective-model evidence, the SDK returned `subtype: success` with
  `is_error: true` and no error text; Swallowtail records typed
  `ProviderFailed` code `swallowtail.claude-agent.sdk.provider_failed` with
  native exit code `1`. Card 102 owns the real consumer editing proof. g05.032
  card 100.

## [0.4.1] - 2026-09-05

### Added
- complete Contract 061 candidate F across Kimi ACP, headless, local-server,
  and Kimi Platform routes. Prepared facades now publish exact bounded route
  contributions; Kimi ACP adds outcome-backed catalogue observation and a
  projected-open seam that preserves exact reasoning/Plan acknowledgement and
  negotiated-model evidence without changing the existing open path. The four
  ledgers prove 75 emitted and 14 construction-time withheld rows across 89
  tuples. g05.009 card 034.
- add provider-neutral compound acknowledgement truth for independently
  state-associated reasoning and Plan halves. Exact provider values attach
  only to effective or rejected halves; reasoning-first rejection may mark a
  requested Plan half terminally not dispatched without claiming pending
  work. The value remains post-open, observation-only evidence and adds no
  request, mutation, routing, or acknowledgement authority. g05.009 card 079.
- select the `claude-agent.sdk` permission mode at open and change it
  mid-session. `ClaudeAgentSdkSessionProfile` binds an explicit admitted tool
  set and one of `default`, `plan`, or `acceptEdits` as an additive prepared
  input; the sidecar passes the admitted set as `tools`, never sets
  `allowedTools`, adds every withheld admissible tool to `disallowedTools`,
  and exposes a `set_permission_mode` wire command whose confirmed mode the
  new `ClaudeAgentSdkPreparedSession::open_route_session` handle returns or
  fails typed. `bypassPermissions`, `auto`, and `dontAsk` are unrepresentable
  and are refused by name before the SDK is loaded. The default profile is the
  unchanged read-only `Read`/`Glob`/`Grep` set under `default` mode.
- run a multi-turn `claude-agent.sdk` editing session. Admitting `Edit`,
  `Write`, or `MultiEdit` binds `ResourceAccess::ReadWrite` on the
  working-resource lease into the plan, the session access policy, and the
  `claude-agent-sdk-ambient-read-write` instance policy; a host that resolves
  a read-only lease fails the agreement before the sidecar starts. The lease
  is a location scope under `AmbientHost` and makes no bounded-filesystem
  claim. Every write is offered to the consumer through `canUseTool` with its
  tool input intact unless the consumer chose `acceptEdits`, and the
  provider-free fake-SDK proofs show a two-turn session writing only what the
  host admitted. g05.029 card 080.
- complete Contract 061 candidate C consumer route projections across
  `swallowtail-adapter-antigravity`, `swallowtail-adapter-bedrock`, and
  `swallowtail-adapter-cursor`. Prepared catalogue, headless, continuation,
  runtime, and ACP operation facades emit exact route and capability truth
  with deterministic per-route ledgers proving 51 emitted and 43 withheld rows
  across 94 census tuples, while keeping four no-control audits as negative
  coverage and withholding catalogue activity observation. g05.009 card 069.
- publish Contract 061 consumer-route projection for candidate J: exact 10
  `llama-cpp.attached`, 6 `llama-cpp.owned`, and 19 `ollama.attached` rows
  with 32 emitted and 3 construction-time withheld. g05.009 card 068.

### Changed
- scope shared preflight consumer-tool exclusion to bounded writable session
  profiles, admitting tool calls for ambient read-write harnesses while
  preserving the existing bounded rejection. g05.029 card 089.

## [0.4.0] - 2026-09-04

### Breaking
- require `InteractiveSessionHandle::close` callers to provide exact
  `HostServices` and a `SessionCleanupRequest` with one caller-selected
  absolute deadline. The boundary now covers active-turn interruption,
  escalation, task and pump joins, credential release, and resource release;
  expiry reports failed cleanup rather than allowing an unbounded or falsely
  clean close. The prior zero-argument close has no compatibility shim. This
  coordinated public API break targets `v0.4.0`. g05.023 card 058.
- remove the previously guaranteed but unqualified OpenAI Background `minimal`
  reasoning value from exact GPT-5.6 `openai.background` preparation. Admitted
  values are `none`, `low`, `medium`, `high`, `xhigh`, and `max`; `minimal` now
  fails before endpoint, credential, request, or provider work. The corrected
  opaque facade point is `openai-responses-background-2026-08-23` with private
  behavior revision `openai.responses-background-v2`. This guaranteed-behavior
  shrink requires coordinated pre-1.0 minor `v0.4.0`. g04.044.
- reject selected-tier OpenAI Background checkpoint restart reconciliation
  before network work. Explicit default-tier dispatch remains ordinary attached
  and one-reattachment only; selected-tier checkpoints cannot be restarted.
  Research 196, g04.049.
- cap `kimi-code.acp` at `0.38.0` under `QualifiedOnly`. Exact `0.39.0` and
  `0.39.1`, unpublished `0.38.1`/`0.39.2`, and farther `0.40.x` fail closed
  because the newer terminal path creates an uncontained local process. This
  shrinks the previously permitted unverified-newer posture above the maintained
  range. g05.017, card 043.

### Added
- add provider-neutral, operation-scoped task-reap reservations. The exact
  selected `ScopedTaskService` must grant owned reap authority before
  credential, resource, process, task, or provider effects, then binds that
  grant to one exact-host/exact-scope task before its future is polled.
  Unsupported, closing, and capacity-exhausted hosts fail at reservation;
  valid later relinquishment cannot lose a shutdown or capacity race.
  Host-local closes reservation admission during explicit shutdown, waits
  unused grants and reservation-backed tasks to settle, and joins bounded
  per-reservation reapers outside the task tree. Starting a reserved join moves
  the worker into that host ownership before returning its observation future,
  so cancelling the future before or after polling cannot detach work or
  release shutdown early. Forged or mismatched authority fails closed before
  task work. Ordinary unreserved spawn, explicit join, and join-on-drop are
  unchanged. The grant is ownership, not a boolean support probe, and there is
  no global task parking. Contracts 009, 010, 017, 019, and 047; g05.025 card
  061.
- add provider-neutral scoped-task relinquishment for caller-bounded
  operations. The selected `ScopedTaskService` accepts an unfinished
  `JoinedTask` only under its exact execution host and `ScopeId`, clears caller
  ownership only after a host reaper accepts it, and returns
  `TaskRelinquishOutcome::AcceptedForReap`. The result is not joined or cleanup
  completion. Host-local retains each per-transfer reaper under the outer
  selected-host lifecycle, whose explicit shutdown joins all retained reapers
  outside the task tree. Task-service clones carry weak handoff authority only.
  There is no adapter-global parking. Wrong authority, finished tasks, and
  repeat transfer fail closed while ordinary join/drop ownership stays
  unchanged. Contracts 009, 010, 019, and 037; g05.024 card 060.
- separate root process exit from owned descendant-tree completion in runtime
  process evidence: `ProcessTreeCompletion::{RootOnly, OwnedTreeEmpty}`,
  `ProcessExit::attesting_empty_owned_tree`, and
  `ProcessExit::tree_completion`. `ProcessExit::new` stays root-only, so no
  existing caller or fixture host gains a tree claim, and only a host whose
  concrete mechanism observed an empty owned tree may construct the attested
  state. `swallowtail-host-local` proves descendant enrollment and termination
  but cannot observe emptiness under `forbid(unsafe_code)`, so every local
  exit reports root-only on every platform. Card 059 falsified the candidate
  primitives natively — setsid escape, descriptor end-of-file with a live
  child, released-group identity, and reparenting to `launchd` — and found no
  sound owned-tree observation within the current ordinary host-local authority
  on macOS; the host stays root-only and adds no unsafe. Contracts 010 and 019,
  g05.023 cards 057 and 059.
- publish exact negotiated model-options observation on projected Claude Agent
  ACP session open: after existing model confirmation, parse one bounded
  `configOptions[id=model]` select, retain it on the session handle through
  the existing runtime seam, and emit only
  `feature.negotiated-model-options-observation` from a distinct active
  source. Required missing model entry fails both public opens through
  existing confirmation. Snapshot-detail malformation that still confirms
  `currentValue` is no snapshot on preserved `open_session` and close+fail on
  `open_session_with_projection` with
  `swallowtail.negotiated_model_options.invalid`. Load, resume, catalogue, and
  prepared contribution stay negative. Research 279, g05.022 card 056.
- add the provider-free `claude-agent.sdk` route: Anthropic's official
  `@anthropic-ai/claude-agent-sdk` `0.3.259`, carrying native `2.1.259`, driven
  through a source-tagged Node sidecar (exact Node `22.23.2`) over the private
  bounded `swallowtail-claude-agent-sdk-jsonl-v1` wire at behavior revision
  `claude-agent.sdk-v1`. Five qualified-only one-point axes bind the SDK
  wrapper, native binary, runtime, wire, and sidecar revision independently; no
  ACP or Claude Code claim transfers in either direction. The application
  provisions the runtime, the `.` SDK entry point, its peer dependencies, and
  the platform package; Swallowtail never installs, vendors, updates, repairs,
  or redistributes them. Swallowtail holds no subscription credential: it
  leases a delegated reference, admits only `firstParty` provenance with an
  `oauth` key source, refuses account identity fields, and mechanically forbids
  the credential-bearing `/bridge` and `/browser` subpaths, API-key helpers,
  and cloud auth refresh. The selected model is sent explicitly and confirmed
  from `system/init` evidence. Tool availability is restricted with `tools` and
  never auto-allowed with `allowedTools`; the read-only allow-list is enforced
  before any consumer round trip, and an allowed decision returns the
  provider's tool input unchanged while that input never crosses the wire. Open,
  start-turn, interrupt, and close are bounded by caller-supplied host
  deadlines that bound the return and the cleanup after it. The route is
  Unix-only: Windows retains no tree owner, so the addable row reports
  `Unsupported` and open refuses there. Close reports `Clean` only on
  `ProcessTreeCompletion::OwnedTreeEmpty`; on ordinary macOS, confirmed root
  completion after the declared descendant termination attempt is the
  operator-accepted `Degraded` posture, and an observed surviving descendant or
  unconfirmed root exit is `Failed`. Session persistence is disabled, so
  resume, fork, session management, model, effort, thinking, usage detail, MCP,
  hooks, plugins, skills, subagents, Bash, and terminal remain later layers.
  The exact package and native points were rebound from `0.3.258` after a full
  15-file package-tree inventory across the hop: 7 files identical, 8 changed,
  none touching the mapped subset. The new `permissionPrompts` selector,
  `user_message_uuids` correlation fields, task summary documentation, managed
  MCP settings, and the bridge and browser implementation changes are all
  classified unmapped, the credential-bearing subpath declarations are
  byte-identical, and `sdkCompat.harnessSchema` is unchanged, so the behavior
  revision, wire, Node, and sidecar source-tag axes all stay put.
  Every effect is preceded by cleanup authority: open reserves the
  open-guardian, pump, and close-guardian lanes from the exact selected task
  service and starts both guardian tasks before it acquires a credential,
  resolves a resource, starts the sidecar, or contacts the provider. A host that
  cannot commit those lanes, or cannot create those workers, refuses the whole
  operation there; after open returns, activating the cleanup guardian cannot
  fail. One enclosing guardian owns the connection, process, pump, remaining
  turn-deadline task, and both leases, and runs the ordered continuation whole:
  interrupt, native close, host termination request, root observation, pump
  join, working-resource release, credential release. Close hands that set over
  before it returns a future, so a runtime that refuses an elapsed deadline,
  missing time service, or wrong host without polling cannot strand live state.
  Caller expiry, caller cancellation, and a dropped cleanup future all transfer
  the guardian — never the pump alone — under the held exact-host, exact-scope
  reservation rather than joining it on the dropping thread, so no lease is
  released around still-live work and the caller reports unconfirmed cleanup
  without waiting. Cancelling a pending open also releases its guard's cleanup
  signal before that handoff, so the credential and working resource it holds
  are released immediately instead of at the abandoned open deadline. `AcceptedForReap` stays ownership transfer only and never
  becomes join or cleanup evidence. Research 278 and 280, Contracts 009, 010,
  017, 019, 029, and 047, g05.022 card 055.
- add `pi.sdk-sidecar` as a qualified exact `0.84.2` route through the
  application-provisioned Node `22.23.2` runtime, source-tagged sidecar, and
  private `swallowtail-pi-sdk-jsonl-v1` wire. New, load, and resume use the
  exact host-leased cwd with bounded typed replay on load, durable app-owned
  session state, no archive/restore/delete, and no substitution for `pi.rpc`.
  New consumers provision those exact runtime, sidecar, wire, SDK, and session
  directory axes; existing v0.3.3 consumers have no action unless opting into
  the new route. Rollback reverts to `v0.3.3`, omits the route and sidecar
  calls, and does not mix workspace versions or alias `pi.rpc`. Research 181
  and 228, g04.033/card 092 and g04.081.
- bind Cline ACP portable `HarnessMode::Plan` on exact `3.0.55`: optional
  `ClineSessionProfileInput::with_harness_mode(Plan)` advertises
  `HarnessModeSelection(Plan)`, requires unique `session/new` plan membership,
  sends one `session/set_config_option` `{configId: mode, value: plan}`, and
  requires response `mode.currentValue = plan` before readiness. Omission keeps
  current initialize/`session/new`/prompt frames with no mode request or Plan
  claim. Fresh context-losing replacement renegotiates the immutable selection.
  Plan widens no permission, auto-approve, resource, isolation, or account
  authority. Research 240, g04.086.
- bind OpenAI Realtime session-scoped reasoning effort on facade
  `openai-realtime-reasoning-2026-08-27`: admit exact
  `minimal|low|medium|high|xhigh` through
  `OpenAiRealtimeSessionProfileInput::with_reasoning_mode`, encode
  `session.update.session.reasoning.effort`, require matching
  `session.updated` acknowledgement, preserve omission bytes, and retain
  superseded `openai-realtime-2026-07-22` proof. Research 236, g04.084.
- add closed adapter-local `ClaudeCodeMaximumTurns` on `claude-code.headless`:
  `ClaudeCodeRunProfileInput::with_maximum_turns` dispatches one canonical
  `--max-turns <n>` appended to the existing command on the exact versions
  Research 226 probed. The type admits positive 32-bit integers only: exact
  artifacts show the native parser coerces with `Number` and rejects only
  `NaN`, so zero, negatives, fractions, `Infinity`, exponent, hexadecimal,
  grouped-digit, and empty inputs all parse, while the loop guard is a
  truthiness test under which a resolved `0` disables enforcement outright.
  The version gate is the exact probed set, not the qualified window: both
  `UnverifiedNewer` points and the never-published in-range `2.1.230` reject
  before process work. `ClaudeCodePreparedRun::start_run` is the only surface
  that dispatches a bound: `low_level_driver` returns an unbound driver and no
  public seam attaches one to a hand-built `ClaudeCodeHeadlessDriver`, so a
  bound can never be paired with another run's plan or with a run that omitted
  the selection. Omission
  preserves the exact prior argv and the approved environment, and is not a
  claim of unlimited execution because an ambient `CLAUDE_CODE_MAX_TURNS` stays
  authoritative when the flag is absent; explicit argv overrides it
  unconditionally with no environment inspection or mutation. A counted turn is
  one tool-use round trip only, never output tokens, tool calls, provider
  requests, retries, cost, or wall time. Reaching the native bound stays a
  provider failure with no output and unchanged joined cleanup.
  Research 226, g04.079
- add closed adapter-local `LlamaCppReasoningSelection` on `llama-cpp.owned`:
  `Disabled` dispatches canonical `--reasoning off` on exact
  `b10069-178a6c449` through `LlamaCppOwnedServingSelection::with_reasoning`
  and `LlamaCppOwnedDriver::with_reasoning`; omission preserves the current
  no-reasoning-argument launch and every `--ctx-size` row is unchanged.
  `--reasoning on` and `auto` are withheld as an unobservable per-request
  distinction and an exact synonym for the default; `--reasoning-budget` is
  withheld entirely because exact source discards it without a template
  thinking end tag. This is owned-serving dispatch and applied server state
  only: effective and observed reasoning behavior, model reasoning
  capability, and attached-route reasoning support stay unclaimed.
  Research 225, g04.078
- add closed Cursor-local `CursorHeadlessReadMode` on `cursor-agent.headless`:
  `Ask` dispatches canonical `--mode ask` on exact `2026.07.01-41b2de7`,
  `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, and `2026.08.11-e8db854` with
  `ResourceAccess::Read` only; omission keeps `Read` on `--mode plan` and
  `ReadWrite` on no mode; read-write authority and newer unverified releases
  reject before process work; Ask is qualified dispatch and application only,
  claims no locally enforced read-only boundary, and adds no portable
  `HarnessMode`, isolation, permission, tool, approval, or network authority;
  Research 224, g04.077
- add optional portable `HarnessMode::Plan` on `qwen.headless` exact
  `0.21.15`, `0.22.0`, and `0.22.1` as canonical `--approval-mode plan`;
  omission keeps `--approval-mode default`; applied `session_start.permission_mode`
  is observed; `auto-edit|auto|yolo` stay unselected; Research 222, g04.075
- add typed exact-Codex-Exec model verbosity: `CodexModelVerbosity` admits
  `low|medium|high` on exact published `0.147.0`, `0.148.0`, `0.149.0`, and
  `0.149.1` for seven frozen slugs
  through prepared evidence and `--config model_verbosity="<value>"`; omission
  preserves the prior argv; live-catalog acceptance and effective length stay
  unclaimed; Research 213, g04.066
- add typed exact-llama.cpp-owned-`b10069` context-size selection:
  `LlamaCppContextSize` admits `1..=2147483647` and dispatches exact
  `--ctx-size N` through immutable prepared serving evidence; omission
  preserves the prior eleven-argument launch, while runtime acceptance,
  effective context, model fit, allocation, and observation remain unclaimed;
  Research 203, g04.056
- add typed exact-Mistral-Vibe-`2.24.2` caller-decreasing maximum turns:
  `MistralVibeMaxTurns` admits `1..=8` for one headless print child; omission
  preserves `--max-turns 8`, native limit remains provider-failed, and the
  fixed plan agent, trust, working-resource, deadline, cancellation, and
  cleanup boundaries remain unchanged; Research 199, g04.052
- add typed exact-Qwen-Code-`0.21.15` caller-decreasing headless budgets:
  `QwenSessionTurnBudget` admits `1..=24` and `QwenToolCallBudget` admits
  `0..=16` across structured runs, first and resumed turns, and fresh
  replacement children; omission preserves `--max-session-turns 24` and
  `--max-tool-calls 16`, the fixed 60-second wall bound and tool policy remain
  unchanged, and the process-local counters reset for every child; Research
  198, g04.051
- add typed DeepSeek V4 Pro adapter-local non-thinking selection for one-request
  structured runs: `DeepSeekThinkingMode::disabled()` dispatches exact
  `thinking.type=disabled`, omits `reasoning_effort`, and carries no portable
  `ReasoningSelection`; ordinary responses remain available while unexpected
  private `reasoning_content` fails closed, and every direct-continuation path
  remains enabled-only; Research 197, g04.050
- add typed OpenAI Background adapter-local standard service-tier selection:
  `OpenAiBackgroundServiceTier::standard()` dispatches exact Responses
  `service_tier: "default"` on ordinary attached runs and one in-process
  reattachment; omission preserves prior create bytes, active-run detachment
  rejects before effects, selected-tier checkpoints reject restart
  reconciliation before network work, and returned tier, price, latency,
  capacity, entitlement, and provider acceptance remain unclaimed; the exact
  facade advances to
  `openai-responses-background-2026-08-23-service-tier`; Research 196,
  g04.049
- add typed Gemini Live adapter-local default-only context-window compression
  for `gemini-3.1-flash-live-preview`: selected
  `GeminiLiveContextWindowCompression::sliding_window()` dispatches exact
  `contextWindowCompression.slidingWindow = {}` on initial setup, one planned
  rollover, and fresh restoration; omission preserves prior setup bytes,
  explicit trigger and target token forms remain withheld, no portable
  capability or shared request field is added, and dispatch does not claim
  provider acceptance or effective compression; Research 195, g04.048
- add exact Gemini Live caller-selected output-token maximum for
  `gemini-3.1-flash-live-preview`: positive values through `65_536` dispatch as
  `generationConfig.maxOutputTokens` on initial setup, one planned rollover,
  and fresh restoration; omission preserves the existing setup bytes, the
  maximum composes with every admitted thinking level, unsupported values fail
  before effects, and dispatch does not claim provider acceptance or effective
  generated length; Research 194, g04.047
- add exact Gemini Live reasoning selection for
  `gemini-3.1-flash-live-preview` through portable `minimal`, `low`, `medium`,
  and `high` values on initial setup, one planned rollover, and fresh
  restoration; omission keeps the existing `MINIMAL` bytes without claiming a
  caller selection, unsupported values and sibling OpenAI Realtime use fail
  before effects, and dispatch does not claim provider acceptance or effective
  reasoning depth; Research 193, g04.046
- add exact Qwen Code `0.21.15` reasoning selection for `qwen3.8-max` and
  `qwen3.8-max-preview` through portable `low`, `medium`, `high`, `xhigh`, and
  `max` values on structured runs, first and resumed turns, and fresh
  replacement; a bounded private stream-JSON control exchange rejects ambient
  override or substitution before the user message without claiming provider-
  effective reasoning depth; Research 189, g04.041
- add exact xAI Responses WebSocket reasoning selection for `grok-4.5`
  (`low`, `medium`, `high`) and `grok-4.6` (`low`, `medium`, `high`,
  `xhigh`), plus positive `max_output_tokens` through `2_147_483_647`, on
  structured runs and serial connection-local sessions; controls remain fixed
  through continuation and fresh replacement, and dispatch does not claim
  provider acceptance, effective reasoning depth, or exact generated length;
  Research 187, g04.039
- add exact Anthropic Messages effort selection for `claude-opus-4-7` through
  portable `ReasoningSelection` values `low`, `medium`, `high`, `xhigh`, and
  `max` on structured runs and fixed direct-continuation sessions; dispatches
  `output_config.effort` without adding Messages thinking or claiming effective
  effort; Research 185, g04.037
- add exact DeepSeek V4 Pro reasoning selection for `low`, `high`, and `max`
  across structured runs, tool continuation, later turns, and fresh local
  restoration; the reasoning-selected path and all direct continuation remain
  fixed to `thinking.type=enabled`, private reasoning replay remains adapter-
  held, and dispatch does not claim provider acceptance or effective depth;
  Research 186, g04.038
- add typed Ollama adapter-local context-window selection (`OllamaContextWindow`,
  `with_context_window`) on structured inference and interactive session
  profiles, with Research 184 positive-domain dispatch of `options.num_ctx`
  beside `num_predict`; dispatch does not prove provider acceptance or
  effective allocation; g04.036
- add typed Cursor headless model parameters (`CursorHeadlessFast`,
  `CursorHeadlessContext`, `with_fast`, `with_context`, `with_effort`) with
  Research 183 deliver-now tuples, canonical single-argument `--model`
  dispatch, and qualified `ReasoningSelection` for high effort on
  `claude-opus-4-8` and `claude-opus-5` only; dispatch does not prove provider
  acceptance; g04.035

### Changed
- raise the Codex CLI qualified ceiling from `0.152.0` to official npm
  `@openai/codex` `0.152.1` on the shared `codex.cli` exec and app-server
  axes. Compatible extension of the existing JSONL and workspace-roots
  behaviors: selected mapped flags and methods stay; the complete
  shipped-tree and source-tree deltas are Guardian, test, and version-bump
  bounded; downloaded official binaries were hashed and never executed.
  Unpublished `0.149.2`, `0.150.2`, and `0.151.1` stay incompatible;
  unpublished `0.152.2` remains visible `UnverifiedNewer`. Feature-specific
  exact version sets stay on the `0.147.0..=0.149.1` probed points.
  Research 275, g05.020.
- raise the Claude Code headless and response-only qualified ceilings from
  `2.1.252` to official npm `@anthropic-ai/claude-code` `2.1.257`. Compatible
  extension of the existing stream-JSON behaviors: selected mapped flags stay;
  `--system-prompt-snapshot`, `--bg` resume wording, `sdk-tools.d.ts` extras,
  and changelog `2.1.257` surfaces stay unmapped. Official `2.1.257` help is
  not byte-identical to frozen `2.1.252`. Unpublished `2.1.244`, `2.1.249`,
  and hop-skipped `2.1.253` through `2.1.256` stay incompatible; unpublished
  `2.1.258` remains visible `UnverifiedNewer`. Watcher stays exact `2.1.251`
  and is not live-ready. Maximum-turn and other feature-specific exact
  version sets stay on the `2.1.220..=2.1.241` probed points. Research 273,
  g05.019.
- raise the Claude Agent ACP qualified ceiling from `0.70.0` to official npm
  `@agentclientprotocol/claude-agent-acp` `0.73.0`. Operator restart after
  official latest moved during the unmerged `0.72.0` family. Compatible
  extension of `claude-agent.acp.initialize-meta-extensions-v7`: selected
  mapped ACP routes stay; `#1004` keeps mapped mode `id`/`category` and
  `plan`/`acceptEdits`; `#1045` leaves steering unmapped and the permission
  callback observable contract unchanged; `0.72.0` effort,
  `user_message_uuid` result attribution, PostModelSwitch mirroring, and
  PreModelSwitch session/new fallback stay classified and selected-compatible
  because Swallowtail still confirms model/effort via explicit
  `set_config_option` plus exact match. `0.73.0` changes only `package.json`
  (version plus Agent SDK pin `0.3.252`→`0.3.257`); every `dist/**` file is
  byte-identical to `0.72.0`. Additive `sessionCapabilities.subagents`,
  native subagent/async-task updates, session titles, Providers API, ACP SDK
  `1.4.0` elicitation rename, and the Agent SDK pin stay unmapped. Published
  intermediates `0.71.0`, `0.72.0`, and `0.73.0` are qualified; unpublished
  `0.58.0` stays incompatible; unpublished `0.74.0` remains visible
  `UnverifiedNewer`. Host `0.63.0` stays observation-only. Claude Code and
  the watcher stay untouched. Research 272, g05.018.
- cap `kimi-code.acp` at `0.38.0` under `QualifiedOnly`. Operator A2: indefinite
  posture change with one artifact-level reopen trigger. Claim identity moved
  from `kimi.acp.executable-window-2` to `kimi.acp.executable-window-5` because
  Contract 029 revises the claim when newer-version posture or exclusions
  change. Frozen historical `window-3` and `window-4` stay reserved.
  Segments stay exact `0.28.1` plus `0.29.0..=0.38.0`; exact `0.39.0`
  and `0.39.1` stay excluded. Unpublished `0.38.1`, unpublished `0.39.2`, and
  farther `0.40.x` fail closed with them. No new ACP behavior revision.
  `kimi-code.local-server` and headless stay `AllowUnverified`. g05.017, card
  043.
- correct the Kimi Code headless compatibility split and qualify the
  agent-core-v2 revision through official npm and GitHub
  `@moonshot-ai/kimi-code` `0.39.1`. The default `kimi -p` engine becomes
  agent-core-v2 `runV2Print` at `0.33.0`, not `0.38.0`:
  `experimental-v2.ts` redefines `isKimiV2Enabled()` there from
  `KIMI_CODE_EXPERIMENTAL_FLAG` truthy to `KIMI_CODE_LEGACY_FLAG` not truthy,
  and this adapter never sets that flag. `kimi.headless.stream-json.v1`
  therefore corrects down from `0.29.0..=0.37.2` to `0.29.0..=0.32.0`, and
  `kimi.headless.stream-json.v2` corrects down and extends from exact
  `0.38.0` to `0.33.0..=0.39.1`. `0.33.0..=0.37.2` had been claimed as
  qualified v1 since g04.064 while emitting the `system.version` preamble the
  v1 decoder rejects; host `0.34.0` sat inside that span and now classifies
  as qualified Maintained v2. Unpublished `0.39.2` remains visible
  `UnverifiedNewer` on the headless axis. Research 270, g05.016.
- stop the Kimi Code ACP axis at `0.38.0` and exclude exact `0.39.0` and
  `0.39.1`. From `0.39.0` the agent-core-v2 ACP terminal runner replaces two
  fail-closed errors with a local host-process spawn in the leased working
  resource, and this route always advertises `terminal: false`, so that
  branch always applies. The route declares `HarnessIsolation::AmbientHost`
  with no isolation claim, Contract 015 denies filesystem containment from
  process ownership and treats a terminal request from a terminal-less client
  as scope-stopping, and no adapter or runtime control mediates the spawn.
  Both points classify `Incompatible` rather than unverified-newer, because
  exclusions are assessed before the newer-version path. The ACP ceiling
  stayed `0.38.0`. That landing left `AllowUnverified` above the named
  exclusions; card 043 later moved the posture to `QualifiedOnly`. No new ACP
  behavior revision was created: wire-shape stability across
  `0.38.0`→`0.39.1` is real and is not sufficient to qualify an authority
  change. Research 270, g05.016.
- raise the Pi RPC qualified ceiling from `0.84.3` to official npm
  `@earendil-works/pi-coding-agent` `0.84.4`. Compatible extension of
  `pi.rpc.strict-lf-v0.84.0-message-update-delta`: selected mapped commands
  and argv stay; `clear_queue`, terminal capability overrides, extension UI
  prompt events, bundled bin path, and streaming `usage` stay unmapped.
  Standing-unused help-unselected `--` and `powershell` remain unmapped
  carry-forwards, not new `0.84.4` deltas. Unpublished `0.83.1` stays incompatible;
  unpublished `0.84.5` remains visible `UnverifiedNewer`. `pi.sdk-sidecar`
  stays exact `0.84.2`. Research 268, g05.015.
- raise the Claude Code headless and response-only qualified ceilings from
  `2.1.251` to official npm `@anthropic-ai/claude-code` `2.1.252`. Compatible
  extension of the existing stream-JSON behaviors: selected mapped flags stay;
  `--restricted`, background-session commands, and watcher flags stay unmapped.
  Official `2.1.252` help is byte-identical to frozen `2.1.251`. Unpublished
  `2.1.244` and `2.1.249` stay incompatible; unpublished `2.1.253` remains
  visible `UnverifiedNewer`. Watcher stays exact `2.1.251` and is not
  live-ready. Maximum-turn and other feature-specific exact version sets stay
  on the `2.1.220..=2.1.241` probed points. Research 266, g05.014.
- raise the Codex CLI qualified ceiling from `0.151.0` to official npm
  `@openai/codex` `0.152.0` on the shared `codex.cli` exec and app-server
  axes. Compatible extension of the existing JSONL and workspace-roots
  behaviors: selected mapped flags stay; `fork`, `--thread-source`,
  `thread/turns/list`, `thread/items/list`, `--code-mode-host`,
  `thread/shellCommand`, and ModelProvider auth-recovery notifications stay
  unmapped. Unpublished `0.149.2`, `0.150.2`, and `0.151.1` stay
  incompatible; `0.152.1` remains visible `UnverifiedNewer`. Feature-specific
  exact version sets stay on the `0.147.0..=0.149.1` probed points.
  Research 264.
- raise the Codex CLI qualified ceiling from `0.149.1` to official npm
  `@openai/codex` `0.151.0` on the shared `codex.cli` exec and app-server
  axes. Compatible extension of the existing JSONL and workspace-roots
  behaviors: selected mapped flags stay; `fork`, `--thread-source`,
  `thread/turns/list`, `thread/items/list`, and `--code-mode-host` stay
  unmapped. Published intermediates `0.150.0` and `0.150.1` are qualified;
  unpublished `0.149.2` and `0.150.2` stay incompatible; `0.151.1` remains
  visible `UnverifiedNewer`. Feature-specific exact version sets stay on
  the `0.147.0..=0.149.1` probed points. Research 262.
- raise the Claude Code headless and response-only qualified ceilings from
  `2.1.241` to official npm `@anthropic-ai/claude-code` `2.1.251`. Compatible
  extension of the existing stream-JSON behaviors: selected mapped flags stay;
  `--restricted`, background-session commands, and watcher flags stay unmapped.
  Published intermediates `2.1.242`, `2.1.243`, `2.1.245`, `2.1.246`,
  `2.1.247`, `2.1.248`, and `2.1.250` are qualified; unpublished `2.1.244` and
  `2.1.249` stay incompatible; `2.1.252` remains visible `UnverifiedNewer`.
  Maximum-turn and other feature-specific exact version sets stay on the
  `2.1.220..=2.1.241` probed points. Research 261, g05.005.
- raise the Qwen headless qualified ceiling through official npm
  `@qwen-code/qwen-code` `0.22.3`: keep exact `0.21.15` reasoning-control,
  extend same-revision `0.22.0..=0.22.3` via published `0.22.2`, keep
  unpublished `0.21.16` incompatible, and leave reasoning/budgets exact
  `0.21.15`. Research 258, g05.004.
- raise the Qwen headless qualified ceiling through official npm
  `@qwen-code/qwen-code` `0.22.1`: keep exact `0.21.15` reasoning-control,
  add same-revision `0.22.0..=0.22.1`, keep unpublished `0.21.16`
  incompatible, and leave reasoning/budgets exact `0.21.15`. Research 216,
  g04.069.
- raise the Pi RPC qualified ceiling through official npm
  `@earendil-works/pi-coding-agent` `0.84.3`: exact published points
  `0.80.10` through `0.84.3`, unpublished `0.83.1` stays incompatible, and
  `0.84.0` keeps private message-update-delta; `toolcall_start` extras,
  `--`, `powershell`, bundled bin path, and streaming `usage` stay
  unmapped. Research 215, g04.068.
- Qualify Kimi Code headless exact `0.38.0` under adapter-private
  `kimi.headless.stream-json.v2` for default agent-core-v2 `runV2Print`
  stream-json. Deprecate `0.29.0..=0.37.2` under
  `kimi.headless.stream-json.v1`. Keep public facade
  `kimi-headless-stream-json-v1`, enforce matching `system.version` preamble at
  runtime, and keep synthetic `0.38.1` permitted `UnverifiedNewer` on the v2
  revision. Research 211, g04.064.
- Retract Kimi Code headless qualified ceiling from `0.38.0` to `0.37.2` under
  existing `kimi.headless.stream-json.v1`. Research 210 proved naked npm
  `0.38.0` `kimi -p` defaults to agent-core-v2 `runV2Print`, not the legacy v1
  corpus Swallowtail audited; the adapter does not set `KIMI_CODE_LEGACY_FLAG`.
  Exact `0.38.0` headless remains visible `UnverifiedNewer` until v2
  stream-json is independently qualified. ACP and local-server `0.38.0`
  qualifications stand. Research 210, g04.063.
- Raised qualified Claude Code headless and response-only ceilings from
  `2.1.238` to official npm `@anthropic-ai/claude-code` `2.1.241`.
  Compatible-extension: official extracted `--help` is byte-identical to
  `2.1.238`. Published intermediates `2.1.239` and `2.1.240` are
  qualified; `2.1.242` remains visible `UnverifiedNewer`. Research 202,
  g04.055.
- Raised qualified Codex CLI ceiling from `0.149.0` to `0.149.1` on the
  `codex.cli` axis (both `codex.exec` and `codex.app-server` routes).
  Compatible-extension: selected mapped flags unchanged, schema bundles
  byte-identical, ModelListParams unchanged. Exec help differs only by
  unmapped `--thread-source`. Research 201, g04.054.
- Raised qualified Gemini CLI ACP and headless ceilings from their previous
  `0.51.0` and `0.51.0..=0.52.0` bounds to maintained
  `0.51.0..=0.56.0` on their separate version axes. Official published
  intermediates are qualified; `0.56.1` remains visible `UnverifiedNewer`.
  Selected lifecycle and stream shapes remain compatible, provider-private
  invalid-stream additions stay unmapped, and transcript management remains
  unsupported. Research 182, g04.034.
- Raised qualified Kimi Code ACP, headless, and local-server ceilings from
  `0.37.2` to official `@moonshot-ai/kimi-code` `0.38.0` on the
  `kimi-code.executable` axis. Compatible-extension: selected ACP,
  headless, and local-server protocol blobs byte-identical; `acp --region`,
  WaitFor, advertised close/delete, and watch-fs `runtime_id` stay
  unmapped. Research 179, g04.032.
- Raised qualified OpenCode HTTP ceiling from `1.18.20` to official npm
  `opencode-ai` `1.18.28` on the `opencode.server` axis. Compatible-extension:
  all eight published patch hops keep `surface-19`; baseline, historical gaps,
  claim id, and `AllowUnverified` stay unchanged. Research 285, g05.028.
- Raised qualified OpenCode HTTP ceiling from `1.18.18` to official npm
  `opencode-ai` `1.18.20` on the `opencode.server` axis. Compatible-extension:
  tagged OpenAPI byte-identical through `1.18.19`; selected execution, delete,
  import, and continuity closures unchanged; `surface-19` kept. Research 176,
  g04.029.
- Raised qualified Antigravity catalogue and headless ceiling from
  `1.1.15` to official GitHub `google-antigravity/antigravity-cli`
  `1.1.17`: `1.1.9..=1.1.17`, published intermediate `1.1.16` qualified,
  `1.1.8` stays independently unqualified, and `mcp` plus `--input-format`
  stay unmapped. Research 177, g04.030.
- Raised qualified Codex CLI ceiling from `0.148.0` to `0.149.0` on the
  `codex.cli` axis (both `codex.exec` and `codex.app-server` routes).
  Compatible-extension: exec help byte-identical, selected mapped flags
  unchanged, ModelListParams unchanged. Research 172, g04.025.
- Raised qualified Qwen Code headless ceiling from `0.21.14` to official
  `@qwen-code/qwen-code` `0.21.15` on the `qwen-code.package` axis.
  Compatible-extension: selected mapped flags unchanged; `types.ts` and
  `systemController.ts` byte-identical; `config.ts` changed only unmapped
  `--session-id` occupancy. Research 173, g04.026.
- Raised qualified Ollama attached ceiling from `0.32.14` to official
  GitHub `ollama/ollama` `v0.32.15` on the `ollama.runtime` axis.
  Compatible-extension: `api/types.go` byte-identical; selected native
  structs and routes unchanged; `routes.go` changed only unselected
  scheduler cache and parser-error cancel. Exclusions `0.32.2` and
  `0.32.10` stay. Research 174, g04.027.
- Raised qualified Claude Code headless and response-only ceilings from
  `2.1.235` to official npm `@anthropic-ai/claude-code` `2.1.238`.
  Compatible-extension: installer wrapper files match `2.1.235` except
  the version pin; selected stream-JSON flags stay; published
  intermediates `2.1.236` and `2.1.237` are qualified. Later stables
  stay AllowUnverified. Research 175, g04.028.
- Raised qualified Oh My Pi RPC ceiling from `17.3.8` to official npm
  `@oh-my-pi/pi-coding-agent` `17.4.0`: maintained `17.2.9..=17.4.0` on
  `oh-my-pi.rpc-v2-v17.2.9`. Minor-line step; private-milestone checked
  and mapping unchanged. Unpublished `17.3.6` stays. Tokenizer JS API,
  `omp ps`, `/cleanse`, and extended-context stay unmapped. Research 178,
  g04.031.
- advance current source to 40 packages and 49 production routes, including
  `pi.sdk-sidecar` and `claude-agent.sdk`, while preserving the immutable
  `v0.3.3` 40-package, 47-route baseline; publish
  [v0.4.0 candidate release notes](docs/releases/0.4.0.md) with every classified
  break, upgrade, rollback, package, route, known limit, and source-only
  distribution truth

## [0.3.3] - 2026-08-19

### Added
- add the separately selectable `swallowtail-adapter-deepagents` package and
  exact Deep Agents ACP `0.1.25` route with qualified-only claim, empty extra
  argv, and host-owned provider API keys without a credential lease
- add the separately selectable `swallowtail-adapter-kiro` package and exact
  Kiro CLI ACP `2.18.1` route with qualified-only claim and local-account
  access
- add the separately selectable `swallowtail-adapter-openhands` package for
  exact OpenHands Agent Server `1.42.1` identity, driver, and prepared
  facade; `openhands.agent-server` is not a production route because live
  HTTP/WebSocket conversation stays unwired
- add the separately selectable `swallowtail-adapter-cline` package and exact
  Cline `3.0.55` ACP and headless routes with qualified-only claims,
  local-account access, and no credential lease
- add the separately selectable `swallowtail-adapter-goose` package and exact
  Goose `1.46.0` ACP route with qualified-only claim and local-config access
- add the separately selectable `swallowtail-adapter-copilot-cli` package and
  exact Copilot CLI `1.0.80` ACP route with qualified-only claim, visible
  public preview, and host-account access
- add the separately selectable `swallowtail-adapter-mistral-vibe` package
  and exact Mistral Vibe `2.24.2` headless route with qualified-only claim
  and local-config access
- add the separately selectable `swallowtail-adapter-qoder` package and exact
  Qoder CLI `1.1.25` headless route with qualified-only claim and
  local-config access
- add the separately selectable `swallowtail-adapter-zcode` package and exact
  ZCode app-server `0.16.3` route with qualified-only claim, host-approved
  Node payload, and no credential lease
- add the separately selectable `swallowtail-adapter-deepseek-harness` package
  and exact DeepSeek Harness JSON-RPC `0.1.0rc6` and local-server `0.1.0-rc.6`
  routes with qualified-only claims, host-owned Cordis configuration, and no
  credential lease

### Changed
- raise the Antigravity catalogue and headless qualified ceiling through
  official GitHub `google-antigravity/antigravity-cli` `1.1.15`:
  `1.1.9..=1.1.15`, `1.1.8` stays independently unqualified, and
  `--input-format` plus Gemini API-key sign-in stay unmapped
- raise the Oh My Pi RPC qualified ceiling through official npm
  `@oh-my-pi/pi-coding-agent` `17.3.8`: maintained `17.2.9..=17.3.8` on
  `oh-my-pi.rpc-v2-v17.2.9`; unpublished `17.3.6` stays; `providers.cacheRetention`,
  advisor, ACP, session switching, and subagent authority stay unmapped
- raise Kimi Code ACP, headless, and local-server qualified ceilings through
  official `@moonshot-ai/kimi-code` `0.37.2`: ACP and headless reuse
  declared-effort and stream-json through `0.29.0..=0.37.2`; local-server
  heartbeat-ping extends to `0.35.0..=0.37.2`; advertised ACP close/delete,
  `acp --login`, terminal-auth metadata, and watch-fs `runtime_id` stay
  unmapped
- raise the Qwen headless qualified ceiling through official npm
  `@qwen-code/qwen-code` `0.21.14`: `0.19.11..=0.20.1` and
  `0.21.0..=0.21.14`, unpublished stable `0.20.2` stays incompatible, and
  `qwen sessions ps` / `/advisor` / live-session registry stay unmapped
- raise the Grok Build ACP qualified ceiling through official npm
  `@xai-official/grok` `1.0.5`: maintained `1.0.4..=1.0.5` on
  cached-token-model-4-6-v3; alpha `1.0.6` stays UnverifiedNewer and is not
  official latest; `--leader-socket` and vendor `_x.ai/*` notifications stay
  unmapped
- raise the Claude Code headless and response-only qualified ceilings through
  official npm `@anthropic-ai/claude-code` `2.1.235`: headless
  `2.1.220..=2.1.235`, response-only `2.1.227..=2.1.235`; later stables stay
  AllowUnverified, and spellcheck plus unused help flags stay unmapped
- raise the Claude Agent ACP qualified ceiling through official npm
  `@agentclientprotocol/claude-agent-acp` `0.70.0`: `0.53.0..=0.70.0`
  excluding unpublished `0.58.0`; Providers API, goal, Air, and file-change
  initialize `_meta` stay unmapped
- raise the Codex exec, app-server, lifecycle, and thread-catalogue qualified
  ceiling through official npm `@openai/codex` `0.148.0`; later stables stay
  AllowUnverified, existing gaps stay incompatible, and `fork` / `thread/fork`
  / Bedrock stay unmapped
- raise the Pi RPC qualified ceiling through official npm
  `@earendil-works/pi-coding-agent` `0.84.2`: exact published points
  `0.80.10` through `0.84.2`, unpublished `0.83.1` stays incompatible, and
  `0.84.0` adds private message-update-delta; streaming `usage` on
  `message_update` stays unmapped
- raise the Ollama attached native runtime qualified ceiling through official
  GitHub `v0.32.14`: `0.14.0..=0.32.14` reuses `ollama.native-text-v1`, keeps
  `0.32.2` excluded, and adds GitHub-prerelease `0.32.10`
- add exact Cursor Agent catalogue, ACP, and headless milestones
  `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, and `2026.08.11-e8db854`;
  calendar gaps stay unsupported
- replace the `claude-code.response-only` patch-version equality gate with a
  protocol-compatibility policy: `2.1.227` remains the proven floor,
  `2.1.228` adds live evidence, the qualified ceiling is `2.1.235`, later
  stable releases may run provisionally, known-bad releases can be denied
  explicitly, and every run still fails closed on command, init, tool/MCP,
  thinking, usage, assistant, or terminal drift
- advance current source to 40 packages and 47 production routes while
  preserving the immutable `v0.3.2` 30-package, 36-route baseline; publish
  [v0.3.3 candidate release notes](docs/releases/0.3.3.md) with upgrade,
  rollback, package, route, and source-only distribution truth

### Fixed
- upgrade transitive `h2` `0.4.15` to `0.4.17` for RUSTSEC-2026-0258
  unbounded empty DATA frames

## [0.3.2] - 2026-08-11

### Added
- add the separately selectable `swallowtail-adapter-command-code` package and
  exact Command Code `1.15.1` headless route with plan-mode one-shot runs,
  bounded NDJSON activity and usage, local provider account access, and
  explicit retained interactive continuity through exact session resume
- add the separately selectable `swallowtail-idioms` package with portable
  idiom records, deterministic confidence decay and merge, lint, bounded
  selection, fail-soft signal recording, a static-rules backend, and registry
  pull/push merge without transport authority
- add opt-in route-path idiom delivery through runtime host ports, fixed
  caller/provider/route/session folding, capability gating, and Codex
  app-server conformance
- add the exact Claude Code `2.1.227` `claude-code.response-only` route for one
  bounded text response through local Max/OAuth with empty tools and MCP, no
  working resource or retained session, and no structured-output claim

### Changed
- classify exact Claude Code medium-effort thinking-token estimates as
  content-free coalescible progress while validating and discarding the empty
  private-thinking envelope before the single assistant text result
- admit Codex app-server child activity from the exact root
  `subAgentActivity(kind=started)` spawn confirmation without weakening
  fail-closed handling for unobserved child identities
- advance current source to 30 packages and 36 production routes while
  preserving the immutable `v0.3.1` 28-package, 34-route baseline; publish
  [v0.3.2 candidate release notes](docs/releases/0.3.2.md) with upgrade,
  rollback, package, route, and source-only distribution truth

### Fixed
- diagnose approved npm shebang launches that require an exact host-side
  interpreter recipe when ambient `PATH` is intentionally absent
- keep Effigy doctor health routing cheap instead of invoking broad validation

## [0.3.1] - 2026-08-08

### Added
- publish [v0.3.1 release notes](docs/releases/0.3.1.md) for the compatible
  debug-observation and provider-session history patch
- add Contract 054 portable provider-session history pages: plan/request/
  response, plan-bound older cursors, `Exact` / `AtLeast` / `Unknown` totals,
  newest-first window helper, and a read-only driver role distinct from load
  and reconciliation
- prove `codex.app-server` synthetic newest-first history pages over bounded
  `thread/read(includeTurns: true)` without control side effects
- prove `opencode.http` and retained `alibaba.conversations` history pages over
  their existing ascending replay walks without live handles or control side
  effects; runtime history plans accept resource-free DirectModelInference
  posture as well as ambient working-resource harness routes
- document provider-session history pages and the Codex, OpenCode, and Alibaba
  retained mappings; ACP load-as-history routes stay unsupported
- add Contract 053 opt-in debug observation: structured `DebugObservation`
  records, defaulted `DiagnosticObserver::observe_debug`, and fail-soft
  `HostServices` emit helpers for restricted wire and lifecycle context
- emit failure-path debug observations across shared discovery/prep, ACP/RPC,
  headless, hosted HTTP/SSE/WS, realtime, remote ACP, Anthropic managed-agent,
  Ollama, llama.cpp attached/owned, and Bedrock catalogue surfaces without
  changing safe diagnostics
- document host opt-in wiring and the current emitter inventory in the
  debug-observation guide and runtime example

## [0.3.0] - 2026-08-08

### Added
- expose shared runtime helpers for installed-executable discovery, prepared
  plan construction, ordered event emission, and terminal projection
- add exact malformed-inbound diagnostics and regression coverage across the
  Codex app-server boundary

### Changed
- **Breaking:** make `codex_cli_binding` and `ollama_runtime_binding` return
  `Option<InterfaceVersionBinding>` so malformed provider-observed versions
  fail closed instead of reaching an infallible parse path
- centralize repeated adapter discovery, preparation, run-loop, and runtime
  plan-family machinery without changing the 28-package or 34-route set
- publish [v0.3.0 candidate release notes](docs/releases/0.3.0.md) with the
  exact API migration, rollback, and source-only distribution boundary

### Fixed
- bound local-process reader joins, preserve force-stop race truth, and close
  runtime waiter and sender-drop paths without hangs
- enforce remote ACP deadlines and joined worker cleanup across HTTP and
  WebSocket transports
- remove provider-reachable panic paths, reject malformed versions and
  unexpected tool-call shapes safely, and preserve exact adapter diagnostics
- make docs indexes, route inventory, MSRV selection, and literal-only version
  parse checks deterministic release gates

## [0.2.0] - 2026-08-06

### Added
- add the separately selectable `swallowtail-adapter-muse` package and exact
  `muse-code.headless` route for local Meta account access to
  `meta` / `muse-spark-1.2`
- add explicit Muse reasoning effort, prepared read-only execution, bounded
  JSONL activity, exact signed-payload discovery, and operator-gated live
  acceptance evidence

### Changed
- promote the coordinated source release to 28 packages and 34 production
  routes without changing crates.io, GitHub Release, binary, or installer
  posture
- raise the verified Rust floor to `1.95.0` for all packages and retire the
  separate Bedrock override; this breaking requirement makes the release
  `0.2.0`
- split Muse event and corpus internals before release without changing public
  API, diagnostics, or guaranteed behavior
- publish [v0.2.0 release notes](docs/releases/0.2.0.md) with exact upgrade,
  rollback, support, and known-limit guidance

## [0.1.1] - 2026-08-06

### Changed
- publish [v0.1.1 release notes](docs/releases/0.1.1.md) for the compatible
  source-tag repair

### Fixed
- preserve accepted Anthropic Managed Agents cancellation as `Cancelled` when
  cancellation and the operation deadline become ready concurrently
- synchronize Kimi detachment conformance with the fixture peer before
  asserting observer-close evidence under workspace contention

## [0.1.0] - 2026-08-06

### Added
- selected one annotated Git tag as the initial distribution; crates.io,
  GitHub Release assets, binaries, sidecars, and installers are excluded
- coordinated 27 independently selectable Rust library packages at version
  `0.1.0`
- qualified 33 production routes across installed harnesses, attached and
  owned local runtimes, hosted APIs and SDKs, and realtime services
- made adapter-local prepared facades the normal integration path while
  retaining provider-neutral low-level runtime roles
- added explicit configured-instance, model-route, access, host-service,
  policy, preflight, operation, event, terminal, and cleanup evidence
- added model catalogue, structured-run, interactive-session, realtime,
  callback, activity, task-list, subagent, lifecycle, reconciliation,
  restoration, detachment, and failure contracts where routes support them
- kept provider credentials, billing, prompts, tools, routing, retry, fallback,
  persistence, and product UI downstream
- retained exact provider and harness version qualification independently from
  the Swallowtail package version
- established Rust `1.90.0` as the general floor, Rust `1.94.1` for Bedrock,
  and Apple Silicon macOS as the initial verified target
- removed Bedrock's legacy Rustls 0.21 dependency path and automated advisory,
  license, and source policy
- replaced declaration hashing with a pinned 27-package semantic public API
  baseline containing 7,819 normalized entries
- documented every supported public item and made all 27 crates deny missing
  public documentation
- completed canonical guidance and compiling normal-path examples for all 33
  routes and every portable feature family
- added deterministic package, API, documentation, MSRV, security, route,
  guide, example, facade, lifecycle, and external-consumer validation surfaces

### Changed
- Established this tag as the first pre-1.0 public API and guaranteed-behavior
  baseline. Compatible changes advance the patch version; breaking API or
  guaranteed-behavior changes advance the minor version; provider
  qualification remains a separate axis.
- Defined migration from existing path or revision integrations: move every
  direct dependency to the same tag and update the lock file atomically.
  Rollback restores the previous manifests and lock file; `v0.1.0` never moves.
- Added [v0.1.0 release notes](docs/releases/0.1.0.md) covering package
  selection, route inventory, installation, limits, and remaining gates.
