# 424 Antigravity Headless 1.3.1 Currentness Stop

Status: finite task result; no qualification extension. The approved
provider-free child-status and soft-denial adaptations are present and
documented. The exact claim, its segments, hole, behavior revision, and retry
pin remain unchanged. Exact Mac ARM64 static mapping now locates the newer
resource and retry code on selected paths, but does not close the RPC,
permission-callback, resource-alias, request-budget, retry-class, or
approved-environment auth edges listed below. Current official stable `1.3.2`
was published after the frozen identity through `1.3.1`; it remains
`UnverifiedNewer`. Windows sandbox runtime work is deferred to the
windows-compatibility lane and is not a gate or platform claim change here.
Contract 036 classifies the approved child-lifecycle correction as breaking if
shipped.

Owner: swallowtail#111 (`antigravity.headless`)
Date: 2026-10-09
Axis: `antigravity-cli.release`
Package: `swallowtail-adapter-antigravity`

## Current state

The official GitHub stable channel was re-probed on 2026-10-08. At the frozen
artifact cutoff, its latest release was `1.3.1`, published
`2026-10-07T03:22:02Z`; the tag commit is
`968f1170bd0e002e9d0914730975bc8a2cc65861`. A fresh channel check on
2026-10-09 now finds `1.3.2`, released `2026-10-08T22:39:48Z` at short commit
`8c1310b`. This point postdates the frozen identity. No `1.3.2` artifact was
downloaded, inspected, or added to the hop ledger; it is `UnverifiedNewer`.
Its release notes mention plugin installation and a changed
`--dangerously-skip-permissions` effect in `/plan`. Those notes do not prove
the selected headless effect, so no behavior or permission conclusion is
transferred. The headless claim stays at exact `1.2.11`. Keep claim
`antigravity.headless.release-window-2`, baseline
`1.1.9`, deprecated segment `1.1.9..=1.1.17`, maintained exact point `1.2.11`,
interior hole `1.1.18..=1.2.10`, and the exact
`AGY_CLI_MODEL_API_MAX_RETRIES=0` pin. Catalogue evidence is not transferred.

This record adds route-specific hop classifications over the frozen release
identity and inspected archive trees in
[`tests/fixtures/antigravity-cli-1.3.1/`](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/).
For every hop, the official source comparison is one commit with only
`CHANGELOG.md` changed. The public asset manifest records eight assets per
release. Linux x64 and Mac ARM64 release archives were downloaded and their
published digests matched; each inspected archive contains exactly one regular
file, `antigravity`, with its digest and size recorded. Both `1.2.17` Windows
archives were also downloaded, matched to their published digests, and unpacked
offline. Each contains one `antigravity.exe`; its digest and PE architecture
are recorded in the route-specific evidence fixture. Static strings were
inspected; no binary was executed and Windows sandbox runtime behavior remains
unproven. Under the 2026-10-09 scope ruling, Windows runtime proof belongs to
the later windows-compatibility lane; no Windows claim or support boundary
changes in this task. Release notes are discovery evidence, not a substitute
for artifact identity. The unpublished `1.2.18` point inside the version
interval remains without a release or tag; it remains outside the claim.

