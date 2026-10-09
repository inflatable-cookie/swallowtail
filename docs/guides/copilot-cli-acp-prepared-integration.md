# GitHub Copilot CLI ACP Prepared Integration

Use `swallowtail-adapter-copilot-cli` for the installed GitHub Copilot CLI ACP
agent. The route is `copilot-cli.acp`; the driver ID is
`swallowtail.copilot-cli.acp`. It owns initialize plus one bounded
`session/prompt` over ACP v1 stdio on a host-approved `copilot --acp --stdio`
child.

This is a separate family from Copilot CLI TCP `--port`, the Copilot IDE, the
GitHub Copilot API, and interactive-only slash commands. Public preview stays
visible. Swallowtail does not pass `--yolo`, `--allow-all`, or server-start
tool/effort flags.

The package is additive unreleased source after `v0.3.2`. Consumers must pin an
explicitly reviewed commit containing it. No version bump, tag, registry
publication, or harness installation is part of this route.

New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Selected Boundary

Preparation requires all of the following:

- exact package axis `copilot-cli.package`
- exact npm wrapper `1.0.80`
- host-approved `copilot` executable and isolated environment
- `copilot_cli_host_account_access_profile` with no credential reference
- working resource, plus host services for task, process, and working-resource
  ownership

The claim is qualified-only. Later packages do not inherit this route.
`UnverifiedNewer` is not a Copilot CLI ACP execution posture. Public preview is
visible as `ExperimentalObserved` and `COPILOT_CLI_ACP_MATURITY`.

Research 404 froze the exact `1.0.80`, first affected stable `1.0.81`, and
current stable `1.0.93` Darwin ARM64 artifacts. Under the no-network sandbox,
all three initialized, advertised the `copilot-login` auth method, and returned
JSON-RPC `-32000 Authentication required` for `session/new` with only a
synthetic token placeholder. No session, permission request, cancellation, or
tool call followed. This does not establish shipped permission behavior or
change the exact `1.0.80` claim.

Swallowtail does not install Copilot CLI, search `PATH`, run GitHub login, bind
`GH_TOKEN` / `GITHUB_TOKEN` as a Swallowtail lease, or default `--yolo`.
Host-owned GitHub Copilot login or BYOK stays outside the prepared plan.

[Research 425](../research/425-copilot-acp-authenticated-proof-preparation.md)
prepares a separately gated one-off permission proof. Its fake broker and
execution-plan schema do not change this prepared session path or establish
that the frozen artifacts can accept delegated authentication, select Auto,
bound retries, or expose the selected model. At that point the proof remained
disabled pending exact prerequisites; the later host-login/Auto ruling and
bounded original attempt are recorded in Research 428 below.

[Research 426](../research/426-copilot-acp-pre-prompt-discovery-stop.md)
records a fake-only pre-prompt discovery gate and a stop before original
artifact start. Current official docs do not bind the frozen versions'
Keychain item, complete authentication/model endpoint set, or a prompt-free
model catalogue. No authenticated observation or route claim changed.

[Research 428](../research/428-copilot-acp-host-login-permission-proof-stop.md)
records the approved host-login and Auto boundaries and a first original
`1.0.93` start. The hash-verified vendor CLI was allowed to read its existing
login through macOS `securityd`; no item-level filter or credential broker is
claimed, and the harness did not handle a token. Auto remains dynamic, the
underlying model and vendor request count are unobserved, and only vendor
selection/retries within one prompt and the 60-second ceiling are allowed.
The default-deny official-destination proxy and fake cancellation/no-effect
controls passed under a fake sandbox profile. That profile differs from the
original: it maps Python `sys.prefix`, and no fake admitted the vendor-owned
host-home authentication metadata reads allowed by the ruling. The original
profile denied all host-home reads, not only unrelated reads. This could have
blocked startup if the CLI needed an approved metadata read. The original
`sandbox-exec` launcher exited code 1 before ACP `initialize`; no egress-proxy
destination or ACP authentication request, session, prompt, permission or
effect was observed. The record does not show whether the vendor binary
reached startup, and stderr was discarded, so the cause is unknown. This
consumed the sole
`1.0.93` invocation; `1.0.81` and `1.0.80` were not started. Permission
behavior remains unproved and all route claims, including exact `1.0.80`, are
unchanged. See the committed
[preflight record](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/permission-proof-preflight-record.json)
and [original execution record](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/permission-proof-execution-record.json).

