# Research 480: Claude Agent SDK 0.3.322 identity

Status: evidence only. `claude-agent.sdk` stays qualified through `0.3.317`
with native `2.1.317`. No package or native claim moves. Every published hop
above the accepted pair is identified and classified here. None of them can
qualify yet: each changes the native binary, and the selected native behavior
is unproven.

Route: `claude-agent.sdk`
Axes: `claude-agent.sdk.package` (`0.3.284..=0.3.317`) and
`claude-agent.sdk.native` (`2.1.284..=2.1.317`), point-for-point coupled.
Node `22.23.2..=22.23.3`, wire and sidecar source stay separate.
Not a Claude Code, ACP, or shared-schema claim.

## Decision

Hold. Keep the package and native claims, the `0.3.293`/`2.1.293` default,
the finite 31-pair map, and the `0.3.259`/`2.1.259` registered-tool live point
exactly as they are.

Reason: the official npm `latest` is `0.3.322` with native `2.1.322`. The
published hops above the accepted pair are `0.3.318`, `0.3.319`, `0.3.321`,
and `0.3.322`. `0.3.320` is absent from npm. For each hop, the bundled native
binary differs from `2.1.317` and from every earlier hop. Identity manifests,
string markers, and wrapper similarity do not prove changed opaque native
control flow. The CLI route stopped on the same gap at `2.1.322`
(Research 478), and CLI review does not qualify this SDK/native pair.

Qualification needs a native behavior proof that this task is not authorized
to run. The follow-up requirement is below.

## Official identity

Probed on 2026-11-12 from the registry packument
(`https://registry.npmjs.org/@anthropic-ai/claude-agent-sdk`).
`dist-tags.latest` and `next` are both `0.3.322`.

| SDK | Native (manifest) | npm publish (UTC) | Tarball SHA-256 | Native binding |
| --- | --- | --- | --- | --- |
| `0.3.317` (accepted) | `2.1.317` | `2026-11-03T00:02:04Z` | `e91e5a4347e0cd8c51c5d2e8dac005769c2d1ddcb53102f14b5506646485112f` | darwin-arm64 and linux-x64 match CLI `2.1.317` |
| `0.3.318` | `2.1.318` | `2026-11-03T21:55:44Z` | `7a79b4aa90dec557caf4638a8403e21649c371c4f6d0113cdd44a6da6b27836e` | manifest only |
| `0.3.319` | `2.1.319` | `2026-11-06T00:59:53Z` | `6bfb9ccb6a5d15686f5c7dd12d7a7da6d7b486ca6e1331ac8a7ab98a2d302001` | darwin-arm64 and linux-x64 match CLI `2.1.319` |
| `0.3.321` | `2.1.321` | `2026-11-09T19:03:36Z` | `c8685dc9fbe245f90189e14acd5a3b74773e2027bdaff95b95b729fbb53ef6f5` | darwin-arm64 and linux-x64 match CLI `2.1.321` (Research 478 row) |
| `0.3.322` | `2.1.322` | `2026-11-10T01:57:38Z` | `5db6881a4da75539d8b090f0a4b791043f9cbdbae439e5f9cf27356b28c59a93` | darwin-arm64 and linux-x64 match CLI `2.1.322` |

Each tarball matched the registry SHA-1 and SHA-512 integrity before
inspection. Each wrapper pins `claudeCodeVersion` and all eight optional
platform packages to its exact version. Each shipped `manifest.json` names the
native version, a private native commit, a build date, and eight platform
checksums and sizes. The private manifest commit differs from the public CLI
GitHub commit, so the binding is by binary digest, not by commit.

The complete file ledger is in the
[`0.3.322` fixture](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.322/README.md).
Every hop has 21 files. The `0.3.317` per-file SHA-256 values reproduce the
retained Research 472 ledger exactly. The retained tree digest uses a rule the
new files do not reproduce, so the per-file hashes are the binding. The new
fixture does not restate that tree digest.

## Method

- Downloaded the four new tarballs from the registry URLs and re-fetched
  `0.3.317` to recheck its digest. Nothing was
  installed or executed. The platform native packages were not downloaded.
  The native binding uses the shipped manifests and, for darwin-arm64 and
  linux-x64, the digests already frozen in the CLI fixtures.
- Extracted each wrapper into a fresh scratch directory and hashed every file.
- Compared `sdk.mjs` at each adjacent hop with three checks:
  - anchor windows around selected surfaces, with identifiers masked and
    string literals masked;
  - the exact unmasked literal ledger;
  - direct reads of the selected code.
- Checked the import closure of `sdk.mjs` at every hop. Static imports are
  Node builtins only. No dynamic `import(` appears. No `core-*.mjs` chunk is
  reached. The chunk filenames rotate at every hop, and they are tree-bound
  only.

## Selected-surface classification

The sidecar opens the root export with an async-iterable prompt, explicit
`settingSources: []`, `plugins: []`, `skills: []`, `agents: {}`, `hooks: {}`,
a `canUseTool` callback, a custom `spawnClaudeCodeProcess`, and the native
`pathToClaudeCodeExecutable`. It calls `initializationResult`,
`supportedModels`, `accountInfo`, `mcpServerStatus`, `setPermissionMode`,
`setModel`, and `interrupt`.

