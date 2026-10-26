# Research 454: Qwen Code 0.26.1 Identity And Headless Qualification

Observed 2026-10-26. This record qualifies only `qwen.headless` on the npm
package `@qwen-code/qwen-code`. It follows Research 375's `0.25.0` ceiling
and does not borrow evidence from a sibling Qwen route.

The 2026-10-26 recheck of `npm view @qwen-code/qwen-code dist-tags --json`,
including the probe immediately before push, returned `latest=0.26.1`,
`preview=0.25.4-preview.1`, and
`nightly=0.26.1-nightly.20261025.8e28d5380c`. No stable newer than `0.26.1`
was published. Preview and nightly are not stable releases.

## Decision

Raise the `qwen-code.package` AllowUnverified ceiling from `0.25.0` to
`0.26.1`. Keep the existing `qwen-code.headless.package-window-2` claim,
`0.19.11` baseline, segment statuses, and behavior revision
`qwen-code.headless.v0.21.15-reasoning-control`. The maintained segment is
`0.22.0..=0.26.1`. The older deprecated segments remain `0.19.11..=0.20.1`
and `0.21.0..=0.21.14`; exact `0.21.15` remains maintained. Exclude
unpublished `0.20.2`, `0.21.16`, `0.22.4`, `0.23.5`, and `0.25.4`. Plan
remains exact through `0.22.3`. Reasoning and budget selections remain exact
at `0.21.15`. `0.26.2` is the first unpublished later stable and stays
`UnverifiedNewer`.

`0.26.0` opens a new minor line. The selected CLI argv, stream-json result
subtypes, permission set, and SSH workspace guard are unchanged across every
hop, so the new line is a compatible extension of the existing behavior
revision. No new public operation is mapped.

The Research 375 read-only working-resource guard stays. `ssh-workspace-store.ts`
and its test are byte-identical from `0.25.0` through `0.26.1`
(`737d3fa0e3ecac680f8ec77f1896dbf4c0f1cf1aa3c56af01f566f795a8ded55`, 4,449
bytes; `d9328a425831f27015cca278c235ed293bace5db735270541732fd2a8e669a85`,
4,585 bytes). `approval-mode-value.ts` is byte-identical on the same window
(`b70d51f6ffdddfc066e1dc75a04601a5d6dbcf71e8920a2f3f46eea25f3c5467`, 1,170
bytes). No credential state is read. No release or tag authority follows.

## Official Channel And Artifact Identity