[Research 429](../research/429-copilot-acp-proof-startup-diagnostics-correction.md)
records a fake-only correction. The native fake and a future original share one
verified profile generator. Its sole host-home read exception is the literal
`.copilot/config.json` path, selected from current official documentation;
that documentation is not frozen-version-specific and does not prove the
`1.0.93` binary needs the file. The fake reads only a synthetic substitute and
proves adjacent home/repository reads, home/repository writes, shell execution,
and direct or unlisted egress remain denied. Its startup evidence applies only
to the fake. Stderr collection is concurrent: at most 4096 bytes are retained
in memory and the reported total count caps at 65536. Only safe categories and
counts are recorded; raw stderr is never retained in records or output. The
original startup cause remains unknown, its `1.0.93` invocation remains
consumed, and no route claim changes. Any future original attempt needs
separate exact authority.

[Research 430](../research/430-copilot-acp-renewed-1-0-93-permission-proof-stop.md)
records the one renewed `1.0.93` invocation under decision
`411be8ce-77a0-4a50-930f-d6aeacdffce9`. It consumed its one-shot record and
exited before ACP `initialize` with a `sandbox-denial` stderr category. No
prompt or permission evidence was produced. The denied operation is unobserved,
the older artifacts were not started, and no route claim changes.

[Research 431](../research/431-copilot-acp-offline-startup-trace-stop.md)
confirms the same executable digest initialized in Research 404 under a
different argv, environment, home-path, and sandbox tuple. The failure cause
is unknown. `sandbox-denial` means a stderr marker was present; it does not name
an operation or path. On the current audit host, the `com.apple.securityd`
launch-daemon label advertises `com.apple.SecurityServer` as a Mach service;
no access grant follows from that correction. Exact SEA startup remains
unrecovered, and original-artifact admission is blocked until fake-only
descendant containment and diagnostic gates are reviewed.

