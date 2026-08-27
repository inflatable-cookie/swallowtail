# 239 Gemini CLI Headless Sandbox Evidence

Status: promoted; empty deliver-now
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.085 / 240

## Question

Which exact qualified `gemini-cli.headless` version, platform, backend, and
value rows can select native sandboxing process-privately through the enterprise
API-key route and confirm activation before prompt effects?

## Decision

No. Research 239 admits an empty deliver-now set. No sandbox binding is admitted
on `gemini-cli.headless` for any published point in qualified
`0.51.0..=0.56.0`.

Across every published stable tag in that window, Gemini CLI exposes a native
full-process sandbox surface through boolean `--sandbox` / `-s`,
`GEMINI_SANDBOX`, and `settings.tools.sandbox`. Tagged source selects backends
from `docker|podman|sandbox-exec|runsc|lxc|windows-native`, starts them through
`start_sandbox()`, and may re-exec the CLI inside that backend before the
ordinary headless path continues. Official docs list flag-before-env precedence;
tagged source and its unit tests give `GEMINI_SANDBOX` precedence over argv and
settings. The qualified Swallowtail route remains ambient-host, omits any
sandbox argv or env injection, and stream-json exposes no sandbox field. Exit
code `44` is a sandbox failure class only. Selecting a flag or environment value
is not filesystem, process, credential, or network containment.

## Method And Boundary

Official sandbox and headless documentation plus exact public GitHub source for
every published stable tag in `0.51.0..=0.56.0` were inspected on 2026-08-27.
Decisive loader, argv option, lifecycle entry, starter, stream-json types, and
precedence-test blobs were fetched and digested per tag. Narrative detail uses
ceiling tag `v0.56.0` commit `b6e23a7dc29eb15fede4bbe646d91869e948b45a` where
those decisive files are byte-identical to the earlier published points.
Prepared-route argv, harness posture, guide text, and exit-code mapping were
audited in-repo. No Gemini install, executable launch, sandbox backend start,
image pull, OAuth, credential capture, provider prompt, paid inference, or host
mutation was used.

Sibling Research 230 freezes headless settings/process shape for thinking and
does not qualify sandboxing. Research 182 supplies the published-point census
for the qualified window. Gemini ACP and Live stay out of scope.

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
| `packages/core/src/output/types.ts` | identical at all eight: `23f7ea24497c88a703e0e4f8b6deb8bda969c2c2a32ca213beacfae46d798341` |
| `packages/cli/src/utils/sandbox.ts` | `dac331000eb2eb28407dffb56ac31429a7324b087adfb94d3330ae206eb84e03` for `0.51.0..=0.54.4`; `df009ae28efa17347101731334486a3bb15f3aabe48d61813dc3dd76f3cb7c3f` for `0.55.1` and `0.56.0` |
| `docs/cli/sandbox.md` | `1dbc39a3fa336ce0ce8aa9475b6f16f3c207abb3c7667ac1d3c61db67f90019a` for `0.51.0`/`0.52.0`; `2d2c13347d2d6e26dfdc3a33d7b43f12d242fa679eb1220c85153b824e8c0df4` from `0.53.0` through `0.56.0` |
| `packages/cli/src/nonInteractiveCli.ts` | `e220c84af948e79f61a08bb961d2bca940f5bacb066a9c167039b873644f94d2` through `0.53.0`; `fe569c4ac3436a851c991e0e916554b63bd9b9eb0bf5dee644f9258fad5ba298` from `0.53.1` through `0.56.0` |

Cross-version seam verdict:

| Gate family | Stability |
| --- | --- |
| Backend membership / platform gates / env>argv precedence | identical `sandboxConfig.ts` and tests at every published point |
| Boolean `--sandbox` argv | identical `config.ts` option at every published point |
| Lifecycle re-exec into `start_sandbox` | identical `gemini.tsx` call sites at every published point |
| Stream-json activation field | identical `InitEvent` (`type`, `session_id`, `model`; no sandbox) at every published point |
| Backend starter implementation | still exports `start_sandbox` and spawns backends; `0.55.1` adds embedded Seatbelt profile temp-file fallback and cleanup handlers only |
| Docs Seatbelt default wording | tightened at `0.53.0`; docs precedence claim remains flag-before-env while source stays env-first |
| Headless emitter | `0.53.1` InvalidStreamError delta only; no sandbox confirmation field added |

