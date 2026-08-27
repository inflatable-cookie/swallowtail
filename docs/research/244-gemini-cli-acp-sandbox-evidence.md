# 244 Gemini CLI ACP Sandbox Evidence

Status: promoted; empty deliver-now
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.087 / 247

## Question

Which exact qualified `gemini-cli.acp` version, platform, backend, value, and
lifecycle rows, if any, can select native sandboxing process-privately and
confirm ACP-child activation before readiness without overstating containment?

## Decision

No. Research 244 admits an empty deliver-now set. No sandbox binding is admitted
on `gemini-cli.acp` for any published point in qualified `0.51.0..=0.56.0`.

Across every published stable tag in that window, Gemini CLI exposes the same
native full-process sandbox surface used by headless: boolean `--sandbox` /
`-s`, `GEMINI_SANDBOX`, and `settings.tools.sandbox`. Tagged source selects
backends from `docker|podman|sandbox-exec|runsc|lxc|windows-native`, starts
them through `start_sandbox()`, and may re-exec the CLI inside that backend
before `runAcpClient` ever opens ACP NDJSON on stdio. Official ACP docs name
stdio JSON-RPC and the initialize/session methods; they do not name a sandbox
control or confirmation field. Official sandbox docs still claim
flag-before-env precedence; tagged source and its unit tests give
`GEMINI_SANDBOX` precedence over argv and settings. The qualified Swallowtail
route remains ambient-host, encodes only `--acp` plus
`--approval-mode plan|auto_edit`, and injects no sandbox argv or env.
Selecting a flag or environment value is not filesystem, process, credential,
or network containment.

Research 239 remains headless contrast only. Its empty-set conclusion is not
copied as ACP proof; ACP spawn, stdin handling, and initialize/session seams
were traced independently and close the same empty outcome for additional
ACP-specific gates.

## Method And Boundary

Official sandbox and ACP documentation plus exact public GitHub source for
every published stable tag in `0.51.0..=0.56.0` were inspected on 2026-08-27.
Decisive lifecycle, loader, argv option, ACP transport, initialize dispatcher,
and precedence-test blobs were fetched and digested per tag. Narrative detail
uses ceiling tag `v0.56.0` commit `b6e23a7dc29eb15fede4bbe646d91869e948b45a`
where those decisive files are byte-identical to the earlier published points.
Prepared-route argv, harness posture, guide text, and protocol fixtures were
audited in-repo. No Gemini install, executable launch, sandbox backend start,
image pull, OAuth, credential capture, provider prompt, paid inference, or host
mutation was used. Enterprise Developer API-key access remains the selected
route; consumer login stays out.

## Published Points

| Version | Git tag | Commit | Published |
| --- | --- | --- | --- |
| `0.51.0` | `v0.51.0` | `8d951de3855750d5f8219d65ae22524b606133b6` | yes |
| `0.52.0` | `v0.52.0` | `d14583b926769bd98f807cdc6b1ca50e91ae26ec` | yes |
| `0.53.0` | `v0.53.0` | `decc0b46c6e11f8cad90710dcfb38fc3cdb24a96` | yes |
| `0.53.1` | `v0.53.1` | `19a68016bdc9cd4177a155846dd51f282c3c1c59` | yes |
| `0.54.0` | `v0.54.0` | `a74b483d14a93159fa36e7ee9e32cf44bda594df` | yes |
| `0.54.1` | — | — | unpublished |
| `0.54.2` | — | — | unpublished |
| `0.54.3` | — | — | unpublished |
| `0.54.4` | `v0.54.4` | `983bbb89d36718e3c97618978fb938ed9b5856c9` | yes |
| `0.55.0` | — | — | unpublished |
| `0.55.1` | `v0.55.1` | `41327e407da58aa01c409ef6685b7b5d379f295e` | yes |
| `0.56.0` | `v0.56.0` | `b6e23a7dc29eb15fede4bbe646d91869e948b45a` | yes |
| `0.56.1` | — | — | unpublished |

Decisive cross-version seam digests (SHA-256 of tagged file bytes):