| Hop | Headless classification | Selected-path evidence and remaining gate |
| --- | --- | --- |
| `1.2.11→1.2.12` | Approved-environment evidence gate | Quota behavior changes for `GEMINI_API_KEY` sessions. The prepared access profile has no credential reference, but `EnvironmentRef` does not establish whether that variable reaches the child. |
| `1.2.12→1.2.13` | Retry-control evidence gate | The model API now follows server retry delays and stops on delays over 30 seconds or daily/billing caps. Retry-pin behavior is proven only on exact `1.2.11`; keep `0` pinned and do not transfer its artifact proof. |
| `1.2.13→1.2.14` | Compatible selected schema extension | Startup now rejects unsupported schema input and non-object roots. The adapter supplies inline JSON with an object root. Keep the existing schema decoder and failure coverage. |
| `1.2.14→1.2.15` | Approved soft-denial route limit | A denied permission is respected without alternate command, script, or tool workarounds. The official guide documents a continuing run with exit `0` and an stderr notice. Run completion does not prove each requested tool executed; no structured denial projection is claimed. |
| `1.2.15→1.2.16` | Selected lifecycle extension; identity-only child status is `Unknown` | Headless print now waits for background commands; image generation appears as a subagent. The adapter drains stdout through EOF and waits for the process. The approved fallback preserves child identity without inferring completion. |
| `1.2.16→1.2.17` | Windows behavior deferred | Windows sandbox behavior changes on the selected `ProviderEnforced --sandbox` path. Exact Windows assets and PE digests remain inventoried, but runtime proof is deferred to the windows-compatibility lane; this task makes no Windows support claim. |
| `1.2.17→1.3.0` | Selected path/resource mapping found; effects remain unresolved | Research 401 maps project-path and custom-agent discovery into the Mac ARM64 selected headless entrypoint and resolves relative resource paths in the custom-agent loader. The value flow through the service boundary to a tool or permission decision remains unresolved; no capability or unchanged-behavior claim follows. |
| `1.3.0→1.3.1` | Identity-only child-status `Unknown`; selected resource and retry/auth edges remain | Research 401 maps `loadMDAgent`'s new `setDefaultAgentPath` call and relative-path resolution. It does not establish the downstream selected tool effect. The error note names `/agents` and the running-agent list; official stream docs describe child identity, log, and workspace fields but no child error/status field. Preserve identity as `Unknown`; API-key retry remains conditional on exact retry and approved-environment evidence. |

The 2026-10-08 official headless guide distinguishes three error scopes. A
failed tool step may carry `tool_info.error` with `type` and `message`; the
adapter maps that field to `ActivityStatus::Failed`. A failed whole run carries
`result.status` and `result.error`; the adapter maps it to
`TerminalStatus::ProviderFailed`. Neither field is a child status. The
documented `subagent_info.subagents` fields are `type_name`, `role`,
`conversation_id`, `log_uri`, and `workspace_uris`. Under the approved Contract
023 fallback, preserve those identities with `SubagentStatus::Unknown` when no
child status field is present. The 1.3.1 release note describes `Error:` only
in `/agents` and the running-agent list. `headless-run-error.jsonl` is
synthetic provider-free projection coverage combining documented child
identity and outer-run error shapes; it is not a captured child-error stream.

The machine-readable hop ledger records each tag, publication time, changed
source path set, runtime inventory pointer, classification, and exact remaining
proof. Tests assert its exact key and path sets against the frozen artifact
identity and tree corpus. The separate
[`headless-tool-error.jsonl`](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/headless-tool-error.jsonl)
fixture uses the documented `tool_info.error` object shape and tests that an
observed tool error maps to `ActivityStatus::Failed` without adding an approval
bypass. It is synthetic protocol coverage; it is not evidence that Antigravity
1.2.15 emits a denial event in stdout.

## Selected path mapping

The adapter builds `--print` plus `--output-format stream-json` in
`headless_command::arguments`; the provider-enforced profile adds `--sandbox`,
and the builder never passes `--dangerously-skip-permissions`. The pump parses
only stdout stream-JSON events. The decoder maps a non-null
`step_update.tool_info.error` to failed tool activity; it does not turn stderr
text into a tool failure. Child identity is decoded from `subagent_info`; when
no child status is present, the projection is `SubagentStatus::Unknown`. The
child status does not inherit the enclosing step or whole-run outcome.

