# 388 Antigravity Headless 1.3.1 Currentness Stop

Status: the authorized provider-free child-status and soft-denial adaptations
are implemented and documented; qualification remains blocked at the existing
claim pending selected path/config mapping, Windows sandbox runtime, and
approved-environment retry/auth proof. Contract 036 classifies the child
lifecycle correction as breaking if shipped.

Owner: swallowtail#111 (`antigravity.headless`)
Date: 2026-10-08
Axis: `antigravity-cli.release`
Package: `swallowtail-adapter-antigravity`

## Current state

The official GitHub stable channel was re-probed on 2026-10-08. Its latest
release was `1.3.1`, published `2026-10-07T03:22:02Z`; the tag commit is
`968f1170bd0e002e9d0914730975bc8a2cc65861`. The headless claim stays at exact
`1.2.11`. Keep claim `antigravity.headless.release-window-2`, baseline
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
unproven.
Release notes are discovery evidence, not a substitute for artifact identity.
The unpublished `1.2.18` point inside the version interval remains without a
release or tag, and `1.3.2` is the first unpublished point after current stable;
neither point is added to the claim.

| Hop | Headless classification | Selected-path evidence and remaining gate |
| --- | --- | --- |
| `1.2.11→1.2.12` | Approved-environment evidence gate | Quota behavior changes for `GEMINI_API_KEY` sessions. The prepared access profile has no credential reference, but `EnvironmentRef` does not establish whether that variable reaches the child. |
| `1.2.12→1.2.13` | Retry-control evidence gate | The model API now follows server retry delays and stops on delays over 30 seconds or daily/billing caps. Retry-pin behavior is proven only on exact `1.2.11`; keep `0` pinned and do not transfer its artifact proof. |
| `1.2.13→1.2.14` | Compatible selected schema extension | Startup now rejects unsupported schema input and non-object roots. The adapter supplies inline JSON with an object root. Keep the existing schema decoder and failure coverage. |
| `1.2.14→1.2.15` | Approved soft-denial route limit | A denied permission is respected without alternate command, script, or tool workarounds. The official guide documents a continuing run with exit `0` and an stderr notice. Run completion does not prove each requested tool executed; no structured denial projection is claimed. |
| `1.2.15→1.2.16` | Selected lifecycle extension; identity-only child status is `Unknown` | Headless print now waits for background commands; image generation appears as a subagent. The adapter drains stdout through EOF and waits for the process. The approved fallback preserves child identity without inferring completion. |
| `1.2.16→1.2.17` | Windows sandbox runtime evidence gate | Windows sandbox behavior changes on the selected `ProviderEnforced --sandbox` path. Both exact Windows assets and PE digests are now frozen, but static inspection does not prove runtime enforcement; do not narrow platform support. |
| `1.2.17→1.3.0` | Selected path and custom-agent extension review | Special-character/Windows paths and project-scoped custom-agent discovery change. The verbosity and `/diff` changes are TUI-only under explicit stream JSON. The release note names selected path/config behavior, but does not establish the exact headless mapping or whether newly recognized project agents broaden the selected tool behavior. |
| `1.3.0→1.3.1` | Identity-only child-status `Unknown`; path/config and retry/auth gates remain | The error note names `/agents` and the running-agent list. Official stream docs describe child identity, log, and workspace fields, but no child error/status field; preserve identity as `Unknown`. Relative custom-agent path resolution is also a selected config change whose exact headless mapping remains unproven; the API-key retry note remains conditional on exact pin and approved-environment evidence. |

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
adapter now retains child identity and projects `SubagentStatus::Unknown` when
the selected stream has no usable child status. Synthetic fixtures prove that
identity does not inherit the enclosing step or whole-run outcome; they do not
pretend to be a provider capture.

Three independent evidence gates remain. First, the `1.3.0` and `1.3.1`
release notes change special-path handling, project-scoped agent discovery,
and custom-agent path resolution. The selected working directory and project
configuration can feed this headless path; the notes alone do not establish
whether this repairs existing behavior or broadens the selected tool behavior.
Freeze exact artifact-level evidence for the selected path/config mapping
before qualifying it.

Second, the exact `1.2.17` Windows x64 and
ARM64 archives are digest-verified and unpack to one inventoried PE executable
each. Static string tables contain the retry variable, API-key provider, and
terminal-sandbox markers; these strings do not establish retry control flow or
Windows sandbox enforcement. Qualification needs provider-free proof on the
exact Windows runtime for the selected `--sandbox` path. Do not narrow Windows
support based on this gap.

Third, the `1.2.12`, `1.2.13`, and `1.3.1` notes also change API-key quota or retry
behavior. The exact `AGY_CLI_MODEL_API_MAX_RETRIES=0` behavior is proven only
on `1.2.11`, and the selected `EnvironmentRef` does not expose an approved
child-environment key set. Qualification needs exact newer-artifact proof
that the zero pin still bounds retries and approved-environment evidence that
`GEMINI_API_KEY` is absent from this personal-subscription child environment.
No credential value was accessed and no new environment restriction is
asserted.

## Proposed authority-preserving continuation

Keep the existing claim and all qualified points while resolving all three gates:
provide exact, provider-free proof of retry pin semantics for the affected
newer artifacts and evidence for the approved child environment's API-key
boundary; provide exact-artifact Windows runtime proof for the selected
`--sandbox` path. If those proofs require live provider work, credentials,
host mutation, a new environment restriction, or narrower platform support,
return for the needed separate authority instead of weakening the evidence.
The approved soft-denial route limit and identity-only `Unknown` child
projection remain private behavior adaptations with no new operation or
approval bypass.

If the child-lifecycle correction ships, Contract 036 requires a pre-1.0 minor
release. This task does not set a package version or authorize a release; the
planner must place it on an appropriately versioned source release. Preserve
the earlier `1.2.11` segment and the `1.1.18..=1.2.10` hole. Do not infer live
provider honoring from static proof.

No provider prompt, live turn, credential access, CLI version command,
installation, host update, claim change, guide/matrix change, release change,
tag, or publication occurred.

## Sources

- [Official Antigravity CLI stable release `1.3.1`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.3.1)
- [Official Antigravity CLI stable release `1.3.0`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.3.0)
- [Official Antigravity CLI stable release `1.2.17`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.2.17)
- [Official headless-mode stream and permission documentation](https://www.antigravity.google/docs/cli/headless/)
- [Frozen route-specific runtime identity fixture](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/headless-currentness.json)
- [1.2.15 provider-free denial evidence fixture](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/headless-denial-1.2.15-evidence.json)
- [1.2.17 Windows x64 and ARM64 artifact evidence](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/headless-windows-1.2.17-evidence.json)
- [Complete selected Linux x64 and Mac ARM64 archive inventory](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/dist-inventory.json)
- [Complete public asset manifests and source-hop inventory](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/identity.json)
- [Research 359: exact `1.2.11` retry pin](./359-antigravity-headless-retry-pin-evidence.md)
- [Contract 023: Antigravity denial and child-status ruling](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md#antigravity-headless-denial-and-child-status)
- [Contract 029: interface version qualification](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 036: source compatibility boundary](../knowledge/contracts/036-crate-release-and-compatibility-boundary.md#coordinated-pre-10-version)
