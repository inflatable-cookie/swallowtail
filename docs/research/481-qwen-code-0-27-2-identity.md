# Research 481: Qwen Code 0.27.2 Identity And Headless Qualification (0.26.7–0.27.2 Hops)

Observed 2026-11-12. This record qualifies only `qwen.headless` on the npm
package `@qwen-code/qwen-code`. It follows Research 475's `0.26.6` ceiling
and does not borrow evidence from a sibling Qwen route. The task brief named
npm `latest` `0.27.2` as the target.

The 2026-11-12 probe of `npm view @qwen-code/qwen-code dist-tags --json`
returned `latest=0.27.2`, `preview=0.27.2-preview.0`, and
`nightly=0.26.9-nightly.20261107.65e1f1aeaa`; older jobbench, gdpval, dev,
and notes-compaction tags still point at retired prerelease lines. ACP
registry agent `qwen-code` also reports `0.27.2`; that registry row is
discovery metadata, not a claim. The only published stables after `0.26.6`
are `0.26.7`, `0.26.8`, `0.26.9`, `0.27.0`, `0.27.1`, and `0.27.2`, so there
are no interior gaps to exclude. The prereleases published in the same
window (`0.26.7-nightly.20261104.b9e20830f7`,
`0.26.8-nightly.20261105.7bd78683e6`,
`0.26.9-nightly.20261106.80fd7c3353`,
`0.26.9-nightly.20261107.65e1f1aeaa`, and `0.27.2-preview.0`) are ignored
stable-channel noise.

## Decision

Raise the `qwen-code.package` AllowUnverified ceiling from `0.26.6` to
`0.27.2`. Keep the existing `qwen-code.headless.package-window-2` claim,
`0.19.11` baseline, segment statuses, and behavior revision
`qwen-code.headless.v0.21.15-reasoning-control`. The maintained segment is
`0.22.0..=0.27.2`. The older deprecated segments remain `0.19.11..=0.20.1`
and `0.21.0..=0.21.14`; exact `0.21.15` remains maintained. The unpublished
interior stables `0.20.2`, `0.21.16`, `0.22.4`, `0.23.5`, and `0.25.4` stay
excluded. Plan remains exact through `0.22.3`. Reasoning and budget
selections remain exact at `0.21.15`. There is no unpublished later stable
at the 2026-11-12 observation; a later stable stays visible
`UnverifiedNewer` under the latest qualified behavior.

Every hop is a compatible extension. Selected CLI argv, the stream-json
result/error subtype schema, system-controller control types, approval-mode
parsing, permission flow, Plan policy, reasoning effort, the SSH workspace
store and its test, and the adapter's read-only working-resource guard are
byte-identical from `0.26.6` through `0.27.2`. No new public operation is
mapped. `packages/cli/src/nonInteractive/types.ts`,
`control/controllers/systemController.ts`, `core/config/approval-mode.ts`,
`config/approval-mode-value.ts`, `core/permissionFlow.ts`,
`core/plan-mode-shell-policy.ts`, `core/reasoning-effort.ts`,
`tools/exitPlanMode.ts`, `serve/ssh-workspace-store.ts` (with its test),
`nonInteractive/sdk-human-input.ts`, `utils/interactive-mode.ts`,
`services/loop-protection.ts`, `services/loopDetectionService.ts`, and
`core/dashscope-source-config.ts` are one hash each across all seven
versions.

## Official Channel And Artifact Identity

