# 025 Installed Harness Compatibility And Codex Range Selection

Status: promoted
Owner: Tom
Updated: 2026-07-23

## Question

Which installed harness route should receive the first maintained compatibility
retrofit, and what exact release points can enter its qualification corpus?

## Method

Sources were accessed 2026-07-23. Evidence includes official documentation,
maintained release records, tagged source, generated schema artifacts, package
registries, local executable version output, and existing Swallowtail fixtures.
No login, credential, provider request, binary installation, container, model
download, or consumer edit was used.

The audit keeps executable release, wire protocol, behavior revision, protocol
facade, model catalogue, access, configured instance, and adapter release
separate.

## Installed Harness Posture

| Route | Swallowtail point | Latest maintained release | Runtime observation | Claim posture |
| --- | --- | --- | --- | --- |
| Codex exec | source and local evidence `0.144.6`; no binding | `0.145.0`, 2026-07-21 | `codex --version` exists; driver has no discovery role or runtime check | unqualified |
| Codex app-server | source and local evidence `0.144.6`; v2 facade | `0.145.0`, 2026-07-21 | same executable probe; protocol handshake has no release field | unqualified |
| OpenCode HTTP | exact `1.14.48` fixture and rejection | `1.18.4`, 2026-07-20 | `/global/health` returns exact server version | exact-only; no descriptor claim |
| Gemini CLI ACP | CLI `0.51.0`; ACP v1; schema `v1.19.0` | `0.52.0`, 2026-07-22 | `--version` probe and `agentInfo.version` handshake check | exact-only; no descriptor claim |
| Kimi Code ACP | CLI `0.28.1`; ACP v1; schema `v1.19.1` | `0.29.0`, 2026-07-22 | `--version` probe and `agentInfo.version` handshake check | exact-only; no descriptor claim |
| Qwen Code headless | package `0.19.11` | `0.20.1`, 2026-07-21 | first stream event carries `qwen_code_version` | exact-only; no descriptor claim |
| Pi RPC | package `0.80.10` | `0.81.1`, 2026-07-21 | configured binding only; no production version probe | one-point descriptor claim |

The local host currently exposes Codex `0.145.0` and OpenCode `1.14.48`.
Gemini, Kimi, Qwen, and Pi are absent. Package and release evidence does not
promote their latest releases into support.

Kimi Code here means the maintained TypeScript
`MoonshotAI/kimi-code` route. The separately released Python
`MoonshotAI/kimi-cli` line is not an interchangeable executable or compatibility
axis.

Research 024's Codex `0.144.6` point is stale. It remains a useful checkpoint,
but `0.145.0` is the latest stable release. All `0.146.0-alpha.*` releases are
prereleases and stay rejected.

## Shared Discovery Gap

The routes expose three different incomplete patterns:

- OpenCode, Gemini, Kimi, and Qwen observe exact versions only after a
  production process or connection effect starts.
- Pi binds an exact version in preflight but does not prove the selected
  executable has that version.
- Codex does neither.

`DiscoveryOutcome` currently carries only status and a safe diagnostic.
`DiscoveryRequest` carries only an execution-host id. It cannot ask a driver to
probe one explicit host-approved executable target, return an exact interface
version binding, classify that point against the driver claim, or bind a
deadline.

The first retrofit therefore needs a narrow shared contract:

- the host supplies one explicit executable candidate; drivers do not search,
  install, upgrade, or fall back through ambient `PATH`
- the process host runs a bounded, joined, non-authenticating version command
- the result carries safe exact interface bindings and compatibility
  classification without returning a path or raw stdout
- discovery remains a candidate observation; it does not create or promote a
  configured instance
- absence, malformed output, unsupported release, timeout, cancellation, and
  cleanup failure remain distinct
- remote-authoritative hosts run the probe where the executable will run

Codex, Pi, and later installed-harness retrofits can share that mechanism.
Provider-specific version parsing stays inside each adapter.

## Codex Exec Evidence

Official current documentation defines `codex exec` as the non-interactive
surface and `--json` as newline-delimited state-change events. The tagged
`exec_events.rs` subset used by Swallowtail—turn start, completed item, agent
message, reasoning, web search, turn completion, failure, and token usage—is
present from `0.80.0` through `0.145.0`.

That wire continuity does not prove invocation continuity:

| Point | Evidence | Consequence |
| --- | --- | --- |
| `0.80.0` | JSONL, image, schema, sandbox, model, and skip-repository flags exist | historical corpus candidate only |
| `0.99.0` | `--ephemeral` and the current top-level web-search mode appear | legacy behavior milestone |
| `0.121.0` | no `--ignore-user-config` or `--ignore-rules` | rejection point for the current isolated invocation |
| `0.122.0` | both isolation flags are introduced | first current-behavior baseline candidate |
| `0.130.0` | stable interior point | corpus checkpoint |
| `0.140.0` | stable interior point | corpus checkpoint |
| `0.144.6` | prior Swallowtail implementation point | regression checkpoint |
| `0.145.0` | latest stable release | latest-qualified candidate |
| `0.146.0-alpha.4` | current prerelease | explicit rejection point |

Swallowtail currently depends on both `0.122.0` isolation flags. Omitting them
can allow user configuration or rules to alter a supposedly bounded structured
run. A `0.80.0` through `0.121.0` claim therefore needs a separate legacy
behavior revision with a proven isolated configuration home or equivalent
host mechanism. It cannot be inferred from stable JSONL.

