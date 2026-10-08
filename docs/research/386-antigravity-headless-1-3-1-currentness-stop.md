# 386 Antigravity Headless 1.3.1 Currentness Stop

Status: qualification stopped with the existing claim unchanged.

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
identity and complete inspected archive trees in
[`tests/fixtures/antigravity-cli-1.3.1/`](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/).
For every hop, the official source comparison is one commit with only
`CHANGELOG.md` changed. The public asset manifest records eight assets per
release. Linux x64 and Mac ARM64 release archives were downloaded and their
published digests matched; each inspected archive contains exactly one regular
file, `antigravity`, with its digest and size recorded. Windows assets have
published archive digests but were not unpacked. No binary was executed.
Release notes are discovery evidence, not a substitute for artifact identity.
The unpublished `1.2.18` point inside the version interval remains without a
release or tag, and `1.3.2` is the first unpublished point after current stable;
neither point is added to the claim.

| Hop | Headless classification | Selected-path evidence and remaining gate |
| --- | --- | --- |
| `1.2.11→1.2.12` | Approved-environment evidence gate | Quota behavior changes for `GEMINI_API_KEY` sessions. The prepared access profile has no credential reference, but `EnvironmentRef` does not establish whether that variable reaches the child. |
| `1.2.12→1.2.13` | Retry-control evidence gate | The model API now follows server retry delays and stops on delays over 30 seconds or daily/billing caps. Retry-pin behavior is proven only on exact `1.2.11`; keep `0` pinned and do not transfer its artifact proof. |
| `1.2.13→1.2.14` | Compatible selected schema extension | Startup now rejects unsupported schema input and non-object roots. The adapter supplies inline JSON with an object root. Keep the existing schema decoder and failure coverage. |
| `1.2.14→1.2.15` | New private denial milestone needs selected-stream proof | A denied permission is respected without alternate command, script, or tool workarounds. This affects selected headless tool/permission behavior. Preserve operations and the no-approval-bypass boundary. The exact denied stdout event is not published. |
| `1.2.15→1.2.16` | Selected lifecycle extension; child-status gate remains | Headless print now waits for background commands; image generation appears as a subagent. The adapter drains stdout through EOF and waits for the child, but child completion is not proven by identity-only fields. |
| `1.2.16→1.2.17` | Windows sandbox runtime evidence gate | Windows sandbox behavior changes on the selected `ProviderEnforced --sandbox` path. The Windows binaries were not unpacked or exercised; do not narrow platform support. |
| `1.2.17→1.3.0` | Selected path and custom-agent extension review | Special-character/Windows paths and project-scoped custom-agent discovery change. The verbosity and `/diff` changes are TUI-only under explicit stream JSON. Exact selected path mapping remains unverified from the opaque artifact. |
| `1.3.0→1.3.1` | Child-status stream evidence gate | The error note names `/agents` and the running-agent list. Official stream docs describe child identity, log, and workspace fields, but no child error/status field. This does not establish the selected stream's child outcome. API-key retry remains conditional on environment evidence. |

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
text into a tool failure. Child identity is decoded from `subagent_info` and
currently projected as `SubagentStatus::Completed`.

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
selected stdout, so it does not authorize a synthetic version-specific stream
fixture.

Static inspection of the exact `1.3.1` Mac ARM64 artifact (archive SHA-256
`ef5e385b32afda4cf1612368bb4bf155d3f8f4c55d51488649f508baefe77c86`, extracted
CLI SHA-256
`88db8b4d21ece4999fa58e0b54cea77154e47b319ee178d086c446262317f3fa`) places
`PollPrintmode` through `steps.ExtractSubagentInfo` to
`streamJSONEmitter.EmitStepUpdate`. The inspected call/output shape exposed no
child error/status field, but this absence in disassembly does not prove a
successful child or the complete serialized schema. No exact selected error
stream is retained; preserve the `Completed` mapping until the contract's
evidence/ruling gate is resolved.

## Why qualification stops

The current official [headless guide](https://www.antigravity.google/docs/cli/headless/)
states that a permission-required tool that cannot obtain approval is
soft-denied: the run continues with exit code `0`, and a notice is printed to
stderr. The guide separately says a failed tool event may contain
`tool_info.error` with `type` and `message`, but it does not connect a
soft-denial to that stdout object. Swallowtail parses stdout for stream JSON and
ignores stderr. Without a version-specific selected-stream shape, an exact
`1.2.15` denial fixture would invent provider output. The route can retain its
existing no-bypass behavior, but the denial milestone cannot be claimed yet.

The same guide documents `subagent_info.subagents` identity fields, not a
child failure status. The `1.3.1` release note describes errors in TUI lists;
static identity fields and disassembly do not prove a successful child. No
provider-free exact error stream was found in the released documentation or
retained corpus. Do not change the current `SubagentStatus::Completed` mapping
on that evidence. If no usable error field can be established, request a
separate ruling for an unknown/omitted lifecycle projection before changing
that consumer-visible status.

Two independent artifact gates remain as well. The `1.2.12`, `1.2.13`, and
`1.3.1` notes touch API-key quota or retry behavior, while the selected
`EnvironmentRef` is opaque. In addition, `1.2.17` changes the Windows sandbox
path, but Windows artifact trees and runtime behavior were not inspected. No
platform support is narrowed and no environment or retry behavior is guessed.

## Proposed authority-preserving continuation

Keep the existing claim and all qualified points. For a later qualification,
retain `AGY_CLI_MODEL_API_MAX_RETRIES=0`, establish its exact semantics on each
selected newer artifact, and prove the approved environment cannot introduce
an unqualified API-key path. Review exact Windows artifact identity and the
selected sandbox behavior. Freeze a provider-free exact `1.3.1` child-error
stream if available. If that stream contains a usable child error field, map it
to the existing `Failed` vocabulary. If it does not, obtain the separate
unknown/omitted lifecycle ruling before altering `Completed`.

Once these gates are resolved, a private behavior milestone may begin at
`1.2.15` for the denied-permission behavior while preserving the operations and
no-approval-bypass boundary. Keep the earlier `1.2.11` segment and the
`1.1.18..=1.2.10` hole. Do not add a permission approval operation, alter the
public lifecycle, or infer live honoring from static proof.

No provider prompt, live turn, credential access, CLI version command,
installation, host update, claim change, guide/matrix change, release change,
tag, or publication occurred.

## Sources

- [Official Antigravity CLI stable release `1.3.1`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.3.1)
- [Official headless-mode stream and permission documentation](https://www.antigravity.google/docs/cli/headless/)
- [Frozen route-specific runtime identity fixture](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/headless-currentness.json)
- [Complete selected Linux x64 and Mac ARM64 archive inventory](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/dist-inventory.json)
- [Complete public asset manifests and source-hop inventory](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/identity.json)
- [Research 359: exact `1.2.11` retry pin](./359-antigravity-headless-retry-pin-evidence.md)
- [Contract 023: Antigravity denial and child-status ruling](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md#antigravity-headless-denial-and-child-status)
- [Contract 029: interface version qualification](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