| Seam | Digest stability across published points |
| --- | --- |
| `packages/cli/src/config/sandboxConfig.ts` | identical at all eight: `08c340670d3b85b827047784f89d697903db58b994efee430f0218286a1b029f` |
| `packages/cli/src/config/sandboxConfig.test.ts` | identical at all eight: `efe1b21f210f9e410304a00828969ff2ae837479617edde4f43b4ad4f38f9a64` |
| `packages/cli/src/config/config.ts` | identical at all eight: `5100bcd48f798d04b9463bd72680af7202f331de566321b1c29f5f8710c2c44c` |
| `packages/cli/src/gemini.tsx` | identical at all eight: `1ac297b1af4cca39f358fc5c90c18059e275c4c393cfba152f73215e03ead828` |
| `packages/cli/src/acp/acpStdioTransport.ts` | identical at all eight: `08f79d2b2048d0c3d817fe8d8c85f92338c879711eb8a075b41db210b58cd4a5` |
| `packages/cli/src/acp/acpRpcDispatcher.ts` | identical at all eight: `0efad1ad0db341802cc27710a5d5ab522666f3cd1a229faece8094a81a992660` |
| `packages/cli/src/utils/sandbox.ts` | `dac331000eb2eb28407dffb56ac31429a7324b087adfb94d3330ae206eb84e03` for `0.51.0..=0.54.4`; `df009ae28efa17347101731334486a3bb15f3aabe48d61813dc3dd76f3cb7c3f` for `0.55.1` and `0.56.0` |

Cross-version seam verdict:

| Gate family | Stability |
| --- | --- |
| Backend membership / platform gates / env>argv precedence | identical `sandboxConfig.ts` and tests at every published point |
| Boolean `--sandbox` argv | identical `config.ts` option at every published point |
| Lifecycle re-exec before ACP | identical `gemini.tsx` sandbox hop then later `runAcpClient` at every published point |
| ACP stdio transport | identical `acpStdioTransport.ts` at every published point |
| Initialize capabilities | identical `acpRpcDispatcher.ts`; no sandbox field |
| Backend starter implementation | still exports `start_sandbox`, inherits stdio, and may drain non-TTY stdin; `0.55.1` adds Seatbelt temp-file fallback only |

