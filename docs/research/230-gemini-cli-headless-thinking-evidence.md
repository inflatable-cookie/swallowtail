# 230 Gemini CLI Headless Thinking Evidence

Status: promoted; empty deliver-now
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.082 / 229

## Question

Which exact `gemini-cli.headless` version, model, and thinking-configuration
rows can be bound process-privately through the enterprise API-key route with
pre-effect rejection and exact dispatch or effective confirmation?

## Decision

No. Research 230 admits an empty deliver-now set. No typed thinking binding is
admitted on `gemini-cli.headless`.

Exact `@google/gemini-cli@0.56.0` exposes thinking only through ambient
`settings.modelConfigs` resolution into `generateContentConfig.thinkingConfig`.
Official generation-settings documentation names `thinkingBudget` and
`includeThoughts` as SDK fields. Tagged source shows Gemini 2.5 chat aliases
inherit `thinkingBudget: 8192` while Gemini 3 chat aliases inherit
`thinkingLevel: HIGH`. There is no headless argv or env seam for caller-selected
thinking. The qualified stream-json transport exposes no requested, dispatched,
or effective thinking field before or during a run.

## Method And Boundary

Official headless, settings, and generation-settings documentation plus exact
public GitHub source for tag `v0.56.0` were inspected on 2026-08-27. Decisive
configuration, settings-loading, model-config, stream-json, and prepared-route
blobs were fetched from commit `b6e23a7dc29eb15fede4bbe646d91869e948b45a`.
`@google/genai@1.30.0` type definitions were inspected for `ThinkingConfig` and
`ThinkingLevel`. No Gemini install, executable launch, OAuth, credential capture,
provider prompt, or paid inference was used.

