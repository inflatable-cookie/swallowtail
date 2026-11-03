# Research 475: Qwen Code 0.26.6 Identity And Headless Qualification (0.26.3–0.26.6 Hops)

Observed 2026-11-03. This record qualifies only `qwen.headless` on the npm
package `@qwen-code/qwen-code`. It follows Research 465's `0.26.2` ceiling
and does not borrow evidence from a sibling Qwen route. An operator dispatch
extended the planned `0.26.3` hop to every published stable through npm
`latest`: the run walks `0.26.3`, `0.26.4`, `0.26.5`, and `0.26.6` and keeps
`0.26.2` as the reviewed ceiling until each hop closes.

The 2026-11-03 probe of `npm view @qwen-code/qwen-code dist-tags --json`
returned `latest=0.26.6`, `preview=0.26.3-preview.0`, and
`nightly=0.26.5-nightly.20261102.0441faff21` (older jobbench, gdpval, dev,
and notes-compaction tags still point at prerelease lines from retired
minors). ACP registry agent `qwen-code` also reports `0.26.6`; that registry
row is discovery metadata, not a claim. No stable newer than `0.26.6` was
published. The only stable-adjacent prerelease after `0.26.2` is
`0.26.3-preview.0`; `0.26.3` through `0.26.6` are the only published stables
in the window, so there are no interior gaps to exclude.

## Date Qualification Of Older Snapshots

Research 465's frozen `qwen-code-0.26.2/identity.json` labels
`0.26.3` `first_unpublished_later_stable` and records
`channel_reprobe.stable_current: true`. Both were observation-time facts of
2026-10-28, when npm `latest` moved after that record's identity commit.
They stay unrewritten; this record supersedes them with the 2026-11-03
observation. The research index lines for Research 454 and 465 are
date-qualified the same way. No frozen artifact in either older fixture is
edited.

## Decision

Raise the `qwen-code.package` AllowUnverified ceiling from `0.26.2` to
`0.26.6`. Keep the existing `qwen-code.headless.package-window-2` claim,
`0.19.11` baseline, segment statuses, and behavior revision
`qwen-code.headless.v0.21.15-reasoning-control`. The maintained segment is
`0.22.0..=0.26.6`. The older deprecated segments remain `0.19.11..=0.20.1`
and `0.21.0..=0.21.14`; exact `0.21.15` remains maintained. The unpublished
interior stables `0.20.2`, `0.21.16`, `0.22.4`, `0.23.5`, and `0.25.4` stay
excluded. Plan remains exact through `0.22.3`. Reasoning and budget
selections remain exact at `0.21.15`. There is no unpublished later stable
at the 2026-11-03 observation; a later stable stays visible
`UnverifiedNewer` under the latest qualified behavior.

Every hop is a compatible extension. Selected CLI argv, stream-json result
subtypes, the permission flow, approval mode, and the SSH workspace guard
are unchanged. No new public operation is mapped.

The Research 454 read-only working-resource guard stays.
`ssh-workspace-store.ts` and its test are byte-identical from `0.26.2`
through `0.26.6` (`737d3fa0e3ecac680f8ec77f1896dbf4c0f1cf1aa3c56af01f566f795a8ded55`,
4,449 bytes; `d9328a425831f27015cca278c235ed293bace5db735270541732fd2a8e669a85`,
4,585 bytes), as is `approval-mode-value.ts`
(`b70d51f6ffdddfc066e1dc75a04601a5d6dbcf71e8920a2f3f46eea25f3c5467`, 1,170
bytes). No credential state is read. No release or tag authority follows.

## Official Channel And Artifact Identity

