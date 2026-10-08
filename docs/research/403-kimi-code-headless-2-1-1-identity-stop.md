# 403 Kimi Code Headless 2.1.1 Identity and Security Stop

Status: operator ruling required; production claim unchanged  
Owner: version-currentness task 131  
Date: 2026-10-08  
Authority: Tom's 2026-10-07 approval for the remaining one-family exact-pin,
major-line, and schema/scheme-reset scope; Contract 029; the
[version-currentness checkpoint](../knowledge/operations/version-currentness-checkpoint.md#pre-v052-sweep-authority)

## Question

Can the selected `kimi-code.headless` v2 `stream-json` surface extend from its
qualified `0.43.0` ceiling through the official stable `2.1.1` major line while
preserving the legacy v1 segment and all other family claims?

The published npm chain after the current ceiling is `0.43.1`, `2.0.0`,
`2.0.1`, `2.0.2`, `2.1.0`, and `2.1.1`. The current npm `latest` and GitHub
latest stable both resolve to `2.1.1`; npm published it at
`2026-09-24T07:27:15.480Z` and GitHub published its release at
`2026-09-24T07:24:08Z`.

## Method and identity

Re-probed npm and GitHub latest before selecting artifacts. Downloaded each npm
tarball without executing it, recomputed the registry `integrity` and
`shasum`, and calculated its SHA-256. The package is `@moonshot-ai/kimi-code`,
the runtime entry is `dist/main.mjs`, and all seven points declare
`node >=22.19.0`. The npm `package.json` has no `gitHead`; source tags are
recorded independently and are not treated as proof of package-source parity.

Each source tag resolves to the exact tag object, commit, and tree below. The
fixture also records all npm and GitHub release identity fields, every
published GitHub asset digest, each complete npm file tree, each complete
tagged-source file tree, and the per-hop changed path sets.

| Version | npm published | npm tarball SHA-256 | `dist/main.mjs` SHA-256 | Git tag object / commit / tree |
| --- | --- | --- | --- | --- |
| `0.43.0` | 2026-09-14T12:10:41.073Z | `225bc17f06243edf6bcf0fc82bbe8838cab1cd426eb8e467e9ebab93d53ace90` | `5b300a573d306f879361eb7a2baec3a19448ea94eadca28001408b19c42eeb69` | `eb83293155ef86d7e5fccf8f764e9d698b619988` / `ffa94fae854dedf594919acbea280d98cbe8e14e` / `cfad0ffd418c396bae69a96322050738aa49dcda` |
| `0.43.1` | 2026-09-15T06:57:35.600Z | `2ac671a704bc4f4d6f0cd1ffcec76ab10f185c376aa7edd6b0450f210563603c` | `fb44806fa7e5773749131b841bfd23d96a9b07c97bad101872b01c11aabecd76` | `a3c66ba1b019c37e4069c5611266669d0438eb62` / `75ac010bcb2050338444455de8328492d152c919` / `b102aab935f2cc837ab78b0aae37d4d02aca6e1b` |
| `2.0.0` | 2026-09-17T05:28:50.465Z | `d1450598a1844d9bc204f07ae9fd3bb371e7df1ec841cf7c9e86bde7c09d3cd1` | `425690c99c74f6270218d486e1b4aa799e15fe5a4853dd87a8c19a7b45e55ac5` | `834bdd50b10b53b90d2ce5d72f5864a8a2309492` / `1b89e4b039f052d10f258464413b2047acca12ba` / `e0455fd76cf5dc3c0291661105672ab1876c90ab` |
| `2.0.1` | 2026-09-18T13:21:56.632Z | `5b0dfb03a3e5f79b0030888c6b69679459afe8f1dc1e4175d921fe5701dd3b3e` | `1e561c30f07b4f989cd604f735ef9f30806d610f19cb855a099d150c7eccf495` | `0a3f49c1a1e9028c16b2e408a2e82ad4b4427805` / `caf7d4e2fef06967280b325da06e44a4b0516eba` / `3773a94fcd3a1890ffa9c48ccc31297625d85c71` |
| `2.0.2` | 2026-09-19T12:16:26.024Z | `432cd0b0ed4184d01c29c5ef21a88303b81d3539f3a771ec64af6cbfa4aa8a77` | `56d12cc74e54b0b82a2e2d8bdbbb9d6a5c9b27a9e5c9a09e0d5662b67fce1a89` | `c98339a1867e0843d192b8f7454d7ae3dcd5d8d8` / `9d07f634be94ebeb1deba2f55d247807cf729315` / `6ccfdc69ac48d00137a2e860b232fd25c9e0f74b` |
| `2.1.0` | 2026-09-23T12:39:56.279Z | `25a081b7783806226434aab6c40462192d3f9e362391cba7b068af02d8b5fd71` | `6f801a114b5a5d708d9fec1ea0002ac42ec19841c088a96aecd7e6117b8b1d59` | `93a83aafacf91cc595a7b66f0ecb637f23859f77` / `52437299ff78de3d0aff7f38f054e5eb20c512e5` / `f75b5366d0a9b130ff9ff593ab202b990a4ea65e` |
| `2.1.1` | 2026-09-24T07:27:15.480Z | `6690a29d7b5e14812754dd100136b7f4ee2577add8895f00fe27025b4412049f` | `2563a338c94868a34e1bfd2e5579067d5a472acf4cb0bf350a3bfe1092bbe59d` | `a00639d0654d2c91c6f0c1388267b62ad51f8b9c` / `f67e6398fb3210ad8ace970e2dfd5bcc984ed61f` / `3269cb11ea507083ee03b975d4e3edd795a12d9f` |

The installed host remains Kimi `0.34.0`, executable SHA-256
`9f4337e10da47843f6b550474012a53ba8b30dd665f83b176a5cd479c5f7e859`, size
`176894272`. It was observed through an isolated `--version` check and digest
only. No provider prompt, authentication, live session, host update, or
artifact execution occurred.

## Selected headless surface

The route remains `kimi --model <model> --prompt <content> --output-format
stream-json`. The v2 default engine is agent-core-v2 `runV2Print`, which emits
the `system.version` preamble. The v1 segment remains `0.29.0..=0.32.0`; the
v2 segment currently remains `0.33.0..=0.43.0` under
`kimi.headless.stream-json.v2`. The selected writers, output-format options,
retry payload, and event-dispatch grammar match across all seven npm bundle
points. The ten dispatched event labels and the
`system.version`, `turn.step.retrying`, and `session.resume_hint` meta records
are frozen in `protocol.json` and `bundle-oracles.json`.

## Published-hop classification

| Hop | Selected-path result |
| --- | --- |
| `0.43.0→0.43.1` | No selected CLI, loop, tool, or permission source path changes. The npm tree changes web assets, native prebuilds, package metadata, and the bundle; selected writers and dispatcher retain their bundle digests. |
| `0.43.1→2.0.0` | The selected CLI and stream writers stay stable. Agent-loop changes carry internal prompt metadata and cancellation reasons; subagent resume and media-upload helpers change. The added install command is outside headless. |
| `2.0.0→2.0.1` | `run-v2-print.ts` waits for an in-flight cron-steered turn before treating the background schedule as quiescent. Event writers and dispatcher stay stable. AskUserQuestion denial text changes as provider-generated tool output data. The dangerous-command yolo change is not selected: the print runner sets `nonInteractive: true`, which omits that policy. |
| `2.0.1→2.0.2` | Agent-core-v2 suppresses a duplicate steering event for the active message and appends a seed record to an existing session journal. These are private loop/history changes. |
| `2.0.2→2.1.0` | **Security/authority stop.** Kimi adds `realpath-access.ts` and wires it into built-in `Read`, `Write`, `Edit`, `Glob`, `Grep`, and `ReadMediaFile` tools. The checks block symlink escapes, sensitive targets, dangling links, and writes through symlink aliases to `.kimi-code/local.toml`. The runner invokes these tools through the selected v2 agent core, so this changes the provider's effective filesystem access. |
| `2.1.0→2.1.1` | The helper and six tool guards are removed, restoring their `2.0.2` source blobs. Static inspection of npm `dist/main.mjs` finds the realpath guard symbols only in `2.1.0`. The later reversal does not erase the authority-changing `2.1.0` hop from the published chain. |

The `2.1.0` change in `git-cwd-write-approve.ts` separately declines automatic
approval for project-local config writes. That condition is preempted in the
selected route: `runV2Print` sets permission mode to `auto`,
`AgentPermissionPolicyService` evaluates the unchanged `auto-mode-approve`
before `git-cwd-write-approve`, and the first policy returns approval. This
does not remove the independent realpath checks in tool implementations.

The package inventory has 543, 545, and 541 files at `0.43.0`, `0.43.1`, and
each `2.x` point respectively. The full npm and tagged-source per-hop trees,
all changed paths, selected source hashes, bundle oracles, and static policy
marker counts are in
[`tests/fixtures/kimi-code-2.1.1/`](../../crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-2.1.1/).

## Outcome and next step

No production claim changed. Headless stays qualified at `0.43.0`; `2.1.1`
remains visible as `UnverifiedNewer` under the existing `AllowUnverified`
posture. The exact `2.1.0` filesystem-authority change needs an operator ruling
before this family can qualify the major line. The ruling must say whether to
keep the ceiling while an adaptation is prepared, or qualify `2.1.1` with
`2.1.0` left as an unsupported gap. This is a currentness stop with a next
adaptation step, not a terminal family outcome.

No guide, feature matrix, ACP or local-server claim, changelog, public API,
behavior revision, or exclusion changed in this identity-only stop.

## Sources

- [npm package `@moonshot-ai/kimi-code`](https://www.npmjs.com/package/@moonshot-ai/kimi-code)
- [GitHub releases](https://github.com/MoonshotAI/kimi-code/releases)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
- [Research 369 all-route checkpoint](./369-all-route-version-currentness-checkpoint.md)
- [Research 325 Kimi Code 0.43.0 installed identity](./325-kimi-code-0-43-0-installed-identity.md)