| Hop | Selected SDK surface | Native | Status |
| --- | --- | --- | --- |
| `0.3.318` | The argv builders, spawn, initialize, `canUseTool`, `supportedModels`, and the Query method windows are structurally identical to `0.3.317`. The literal ledger adds nothing selected. | Changed; no CLI digest in repo | Native proof required |
| `0.3.319` | The argv builders match `0.3.318` apart from minifier renames. A new managed-policy projection (`Ex`) carries `permissions.denyInsteadOfPrompting` as a restrictive key. `canUseTool` and `supportedModels` windows change through insertions and a private-field initialiser. | Changed. The setting denies a prompt-requiring call without calling `canUseTool`. Whether managed policy can set it while `settingSources` is empty is unverified; the native has not been run. | Permission-mediation change on this route. Needs a ruling and native proof. |
| `0.3.321` | The `--await-initialize` branch is gated by `Y5`, which requires both `pathToClaudeCodeExecutable` and `spawnClaudeCodeProcess` to be unset. The sidecar sets both, so the flag is not emitted. The `sdk-url` predicate is unchanged for this route. `writePrompt` now goes through `afterInitialize` and `writeStreamMessage`; that is the string-prompt path, which this sidecar does not use. `streamInput`, `waitForRunEnd`, `armRunEndCeiling`, and `endRun` windows are identical. `promptHold` and `initializeWritten` windows change and are not fully classified. The shared `Ki` key sanitizer gains a telemetry marker, and its callers are not classified. | Changed | Residual unclassified SDK changes plus native proof required |
| `0.3.322` | Masked whole-file structure is identical to `0.3.321`. Literal changes are the version stamp and one description string. | Changed; the CLI `2.1.322` native proof is the open stop in Research 478 | SDK unchanged from `0.3.321`; native proof required |

Declarations (`sdk.d.ts`, `sdk-tools.d.ts`) change at each hop. Those files
are not runtime evidence. They are classified in the fixture only where a
selected setting could become reachable.

## Claims and boundaries

- The package and native claims stay baseline `0.3.284`, latest qualified
  `0.3.317`/`2.1.317`, `QualifiedOnly`, behavior revision
  `claude-agent.sdk-v1`, and the unpublished `0.3.297`, `0.3.298`, `0.3.313`
  exclusions. The new hops are not added to the maintained pair map.
- A test pins the claims to refuse `0.3.318`, `0.3.319`, `0.3.321`,
  `0.3.322`, and `0.3.323` on the package axis, and `2.1.318`, `2.1.319`,
  `2.1.321`, `2.1.322`, and `2.1.323` on the native axis.
- No pair is extended, so the default, the registered-tool live point, the
  Node segment, the wire, and the sidecar source tag are unchanged.
- No live provider turn, login, credential, install, host update, prompt,
  release, or direct push occurred. No vendor executable was run.

## Follow-up requirement

Do not extend any pair above `0.3.317` on identity evidence alone. The
follow-up task needs one of these, decided by the planner:

1. Native behavior proof for the selected SDK paths on the Darwin arm64 and
   Linux x64 binaries named above (`2.1.318`, `2.1.319`, `2.1.321`, and
   `2.1.322`). It must cover the init handshake, the streamed input and
   result frames, the `canUseTool` permission path with and without managed
   `denyInsteadOfPrompting`, `--await-initialize` absence on this route, usage
   and failure projection, interrupt, and joined close. Either a hermetic run
   of the native with an isolated home and a local fake endpoint (needs
   operator authority to execute vendor binaries) or a static control-flow
   method that the CLI Research 478 review has not yet accepted.
2. A ruling on `denyInsteadOfPrompting` for the SDK route. Options: preserve
   the managed-policy denial under a distinct SDK behavior revision, or detect
   the managed setting and refuse the pair. This is a product decision, not a
   proof.
3. A closure of the `0.3.321` residuals: `promptHold`, `initializeWritten`,
   and the `Ki` telemetry callers.

Until those close, `0.3.322` stays official latest that the claims do not
qualify. The claims refuse it on both axes, and this record is its evidence.
Nothing here is an `UnverifiedNewer` posture change: the claims are
`QualifiedOnly` through `0.3.317`, and the refusal is by claim arithmetic, not
by a separate host or adapter posture.

## Sources

- [npm package metadata](https://registry.npmjs.org/@anthropic-ai/claude-agent-sdk)
- [Frozen `0.3.322` fixture](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.322/README.md)
- [Research 472](./472-claude-agent-sdk-0-3-317-qualification.md) (accepted base)
- [Research 478](./478-claude-code-response-only-2-1-322-identity.md) (CLI native stop)
- [Research 477](./477-claude-code-headless-and-response-only-2-1-319-identity.md) (CLI `2.1.319` and `denyInsteadOfPrompting`)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