The selected channel is npm `dist-tags.latest` for
[`@qwen-code/qwen-code`](https://www.npmjs.com/package/%40qwen-code/qwen-code).
The GitHub TypeScript SDK is a separate product and does not identify this
CLI package. GitHub tags `v0.26.3` through `v0.26.6` identify the source
snapshots; each npm `gitHead` equals its tag commit and differs from the
SLSA checkout.

Every recorded hop is bound by npm SLSA provenance v1. The attestation
subject SHA-512 equals the retained tarball SHA-512, and
`resolvedDependencies.gitCommit` is the GitHub Actions checkout of
`.github/workflows/release.yml` on `refs/heads/main`
(`workflow_dispatch`, builder
`https://github.com/actions/runner/github-hosted`). That SLSA commit
differs from the version tag. All eleven mapped TypeScript files, all seven
reviewed safety-boundary files, and the new `dashscope-source-config.ts`
feeding file at each SLSA commit are byte-identical to the frozen tag
snapshots. The tag-to-SLSA tree delta is 26 workspace `package.json` /
`qwen-extension.json` version bumps and no `/src/` path at every recorded
version. The shipped package contains no `.map` files.

The regenerated `0.26.2` inventory is content-identical to the frozen
Research 465 fixture inventory, and the `0.26.2` tarball SHA-256
`01020fa2c701a1eb385c2f5405686bf25c5aa0b8e5cd8893e075cf0ad4d77e8f`, shasum
`308409e71b1e57a1e27c709023f16a4ed73a5211`, source commit
`402911146f6b2a80cb97423309faf1956f6defe4`, and SLSA commit
`dee6ace9d8f84d98084abf292f2b0a5ffd3dfa22` match that record. Research 465
remains the `0.26.2` identity record. Exact `dist.integrity`, file count,
unpacked size, inventory digest, and source identity are frozen in
`crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.26.6/identity.json`.

| npm version | Published (UTC) | Files | npm shasum | Tarball SHA-256 | Official source tag commit |
| --- | ---: | --- | --- | --- | --- |
| `0.26.3` | 2026-10-28 14:41:55.426 | 1,380 | `afd95029b5e592121f8f463c6b35a2913bcec722` | `55194c90e1c376fbdf682247a787e12e58d159a0a11c27f162ec9b2f48cee0dd` | [`v0.26.3`](https://github.com/QwenLM/qwen-code/tree/v0.26.3) `eeee1443a7e36ab22b2f1dcded8dec64b621a7ee` |
| `0.26.4` | 2026-10-29 14:41:23.196 | 1,384 | `37b00be36b5060ae583ef78d2a20925d03c7db66` | `a7640f1c0656b7e54ea5995335ed6cc18ee219b8b4200fe8a6141cff148aaa89` | [`v0.26.4`](https://github.com/QwenLM/qwen-code/tree/v0.26.4) `715a550cfca0135e00d4015d1ca82a8167e81fff` |
| `0.26.5` | 2026-11-01 03:27:45.789 | 1,390 | `318a2f86fa4e4e6c24b264d333751456bbfb906e` | `fb8dd2b327beedbe639966fca24d27b1b2860b148b78628812178262af871d1a` | [`v0.26.5`](https://github.com/QwenLM/qwen-code/tree/v0.26.5) `db0e4e24a0d89b211b2a15f4af2be22adac71dfc` |
| `0.26.6` | 2026-11-02 18:35:34.846 | 1,390 | `bcf574b8c25f2fa75677f2c86ef30fc393b2cbfb` | `6d6889ae71ce6f3a892d597ba731edf1a45f34f3443ba1e55080511d1794b9cb` | [`v0.26.6`](https://github.com/QwenLM/qwen-code/tree/v0.26.6) `042c023b7beb4ad2a9ef95b2906b3564e3f23089` |

| npm version | SLSA gitCommit | Tarball SHA-512 (attestation subject) | Tag-to-SLSA selected-source delta |
| --- | --- | --- | --- |
| `0.26.3` | `8345f5ceec5ebe6179c1f6a97ad811ccf1b69212` | `cfc74bd7e376788d51c109bfb5c7899856f13ed7629f84caa45b9ca1abbc365e1b9bf0ec5eca2140b14f338e6ddb4b15cc069883c687f2a2e95f94422225c4dc` | none |
| `0.26.4` | `2919258c6f612e70694f2b6f9f96d4105506319a` | `61e46251a1f777e2713e26144b87866a5b0a453b88114a3be4958f59d541a29b5d6c01f71497566c6b63b3fb321021144c7e6a9349eef2d45bd473042a7d08d9` | none |
| `0.26.5` | `fe30b5daaa6ae3bef16370393200204c4a82c28e` | `8aa701a8b40398abceb51715bc5efc97b6dfd0da5dca6185068d2a358d1abac3dece1b7e3cb5e4875e6f9c820b6812ad33805a6ed1d2bfad3159fb4ca2627b87` | none |
| `0.26.6` | `0441faff2149f5793f2aa36435b3f17c376bd3d3` | `3fe0403cd4d39205000fbada61b61ed78a05bb0b004f7c7fbbe42030b734c1a35af78b7aa8b2e1d74ed9f400bea28ca14393fe3b7923158c3d6f2aa9844672f5` | none |

SLSA invocation identifiers: `0.26.3`
`.../actions/runs/40690606053/attempts/1`, `0.26.4`
`.../actions/runs/40855785057/attempts/1`, `0.26.5`
`.../actions/runs/41209751466/attempts/3`, `0.26.6`
`.../actions/runs/41485298864/attempts/1`. DSSE payload SHA-256 values are
frozen in `source-to-shipped.json`.

## Complete Published-Hop Ledger

`dist-inventory-0.26.2.json` through `dist-inventory-0.26.6.json` list the
SHA-256 of every file extracted from those exact official npm tarballs.
Each `dist-diff-<from>-to-<to>.json` records the complete added, removed,
changed, and byte-identical path sets. The identity test recomputes those
path sets from the inventories. The source snapshots preserve the eleven
mapped files plus the reviewed safety-boundary files — now including
`dashscope-source-config.ts` — at all five versions.

Reviewed files other than `loopDetectionService.ts` are byte-identical
across the whole window. `loopDetectionService.ts` moves from
`6f306ca6e1c15260ff76988aedc8a2752f429add0d90241be32d1e8d388f29e3` (82,986
bytes) to
`f60175b3d4c45ca6bdfeb83bf4090932d5f5a7de45863929d2f909426ad8d80e`
(85,687 bytes) at `0.26.6`. `loop-protection.ts`, `sdk-human-input.ts`,
`interactive-mode.ts`, `approval-mode-value.ts`, and the SSH workspace
store with its test stay put. `dashscope-source-config.ts` is byte-identical
(`d5f505aa36a09fdbfae565d42be538ce8e30ff1cd239008011350fdcc2766056`,
3,828 bytes) from its `0.26.2` introduction through `0.26.6`; this record
freezes that feeding-file identity so the DashScope header behavior reviewed
in Research 465 is now hash-bound. The unified-loop bound files and the
autofix tool are frozen the same way: core `config.ts`
(`c6d83da6ee3fa3596fd9bb10e3b0056833bcebd978203df15a730b7a0b46de35` 520,072 bytes at `0.26.2` →
`f2d1cda0a9ba10d5362a04a684dfe05a708d366336d1693401a0ece6c84660d4` 530,385 bytes
at `0.26.6`), `nonInteractiveCli.ts` (`8e86d142495e60f02815efd923847ff7c5d17ddde365d16d331a1512cc7c4726`
163,603 bytes → `1fca9b12c0ee5bca2121c4eeaa77cfc03c69c7cc5db78dc8f644c25364d02590`
166,411 bytes), `settingsSchema.ts` (`7023360b8a65e990eff5345888c4602746fb3df4d56cce86fc56283bbd1d34da`
243,503 bytes → `dde857f3b609301494d6c45b2fef272bfc6eacb81d030af395917bdc7d1e35b6` 243,865
bytes at `0.26.5`), and `autofix-pr.ts` (`08e85f7b7dc508df2dbf81ea69a465a5790645ae320667e429f25d49a95f0947`
52,769 bytes → `5efe669e1372b417943832eddb01fd3a32e1a5b19f4e523dc84bc006b9b53c2f` 53,837
bytes at `0.26.5`, where the `in_place` variant arrives). The identity test
mutation-checks the bound snippets listed below inside those frozen
snapshots at every recorded version.

| Hop | Added | Removed | Changed | Identical | Changed selected source files |
| --- | ---: | ---: | ---: | ---: | --- |
| `0.26.2 → 0.26.3` | 350 | 348 | 20 | 1,010 | `config.ts`, `session.ts`, `types.ts`, `systemController.ts`, `top-level-options.ts` |
| `0.26.3 → 0.26.4` | 347 | 343 | 21 | 1,016 | `dashscope.ts` |
| `0.26.4 → 0.26.5` | 413 | 407 | 39 | 938 | `config.ts`, `dashscope.ts` |
| `0.26.5 → 0.26.6` | 337 | 337 | 20 | 1,033 | `session.ts` (plus reviewed `loopDetectionService.ts`) |

`approval-mode.ts`, `exitPlanMode.ts`, `permissionFlow.ts`,
`plan-mode-shell-policy.ts`, `reasoning-effort.ts`,
`systemController.ts` (after `0.26.3`), and `types.ts` (after `0.26.3`)
stay byte-identical across the remaining hops. Their frozen hashes are in
`selected-source-identities.json` and `reviewed-source-identities.json`.

## Selected-Behavior Review

- **`0.26.2 → 0.26.3`:** `top-level-options.ts` and `config.ts` add
  `--workflow-mode on|off` and a `workflowMode` settings field; the adapter
  argv does not pass it, and the value is undefined unless set.
  `types.ts` adds an optional `workflowMode` session-start parameter and two
  new control-request subtypes `get_workflow_mode` and
  `set_workflow_mode`; both are requests, not result subtypes, and the
  route sends neither. `systemController.ts` handles the two subtypes,
  stages an initial workflow mode when the session-start parameter carries
  one (this route sends none), and advertises additive
  `can_get_workflow_mode` / `can_set_workflow_mode` capability booleans;
  the route's initialize parsing keys on its existing capability fields,
  which are unchanged. `session.ts` SDK-mode changes: the initialize
  dispatch now sets `propagateError`, so an initialize failure re-throws
  after the error control response is sent instead of being swallowed; a
  duplicate `initialize` while initialization is in flight gets a typed
  error control response, and a post-initialization `initialize` waits for
  initialization before dispatch. This route sends exactly one
  `initialize` per child as the first stream-json input line, and that
  first request still routes through the dedicated SDK-mode initialization
  branch, so the handshake bytes and responses are unchanged. The route
  declares no MCP, so the SDK-MCP propagation flag has no other consumer.
- **`0.26.3 → 0.26.4`:** `dashscope.ts` is comment-only: configured
  reasoning presets declare accepted tiers and bypass the fallback ladder.
  No code path moves.
- **`0.26.4 → 0.26.5`:** `dashscope.ts` widens a comment's tiered-effort
  family list (provider-internal request shaping). `config.ts` stops a
  partially started LSP client when initialization throws so a failed init
  leaves no orphaned language-server processes. This route configures no
  LSP servers; the cleanup path is unreachable for it.
- **`0.26.5 → 0.26.6`:** `session.ts` annotates user messages that answer
  tool calls or carry tool blocks (`promptParts`, `fromTool`) so the
  model history can distinguish them from typed prompts. This route sends
  one string-content stdin prompt, for which both helpers are no-ops.
  Reviewed `loopDetectionService.ts` gives queued loop-protection reminders
  a prepare/commit/abandon delivery lifecycle: a reminder is credited only
  when the carrying request reaches the model and is restored when a send
  is abandoned. Detector selection is unchanged — heuristic detectors stay
  off unless configured, and this route does not set `model.loopProtection`.
  The adapter's frozen `--max-tool-calls` / `--max-session-turns` bytes
  still bound each child. This is provider-internal bookkeeping; no mapped
  result subtype changes.
- **Unmapped core config:** new commands and surfaces behind `workflow`
  tooling, workflow mode, and managed-session docs changes remain
  unselected. Stream-json result subtypes are unchanged after `0.26.3`
  (`types.ts` byte-identical from that point).
- **`autofix_pr` boundaries preserved:** the `AutofixPrTool` is still
  registered in the general tool registry independent of `--core-tools`,
  safe mode still ignores the allowlist, and the frozen exclude list still
  does not name `autofix_pr`, so the model can reach it independently of
  the CLI command — the reviewed Research 465 boundary stands. The tool
  implementation is now frozen as reviewed key `autofix_pr_tool`
  (`08e85f7b7dc5…` 52,769 bytes at `0.26.2`–`0.26.4`;
  `5efe669e1372…` 53,837 bytes at `0.26.5`–`0.26.6`), and the identity test
  mutation-checks the `status` / `attended` / `unattended` command set at
  every recorded version. `0.26.5` adds an `in_place` prepare option whose
  watches run in the user's own checkout and refuse unattended grants —
  the frozen `0.26.5`/`0.26.6` snapshots contain the
  `watch.inPlace === true` refusal with its `always attended` message, and
  the test fails if any earlier snapshot gains those snippets. `status`
  still launches the status helper only, `attended` still revokes a grant
  without launching a helper, every acting command still defaults to ask,
  the headless text-input route still auto-denies asks, and Plan mode
  still refuses acting helpers. Existing permission rules can change
  decisions, so this is not a host containment claim. The explicit
  Research 465 decision to keep the frozen exclusion set is preserved:
  these bounded effects do not require a new adapter operation or a
  changed argv contract, and the tool is not mapped as a Swallowtail
  operation.

### Unified Agent Loop Opt-In Bounds

Research 465 recorded that the delegated environment or existing settings
can opt into `experimental.unifiedAgentLoop`; this record bounds the
opted-in path from the exact `0.26.6` source, with the bound-bearing files
frozen and mutation-checked instead of inferred from the default. Opt-in is
real and hash-bound: core `config.ts` — frozen as reviewed key
`unified_loop_core_config` — implements `isUnifiedAgentLoopEnabled()`,
which returns true when `QWEN_CODE_ENABLE_UNIFIED_AGENT_LOOP'] === '1'`
holds in the child environment — which the adapter passes through unchanged
— and otherwise reads the settings-derived field whose assignment in CLI
`config.ts` (`unifiedAgentLoopEnabled: settings.experimental?.unifiedAgentLoop
?? false`) is already frozen in the mapped set. `settingsSchema.ts` —
frozen as `unified_loop_settings_schema` — declares the
`experimental.unifiedAgentLoop` entry with `default: false` and documents
the same environment opt-in. The enabled non-interactive loop runs
`runAgentLoop` (`agent-loop.ts`) with round-scoped scope objects; the
wiring — frozen as `unified_loop_non_interactive_cli` — selects it with
`const sharedAgentLoop = config.isUnifiedAgentLoopEnabled?.() === true;`,
and the disabled branch keeps the existing loop in the same
`nonInteractiveCli.ts` turn driver.

- **Behavior:** both branches feed the same stream-json adapter —
  `adapter.processEvent(event)` per model event, the same
  `startAssistantMessage` / `finalizeAssistantMessage` pairing, the same
  `LoopDetected` handling, and the same formatted-error reporting. Terminal
  reasons map one-to-one onto the legacy outcomes: `surface_ended` to the
  success envelope, `loop_detected` and `duplicate_provider_call` to the
  loop-detected result, `cancelled` to the shared abort routing, and
  `max_session_turns` to the same max-turns error handler. `types.ts` is
  byte-identical from `0.26.3` on, so the wire schema does not move with
  the opt-in. All of these snippets are mutation-checked inside the frozen
  `nonInteractiveCli.ts` snapshots at every recorded version.
- **Cancellation:** both branches poll the same `abortController.signal`;
  the shared loop finalizes the in-flight assistant message before
  `routeAbort()` (`scope.onAbort?.();` precedes it in the frozen wiring),
  the same pairing the legacy branch performs. Adapter-side
  deadline and cancellation still stop and join the child, which the
  deterministic fake-process suite exercises independently of the CLI's
  internal loop choice.
- **Permissions:** both branches execute tools through the same
  `executeToolCall` scheduler. The control-service tool-call update
  callback attaches under the same condition in both branches —
  `InputFormat.STREAM_JSON && options.controlService` gating
  `options.controlService.permission.getToolCallUpdateCallback()` — so the
  `0.21.15` reasoning handshake sees the same updates on either loop.
  Non-interactive text-input asks remain auto-denied, `--exclude-tools`
  deny rules still apply, and safe mode still ignores the core-tools
  allowlist.
- **Budgets:** the shared loop ticks `budgetEnforcer.tickToolCall()` in
  `beforeToolLaunch` before each launch — the frozen comment states
  `--max-tool-calls=N stops at exactly N` and the `structured_output`
  exemption applies only under `--json-schema`, which this route does not
  set — and the session turn limit surfaces as the `max_session_turns`
  reason into the same error handler. The adapter's frozen budget bytes
  therefore remain effective when a delegated environment opts in.

The opt-in is a bounded provider-internal loop choice inside the mapped
non-interactive surface. It adds no public operation, no argv change, and
no claim change; the behavior revision stays
`qwen-code.headless.v0.21.15-reasoning-control`.

The complete artifact diffs include rebuilt distribution chunks, bundled
assets, documentation, translations, and provider-internal modules.
Selected interface source was reviewed from the tagged trees and bound to
the SLSA checkouts. No unrelated provider operation, ACP interface, shell
feature, remote host, or new permission is qualified.

## Deterministic Mapping Proof

`qwen_code_0_26_6_identity` freezes the identity document, the five
inventories, the four hop diffs, the reviewed safety-boundary,
feeding-file, unified-loop bound, and autofix tool sources, and the SLSA
source-to-shipped ledger. It checks the
production claim through `0.26.6`, rejects the unpublished interior points
and the preview and nightly tags, and leaves a synthetic `0.26.7`
`UnverifiedNewer`. It cross-checks the recorded `0.26.2` artifact identity
against the frozen `qwen-code-0.26.2` fixture and proves
`dashscope-source-config.ts` is hash-frozen and byte-identical at all five
versions. It also mutation-checks the unified-loop opt-in, cancel,
permission, and budget snippets inside the frozen core `config.ts`,
`nonInteractiveCli.ts`, and `settingsSchema.ts` snapshots, and the
`autofix-pr.ts` command set plus the `0.26.5`+ in-place always-attended
refusal, at every recorded version. `current_qwen_headless` runs the `0.26.6` claim on a
deterministic fake process and keeps the selected argv, delegated
environment reference, and reserved SSH-path rejection before process
start. No provider prompt, live catalogue, credential use, installation,
host update, or artifact execution occurred.

The fixture snapshots are evidence only. The downloaded tarballs were not
added to the repository.
