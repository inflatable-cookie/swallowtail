# Research 465: Qwen Code 0.26.2 Identity And Headless Qualification

Observed 2026-10-28. This record qualifies only `qwen.headless` on the npm
package `@qwen-code/qwen-code`. It follows Research 454's `0.26.1` ceiling
and does not borrow evidence from a sibling Qwen route.

The 2026-10-28 probe of `npm view @qwen-code/qwen-code dist-tags --json`
returned `latest=0.26.2`, `preview=0.26.3-preview.0`, and
`nightly=0.26.1-nightly.20261026.34769f9d41`. ACP registry agent `qwen-code`
also reports `0.26.2`. That registry row is discovery metadata, not a claim.
At that probe no stable newer than `0.26.2` was published. Preview, nightly, and the
published prereleases `0.26.2-preview.0`, `0.26.2-preview.1`, and
`0.26.3-preview.0` are not stable releases. There is no interior stable
between `0.26.1` and `0.26.2`.

## Decision

Raise the `qwen-code.package` AllowUnverified ceiling from `0.26.1` to
`0.26.2`. Keep the existing `qwen-code.headless.package-window-2` claim,
`0.19.11` baseline, segment statuses, and behavior revision
`qwen-code.headless.v0.21.15-reasoning-control`. The maintained segment is
`0.22.0..=0.26.2`. The older deprecated segments remain `0.19.11..=0.20.1`
and `0.21.0..=0.21.14`; exact `0.21.15` remains maintained. Exclude
unpublished `0.20.2`, `0.21.16`, `0.22.4`, `0.23.5`, and `0.25.4`. Plan
remains exact through `0.22.3`. Reasoning and budget selections remain exact
at `0.21.15`. A later probe, after the identity commit, found npm latest
`0.26.3`; it stays `UnverifiedNewer`.

The hop is a compatible extension. Selected CLI argv, stream-json result
subtypes, permission flow, approval mode, and the SSH workspace guard are
unchanged. No new public operation is mapped.

The Research 454 read-only working-resource guard stays.
`ssh-workspace-store.ts` and its test are byte-identical at `0.26.1` and
`0.26.2` (`737d3fa0e3ecac680f8ec77f1896dbf4c0f1cf1aa3c56af01f566f795a8ded55`,
4,449 bytes; `d9328a425831f27015cca278c235ed293bace5db735270541732fd2a8e669a85`,
4,585 bytes). `approval-mode-value.ts` is byte-identical on the same pair
(`b70d51f6ffdddfc066e1dc75a04601a5d6dbcf71e8920a2f3f46eea25f3c5467`, 1,170
bytes). No credential state is read. No release or tag authority follows.

## Official Channel And Artifact Identity