Provider-free static inspection of the exact `1.2.15` Mac ARM64 artifact
(release archive SHA-256
`66f7e9e8750a506e8a2caaedaadf479f023820f712015c9c55cfb91a2891521b`, extracted
CLI SHA-256
`d15693410c904242c1c3423a579f60a81e018444bb51fbccc6505f988b62a91d`) places
`printmode.session.runTurn` on the selected print-mode path, with calls to
`store.(*Manager).HeadlessDenials` and
`printmode.headlessDenialNotice`. The denial collector checks
`HeadlessSoftDeniedTools`; denial recording is reached from
`UpdateSubagentSteps`, `addFromDiff`, and `handleToolConfirmation`. This ties
the release-note change to selected headless tool/permission handling, rather
than only to an interactive screen. It does not reveal a denial event in
selected stdout. The official headless guide says the soft-denial notice goes
to stderr, the run continues, and its exit code is zero. The approved route
limit therefore states that completion does not prove every requested tool
executed. Since the adapter parses stdout stream JSON and does not parse stderr
as events, the retained `headless-denial-1.2.15-evidence.json` records exact
artifact identity and documented/static facts while leaving a typed denial
projection unclaimed. `headless-tool-error.jsonl` covers the separately
documented tool-error shape with synthetic fixture values; it is not a
permission-denial capture. No approval bypass or alternate-tool workaround is
added.

Static inspection of the exact `1.3.1` Mac ARM64 artifact (archive SHA-256
`ef5e385b32afda4cf1612368bb4bf155d3f8f4c55d51488649f508baefe77c86`, extracted
CLI SHA-256
`88db8b4d21ece4999fa58e0b54cea77154e47b319ee178d086c446262317f3fa`) places
`PollPrintmode` through `steps.ExtractSubagentInfo` to
`streamJSONEmitter.EmitStepUpdate`. The inspected call/output shape exposed no
child error/status field, but this absence in disassembly does not prove a
successful child or the complete serialized schema. No exact selected error
stream is retained. Under the approved fallback, the adapter preserves an
identity-only child as `SubagentStatus::Unknown`; it does not claim the child
failed or completed. The documented whole-run `result.error` is not a child
status, and the TUI release note does not establish a matching stream field.

Contract 036 impact was assessed separately. The child projection changes the
public activity output from inferred `SubagentStatus::Completed` to
`SubagentStatus::Unknown` when the selected stream supplies identity but no
child outcome. Contract 036 classifies weakening lifecycle behavior as
breaking; this consumer-visible correction therefore requires a pre-1.0 minor
release if shipped. Tom approved the projection under Contract 023, but that
ruling does not grant a version bump, release, or tag. This task has no such
authority, so the release line remains for the operator/planner to place.

## Approved adaptations and remaining gates