Empty-set gates therefore hold for every published executable version in the
qualified window, not only the `0.56.0` ceiling.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [Sandboxing](https://geminicli.com/docs/cli/sandbox/) | official methods, docs precedence claim, prerequisites | 2026-08-27 | `71cbb84469746c9e4a43d396f29d79b54b4d3e4a4523d50cc08f25521b5d1dd4` |
| [ACP Mode](https://geminicli.com/docs/cli/acp-mode/) | stdio JSON-RPC; methods; no sandbox control | 2026-08-27 | `f3bfab06bc79fb1e4e41d961d90f45d314c0263185a1bcd70da26b0b3464b5a3` |
| ACP Mode markdown | same body as docs page | 2026-08-27 | `ef2e4d990c6d6793f2696f5ce410530316da252c0825b612c99606c1308c3f8a` |
| GitHub `docs/cli/sandbox.md` @ `v0.56.0` | tagged sandbox doc | 2026-08-27 | `2d2c13347d2d6e26dfdc3a33d7b43f12d242fa679eb1220c85153b824e8c0df4` |
| GitHub `docs/cli/acp-mode.md` @ `v0.56.0` | tagged ACP doc; no sandbox wording | 2026-08-27 | `812af1001c110f474a624c3026c31e66519ca95c251d1f79679d8ebd56edd779` |
| `packages/cli/src/config/config.ts` @ `v0.56.0` | `--sandbox`/`-s` boolean; `--acp` | 2026-08-27 | `5100bcd48f798d04b9463bd72680af7202f331de566321b1c29f5f8710c2c44c` |
| `packages/cli/src/config/sandboxConfig.ts` @ `v0.56.0` | env>argv/settings; backend membership; platform gates | 2026-08-27 | `08c340670d3b85b827047784f89d697903db58b994efee430f0218286a1b029f` |
| `packages/cli/src/config/sandboxConfig.test.ts` @ `v0.56.0` | precedence and rejection unit evidence | 2026-08-27 | `efe1b21f210f9e410304a00828969ff2ae837479617edde4f43b4ad4f38f9a64` |
| `packages/cli/src/gemini.tsx` @ `v0.56.0` | sandbox hop before `runAcpClient`; non-TTY stdin drain | 2026-08-27 | `1ac297b1af4cca39f358fc5c90c18059e275c4c393cfba152f73215e03ead828` |
| `packages/cli/src/utils/sandbox.ts` @ `v0.56.0` | backend spawn; `stdio: 'inherit'` | 2026-08-27 | `df009ae28efa17347101731334486a3bb15f3aabe48d61813dc3dd76f3cb7c3f` |
| `packages/cli/src/acp/acpStdioTransport.ts` @ `v0.56.0` | ACP NDJSON over process stdio | 2026-08-27 | `08f79d2b2048d0c3d817fe8d8c85f92338c879711eb8a075b41db210b58cd4a5` |
| `packages/cli/src/acp/acpRpcDispatcher.ts` @ `v0.56.0` | `initialize` capabilities; no sandbox | 2026-08-27 | `0efad1ad0db341802cc27710a5d5ab522666f3cd1a229faece8094a81a992660` |
| `packages/cli/src/acp/acpSessionManager.ts` @ `v0.56.0` | `session/new` response; no sandbox | 2026-08-27 | `63e38dfcfe035a317acc9e2943810b765e7403ecb6383597633394e8ff214f1e` |
| Research 239 headless sandbox | sibling contrast only | 2026-08-27 | promoted; empty |
| Frozen fixture `gemini-cli-0.56.0/protocol.json` | ACP argv omits sandbox | 2026-08-22 | workspace |
| Prepared guide `gemini-cli-prepared-integration.md` | ambient writes; sandbox promotion bar | 2026-08-27 | workspace |
| Adapter audit `driver.rs` / `prepared_profile/plan.rs` | ACP argv and ambient posture | 2026-08-27 | workspace |

HTML digests identify retrieved documentation bodies. They are not a
compatibility guarantee. Narrative findings below use the `0.56.0` ceiling;
decisive loader, argv, lifecycle, ACP transport, and initialize bytes are
identical at every published point in the qualified window.

## Configuration Surface

| Seam | Exact shape through published `0.51.0..=0.56.0` |
| --- | --- |
| argv | `--sandbox` / `-s` is yargs `boolean` only; no typed backend value on argv |
| ACP argv | `--acp` (and deprecated `--experimental-acp`) selects ACP mode |
| env | `GEMINI_SANDBOX=true\|false\|1\|0\|docker\|podman\|sandbox-exec\|runsc\|lxc\|windows-native` |
| settings | `tools.sandbox` may be boolean, command string, profile path, or object `{enabled,command,image,allowedPaths,networkAccess}` |
| already inside | `SANDBOX` set → loader returns no sandbox command |
| seatbelt profile | `SEATBELT_PROFILE`; default `permissive-open` |

Official docs claim enablement order flag → env → settings. Tagged
`sandboxConfig.ts` states and implements the opposite for the env seam: non-empty
`GEMINI_SANDBOX` replaces the argv/settings value before normalization. Unit
test `should prioritize GEMINI_SANDBOX over CLI and settings` freezes that
behavior.

Truthy boolean selection auto-picks a host command:

| Platform condition | Selected command |
| --- | --- |
| `darwin` and `sandbox-exec` exists | `sandbox-exec` even if docker also exists |
| otherwise docker exists and sandbox is true | `docker` |
| otherwise podman exists and sandbox is true | `podman` |
| sandbox true and none found | `FatalSandboxError` |
| `runsc` / `lxc` | never auto-detected; must be explicit |

## Closed Backend And Platform Inventory

This inventory is frozen source membership. It is not a deliver-now admission
table.

| Backend value | Platform gate in loader | Host prerequisite | Image required |
| --- | --- | --- | --- |
| `sandbox-exec` | intended for macOS Seatbelt; auto on darwin when boolean true | `sandbox-exec` on PATH | no; native |
| `docker` | none beyond command existence | Docker installed and usable | yes unless native carve-out |
| `podman` | none beyond command existence | Podman installed and usable | yes unless native carve-out |
| `runsc` | Linux only; also requires `docker` | runsc + Docker runtime | yes |
| `lxc` | not auto-detected; docs: Linux + pre-running container | `lxc` on PATH; running container | treated as native for image gate |
| `windows-native` | Windows only | built-in path; docs note persistent `icacls` integrity changes | no; native |
| `true` / `1` | resolves through auto-detect above | host-dependent | host-dependent |
| `false` / `0` / empty | disables | none | n/a |
| other string | rejected | n/a | n/a |

Unsupported or missing command strings throw `FatalSandboxError` during
`loadSandboxConfig()` before `start_sandbox()`. That is parser/host-prerequisite
rejection, not portable preflight on the Swallowtail plan surface. Auth refresh
may already have run earlier in `gemini.tsx` before the sandbox hop.

## ACP Lifecycle And Truth Layers

When sandbox config resolves and `SANDBOX` is unset, `gemini.tsx` may refresh
auth, then:

1. If stdin is not a TTY, drain it with `readStdin()` and inject the bytes into
   `--prompt` on the re-exec argv.
2. Call `start_sandbox()` with `stdio: 'inherit'`.
3. Await the child, clean up, and exit.
4. Only when the sandbox hop is skipped does the process reach
   `config.getAcpMode()` → `runAcpClient`, which binds ACP NDJSON to process
   stdin/stdout.

ACP therefore starts only after parent sandbox parsing and either hop-skip or
child re-exec. For a piped ACP host, the parent non-TTY stdin drain path
consumes the protocol stream before any ACP server exists. Child inheritance
of stdio does not create an initialize/session sandbox confirmation field.

| Layer | Exact availability on qualified ACP |
| --- | --- |
| Requested | no Swallowtail portable sandbox request surface |
| Argv/environment-encoded | native seams exist; qualified route encodes neither |
| Parent parsed | `loadSandboxConfig` runs before `runAcpClient` |
| Backend-started | requires `start_sandbox()` and host backend/image readiness |
| ACP child connected | only after hop skip or re-exec child; not confirmed by initialize |
| Accepted | no initialize or `session/new` sandbox field |
| Effective | requires observing the re-exec/backend child; not prompt-free |
| Contained | not inferred from flag/env selection |
| Observed | initialize advertises loadSession/prompt/mcp capabilities only |

`initialize` `agentCapabilities` remain `loadSession`, `promptCapabilities`, and
`mcpCapabilities`. `session/new` returns session id, modes, and models. Official
ACP Mode docs list the same method families and name no sandbox control.

## Prepared Route Audit

Qualified ACP argv from `driver.rs`:

```text
--acp
--approval-mode plan|auto_edit
```

No `--sandbox`. Validation requires `HarnessConfigurationPosture::Ambient` and
`HarnessIsolation::AmbientHost`. The process launch reuses one host-owned
`EnvironmentRef` and injects no `GEMINI_SANDBOX`, no sandbox disable, and no
isolated settings path for sandbox keys. The prepared guide states ambient
writes are not filesystem or descendant-process containment and that sandbox
claims need exact surface evidence before promotion.

Omission therefore keeps current ACP argv/environment and makes no sandbox,
backend, isolation, filesystem, network, or process-containment claim. Ambient
`GEMINI_SANDBOX` or `settings.tools.sandbox` can still enable sandboxing without
a Swallowtail request. Because env beats argv, a hypothetical later `--sandbox`
encoding would still lose to ambient `GEMINI_SANDBOX` under the current
ambient-host posture.

Enterprise Developer API-key access, Plan selection on the read-only profile,
and current read/read-write profiles stay unchanged. Consumer login stays out.

## Why No Deliver-Now Row Survives

| Gate | Finding |
| --- | --- |
| Closed version membership | eight published points in `0.51.0..=0.56.0` enumerated with tag/commit; unpublished patch holes recorded; decisive loader/argv/lifecycle/ACP seams identical |
| Closed platform/backend/value | backend table exists in identical `sandboxConfig.ts`, yet effective membership depends on installed commands, running daemons/containers, and images |
| Process-private precedence | `GEMINI_SANDBOX` overrides argv/settings at every published point; ambient settings apply when env and argv omit; qualified route injects neither |
| Parent-to-child ownership | sandbox hop and optional stdin drain happen in the parent before `runAcpClient`; ACP owns only the post-hop process |
| Pre-readiness activation | initialize/`session/new` have no sandbox field at every published point; confirming the backend requires starting it or inspecting inside it |
| No backend install/start | docker/podman/runsc/lxc need installed backends; `start_sandbox` may pull images; Windows native mutates ACL state per docs |
| Joined failure / cleanup | parent awaits sandbox child then exits; that is not ACP initialize acceptance or portable containment |
| Selection ≠ containment | official security notes and Contract 037 posture forbid treating a flag as portable isolation |
| Headless contrast only | Research 239 freezes the shared loader/backends; ACP proof does not promote its conclusion without the ACP lifecycle above |

## Deliver-Now Set

Empty. For every published executable version in `0.51.0..=0.56.0`, no
platform / backend / value / lifecycle row meets all gates: process-private
selection on the qualified ambient ACP route, closed host-independent
membership, parent-to-ACP-child ownership with activation before readiness,
pre-effect rejection without backend work, and prompt-free confirmation
without treating selection as containment.

## Withheld Surfaces (Not Empty-Set Reasons Alone)

| Surface | Disposition |
| --- | --- |
| `security.toolSandboxing` | separate tool-level sandboxing; not the full-process `--sandbox` lane |
| Seatbelt profiles / `SEATBELT_PROFILE` | macOS profile detail after backend selection |
| Sandbox expansion UI | interactive permission expansion; not ACP confirmation |
| Exit code `44` | headless sandbox failure class; not an ACP initialize field |
| Gemini headless or Live | sibling routes; Research 239 contrast only |
| Consumer OAuth / login | out of route; enterprise API-key access retained |

## Continuation

Card 247 closes with an honest empty set. Production sandbox binding on
`gemini-cli.acp` stays blocked until a bounded child env/settings contract can
win over ambient configuration, reject unsupported
version/platform/backend/value rows before process and backend work, keep ACP
stdio intact across any re-exec, and expose readiness-time activation evidence
without treating selection as containment.

## Evidence

- [sandbox-evidence.json](../../crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-acp-0.56.0-sandbox/sandbox-evidence.json)