Sibling Gemini Live thinking evidence (Research 193) and Gemini ACP session
options do not promote to this route.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [Headless mode](https://geminicli.com/docs/cli/headless/) | stream-json event families; no thinking control | 2026-08-27 | `7badcdfa83d7b8c60f510ab9f40c847d80a96a87050fd3ddd02ce3fef3e1746c` |
| [Settings](https://geminicli.com/docs/cli/settings/) | `ui.inlineThinkingMode` is display-only | 2026-08-27 | `19ea962aa8dda0572e52c1a0efbb9594fc5a3e69512983c2163238380efcf011` |
| [Advanced model configuration](https://geminicli.com/docs/cli/generation-settings/) | `modelConfigs` aliases/overrides; `thinkingConfig` vocabulary | 2026-08-27 | `1fc2317c961900542679d4e9f6972371076b8ede3dc8c7fc44bd2a8de4d53d06` |
| GitHub `docs/cli/generation-settings.md` @ `v0.56.0` | tagged doc cross-check | 2026-08-27 | `06d1a138349ec1c97202936557042c87de992e729268cda82c7f83d6b3356a3f` |
| npm `@google/gemini-cli@0.56.0` identity | qualified package point | 2026-08-22 | Research 182 |
| `packages/cli/src/config/config.ts` @ `v0.56.0` | argv surface; no thinking flag; passes `settings.modelConfigs` | 2026-08-27 | `5100bcd48f798d04b9463bd72680af7202f331de566321b1c29f5f8710c2c44c` |
| `packages/cli/src/config/settings.ts` @ `v0.56.0` | settings precedence; `GEMINI_CLI_SYSTEM_*` paths | 2026-08-27 | `31b771bc8b7960f0cb6f9aa347378af6973a1054665caa0fabf7b3836940bba3` |
| `packages/cli/src/config/settingsSchema.ts` @ `v0.56.0` | `modelConfigs` schema; legacy `thinkingBudget` migration | 2026-08-27 | `df5e1939dd6313ffbe0e1e182af83efbf9e55bca99482e0223faf9b5bbe93e6d` |
| `packages/core/src/config/defaultModelConfigs.ts` @ `v0.56.0` | built-in alias defaults for thinking | 2026-08-27 | `84d9f2230d9bec00ade567ef760df9b1645dfcf74917d3cac1f1e78c2f5f173b` |
| `packages/core/src/services/modelConfigService.ts` @ `v0.56.0` | alias/override resolution pipeline | 2026-08-27 | `e3dfc0e1133bd4153d1c4557e6c39b1f577a08504e689b0e26b67cc228ac019b` |
| `packages/core/src/config/models.ts` @ `v0.56.0` | `DEFAULT_THINKING_MODE = 8192`; model resolution hooks | 2026-08-27 | `b7ec5ce10bc4164ca2efb2aecb2fdcd7d1afeb29c17082ab45537c51a61ca0dd` |
| `packages/cli/src/nonInteractiveCli.ts` @ `v0.56.0` | headless stream emission; thought-only invalid stream class | 2026-08-27 | `fe569c4ac3436a851c991e0e916554b63bd9b9eb0bf5dee644f9258fad5ba298` |
| `packages/core/src/output/types.ts` @ `v0.56.0` | `init` event fields | 2026-08-27 | `23f7ea24497c88a703e0e4f8b6deb8bda969c2c2a32ca213beacfae46d798341` |
| `packages/core/src/core/geminiChat.ts` @ `v0.56.0` | thought parts filtered before assistant emission | 2026-08-27 | `509d03416ad093b15fdf1fa891ccc719591a4d0a02976f9086b5546033e64873` |
| `@google/genai@1.30.0` `ThinkingConfig` / `ThinkingLevel` | SDK vocabulary at Gemini CLI core dependency | 2026-08-27 | npm tarball inspection |
| Research 182 `0.56.0` identity | separate ACP/headless qualification through `0.56.0` | 2026-08-22 | promoted |
| Research 045 headless corpus | historical invocation/event boundary | 2026-07-28 | promoted |
| Frozen fixture `gemini-cli-0.56.0/protocol.json` | selected argv absent thinking flags | 2026-08-22 | workspace |
| Prepared guide `gemini-cli-prepared-integration.md` | route rejects reasoning selection | 2026-08-27 | workspace |
| Adapter audit `headless_command.rs` / `headless_validation.rs` | omission and preflight rejection posture | 2026-08-27 | workspace |

HTML digests identify retrieved documentation bodies. They are not a
compatibility guarantee. Findings below are source-level for the exact tagged
package point.

## Configuration Surface

Thinking is not a headless argv dimension. The frozen qualified argv carries
explicit model, stream-json output, plan approval, disabled extensions/MCP,
skip-trust, and one session id only. Official headless documentation lists
`init`, `message`, `tool_use`, `tool_result`, `error`, and `result` events and
names no thinking-control flag.

Generation configuration lives under durable settings:

| Layer | Mechanism | Thinking relevance |
| --- | --- | --- |
| argv / env | `--model`, `GEMINI_MODEL`; no thinking flag or env | not selectable at CLI boundary |
| settings merge | schema defaults → system defaults → user → workspace → system | `modelConfigs` and UI display keys participate |
| alias chain | `customAliases` + built-in aliases | merges `generateContentConfig.thinkingConfig` |
| overrides | `overrides` + `customOverrides` | match on model alias/name and optional `overrideScope` |
| runtime hooks | `registerRuntimeModelOverride` | model-fallback rotation only; not a public headless seam |

Built-in defaults at `0.56.0`:

| Alias | Inherited thinking defaults |
| --- | --- |
| `chat-base` | `includeThoughts: true` |
| `chat-base-2.5` | `thinkingBudget: 8192` (`DEFAULT_THINKING_MODE`) |
| `chat-base-3` | `thinkingLevel: HIGH` |

Concrete model aliases (`gemini-2.5-pro`, `gemini-2.5-flash`, `gemini-3.1-pro-preview`, etc.) inherit through the matching chat-base family. `modelDefinitions.features.thinking` records static capability metadata but does not itself dispatch a caller-selected value.

SDK vocabulary at `@google/genai@1.30.0`:

| Field | Meaning |
| --- | --- |
| `includeThoughts` | include thought parts in generation |
| `thinkingBudget` | token budget; `0` disabled, `-1` automatic |
| `thinkingLevel` | `THINKING_LEVEL_UNSPECIFIED`, `LOW`, `HIGH` |

This is not the Gemini Live `minimal|low|medium|high` vocabulary. Live evidence
does not promote here.

## Settings Precedence And Child Isolation

`loadSettings()` always reads user and workspace settings from disk, then merges
with optional system defaults and system overrides. Precedence for single values
is: schema defaults, system defaults, user, workspace, system (last wins).

Child-only env vars can redirect the system layer without mutating host files:

- `GEMINI_CLI_SYSTEM_SETTINGS_PATH`
- `GEMINI_CLI_SYSTEM_DEFAULTS_PATH`

That redirect does not skip ambient reads. User and workspace files still load,
and `modelConfigs.customOverrides` from ambient settings append to the override
list before alias resolution. Swallowtail's prepared headless route uses
`HarnessConfigurationPosture::Ambient` and passes one host-owned
`EnvironmentRef`; it does not inject temporary settings files or raw env keys
for thinking.

Therefore no caller-bound, process-private seam overrides ambient settings
without reading host configuration.

## Prepared Route Audit

Swallowtail's qualified headless command builder passes no thinking setting.
Preflight validation rejects any non-empty portable `reasoning_mode` before
process work. The prepared guide states both Gemini CLI branches reject
reasoning selection.

Omission therefore preserves the current argv/environment and makes no portable
selection claim. It does not prove provider-side thinking is absent: built-in
alias defaults and ambient `modelConfigs` may still apply thinking internally.

Configured, dispatched, effective, and observed thinking remain distinct:

| Layer | Headless availability |
| --- | --- |
| Requested | Swallowtail rejects portable reasoning requests |
| Configured | ambient settings + built-in alias defaults; not adapter-bound |
| Dispatched | no argv/env dispatch surface on qualified route |
| Effective | requires provider request inspection; not prompt-free |
| Observed | no; stream-json excludes thought content from assistant events |

The `init` event exposes only `session_id` and resolved `model`. Assistant
`message` events carry stripped text content; thought parts are filtered before
emission. A `THINKING_ONLY_RESPONSE` invalid-stream class exists, but detecting
it requires a provider prompt and is not selected-value confirmation.

## Model And Value Membership

Exact model/value rows cannot be closed without authenticated resolution context:

- `resolveModelId()` and classifier resolution depend on `hasAccessToPreview`,
  experiment flags, and `useGemini3_1` / `useGemini3_5Flash` runtime facts.
- Preview downgrade paths rewrite requested preview models to stable fallbacks
  when preview access is unavailable.
- `features.thinking` is static registry metadata; the effective field
  (`thinkingBudget` vs `thinkingLevel`) depends on alias family, not a portable
  Swallowtail vocabulary.

Unsupported thinking values are not rejected before spawn. There is no CLI
parse surface. Settings validation may warn on malformed JSON, but thinking
values are not preflight-bound to model capability without a provider round trip.

## Why No Deliver-Now Row Survives

| Gate | Finding |
| --- | --- |
| Process-private seam | thinking is settings-backed; ambient user/workspace files always load |
| No-substitution | built-in alias defaults apply even when Swallowtail omits portable reasoning |
| Ambient shadowing | route posture is ambient; adapter cannot fail closed against host `modelConfigs` |
| Selected-model agreement | effective config depends on alias family and runtime resolution context |
| Pre-effect rejection | no argv/env thinking surface; unsupported values are not rejected before process work |
| Confirmation | stream-json exposes no thinking field; reasoning output is not selected-value proof |
| Transport ownership | `ProcessRequest` carries opaque `EnvironmentRef`; adapter sets no thinking env or settings injection |

## Deliver-Now Set

Empty. No executable-version / selected-model / portable-value row meets all
gates: exact process-local transport without ambient override, selected-model
agreement at preparation, pre-spawn rejection for unsupported values, and
prompt-free dispatch or effective confirmation on `gemini-cli.headless`.

## Withheld Surfaces (Not Empty-Set Reasons Alone)

| Surface | Disposition |
| --- | --- |
| `settings.modelConfigs.customAliases` / `customOverrides` | ambient or enterprise-managed configuration; not adapter-bound on current route |
| `GEMINI_CLI_SYSTEM_SETTINGS_PATH` | child env redirect; still reads ambient user/workspace settings |
| `ui.inlineThinkingMode` | display-only UI setting |
| `--model` alias selection | selects alias chain, not an independent portable thinking value |
| Gemini ACP negotiated model options | sibling route; no promotion |
| Gemini Live thinking levels | sibling route; different vocabulary and transport |
| `registerRuntimeModelOverride()` | internal fallback hook; not a public headless binding |

## Continuation

Card 229 closes with an honest empty set. Production reasoning binding on
`gemini-cli.headless` stays blocked until Gemini CLI exposes a headless-local,
prompt-free confirmation transport or Swallowtail gains a bounded,
fail-closed child settings contract that proves thinking agreement before spawn
without ambient host shadowing.

## Evidence

- [thinking-evidence.json](../../../crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-headless-0.56.0-thinking/thinking-evidence.json)
