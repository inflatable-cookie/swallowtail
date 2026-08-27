# 235 Gemini CLI ACP Thinking Evidence

Status: promoted; empty deliver-now
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.083 / 234

## Question

Which exact `gemini-cli.acp` version, model, value, profile, and lifecycle rows
can bind caller-selected thinking through an ACP-confirmed or process-private
seam without ambient settings authority or provider prompting?

## Decision

No. Research 235 admits an empty deliver-now set. No typed thinking binding is
admitted on `gemini-cli.acp`.

Exact `@google/gemini-cli@0.56.0` applies thinking only through the same
settings-backed `settings.modelConfigs` resolution into
`generateContentConfig.thinkingConfig` used by the interactive client path.
ACP `initialize` and `session/new` expose auth, load-session, prompt media, MCP,
approval modes, and negotiated model ids. They expose no thinking vocabulary,
no `configOptions`, and no `session/set_config_option` handler. The qualified
ACP source does not emit `config_option_update`. `agent_thought_chunk` is
observation during or after prompt work, not selected-value confirmation
before prompt effects.

## Method And Boundary

Official ACP-mode, settings, and generation-settings documentation plus exact
public GitHub source for tag `v0.56.0` were inspected on 2026-08-27. Decisive
ACP dispatcher, session manager, session prompt path, client `sendMessageStream`,
settings-loading, and model-config blobs were fetched from commit
`b6e23a7dc29eb15fede4bbe646d91869e948b45a`. Research 230 settings-loader
evidence was reused only as a cross-route lead; its headless empty-set
conclusion was not promoted. No Gemini install, executable launch, OAuth,
credential capture, provider prompt, or paid inference was used.