The selected channel is npm `dist-tags.latest` for
[`@qwen-code/qwen-code`](https://www.npmjs.com/package/%40qwen-code/qwen-code).
The GitHub TypeScript SDK is a separate product and does not identify this
CLI package. GitHub tags `v0.26.7` through `v0.27.2` identify the source
snapshots; each npm `gitHead` equals its tag commit and differs from the
SLSA checkout commit.

Every recorded hop is bound by npm SLSA provenance v1. The attestation
subject SHA-512 equals the retained tarball SHA-512, and
`resolvedDependencies.gitCommit` is the GitHub Actions checkout of
`.github/workflows/release.yml` on `refs/heads/main`
(`workflow_dispatch`, builder
`https://github.com/actions/runner/github-hosted`). The shipped package
contains no `.map` files. Node `engines` stays `>=22.0.0` at all seven
versions; no blanket host-runtime qualification follows, and the existing
delegated-harness admission boundary is unchanged.

| npm version | Published (UTC) | Files | npm shasum | Tarball SHA-256 | Official source tag commit |
| --- | ---: | --- | --- | --- | --- |
| `0.26.7` | 2026-11-04 04:12:46.020 | 1,389 | `136fc8063b0496010c61d6b7624a245ed71743a9` | `4cb0587ce0eb368230b128ffd4ff999e7b517865145a26fdab14b7e744c307f0` | [`v0.26.7`](https://github.com/QwenLM/qwen-code/tree/v0.26.7) `f45c34d8ccbd651f2f0f4d97c9b013c11af52f5b` |
| `0.26.8` | 2026-11-05 16:34:28.020 | 1,395 | `7a5b04ba6f2decf1651cd79ec3d87337ff07dd6f` | `9233a8f8e88404a3a5ee4f23c99bf7145edc8deb97635c71b8e73b24b2d899d3` | [`v0.26.8`](https://github.com/QwenLM/qwen-code/tree/v0.26.8) `21ed8f1efbb351b973d96e2bc9efa78f278df99c` |
| `0.26.9` | 2026-11-06 16:19:07.514 | 1,398 | `509328fede9d4c20bb02c523f23907fc5d974962` | `383b47fd08735520aa938dcebefeb7775d986c02b73922f2f2241704d42c37e7` | [`v0.26.9`](https://github.com/QwenLM/qwen-code/tree/v0.26.9) `600aff0592bcfc21668ca4aba95a5b20c6491b86` |
| `0.27.0` | 2026-11-07 20:55:40.797 | 1,400 | `a0177d3b93509b65f235343ca91622c25ccd65ea` | `dd1c502c7c545537d93b240a940c4678f5b268a3cf96ab617daa34d6bc4ba8d7` | [`v0.27.0`](https://github.com/QwenLM/qwen-code/tree/v0.27.0) `03b0c2435be1890a21b117860eaf389076787275` |
| `0.27.1` | 2026-11-09 13:42:22.430 | 1,402 | `b1a914d984b9f5e15e644d981ef29b139f99e884` | `c4b2f0f92f7076456c45bc22d714000c6f240a85a2ab6e55ded806696f05581a` | [`v0.27.1`](https://github.com/QwenLM/qwen-code/tree/v0.27.1) `c2eccf7459c8d6d4b2a5df3733d2f3833153763b` |
| `0.27.2` | 2026-11-10 19:32:35.660 | 1,416 | `890275f72633308fc61835d45833d1538638f693` | `3bdf4c84af2208ceaed5a6f8308504457fdb72d997eafba5b243061ad2d007bc` | [`v0.27.2`](https://github.com/QwenLM/qwen-code/tree/v0.27.2) `f42fe0f3251172c1174d00382ece522eafb3c6c2` |

| npm version | SLSA gitCommit | Tarball SHA-512 (attestation subject) | Tag-to-build-checkout selected-source delta |
| --- | --- | --- | --- |
| `0.26.7` | `4593a3b8f2ff412847737a3abac517bd87726667` | `b11156c266a4a6f1776d605f1b533f8a3a191b987c631b34202e1c5d93f77492f211b6d31c8b72873332486e28e02ccb18521fa1f02bba6990268ff172083079` | none |
| `0.26.8` | `7c6f93ca516e5501397b3ccd3c925aa362b0b702` | `fc10fececf9598ea889e761e777b318fb38aa418c0c99eeaa0cc3b63dbcb02eafb412eba86ca223408d468e374cb0e9f4bb58f4f337c985658be7de3740ddcfd` | none |
| `0.26.9` | `ee1a9b1e1e9453023c50a32f4abf5ff767adca35` | `da8ad5b0fdc3af4d60937f124db61359c1597977ed7344170c786094aed83ec230826ba6be725e4c0bb5374d1eb11530b8b71ebd9f24e20a1d2f0ca90ca6f52c` | reviewed boundary only, see below |
| `0.27.0` | `a32684604a12e39eb98de7181d986e33ddd7b3fe` | `79f66dd0b526feb68652347f48d37aa69f1ca6e3479e6134850bdcbd7cd61ed557f6856d0c382e92ad8864dc180c4ac562a1d18f5c6ad8d439263b7648752bfd` | none |
| `0.27.1` | `444ff228ee94ee316fe4a590a6fb5031b106cab5` | `e8b994c11f15b20b16060c99bf496d5498d58686ffd1b1a2c061667c777e41b85e688410fd22c5eac5c3facf447db09705df754944ebd2260b3d2de2109bcd9f` | none |
| `0.27.2` | `74b8cde466120ff8ec73013d582260dd8a1760c7` | `d17de526f408dbada618adcfe62110886c6ed99838e1a695dddbba3defc9716ba1c2535e33dfdc645f1b2290613a06b299e49ef1acf5e66fff8793075c71247b` | none |

SLSA invocation identifiers: `0.26.7`
`.../actions/runs/41762424148/attempts/1`, `0.26.8`
`.../actions/runs/42048824768/attempts/1`, `0.26.9`
`.../actions/runs/42237109278/attempts/1`, `0.27.0`
`.../actions/runs/42387804952/attempts/2`, `0.27.1`
`.../actions/runs/42614599671/attempts/1`, `0.27.2`
`.../actions/runs/42862818748/attempts/1`. DSSE payload SHA-256 values are
frozen in `source-to-shipped.json`.

### The 0.26.9 tag and build checkout

At every recorded version the version tag commit differs from the SLSA
checkout commit, as in Research 475. At `0.26.6`–`0.26.8` and
`0.27.0`–`0.27.2` that delta is exactly 26 workspace `package.json` /
`qwen-extension.json` version bumps. At `0.26.9` the release workflow
checked out a later `main`: its tree differs from the tag in 111 changed
paths plus 11 build-only paths, including hook infrastructure, the
`FileChanged` hook watcher, core config, the settings schema, CI scripts,
Java managed-agent code, and web-shell assets. The published artifact is
built from that checkout, so evidence here freezes the selected-boundary
snapshots from the SLSA checkout and records the tag copy of the two
differing reviewed-boundary files (`packages/core/src/config/config.ts` and
`packages/cli/src/config/settingsSchema.ts`) under
`source/0.26.9-tag/`, hashed separately. No mapped selected source file
differs between the two trees at `0.26.9`.

## Complete Published-Hop Ledger

`dist-inventory-0.26.6.json` through `dist-inventory-0.27.2.json` list the
SHA-256 of every file extracted from those exact official npm tarballs; the
regenerated `0.26.6` inventory is content-identical to Research 475's frozen
fixture inventory. Each `dist-diff-<from>-to-<to>.json` records the complete
added, removed, changed, and byte-identical path sets, and the identity test
recomputes those path sets from the inventories.

| Hop | Added | Removed | Changed | Identical | Changed selected mapped source files |
| --- | ---: | ---: | ---: | ---: | --- |
| `0.26.6 → 0.26.7` | 365 | 366 | 24 | 1,000 | `config.ts` |
| `0.26.7 → 0.26.8` | 456 | 450 | 36 | 903 | `config.ts`, `top-level-options.ts` |
| `0.26.8 → 0.26.9` | 528 | 525 | 22 | 848 | none (reviewed boundary only) |
| `0.26.9 → 0.27.0` | 357 | 355 | 15 | 1,028 | `config.ts` |
| `0.27.0 → 0.27.1` | 415 | 413 | 38 | 949 | `config.ts`, `session.ts` |
| `0.27.1 → 0.27.2` | 498 | 484 | 37 | 881 | `dashscope.ts` |

The complete artifact diffs include rebuilt distribution chunks, bundled
assets, documentation, translations, and provider-internal modules. Selected
interface source was reviewed from the attested build checkout. No unrelated
provider operation, ACP interface, shell feature, remote host, or new
permission is qualified.

## Selected-Behavior Review

- **`0.26.6 → 0.26.7`:** CLI `config.ts` adds a hosted
  `hostPolicy.modelConfiguration` branch, a `modelCatalogRefreshEnabled`
  setting, and routes `modelFallbacks` through the hosted branch. This route
  supplies no `hostPolicy`, so the extraction/fallback expressions are
  unchanged for it; the catalog refresh is the separate catalogue route.
  Core `config.ts` adds `artifactRecordingSupported` (this route exposes no
  `record_artifact` tool and stream-json runs in SDK mode, so artifact
  recording stays off), gates the models.dev refresh on usage statistics,
  and moves the project-hooks trust rule: non-interactive invocations no
  longer needed explicit trust. The route passes `--safe-mode`, and every
  hook accessor returns `undefined` under safe mode, so no hook scope loads;
  the change cannot reach this child. `settingsSchema.ts` documents the
  catalog-refresh opt-out only.
- **`0.26.7 → 0.26.8`:** CLI `config.ts` adds the project-hooks approval
  gate. `gateHooks` is false under safe mode, so `projectHooksPendingApproval`,
  `projectHooksApproved`, and the YOLO re-gate listener are all inert here.
  `top-level-options.ts` adds the `qwen-subscription` auth type to the
  accepted enum; the route selects no auth type and keeps the delegated
  environment. `nonInteractiveCli.ts` adds Mods submitted-prompt handling and
  i18n for TEXT output. `ModSession::create` requires
  `experimental.mods`, and `ModSession::allowed()` refuses safe mode, so
  `modSession` is `None` for this route and every `modSession?` call is a
  no-op: no `mod_prompt` system message, no Mod turn, no `replyOnly` branch.
  The i18n formatter is passed only for `OutputFormat.TEXT`; this route
  selects `stream-json`. Core `config.ts` adds subscription-model refresh,
  `clearAuth`, the `getContentGenerator(allowUninitialized)` overload, and
  web-search registration sync (returns early in safe mode), plus auto-memory
  policy objects. None changes the stream-json schema or argv.
- **`0.26.8 → 0.26.9`:** `autofix-pr.ts` gains coded-refusal parsing
  (`{"refused","next","message"}` as the helper's last stderr line) and sets
  `NODE_NO_WARNINGS=1` for its helpers; the status/attended/unattended
  command set, the default-ask policy, and the `in_place` always-attended
  refusal are unchanged, and the tool stays outside the frozen exclude list
  exactly as Research 465/475 decided. Core `config.ts` adds `skipFileWatching`,
  the `FileChanged` hook event, hook-system close on shutdown, and a
  `.title.json` chat migration; all hook paths stay behind the safe-mode
  accessor gate. `settingsSchema.ts` adds the `FileChanged` hook settings
  entries (hooks loading is off under safe mode).
- **`0.26.9 → 0.27.0`:** CLI `config.ts` only reformats a missing-advisor
  error message. `nonInteractiveCli.ts` writes the stacked-skill overflow
  warning to stderr on the slash-command `submit_prompt` path; the run's
  prompt and exit code are unchanged. `turn.ts` adds an optional
  `finishMessage` on the internal finished event; the stream-json wire types
  are byte-identical. The core tool scheduler and shell internals changed
  without touching the frozen `--exclude-tools` deny loop or the
  approval-mode parse.
- **`0.27.0 → 0.27.1`:** CLI `config.ts` resolves LSP enablement once and
  refuses it under safe mode, bare mode, hosted read-only mode, SSH and
  provisional workspaces, and the execution sandbox; this route passes no
  `--experimental-lsp` and safe mode refuses LSP regardless. Settings gains
  `agents.defaultBackground` and drops `MergeStrategy.REPLACE` from
  `modelProviders`/`providerProtocol`; the route binds its model explicitly
  and does not manage provider declarations. `session.ts` and
  `nonInteractiveCli.ts` add a bounded post-result memory drain: the result
  envelope is written first, then the child may wait up to 5 s (bounded by
  the session EOF window) before exit, and a success terminal waits for the
  memory writes that turn started. This is provider-internal cleanup after
  the terminal envelope; the adapter's own deadline and cancellation still
  stop and join the child, and no wire event or subtype changes. Core
  `config.ts` adds subscription fallback models, `ModelSwitchOptions`, and
  auto-memory policy bookkeeping; `client.ts` tracks date reminders and
  relays `skipMemoryExtraction` for host-relayed turns.
- **`0.27.1 → 0.27.2`:** `dashscope.ts` adds `isolatedRoute` organization/
  project suppression, filters isolated model overrides, and reads cache
  control from the isolated config only on that route. This route sets no
  `isolatedRoute`, so its provider body and cache control behavior are
  unchanged. Core `config.ts` returns the resident auto-memory prompt and
  extends subscription-model fallback handling. `settingsSchema.ts`
  rewrites the unified-loop description.
- **Bounded provider-internal changes:** tool execution internals
  (`shell.ts`, `shell-semantics.ts`, `coreToolScheduler.ts`), subagent and
  background-task plumbing, memory extraction/dream planning, extension
  archive handling (extensions do not load under safe mode), managed-runtime
  and hosted-serve code, Java managed-agent packages, web-shell/TUI, SDK
  packages, i18n, and CI scripts are unmapped. They cannot affect the
  selected argv, the stream-json schema, the permission before-effect
  boundary, the read-only working-resource guard, or the joined child
  lifecycle; the frozen `--exclude-tools` loop and safe-mode gates above
  bound the ones adjacent to tool execution.

### Unified Agent Loop Opt-In Bounds

Research 475's opted-in unified-loop bounds still hold at every new
version, and the identity test mutation-checks them inside the frozen core
`config.ts`, `nonInteractiveCli.ts`, and `settingsSchema.ts` snapshots:
`QWEN_CODE_ENABLE_UNIFIED_AGENT_LOOP` remains the environment opt-in,
`experimental.unifiedAgentLoop` remains the settings opt-in with
`default: false`, the shared loop is selected with
`const sharedAgentLoop = config.isUnifiedAgentLoopEnabled?.() === true;`,
`scope.onAbort?.();` precedes abort routing, the permission
`getToolCallUpdateCallback()` attachment keeps its
`InputFormat.STREAM_JSON && options.controlService` gate, and the
`budgetEnforcer` still stops at exactly `--max-tool-calls=N`. The 0.26.8+
Mods and submitted-prompt branches stay inside that loop but are
unreachable without `experimental.mods`, which safe mode also refuses.

### Safe-Mode Bounds For New Hook, Mod, and LSP Surfaces

The route's frozen argv includes `--safe-mode`. The identity test proves,
per version, that all three core hook accessors (`getUserHooks`,
`getSystemHooks`, `readProjectHooks`) keep the
`this.getBareMode() || this.isSafeMode()` gate, so no hook scope loads and
the new project-hooks approval store, the `FileChanged` event and watcher,
and the trust-rule churn never reach this child. `mod-session.ts` keeps both
its `experimental.mods` creation gate and the `!this.config.isSafeMode()`
permission gate, so the Mods subsystem cannot run. LSP is off by default and
refused under safe mode from `0.27.1`; this route never passes the flag or
setting.

## Deterministic Mapping Proof

`qwen_code_0_27_2_identity` freezes the identity document, the seven
inventories, the six hop diffs, the selected and reviewed source ledgers
(including the new `mod-session.ts` boundary and the `0.26.9` tag copies),
the reviewed safety-boundary, feeding-file, unified-loop bound, safe-mode
gate, and autofix tool sources, and the SLSA source-to-shipped ledger. It
checks the production claim through `0.27.2`, rejects the unpublished
interior points, the nightly and preview tags, and leaves a synthetic
`0.27.3` `UnverifiedNewer`. It cross-checks the recorded `0.26.6` artifact
identity against Research 475's frozen fixture, proves tag/build parity per
file at every hop, and proves the `0.26.9` divergence from the frozen tag
copies instead of asserting it. It mutation-checks the unified-loop, Mods,
hook-accessor, autofix, and post-result-memory-drain snippets at every
version. `current_qwen_headless` and the prepared facade run the `0.27.2`
claim on deterministic fake processes with the selected argv, delegated
environment reference, and reserved SSH-path rejection before process start.
No provider prompt, live catalogue, credential use, installation, host
update, or artifact execution occurred.

The fixture snapshots are evidence only. The downloaded tarballs and source
trees were not added to the repository.

## Sources

- npm package and `dist-tags.latest` for `@qwen-code/qwen-code`
- npm per-version `dist` metadata, tarballs, and SLSA attestations
- GitHub tags `v0.26.7`–`v0.27.2` and the SLSA checkout commits
- ACP registry `latest` JSON (discovery metadata only)
- Research 465 and 475 for the prior ceiling, claim shape, and retained
  decisions