The current official [headless guide](https://www.antigravity.google/docs/cli/headless/)
states that a permission-required tool that cannot obtain approval is
soft-denied: the run continues with exit code `0`, and a notice is printed to
stderr. The guide separately documents `tool_info.error` for a failed tool and
`result.error` for a failed run. It does not connect the soft-denial notice to
either stdout field. Swallowtail parses stdout for stream JSON and ignores
stderr. The 1.2.15 evidence fixture therefore records the exact documented and
static facts, but leaves the denial event shape unproven; it does not authorize
an invented version-specific stream fixture or a denial mapping claim. The
approved route limit is documented: a soft-denied requested tool may not
execute even when the outer run completes successfully. The adapter does not
claim structured denial projection, parse unspecified stderr, add an approval
exchange, or use another tool as a workaround.

The same guide documents `subagent_info.subagents` identity fields, not a
child failure status. The `1.3.1` release note describes errors in TUI lists;
static identity fields and disassembly do not prove a successful child. No
provider-free exact child-error stream was found in the released documentation
or retained corpus. The documented `result.error` belongs to the outer run,
not an individual subagent. Under the approved Contract 023 ruling, the
adapter retains child identity and projects `SubagentStatus::Unknown` when the
selected stream has no usable child status. Synthetic fixtures prove that
identity does not inherit the enclosing step or whole-run outcome; they do not
pretend to be a provider capture.

Research 401 replaces the earlier strings-only review with exact Mac ARM64
function and direct-call mappings for every frozen artifact from `1.2.11`
through `1.3.1`. It maps the selected `--print --output-format stream-json`
entrypoint, workspace and custom-agent resource branches, retry-override
parsing and configuration write, retry executors and classifiers, and
symbolic `GEMINI_API_KEY` references. The `1.3.0` and `1.3.1` path/config
changes are therefore located in code reachable from the selected entrypoint.
The mapping remains Mac ARM64 static evidence; it does not prove Linux
behavior or a compatible extension.

For the eight frozen hops, Research 401 maps the selected retry surface as
follows. These are exact function/call-inventory deltas, not a claim that a
later attempt limit or retryable-class set is understood:

| Hop | Mapped retry/auth finding |
| --- | --- |
| `1.2.11→1.2.12` | The retry classifier adds `isGeminiAPIHardQuotaError`; the release note describes API-key quota handling. |
| `1.2.12→1.2.13` | The classifier adds `isHardQuotaExhaustedMessage` and another retry-delay extraction call. |
| `1.2.13→1.2.14` | The classifier body digest changes; no new helper family is inferred. |
| `1.2.14→1.2.15` | The classifier adds `hasErrorInfoReason`; this does not establish the selected soft-denial stdout shape. |
| `1.2.15→1.2.16` | The mapped API retry executor moves from `gemini_coder` to `oneharness`; its mapped body changes. |
| `1.2.16→1.2.17` | Exact retry bodies are remapped; no additional retry class is inferred. Windows behavior is deferred. |
| `1.2.17→1.3.0` | Exact retry bodies are remapped; no additional retry class is inferred. Selected path and custom-agent resource behavior is mapped, with downstream effect unresolved. |
| `1.3.0→1.3.1` | The same mapped retry helper families remain; `loadMDAgent` newly calls `setDefaultAgentPath`. API-key retry remains an approved-environment gate. |

In every binary, the literal `AGY_CLI_MODEL_API_MAX_RETRIES` has one mapped
reference in `backend.applyModelAPIMaxRetriesOverride`; the function reads and
parses the value and stores it in backend retry configuration. All builds also
map the API-key functions `genai.defaultEnvVarProvider`,
`genai.getAPIKeyFromEnv`, and `ServerBackendConfig.chainedAuthOrDefault`.
Research 401 does not connect the config field to the request loop or prove
which auth result reaches each retry class, so the `0` → one request result
remains exact to Research 359's `1.2.11` artifact and fake proof.

The remaining non-Windows edges are finite and exact:

1. **Resource, tool, and permission flow:** `launchCLI` reaches project-path
   resolution and custom-agent discovery, and the mapped loader resolves
   configured resource paths against an agent-file base. However,
   `store.Manager.SendUserMessage` is not connected through the backend RPC
   dispatch to its server implementation; `newSession` is not connected to
   `Manager.GetDefinedAgentByName`; and resolved resource values are not
   followed into a specific tool or permission decision. The call to
   `permissionManager.EnsurePermissions` reaches its callback indirectly, so
   the selected result is not established. To close this edge, a reviewed
   interprocedural reconstruction must follow those exact functions and values
   through the RPC and callback boundaries for each affected frozen Mac
   artifact. Existing fake-process tests cover the adapter command and its
   projections, not the vendor's internal decisions.
2. **Resource aliases:** from `1.2.16`, workspace roots are passed through
   `EvalSymlinks`; relative config entries are joined to their containing base
   and lexically cleaned; a `Path.Join` call is present in `resolveDir`.
   Research 401 does not establish symlink resolution for each leaf or whether
   two aliased paths resolve to one resource. Exact leaf-value/call-flow
   analysis is needed before claiming alias behavior.
3. **Retry pin and retry classes:** each inspected build reads
   `AGY_CLI_MODEL_API_MAX_RETRIES`, parses base-10 unsigned input and writes a
   valid value, including zero, into backend retry configuration. That does
   not show the field reaching the version-specific request loop or that zero
   means one attempt on versions after `1.2.11`. The executor/classifier
   functions are identified, but the per-class predicate, delay selection,
   attempt budget, and effective floor/cap remain unresolved on the later
   artifacts. Close this only with exact-artifact field and control-flow proof;
   Research 359's fake and request-count proof remains bound to `1.2.11`.
4. **Approved-environment auth:** the API-key provider and chained-auth
   branches are identified, but the mapping does not establish which
   present/absent auth result reaches each retry class. The selected
   `EnvironmentRef` is opaque and proves neither the child key set nor that
   `GEMINI_API_KEY` is absent. Any future absence assertion needs a sanitized
   attestation from the approved-environment owner that exposes no secret
   value; this task creates no new environment boundary.
5. **Platform and latest movement:** Linux x64 selected control flow has not
   been mapped, so the Mac findings do not transfer. Windows sandbox runtime
   work is explicitly deferred to the later windows-compatibility lane, with
   no support narrowing in this task. Stable `1.3.2` postdates the frozen
   identity and remains `UnverifiedNewer`; its release notes alone do not
   qualify any selected behavior.

## Finite follow-up

This task records the completed provider-free adaptation and the best exact
static evidence now available; it does not raise the headless claim. The
prepared follow-up lead is
`eca55612-015c-46ed-a38b-52a85b30f15e`.
The next evidence work is limited to the listed RPC/resource/permission
value-flow and alias edges, exact newer retry-budget/classifier flow, and an
approved-environment owner attestation about whether the key is absent. Use
the already frozen Mac artifacts and static mapping as inputs; do not repeat
their downloads, string scans, or completed analyzer proof. If reviewed static
analysis cannot resolve an edge, the follow-up must request explicit authority
for the precise provider-free runtime method before any vendor artifact is
executed. No credential access, live provider work, install, host mutation,
new environment rule, or support narrowing is authorized here. Linux remains
unproved, Windows is deferred, and `1.3.2` is unverified.

Preserve the existing exact claim and all qualified points while that follow-up
proceeds. The approved soft-denial route limit and identity-only `Unknown`
child projection remain private behavior adaptations with no new operation or
approval bypass.

If the child-lifecycle correction ships, Contract 036 requires a pre-1.0 minor
release. This task does not set a package version or authorize a release; the
planner must place it on an appropriately versioned source release. Preserve
the earlier `1.2.11` segment and the `1.1.18..=1.2.10` hole. Do not infer live
provider honoring from static proof.

No provider prompt, live turn, credential access, CLI version command,
installation, host update, claim change, matrix change, release change, tag,
or publication occurred. The integration guide and Unreleased changelog note
were updated to point to this finite evidence result and the post-freeze
stable-channel movement.

## Sources

- [Official Antigravity CLI stable release `1.3.1`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.3.1)
- [Official Antigravity CLI stable release `1.3.2`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.3.2) (post-freeze; `UnverifiedNewer`)
- [Official Antigravity CLI stable release `1.3.0`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.3.0)
- [Official Antigravity CLI stable release `1.2.17`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.2.17)
- [Official headless-mode stream and permission documentation](https://www.antigravity.google/docs/cli/headless/)
- [Frozen route-specific runtime identity fixture](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/headless-currentness.json)
- [1.2.15 provider-free denial evidence fixture](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/headless-denial-1.2.15-evidence.json)
- [1.2.17 Windows x64 and ARM64 artifact evidence](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/headless-windows-1.2.17-evidence.json)
- [Complete selected Linux x64 and Mac ARM64 archive inventory](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/dist-inventory.json)
- [Complete public asset manifests and source-hop inventory](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/identity.json)
- [Research 359: exact `1.2.11` retry pin](./359-antigravity-headless-retry-pin-evidence.md)
- [Research 401: Mac ARM64 headless static control-flow mappings](./401-antigravity-headless-static-control-flow-evidence.md)
- [Contract 023: Antigravity denial and child-status ruling](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md#antigravity-headless-denial-and-child-status)
- [Contract 029: interface version qualification](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 036: source compatibility boundary](../knowledge/contracts/036-crate-release-and-compatibility-boundary.md#coordinated-pre-10-version)