Empty-set gates therefore hold for every published executable version in the
qualified window, not only the `0.56.0` ceiling.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [Sandboxing](https://geminicli.com/docs/cli/sandbox/) | official methods, docs precedence claim, prerequisites | 2026-08-27 | `71cbb84469746c9e4a43d396f29d79b54b4d3e4a4523d50cc08f25521b5d1dd4` |
| [Headless mode](https://geminicli.com/docs/cli/headless/) | stream-json events; no sandbox control or field | 2026-08-27 | `7badcdfa83d7b8c60f510ab9f40c847d80a96a87050fd3ddd02ce3fef3e1746c` |
| GitHub `docs/cli/sandbox.md` @ `v0.56.0` | tagged doc cross-check | 2026-08-27 | `2d2c13347d2d6e26dfdc3a33d7b43f12d242fa679eb1220c85153b824e8c0df4` |
| npm `@google/gemini-cli@0.56.0` identity | qualified package point | 2026-08-22 | Research 182 |
| `packages/cli/src/config/config.ts` @ `v0.56.0` | `--sandbox`/`-s` boolean argv only | 2026-08-27 | `5100bcd48f798d04b9463bd72680af7202f331de566321b1c29f5f8710c2c44c` |
| `packages/cli/src/config/sandboxConfig.ts` @ `v0.56.0` | env>argv/settings; backend membership; platform gates | 2026-08-27 | `08c340670d3b85b827047784f89d697903db58b994efee430f0218286a1b029f` |
| `packages/cli/src/config/sandboxConfig.test.ts` @ `v0.56.0` | precedence and rejection unit evidence | 2026-08-27 | `efe1b21f210f9e410304a00828969ff2ae837479617edde4f43b4ad4f38f9a64` |
| `packages/cli/src/config/settingsSchema.ts` @ `v0.56.0` | `tools.sandbox` schema and object form | 2026-08-27 | `df5e1939dd6313ffbe0e1e182af83efbf9e55bca99482e0223faf9b5bbe93e6d` |
| `packages/cli/src/gemini.tsx` @ `v0.56.0` | load then `start_sandbox` re-exec before ordinary path | 2026-08-27 | `1ac297b1af4cca39f358fc5c90c18059e275c4c393cfba152f73215e03ead828` |
| `packages/cli/src/utils/sandbox.ts` @ `v0.56.0` | backend spawn, image ensure, seatbelt/docker/podman/lxc | 2026-08-27 | `df009ae28efa17347101731334486a3bb15f3aabe48d61813dc3dd76f3cb7c3f` |
| `packages/cli/src/utils/sandboxUtils.ts` @ `v0.56.0` | seatbelt profile names and container helpers | 2026-08-27 | `f3662d82e0e83405471cf767e4deb29aa1f1ff060b0a14cafb3f1a26d34aa932` |
| `packages/core/src/output/types.ts` @ `v0.56.0` | `init` fields; no sandbox | 2026-08-27 | `23f7ea24497c88a703e0e4f8b6deb8bda969c2c2a32ca213beacfae46d798341` |
| `packages/cli/src/nonInteractiveCli.ts` @ `v0.56.0` | headless stream emission boundary | 2026-08-27 | `fe569c4ac3436a851c991e0e916554b63bd9b9eb0bf5dee644f9258fad5ba298` |
| Research 182 `0.56.0` identity | qualified range and access boundary | 2026-08-22 | promoted |
| Research 230 headless thinking | settings/process seams; sandbox not qualified | 2026-08-27 | promoted |
| Frozen fixture `gemini-cli-0.56.0/protocol.json` | selected argv omits sandbox; exit `44` mapped | 2026-08-22 | workspace |
| Prepared guide `gemini-cli-prepared-integration.md` | route does not force sandbox; promotion bar | 2026-08-27 | workspace |
| Adapter audit `headless_command.rs` / `headless_validation.rs` / `terminal.rs` | omission and ambient harness posture | 2026-08-27 | workspace |

HTML digests identify retrieved documentation bodies. They are not a
compatibility guarantee. Narrative findings below use the `0.56.0` ceiling;
decisive loader, argv, lifecycle, precedence-test, and `InitEvent` bytes are
identical at every published point in the qualified window.

## Configuration Surface

| Seam | Exact shape through published `0.51.0..=0.56.0` |
| --- | --- |
| argv | `--sandbox` / `-s` is yargs `boolean` only; no typed backend value on argv |
| env | `GEMINI_SANDBOX=true\|false\|1\|0\|docker\|podman\|sandbox-exec\|runsc\|lxc\|windows-native` |
| settings | `tools.sandbox` may be boolean, command string, profile path, or object `{enabled,command,image,allowedPaths,networkAccess}` |
| image env | `GEMINI_SANDBOX_IMAGE` then `GEMINI_SANDBOX_IMAGE_DEFAULT` then settings image then package.json `config.sandboxImageUri` |
| already inside | `SANDBOX` set → loader returns no sandbox command |
| seatbelt profile | `SEATBELT_PROFILE`; default `permissive-open` |
| related env | `SANDBOX_FLAGS`, `SANDBOX_MOUNTS`, `SANDBOX_ENV`, `SANDBOX_SET_UID_GID`, `BUILD_SANDBOX`, `GEMINI_SANDBOX_PROXY_COMMAND` |

Official docs claim enablement order flag → env → settings. Tagged
`sandboxConfig.ts` states and implements the opposite for the env seam: non-empty
`GEMINI_SANDBOX` replaces the argv/settings value before normalization. Unit
test `should prioritize GEMINI_SANDBOX over CLI and settings` freezes that
behavior. CLI boolean false still beats a settings object when env is unset.

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
rejection, not portable preflight on the Swallowtail plan surface and not proof
that no credential or auth work already occurred earlier in `gemini.tsx`.

## Lifecycle And Truth Layers

When sandbox config resolves, `gemini.tsx` reads stdin if needed, injects it into
re-exec argv, calls `start_sandbox()`, then exits. `start_sandbox()` spawns the
selected backend, may ensure/pull a container image, and sets in-sandbox
`SANDBOX=...`. Ordinary headless stream-json then runs only inside that child.

| Layer | Exact availability on qualified headless |
| --- | --- |
| Requested | no Swallowtail portable sandbox request surface |
| Argv/environment-encoded | native seams exist; qualified route encodes neither |
| Backend-started | requires `start_sandbox()` and host backend/image readiness |
| Accepted | no stream-json acceptance field |
| Effective | requires observing the re-exec/backend child; not prompt-free on the parent |
| Contained | not inferred from flag/env selection |
| Observed | exit `44` maps sandbox failure only; success has no sandbox init field |

`InitEvent` fields remain `type`, `timestamp`, `session_id`, `model`. Official
headless docs list the same event families and name no sandbox control.

## Prepared Route Audit

Qualified argv from `headless_command.rs`:

```text
--output-format stream-json
--model <explicit-model>
--approval-mode plan
--extensions none
--allowed-mcp-server-names ""
--skip-trust
--session-id <session-id>
```

No `--sandbox`. Validation requires `HarnessConfigurationPosture::Ambient` and
`HarnessIsolation::AmbientHost`. The process launch reuses one host-owned
`EnvironmentRef` and injects no `GEMINI_SANDBOX`, no sandbox disable, and no
isolated settings path for sandbox keys. The prepared guide states the route
does not force Gemini's separate sandbox and that sandbox claims need exact
surface evidence before promotion.

Omission therefore keeps current argv/environment and makes no sandbox,
backend, containment, or isolation claim. Ambient `GEMINI_SANDBOX` or
`settings.tools.sandbox` can still enable sandboxing without a Swallowtail
request. Because env beats argv, a hypothetical later `--sandbox` encoding would
still lose to ambient `GEMINI_SANDBOX` under the current ambient-host posture.

Enterprise Developer API-key access and disabled extensions/MCP stay unchanged.
Consumer login stays out.

## Why No Deliver-Now Row Survives

| Gate | Finding |
| --- | --- |
| Closed version membership | eight published points in `0.51.0..=0.56.0` enumerated with tag/commit; unpublished patch holes recorded; decisive loader/argv/lifecycle/init seams identical |
| Closed platform/backend/value | backend table exists in identical `sandboxConfig.ts`, yet effective membership depends on installed commands, running daemons/containers, and images |
| Process-private precedence | `GEMINI_SANDBOX` overrides argv/settings at every published point; ambient settings apply when env and argv omit; qualified route injects neither |
| Pre-effect rejection | invalid/missing backends throw in the loader, but that is not a Swallowtail plan preflight and backend start itself is out of bounds |
| Prompt-free activation | stream-json has no sandbox field at every published point; confirming the backend requires starting it or inspecting inside it |
| No backend install/start | docker/podman/runsc/lxc need installed backends; `start_sandbox` may pull images; Windows native mutates ACL state per docs |
| Selection ≠ containment | official security notes and Contract 037 posture forbid treating a flag as portable isolation |

## Deliver-Now Set

Empty. For every published executable version in `0.51.0..=0.56.0`, no
platform / backend / value row meets all gates: process-private selection on
the qualified ambient route, closed host-independent membership, pre-effect
rejection without backend work, and prompt-free activation confirmation.

## Withheld Surfaces (Not Empty-Set Reasons Alone)

| Surface | Disposition |
| --- | --- |
| `security.toolSandboxing` | separate tool-level sandboxing; not the full-process `--sandbox` lane |
| Seatbelt profiles / `SEATBELT_PROFILE` | macOS profile detail after backend selection |
| Sandbox expansion UI | interactive permission expansion; not headless confirmation |
| `GEMINI_CLI_HOME` / system settings redirects | settings isolation seams from Research 230; not bound here for sandbox |
| Exit code `44` | failure class only |
| Gemini ACP or Live | sibling routes; no promotion |

## Continuation

Card 240 closes with an honest empty set. Production sandbox binding on
`gemini-cli.headless` stays blocked until a bounded child env/settings contract
can win over ambient configuration, reject unsupported
version/platform/backend/value rows before process and backend work, and expose
prompt-free activation evidence without treating selection as containment.

## Evidence

- [sandbox-evidence.json](../../crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-headless-0.56.0-sandbox/sandbox-evidence.json)