The selected channel is npm `dist-tags.latest` for
[`@qwen-code/qwen-code`](https://www.npmjs.com/package/%40qwen-code/qwen-code).
The GitHub TypeScript SDK is a separate product and does not identify this
CLI package. GitHub tag `v0.26.2` identifies the source snapshot. npm
`gitHead` equals that tag commit and differs from the SLSA checkout.

`0.26.2` is bound by npm SLSA provenance v1. The attestation subject SHA-512
equals the retained tarball SHA-512. `resolvedDependencies.gitCommit` is the
GitHub Actions checkout of `.github/workflows/release.yml` on
`refs/heads/main`, invocation
`https://github.com/QwenLM/qwen-code/actions/runs/40510643634/attempts/2`.
That SLSA commit differs from the version tag. Selected mapped TypeScript
files and the reviewed safety-boundary files at the SLSA commit are
byte-identical to the frozen tag snapshots. The tag-to-SLSA tree delta is
26 workspace `package.json` / `qwen-extension.json` version bumps and no
`/src/` path. The shipped package contains no `.map` files.

The regenerated `0.26.1` inventory matches the frozen Research 454 digest
`606ae19f4569f70057b4d18e259ae465c534a038cf55f3f2d025309e821eeda4`. The
`0.26.1` tarball SHA-256
`9198d276e8583b789ee42a9c38b521d57818b94682d155b2b47741534c90fb73` and source
commit `b475278c4a300acebe6cfc990a9e1b1d18819bfc` match that record. Research
454 remains the `0.26.1` identity record. Exact `dist.integrity`, file count,
unpacked size, inventory digest, and source identity are frozen in
`crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.26.2/identity.json`.

| npm version | Published (UTC) | Files | npm shasum | Tarball SHA-256 | Official source tag commit |
| --- | --- | ---: | --- | --- | --- |
| `0.26.2` | 2026-10-27 15:41:46.241 | 1,378 | `308409e71b1e57a1e27c709023f16a4ed73a5211` | `01020fa2c701a1eb385c2f5405686bf25c5aa0b8e5cd8893e075cf0ad4d77e8f` | [`v0.26.2`](https://github.com/QwenLM/qwen-code/tree/v0.26.2) `402911146f6b2a80cb97423309faf1956f6defe4` |

| npm version | SLSA gitCommit | Tarball SHA-512 (attestation subject) | Tag-to-SLSA selected-source delta |
| --- | --- | --- | --- |
| `0.26.2` | `dee6ace9d8f84d98084abf292f2b0a5ffd3dfa22` | `d242efc08c7f0a653e6d4dade57e180f8dd1c4d09cbf539009a64410c8b86483124ff6ce4859a133e57fea889949611b081fee900e32fc3495c2e9fb03cff834` | none |

The SLSA payload SHA-256 is
`f513ac4c511fad843bece786cda81e5ac2370a9d00081000135b9f18b19f0f67`.

## Complete Published-Hop Ledger

`dist-inventory-0.26.1.json` and `dist-inventory-0.26.2.json` list the
SHA-256 of every file extracted from those exact official npm tarballs.
`dist-diff-0.26.1-to-0.26.2.json` records the complete added, removed,
changed, and byte-identical path sets. The identity test recomputes those
path sets from the inventories. The source snapshots preserve the eleven
mapped files plus the reviewed safety-boundary files at both versions.

Reviewed files other than `loopDetectionService.ts` are byte-identical.
`loopDetectionService.ts` moves from
`35ef3dc21c371547ae64c76d36bfdedb29196a30579abdff049db036a7d15569` (82,674
bytes) to
`6f306ca6e1c15260ff76988aedc8a2752f429add0d90241be32d1e8d388f29e3` (82,986
bytes). `loop-protection.ts`
(`2d42af7d23675e422fea8285e12bccfb0b3a9d291fef7d2bdb83535d4fb9fc20`, 4,108
bytes), `sdk-human-input.ts`
(`defb6d31cd058cad88ce51c2d16379efd3bf06232ad67e825c9fc402eb2de688`, 1,331
bytes), and `interactive-mode.ts`
(`9581159b04035fe8243519f328308fb75949610d1860d6dbcc6ffc14c9ec3597`, 375
bytes) stay put.

| Hop | Added | Removed | Changed | Identical | Changed selected source files |
| --- | ---: | ---: | ---: | ---: | --- |
| `0.26.1 → 0.26.2` | 466 | 460 | 25 | 887 | `config.ts`, `dashscope.ts`, `session.ts` |

`approval-mode.ts`, `exitPlanMode.ts`, `permissionFlow.ts`,
`plan-mode-shell-policy.ts`, `reasoning-effort.ts`, `types.ts`,
`systemController.ts`, and `top-level-options.ts` stay byte-identical.
Their frozen hashes are in `selected-source-identities.json`.

## Selected-Behavior Review

- **`config.ts`:** optional `WEB_SEARCH_ALLOWED_HOSTS` and
  `settings.webSearch.allowedHosts` are empty by default, so web search
  host matching is unchanged unless that setting is present. This route
  excludes `web_fetch` and does not set the search host list.
  `experimental.unifiedAgentLoop` defaults false. The settings schema also
  documents `QWEN_CODE_ENABLE_UNIFIED_AGENT_LOOP=1` as an opt-in. This route
  sets neither, but the delegated environment or existing settings can opt
  in. The `sessionIdGenerated` comment now covers hosted-store
  IDs with chat recording disabled. This route does not pass `--session-id`.
  The `--safe-mode` path still ignores `--core-tools` and still denies
  non-interactive shell, monitor, edit, and write-file tools unless
  explicitly allowed.
- **`session.ts`:** direct mode enqueues the first user message before
  `ensureConfigInitialized`, so a message that arrives during init cannot
  overtake it. `processUserMessage` still awaits initialization before it
  touches config. A single stdin prompt keeps its result order.
  `ModSession` construction is unchanged. This route does not enable mods
  and always passes `--safe-mode`.
- **`dashscope.ts`:** `QWENCLOUD_MAAS_HOSTS` moves to
  `packages/core/src/core/dashscope-source-config.ts` and is re-exported.
  The hosts remain `maas.qianwenaiapi.com` and `maas.qwencloudapi.com`.
  `X-DashScope-Source-Config` is added on the DashScope client default
  headers for those hosts, including the `token-plan.` prefix, and is
  stripped from the session-aware fetch custom headers. Auth, retries, and
  the child environment are unchanged. `reasoning-overrides.ts` uses the
  same host list. Reasoning selection stays exact at `0.21.15`.
- **`loopDetectionService.ts`:** `toolCallPatterns` defaults true. The
  default checks stay. Heuristic detectors stay off unless configured.
  This route does not set `model.loopProtection`.
- **Unmapped core config:** `isUnifiedAgentLoopEnabled` is false unless the
  experimental setting or the env opt-in is set. The non-interactive loop
  calls the shared agent loop only when that flag is true. The false branch
  keeps the existing loop. `types.ts` is byte-identical, so stream-json
  result subtypes are unchanged. Retry stderr may append
  `[kind=..., reason=..., transportCode=..., statusCode=...]`. That text is
  not a mapped result subtype. The new `AutofixPrTool` is registered in the
  general tool registry even under safe mode, which ignores the core-tools
  allowlist. The frozen exclude list does not name `autofix_pr`, so the
  model can reach it independently of the similarly named CLI command.
  Its [implementation](https://github.com/QwenLM/qwen-code/blob/402911146f6b2a80cb97423309faf1956f6defe4/packages/core/src/tools/autofix-pr.ts)
  permits `status` to launch the status helper, which reads watch state, and
  `attended` to revoke an unattended grant without launching a helper. Acting
  commands default to ask; in the selected headless text-input route an ask
  is automatically denied. Plan mode also refuses acting helpers. Existing
  permission rules can change decisions, so this is not a host containment
  claim. Keep the frozen exclusion set: these bounded effects do not require
  a new adapter operation or a changed argv contract. The tool is not mapped
  as a Swallowtail operation. Managed-session and web
  shell changes are a different product surface. The published breaking
  note for managed-agent Web Shell cwd submissions is outside this route.

The complete artifact diff includes rebuilt distribution chunks, bundled
assets, documentation, translations, and provider-internal modules. Selected
interface source was reviewed from the tagged trees. No unrelated provider
operation, ACP interface, shell feature, remote host, or new permission is
qualified.

## Deterministic Mapping Proof

`qwen_code_0_26_2_identity` freezes the identity document, both inventories,
the hop diff, the reviewed safety-boundary sources, and the SLSA
source-to-shipped ledger. It checks the production claim through `0.26.2`,
rejects unpublished `0.22.4`, `0.23.5`, and `0.25.4` and the preview and
nightly tags, and leaves `0.26.3` `UnverifiedNewer`. `current_qwen_headless`
runs the `0.26.2` claim on a deterministic fake process and keeps the
selected argv, delegated environment reference, and reserved SSH-path
rejection before process start. No provider prompt, live catalogue,
credential use, installation, host update, or artifact execution occurred.

The fixture snapshots are evidence only. The downloaded tarballs were not
added to the repository.