The first exec corpus must freeze `0.122.0`, `0.130.0`, `0.140.0`, `0.144.6`,
and `0.145.0`. It must reject `0.121.0`, `0.146.0-alpha.4`, malformed versions,
and unknown newer releases. Only after all checkpoints pass may the descriptor
publish `0.122.0..=0.145.0`.

## Codex App-Server Evidence

Official documentation fixes stdio as newline-delimited JSON-RPC without the
`jsonrpc` header. It also states that generated TypeScript and JSON Schema
outputs are specific to the Codex version that produced them.

The v2 surface has a later honest floor than the six-month device target:

| Point | Evidence | Consequence |
| --- | --- | --- |
| `0.107.0` | no flat v2 schema bundle | rejection point for this driver |
| `0.108.0`, `0.109.0` | source tags exist but no published stable releases | excluded |
| `0.110.0` | v1 RPC methods removed; flat v2 bundle published | first v2 baseline candidate |
| `0.120.0` | core selected methods and fields remain present | corpus checkpoint |
| `0.131.0` | experimental runtime workspace roots appear | behavior milestone |
| `0.140.0` | core selected methods remain present | corpus checkpoint |
| `0.144.6` | prior Swallowtail implementation point | regression checkpoint |
| `0.145.0` | latest stable release | latest-qualified candidate |
| `0.146.0-alpha.4` | current prerelease | explicit rejection point |

The stable selected subset remains present at every published checkpoint:
`initialize`, `model/list`, `thread/start`, `thread/resume`, `turn/start`,
`turn/interrupt`, turn start/completion, item completion, and agent-message
deltas. Required model catalogue fields used by Swallowtail are present at
`0.110.0`.

Experimental fields require `capabilities.experimentalApi = true`. Swallowtail
uses experimental dynamic tools and runtime workspace roots, but its current
mock server does not enforce the gate. The production code can emit
`runtimeWorkspaceRoots` without enabling the capability, and always emits the
later experimental `allowProviderModelFallback: false` field even though false
is the protocol default.

The first app-server corpus must therefore include stable and experimental
schema bundles plus a gate-enforcing server fixture at `0.110.0`, `0.120.0`,
`0.131.0`, `0.140.0`, `0.144.6`, and `0.145.0`. It must prove:

- stable read-only sessions without experimental fields
- explicit experimental opt-in whenever dynamic tools, user-input callbacks,
  or runtime workspace roots are sent
- private dispatch around the `0.131.0` workspace-root milestone
- default-false model fallback without an unnecessary experimental field
- unknown additive notifications remain progress snapshots
- protocol errors, malformed frames, cancellation, deadlines, and joined close
  remain stable

Only after those points pass may the app-server descriptor publish a
`0.110.0..=0.145.0` executable window. The behavior segments are
`0.110.0..=0.130.0` and `0.131.0..=0.145.0`. The protocol facade remains
`codex-app-server-v2`; it is not replaced by the executable version.

## Selection

Codex remains the first retrofit. It has two materially different transports,
an exact local version probe, official version-specific schemas, frequent
releases, and a demonstrated installed-device age problem.

The work is one tranche with separate claims:

1. add explicit installed-executable observation records and discovery bounds
2. freeze the exec and app-server multi-release corpora
3. publish separate executable claims and private behavior dispatch
4. run both drivers through existing conformance plus version rejection

The January-to-current six-month objective remains explicit, not silently
abandoned. The first honest windows reach April for exec and March for
app-server. A continuation audit must test:

- exec `0.80.0..=0.121.0` with isolated state and the `0.99.0` milestone
- legacy app-server before the published v2-only floor, likely as a separate
  driver or a documented unsupported span

That continuation cannot block the current v2/current-isolation proof and
cannot be folded into its claims from release ordering alone.

## Risks

- app-server is documented as a development/debug surface that may change
  without notice
- experimental schema fields can move independently of stable v2 methods
- a successful `--version` probe does not prove authentication, model access,
  catalogue freshness, or entitlement
- the exact Codex executable can expose different model catalogues by account,
  provider, policy, and time
- OpenCode, Gemini, Kimi, Qwen, and Pi latest releases are currently
  unqualified and must continue to fail closed
- moving any baseline later is a later Swallowtail support-policy change, not
  an automatic consequence of upstream releases

## Sources

- [Codex CLI reference](https://developers.openai.com/codex/cli/reference/)
- [Codex app-server protocol](https://developers.openai.com/codex/app-server/)
- [Codex changelog](https://learn.chatgpt.com/docs/changelog)
- [Codex `0.110.0`](https://github.com/openai/codex/releases/tag/rust-v0.110.0)
- [Codex `0.122.0`](https://github.com/openai/codex/releases/tag/rust-v0.122.0)
- [Codex `0.130.0`](https://github.com/openai/codex/releases/tag/rust-v0.130.0)
- [Codex `0.140.0`](https://github.com/openai/codex/releases/tag/rust-v0.140.0)
- [Codex `0.144.6`](https://github.com/openai/codex/releases/tag/rust-v0.144.6)
- [Codex `0.145.0`](https://github.com/openai/codex/releases/tag/rust-v0.145.0)
- [OpenCode `1.18.4`](https://github.com/anomalyco/opencode/releases/tag/v1.18.4)
- [Gemini CLI `0.52.0`](https://github.com/google-gemini/gemini-cli/releases/tag/v0.52.0)
- [Kimi Code `0.29.0`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.29.0)
- [Qwen Code `0.20.1`](https://github.com/QwenLM/qwen-code/releases/tag/v0.20.1)
- [Pi `0.81.1`](https://github.com/earendil-works/pi/releases/tag/v0.81.1)