The selected channel is npm `dist-tags.latest` for
[`@qwen-code/qwen-code`](https://www.npmjs.com/package/%40qwen-code/qwen-code).
The GitHub TypeScript SDK is a separate product and does not identify this
CLI package. GitHub tags `v0.25.0` through `v0.26.1` identify the source
snapshots. npm `gitHead` is absent from the published metadata at `0.25.0`
through `0.25.3` and equals the tag commit at `0.26.0` and `0.26.1`.

The downloaded npm tarballs were inspected offline and never executed. The
regenerated `0.25.0` inventory matches the frozen Research 375 digest
`d04c0285438dc25e308647a6c35d6484c7e2767ec3fb552f16a15786a934c1b6`. The
`0.25.0` tarball SHA-256
`afeab0c85d682f201101319ce05decf3302d7ed7fb6319baa6a592051ea1a1ba` and source
commit `6788c035698a0ada471c958d1e789e96c6cddd9b` match that record. There is
no published stable `0.25.4`; `0.25.4-preview.0` and `0.25.4-preview.1` are
prereleases and stay incompatible. Exact `dist.integrity`, file count,
unpacked size, inventory digest, and source identity are frozen in
`crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.26.1/identity.json`.

| npm version | Published (UTC) | Files | npm shasum | Tarball SHA-256 | Official source tag commit |
| --- | --- | ---: | --- | --- | --- |
| `0.25.1` | 2026-10-11 10:29:32.140 | 1,375 | `5eb56cf0240abde4d7ac55cad83005883e43dcbb` | `674b31eada2d2ffce10c5085eafef15a0a5ac556e597729d28c56d02b5c6302f` | [`v0.25.1`](https://github.com/QwenLM/qwen-code/tree/v0.25.1) `c1a8218f38203e272e6e965778ed6a2795cfeed7` |
| `0.25.2` | 2026-10-15 15:46:50.127 | 1,700 | `8a391f83d9ab4db18b2bde024ad67afd890b40cc` | `4ff46c2e249b41cf342b14d83b31cae24b0ddd57f77bcbabfeb396241738cf25` | [`v0.25.2`](https://github.com/QwenLM/qwen-code/tree/v0.25.2) `8a415cb628db88844e0196bdfdcce83b4f88d812` |
| `0.25.3` | 2026-10-19 06:14:11.135 | 1,434 | `77c5db396d69f0849f66c96c003e5f0e52c82d64` | `e6e5d1b7ffd7442d16f6c0e3954bd5a69dbf81b0ae224f7fb6f3558d93be2742` | [`v0.25.3`](https://github.com/QwenLM/qwen-code/tree/v0.25.3) `b28b0d948fa28b89750ac5f1525b95e1746f47ab` |
| `0.26.0` | 2026-10-21 23:25:58.355 | 1,333 | `637eb60389f651650d555c51444053574dfa9592` | `16dccd12a9ba433d0e7aa9a18637d2e340bf1a2a4ad01f0f3ca1eb5a69e5f2e1` | [`v0.26.0`](https://github.com/QwenLM/qwen-code/tree/v0.26.0) `ac97ce7a70d9b2ea5d62b357e3d9b0851423afc9` |
| `0.26.1` | 2026-10-24 15:52:45.694 | 1,372 | `8cc9023bc4c085b35345351e77dea413ead9284e` | `9198d276e8583b789ee42a9c38b521d57818b94682d155b2b47741534c90fb73` | [`v0.26.1`](https://github.com/QwenLM/qwen-code/tree/v0.26.1) `b475278c4a300acebe6cfc990a9e1b1d18819bfc` |

## Complete Published-Hop Ledger

Each `dist-inventory-<version>.json` lists the SHA-256 of every file
extracted from that exact official npm tarball. Each
`dist-diff-<from>-to-<to>.json` records the complete added, removed,
changed, and byte-identical path sets. The identity test recomputes those
path sets from the inventories. The source snapshots preserve the eleven
mapped files plus the SSH workspace store, its test, and
`approval-mode-value.ts` for `0.25.0` through `0.26.1`. The 0.26.0/0.26.1
safety-boundary sources are frozen as reviewed keys:
`loop-protection.ts` (new at 0.26.0,
`2d42af7d23675e422fea8285e12bccfb0b3a9d291fef7d2bdb83535d4fb9fc20`, 4,108
bytes, identical through 0.26.1), `loopDetectionService.ts` (0.25.3
`811d52a765c24a3121b8de1a918e771b3a7c8643cea36b121cf0b3673bba0aae`, 71,274
bytes; 0.26.0 and 0.26.1
`35ef3dc21c371547ae64c76d36bfdedb29196a30579abdff049db036a7d15569`, 82,674
bytes), `sdk-human-input.ts` (new at 0.26.0,
`defb6d31cd058cad88ce51c2d16379efd3bf06232ad67e825c9fc402eb2de688`, 1,331
bytes, identical through 0.26.1), and `interactive-mode.ts` (new at 0.26.1,
`9581159b04035fe8243519f328308fb75949610d1860d6dbcc6ffc14c9ec3597`, 375
bytes).

| Hop | Added | Removed | Changed | Identical | Changed selected source files |
| --- | ---: | ---: | ---: | ---: | --- |
| `0.25.0 → 0.25.1` | 459 | 442 | 24 | 892 | `config.ts`, `permissionFlow.ts`, `session.ts` |
| `0.25.1 → 0.25.2` | 709 | 384 | 21 | 970 | `config.ts`, `dashscope.ts`, `session.ts`, `top-level-options.ts` |
| `0.25.2 → 0.25.3` | 384 | 650 | 36 | 1,014 | `config.ts`, `dashscope.ts`, `session.ts` |
| `0.25.3 → 0.26.0` | 390 | 491 | 31 | 912 | `config.ts`, `dashscope.ts`, `session.ts`, `types.ts` |
| `0.26.0 → 0.26.1` | 449 | 410 | 48 | 875 | `config.ts`, `dashscope.ts` |

`approval-mode.ts`, `plan-mode-shell-policy.ts`, `reasoning-effort.ts`,
`exitPlanMode.ts`, and `systemController.ts` stay byte-identical on every
hop. Their frozen hashes are in `selected-source-identities.json`.

## Selected-Behavior Review

- **`0.25.0 → 0.25.1`:** `config.ts` registers the unselected `qwen agents`
  command and forwards an optional managed-engine quarantine callback. The
  route does not select either. `session.ts` constructs `ModSession` only
  when `settings.experimental.mods` is set. `ModSession.allowed()` is false
  under `--safe-mode`, which this route always passes. The prompt-id comment
  change does not alter seeding. `permissionFlow.ts` passes optional
  `invocation.mcpIdentity` into the permission context. That value is absent
  for the mapped non-MCP tools. The route declares no MCP.
- **`0.25.1 → 0.25.2`:** `top-level-options.ts` and `config.ts` add
  `--disable-mcp-server`. The adapter argv does not pass it. Under
  `--safe-mode` the older MCP exclude branch does not run, and the new flag
  is a separate union that stays empty. MCP config BOM stripping runs only
  when `--mcp-config` is set. `session.ts` uses empty history for
  continue-last-turn recovery when the last API prompt was cancelled. The
  route uses `--resume` and does not select that recovery control.
  `dashscope.ts` widens the Model Studio host match from `token-plan.*` to
  `*.maas.aliyuncs.com` and stops putting `cache_control` on tools because
  DashScope ignores it. That is provider-internal request shaping. The child
  environment is passed unchanged.
- **`0.25.2 → 0.25.3`:** `config.ts` forces `workflowAncestorTrustProvider`
  to undefined under safe mode, SSH, and agent-host read-only. This route
  uses `--safe-mode`. `session.ts` only moves the `ModSession` import.
  `dashscope.ts` adds the QwenCloud MaaS hosts `maas.qianwenaiapi.com` and
  `maas.qwencloudapi.com`. Same provider-internal bound.
- **`0.25.3 → 0.26.0`:** `config.ts` registers the unselected `qwen daemon`
  command. Bundled generated UI is disabled under safe mode.
  `model.loopProtection` resolves unknown values to the default `warn`.
  Always-on identical-tool, shell, and cap detectors halt in `stop` and
  remind without halting in `warn`. At `0.25.0` those always-on detectors
  halted. Heuristic detectors stay off because `skipLoopDetection` still
  defaults to true, and this route does not set `model.loopProtection`.
  Adapter turn and tool budgets still bound each child. Stream-json result
  subtypes are unchanged. `types.ts` adds optional `origin` on
  `CLIUserMessage`. `readHumanInputFromSdkMessage` requires
  `origin.kind === 'human'`. The Swallowtail user message has `type`,
  `session_id`, `message`, and `parent_tool_use_id`, and no `origin`, so
  human input stays absent. `resolveWorkflowIntent` returns null without
  that input, when workflows are disabled (the default unless settings set
  them), or when the `workflow` tool is not registered. This route excludes
  `workflow`. `session.ts` still runs `assertLegacySessionExecution` unless
  `executionEngine === 'managed'`, which this route does not select.
  `dashscope.ts` moves the cache breakpoint only when `reattachBlockCount > 0`.
  This route sends no attachments. `flattenDeepSeekReasoningEffort` is
  provider-internal. Reasoning selection stays exact at `0.21.15`.
- **`0.26.0 → 0.26.1`:** `isInteractiveInvocation` replaces the inlined
  interactive decision. For every combination of `promptInteractive`,
  `query`, `prompt`, and stdin TTY it matches the previous result: an
  explicit interactive flag wins, a query or prompt is non-interactive, and
  neither is interactive only on a TTY. Output format does not change that
  result. This route sends the prompt on stdin and does not pass `--query`
  or `--prompt`. New commands `memory`, `daemon`, `attach`, and
  `autofix-pr` are unselected. `dashscope.ts` caps `max_tokens` only for
  wire model `minimax-m2.5` above 32,768. This route sets no output-token
  limit except the exact `0.21.15` reasoning control.

The complete artifact diffs include rebuilt distribution chunks, bundled
assets, documentation, translations, and provider-internal modules. Selected
interface source was reviewed from the tagged trees. No unrelated provider
operation, ACP interface, shell feature, remote host, or new permission is
qualified.

## Deterministic Mapping Proof

`qwen_code_0_26_1_identity` freezes the identity document, the six
inventories, the five hop diffs, and the reviewed safety-boundary sources.
It checks the production claim through `0.26.1`, rejects unpublished
`0.25.4` and the preview and nightly tags, and leaves `0.26.2`
`UnverifiedNewer`. `current_qwen_headless` runs the `0.26.1`
claim on a deterministic fake process and keeps the selected argv, delegated
environment reference, and reserved SSH-path rejection before process start.
No provider prompt, live catalogue, credential use, installation, host
update, or artifact execution occurred.

The fixture snapshots are evidence only. The downloaded tarballs were not
added to the repository.