Sibling Gemini Live thinking evidence (Research 193) and Gemini headless
thinking evidence (Research 230) do not promote to this route.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [ACP mode](https://geminicli.com/docs/cli/acp-mode/) | listed methods; no thinking control | 2026-08-27 | `f3bfab06bc79fb1e4e41d961d90f45d314c0263185a1bcd70da26b0b3464b5a3` |
| [Settings](https://geminicli.com/docs/cli/settings/) | `ui.inlineThinkingMode` is display-only | 2026-08-27 | `19ea962aa8dda0572e52c1a0efbb9594fc5a3e69512983c2163238380efcf011` |
| [Advanced model configuration](https://geminicli.com/docs/cli/generation-settings/) | `modelConfigs` aliases/overrides; `thinkingConfig` vocabulary | 2026-08-27 | `1fc2317c961900542679d4e9f6972371076b8ede3dc8c7fc44bd2a8de4d53d06` |
| GitHub `docs/cli/acp-mode.md` @ `v0.56.0` | tagged doc cross-check | 2026-08-27 | workspace tag fetch |
| npm `@google/gemini-cli@0.56.0` identity | qualified package point | 2026-08-22 | Research 182 |
| `packages/cli/src/acp/acpRpcDispatcher.ts` @ `v0.56.0` | `GeminiAgent` methods; no `set_config_option` | 2026-08-27 | `0efad1ad0db341802cc27710a5d5ab522666f3cd1a229faece8094a81a992660` |
| `packages/cli/src/acp/acpSessionManager.ts` @ `v0.56.0` | `session/new` models/modes only | 2026-08-27 | `63e38dfcfe035a317acc9e2943810b765e7403ecb6383597633394e8ff214f1e` |
| `packages/cli/src/acp/acpSession.ts` @ `v0.56.0` | prompt dispatch; thought chunks | 2026-08-27 | `f21c78e7edc1b17972b3d3a94ea507788fcc67fdd1bb0fe857cb97218be63e3b` |
| `packages/cli/src/acp/acpUtils.ts` @ `v0.56.0` | negotiated model list only | 2026-08-27 | `3683efd817485ca2772429864d211fbfe4ff0ac48342d445bd616743c1c693bd` |
| `packages/core/src/core/client.ts` @ `v0.56.0` | `isChatModel: true` dispatch path | 2026-08-27 | `6a6935054eedede5561272c9e53026ac50e604721ec9fbed95fe14d615a6676c` |
| `packages/core/src/core/geminiChat.ts` @ `v0.56.0` | resolved `generateContentConfig` dispatch | 2026-08-27 | `509d03416ad093b15fdf1fa891ccc719591a4d0a02976f9086b5546033e64873` |
| `packages/core/src/services/modelConfigService.ts` @ `v0.56.0` | alias/override resolution pipeline | 2026-08-27 | `e3dfc0e1133bd4153d1c4557e6c39b1f577a08504e689b0e26b67cc228ac019b` |
| `packages/core/src/config/defaultModelConfigs.ts` @ `v0.56.0` | built-in alias defaults for thinking | 2026-08-27 | `84d9f2230d9bec00ade567ef760df9b1645dfcf74917d3cac1f1e78c2f5f173b` |
| `packages/cli/src/config/settings.ts` @ `v0.56.0` | settings precedence; child env paths | 2026-08-27 | `31b771bc8b7960f0cb6f9aa347378af6973a1054665caa0fabf7b3836940bba3` |
| `packages/cli/src/config/settingsSchema.ts` @ `v0.56.0` | `modelConfigs` schema | 2026-08-27 | `df5e1939dd6313ffbe0e1e182af83efbf9e55bca99482e0223faf9b5bbe93e6d` |
| Frozen fixture `gemini-cli-0.56.0/protocol.json` | ACP argv and method subset | 2026-08-22 | workspace |
| Frozen fixture `gemini-cli-acp-v0.51.0/activity-range.json` | emitted vs withheld updates | 2026-07-29 | workspace |
| Frozen fixture `acp-v1-gemini-cli-0.51.0/protocol.json` | `config_options: not_advertised` | 2026-07-20 | workspace |
| Prepared guide `gemini-cli-prepared-integration.md` | route rejects reasoning selection | 2026-08-27 | workspace |
| Adapter audit `prepared_profile/session.rs`, `driver/validation.rs` | omission and preflight rejection | 2026-08-27 | workspace |
| Research 230 headless settings lead | settings-loader vocabulary only | 2026-08-27 | promoted sibling |

HTML digests identify retrieved documentation bodies. They are not a
compatibility guarantee. Findings below are source-level for the exact tagged
package point.

## ACP Protocol Surface

ACP thinking is not a wire dimension. The frozen qualified argv is `--acp` with
plan approval only. Official ACP-mode documentation lists `initialize`,
`authenticate`, `newSession`, `loadSession`, `prompt`, and `cancel`. It names no
thinking-control method.

`GeminiAgent` at `v0.56.0` additionally implements `setSessionMode` and
`unstable_setSessionModel`. It does not implement `session/set_config_option` or
any thinking-specific unstable setter.

| Frame | Thinking-relevant fields | Selection seam |
| --- | --- | --- |
| `initialize` result | `authMethods`, `agentCapabilities.loadSession`, prompt and MCP capabilities | none |
| `session/new` result | `sessionId`, `modes`, `models` | model id only via later unstable setter |
| `session/load` result | `modes`, `models` | model id only; history replay may emit thought chunks |
| `unstable_setSessionModel` result | empty object | changes active model alias; no thinking field |
| `setSessionMode` result | empty object | approval mode only |
| `session/prompt` terminal | `stopReason`, optional quota `_meta` | no thinking field |
| `session/update` notifications | may include `agent_thought_chunk` | observation only |

Compared with Kimi Code ACP fixtures, Gemini does not return `configOptions` on
`session/new` and does not emit `config_option_update`. The frozen protocol
fixture records `config_options: not_advertised`.

## Configuration And Dispatch

Thinking configuration still lives under durable settings, identical to the
headless lead in Research 230:

| Layer | Mechanism | Thinking relevance |
| --- | --- | --- |
| argv / env | `--acp`, `--approval-mode`; no thinking flag or env | not selectable at CLI boundary |
| settings merge | schema defaults → system defaults → user → workspace → system | `modelConfigs` participates |
| alias chain | `customAliases` + built-in aliases | merges `generateContentConfig.thinkingConfig` |
| overrides | `overrides` + `customOverrides` | match on model alias/name and optional `overrideScope` |
| ACP model negotiation | `unstable_setSessionModel` → `config.setModel` | selects alias chain, not portable thinking value |

ACP prompt dispatch uses `GeminiClient.sendMessageStream`, which builds
`ModelConfigKey { model, isChatModel: true }` before calling
`geminiChat.sendMessageStream`. Resolved `generateContentConfig`, including any
ambient or alias-default `thinkingConfig`, is therefore applied on the provider
request path. That is internal dispatch truth, not an ACP-visible caller binding.

Built-in alias defaults at `0.56.0` remain:

| Alias | Inherited thinking defaults |
| --- | --- |
| `chat-base` | `includeThoughts: true` |
| `chat-base-2.5` | `thinkingBudget: 8192` |
| `chat-base-3` | `thinkingLevel: HIGH` |

SDK vocabulary at `@google/genai@1.30.0` remains `includeThoughts`,
`thinkingBudget`, and `thinkingLevel` (`THINKING_LEVEL_UNSPECIFIED`, `LOW`,
`HIGH`). This is not the Gemini Live `minimal|low|medium|high` vocabulary.

## Settings Precedence And Child Isolation

ACP `session/new` calls `loadSettings(cwd)` and `loadCliConfig(settings,
sessionId, argv, { cwd })`. Workspace settings therefore follow the bound working
resource. `GEMINI_CLI_HOME` can redirect user settings without mutating the host
home tree. `GEMINI_CLI_SYSTEM_SETTINGS_PATH` and
`GEMINI_CLI_SYSTEM_DEFAULTS_PATH` can supply a last system layer that wins
conflicting keys and can replace the `customOverrides` array.

Swallowtail's prepared ACP route uses `HarnessConfigurationPosture::Ambient` and
passes one host-owned `EnvironmentRef`. It does not inject isolated settings env
keys. The settings seams are source-level candidates only; they are not bound on
the qualified route today.

## Prepared Route Audit

Swallowtail rejects portable `reasoning_mode` at session preparation and at
session open. The prepared guide states both Gemini CLI branches reject
reasoning selection. Negotiated `models` on `session/new` are bounded
observations only; they are not a portable thinking catalogue.

Omission preserves current `--acp` spawn, initialization, session negotiation,
and model-option behavior with no portable reasoning claim. Built-in alias
defaults and ambient `modelConfigs` may still apply thinking internally.

Configured, dispatched, effective, and observed thinking remain distinct:

| Layer | ACP availability |
| --- | --- |
| Requested | Swallowtail rejects portable reasoning requests |
| Negotiated | `session/new` and unstable model setter expose model ids only |
| Configured | ambient settings + built-in alias defaults; not adapter-bound |
| Dispatched | no ACP wire field; internal `generateContentConfig` only |
| Effective | requires provider request inspection; not prompt-free |
| Observed | `agent_thought_chunk` may appear during prompt or history replay |

`agent_thought_chunk` maps provider thought events to ACP `session/update`. The
qualified activity corpus classifies it as `reasoning_summary_candidate` with
provider-display disclosure. It is not selected-value confirmation before prompt
effects. `THINKING_ONLY_RESPONSE` invalid-stream handling exists on the prompt
path but detecting it requires a provider round trip.

## Lifecycle Disposition

| Operation | ACP thinking disposition |
| --- | --- |
| new | no thinking selection or pre-prompt confirmation on wire |
| follow-up | same; each prompt reuses resolved settings/model config |
| load | `streamHistory` may emit stored thoughts; not a caller binding |
| resume | `loadSession` replay only; no portable thinking setter |
| fresh replacement | Swallowtail opens a new `session/new`; no thinking carry |

## Model And Value Membership

Exact model/value rows cannot be closed without authenticated resolution context:

- `buildAvailableModels()` depends on preview access, experiment flags, and
  Gemini 3.1 / 3.5 runtime facts.
- Preview downgrade paths rewrite requested preview models when preview access is
  unavailable.
- `features.thinking` is static registry metadata; the effective field depends
  on alias family, not a portable Swallowtail vocabulary.

Unsupported thinking values are not rejected before spawn on the ACP route. There
is no ACP parse surface for thinking. Settings validation may warn on malformed
JSON, but thinking values are not preflight-bound without a provider round trip.

## Why No Deliver-Now Row Survives

| Gate | Finding |
| --- | --- |
| ACP selection seam | no `configOptions`, no `session/set_config_option`, no thinking unstable setter |
| Negotiated model isolation | model ids are not portable thinking values |
| Process-private seam | generic child settings seams exist in source, but the qualified route binds no thinking argv, env, or temporary settings file |
| No-substitution | built-in alias defaults apply even when Swallowtail omits portable reasoning |
| Ambient shadowing | not unavoidable with a complete last system layer; current route remains ambient |
| Pre-effect rejection | no ACP thinking surface; unsupported values are not rejected before process work |
| Confirmation | no ACP thinking field before prompt; thought chunks are not selection proof |
| Transport ownership | `EnvironmentRef` is opaque; adapter sets no thinking env or settings injection |

## Deliver-Now Set

Empty. No executable-version / selected-model / portable-value / profile /
lifecycle row meets all gates: exact process-local ACP transport without ambient
override, selected-model agreement at preparation, pre-spawn rejection for
unsupported values, and prompt-free dispatch or effective confirmation on
`gemini-cli.acp`.

## Withheld Surfaces (Not Empty-Set Reasons Alone)

| Surface | Disposition |
| --- | --- |
| `settings.modelConfigs.customAliases` / `customOverrides` | ambient or enterprise-managed configuration; not adapter-bound |
| `GEMINI_CLI_HOME` | child env redirect for user settings path; does not isolate workspace settings |
| `GEMINI_CLI_SYSTEM_SETTINGS_PATH` | child env redirect for the last system layer |
| `unstable_setSessionModel` | negotiated model id only; no thinking vocabulary |
| `agent_thought_chunk` | observation / reasoning-summary candidate; not selection proof |
| `ui.inlineThinkingMode` | display-only UI setting |
| Gemini headless thinking settings lead | sibling route; Research 230 empty set not promoted |
| Gemini Live thinking levels | sibling route; different vocabulary and transport |

## Continuation

Card 234 closes with an honest empty set. Production reasoning binding on
`gemini-cli.acp` stays blocked until Gemini CLI exposes an ACP-local,
prompt-free thinking selection and confirmation transport, or Swallowtail gains
a bounded fail-closed child settings contract that proves thinking agreement
before spawn without depending on host configuration.

## Evidence

- [thinking-evidence.json](../../crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-acp-0.56.0-thinking/thinking-evidence.json)