The task 165 fake-only continuation adds a stage pipe that separates launcher
entry, target-exec errno, and confirmed fake ACP startup. EOF by itself remains
unknown. Stderr evidence stores only bounded marker facets and fixed diagnostic
templates; a sandbox marker has unknown cause. The synthetic profile record
binds current OS identity and the imported `dyld-support.sb` closure, and uses
relative artifact, record, action, scratch, home and repository roles without
persisting personal paths. Fake controls cover config reads and writes, parent
metadata, missing files, symlink and repository escapes, home aliases, proxy
handler completeness, and a known child that escapes its process group. That
child is joined by its fake parent with `waitpid`; arbitrary vendor descendants
remain unbounded. Original-artifact entrypoints remain fail-closed, and the
new launcher is not wired into an original invocation; its exact exec grant
exists only in the task-owned fake profile. The effective original profile
has no permission change. See the
[Contract 023 offline-startup continuation](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md#fake-only-diagnostic-and-cleanup-continuation)
for the evidence boundary.

The credential-free no-child preparation has a separate fake-only result in
[`preparation-record.json`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-no-child-diagnostic/preparation-record.json).
On its recorded macOS build, the imported runtime profile and exact shim/target
exec grants did not permit fork, vfork or posix_spawn after the launcher
transition. The candidate request is data-only: `--acp --stdio`, at most one
initialize, a 60-second ceiling, and synthetic state with host, repository,
auth-service and network access denied. Its `execution_authorized` value is
false. Fake success does not prove Copilot startup, host-login or Auto-model
compatibility; an original attempt needs separate exact authority and review.

[Research 433](../research/433-copilot-acp-no-child-original-initialize-diagnostic.md)
records the one consumed original under that reviewed isolation. The
original-profile fake path passed. The original sent one initialize, reached
stage exec-boundary EOF, and exited before any ACP protocol byte with
`failure_class` `sandbox-denial`. Vendor startup remains unknown. The brief
named archive `98640ca0…`; the executed pairing used archive `f254651a…`.
Tom answered “Approve” to that one historical diagnostic under the
[Consumed Initialize Archive Exception](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md#consumed-initialize-archive-exception);
[Q-007](../knowledge/questions.md#q-007) is answered. The stop-rule breach is
not erased. Future identity mismatches still stop. The attempt is consumed;
reviewers inspect the committed records and must not start the original.
Initialize was not reached; startup cause unknown. Initialization success
would not have proved auth, Auto, permissions, cancel/no-effect, or
supported versions. Fail-closed no-child policy may prevent vendor startup
and is not a route narrowing. The exact `1.0.80` claim is unchanged. Parent
119 stays blocked.

Call `prepare_copilot_cli_acp` with `CopilotCliPreparationInput` and
`CopilotCliPreparationProbe`. The probe classifies the approved target only. It
does not send initialize, create a session, or prompt.

The prepared integration binds the execution host, exact target, observation,
host-account access profile, and preflight evidence. Validate that binding
before reusing it. The access profile is local and unauthenticated: Swallowtail
opens no credential lease.

Wrong axis (`copilot-cli.tcp-port` is not this route), wrong audience, a
credential reference, or an unqualified package fails before ACP work.

## ACP Interactive Session

Create `CopilotCliSessionProfileInput::new` with request identity and a
read-only working resource. There is no open deadline, model route, or
harness-mode option. Call `prepare_session`, inspect `evidence()`, `plan()`,
and `request()`, then `open_session`.

The driver owns one joined stdio child and performs this sequence:

1. spawn `copilot --acp --stdio` with no extra argv
2. `initialize` with host `fs` and `terminal` advertised false
3. `session/new` with `{cwd, mcpServers}` — empty, or one admitted HTTP entry
4. one bounded `session/prompt` of text blocks
5. observe permission requests and cancel; never select `allow_always`
6. join connection, process, and task cleanup

Host `fs/readTextFile` and `fs/writeTextFile` callbacks are rejected. Session
deadline, `session/load`, and `session/close` are unsupported. Unexpected
initialize `agentInfo.version` fails closed against selected `1.0.80`.

Take each turn's event stream and terminal outcome immediately and poll them
concurrently. Cancellation issues `session/cancel` and joins the active turn.
Close the turn and session separately from terminal truth. Ambient-host
isolation is not filesystem or descendant-process containment.

See the compile-tested
[`prepared_copilot_cli_acp` example](../../crates/swallowtail-adapter-copilot-cli/examples/prepared_copilot_cli_acp.rs).

## Consumer-declared MCP

Contract 063 admits one consumer-supplied streamable-HTTP placement. Bind
`CopilotCliAcpRemoteMcpPlacement` whose name is the route-owned
`swallowtail-copilot-cli-acp`. A foreign name fails closed as a collision:
Research 351 shows Copilot CLI skips names that already exist on
agent-configured servers. Stdio client entries are not offered: the provider
rejects them.

HTTP encoding emits ACP `type: "http"` with the consumer URL and headers
verbatim. Structure is validated only: non-empty name, absolute `http`/`https`
URL, well-formed header names. URL and header values never appear in failures,
diagnostics, activity, receipts, `Debug` output, or plan fingerprints: the
public encoder returns `CopilotCliAcpEncodedMcpServers`, whose `Debug` form
redacts those values, and the wire JSON stays crate-private. `sse` stays
modelled through `CopilotCliAcpRemoteMcpPlacement::sse` and is not emitted.

The Contract 061 placement projection names `consumer-supplied-http` when an
HTTP entry is bound. Emission is not honouring: `client_mcp_servers` stays
unavailable pending a live gate.

## Restart, Failure, And Promotion

ACP exposes no public load or resume binding.
`prepare_working_state_restoration` opens a fresh context-losing session after
process loss; it does not recover the interrupted turn or transcript. A prepared
HTTP MCP declaration is carried onto that replacement session.

Handle failures through portable classification and retain the exact
`swallowtail.copilot-cli.acp` diagnostic for support. Do not parse stderr, ACP
data, or GitHub account state to infer retry, auth, terminal, or cleanup truth.
Unauthorized initialize maps to `swallowtail.copilot-cli.acp.host_auth_required`
without GitHub policy.

Promotion of HTTP MCP honouring, TCP `--port`, `--yolo` / `allow_all`,
server-start tool or effort flags, GitHub login as Swallowtail action,
treating preview as stable, Copilot IDE/API coverage, host fs writes, model
selection, usage, session load, or live qualification requires a separate
card, exact version evidence, and matrix coverage. An advertised ACP
capability or CLI flag alone is insufficient.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-copilot-cli
effigy check:examples
```

No login, install, or authenticated prompt is part of deterministic acceptance.
Live evidence stays separately gated and is not claimed by this route.
