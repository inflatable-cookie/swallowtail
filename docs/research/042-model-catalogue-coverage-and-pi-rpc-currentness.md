# 042 Model Catalogue Coverage And Pi RPC Currentness

Status: promoted
Owner: Tom
Date: 2026-07-27

Qwen and Alibaba dispositions and the resulting counts are superseded by
[Research 043](043-model-catalogue-no-closure-audit.md).

## Question

Which current `model_catalog = No` entries lack an upstream discovery surface,
and which only lack Swallowtail adapter coverage?

## Method

Exact tagged harness sources, current maintained-project documentation,
provider API references, frozen Swallowtail corpora, and the 21-row solution
matrix were checked. No executable, credential, account, provider request,
model invocation, or paid operation was used.

## Pi Correction

Pi RPC has exposed `get_available_models` since Swallowtail's exact qualified
`0.80.10` point. It returns full model objects for all configured models.
Those objects include provider and model ids, display name, API family,
reasoning support, input modalities, context window, output limit, and cost
metadata.

Pi's effective catalogue is configuration and access sensitive:

- built-in provider catalogues
- refreshed configured-provider catalogues
- user-defined providers and models from `models.json`
- authentication presence used by Pi's availability filtering

Swallowtail must therefore ask the prepared Pi instance. It must not maintain
a parallel provider/model allowlist. Catalogue discovery does not select a
model or prove that a later invocation will be accepted.

The current maintained package is `0.82.1`. The RPC catalogue command is
unchanged at `0.80.10`, `0.81.1`, and `0.82.1`; `0.82.1` adds a separate
`get_available_thinking_levels` command. The first implementation can retain
`0.80.10` as its guaranteed point and admit later stable versions through the
existing visible unverified-newer posture. Later qualification should add
`0.81.1` and `0.82.1` milestones without raising the baseline.

Evidence:

- [Pi `0.80.10` RPC model commands](https://github.com/earendil-works/pi/blob/v0.80.10/packages/coding-agent/docs/rpc.md#get_available_models)
- [Pi `0.80.10` model object](https://github.com/earendil-works/pi/blob/v0.80.10/packages/coding-agent/docs/rpc.md#model)
- [Pi `0.82.1` RPC model commands](https://github.com/earendil-works/pi/blob/v0.82.1/packages/coding-agent/docs/rpc.md#get_available_models)
- [Pi custom model configuration](https://github.com/earendil-works/pi/blob/v0.82.1/packages/coding-agent/docs/models.md)
- [current Pi package](https://www.npmjs.com/package/@earendil-works/pi-coding-agent)

## Catalogue Source Classes

The common catalogue role may use four source classes:

| Source class | Example | Discovery timing |
| --- | --- | --- |
| dedicated harness catalogue | Pi `get_available_models` | before session; ephemeral harness process allowed |
| attached harness or runtime inventory | OpenCode, Ollama, Kimi local server | before session; bound attached endpoint |
| hosted provider catalogue | Anthropic Models, OpenAI Models, Gemini Models, xAI language models | before inference; bound hosted endpoint and credential |
| cloud control plane | Bedrock `ListFoundationModels` | before inference; separate regional control-plane authority |

ACP session configuration is different. Stable ACP config options may advertise
model selectors in `session/new`, `session/load`, or later session updates.
That is useful negotiated session evidence, but it is not a side-effect-free
pre-session catalogue. Swallowtail must not create a provider session solely
to populate a model picker unless a later contract explicitly authorizes that
lifecycle and retention.

## Current `No` Classification

| Solution | Upstream evidence | Swallowtail disposition |
| --- | --- | --- |
| Pi RPC | exact `get_available_models` at `0.80.10` | ready dedicated catalogue |
| Kimi local server | exact authenticated `GET /models` at `0.28.1` and `0.29.0` | ready attached catalogue |
| Gemini Live | authenticated paginated `models.list` with supported actions | ready separate hosted catalogue branch |
| OpenAI Realtime | authenticated `GET /v1/models` | ready shared OpenAI catalogue branch; no Realtime capability inference |
| OpenAI background Responses | authenticated `GET /v1/models` | ready shared OpenAI catalogue branch; no background capability inference |
| xAI Responses WebSocket | key-scoped `/v1/models` and `/v1/language-models` | ready hosted catalogue branch |
| Gemini CLI ACP | `session/new` advertises available models | negotiated session evidence; not standalone catalogue |
| Kimi Code ACP | model config option advertises available models | negotiated session evidence; not standalone catalogue |
| Claude Agent ACP | caller supplies `availableModels`; agent returns the constrained selector | not discovery |
| Qwen Code headless | `/model` reads configured provider models; no qualified machine-readable headless listing | separate interface evidence required |
| Alibaba Model Studio Conversations | region and workspace model lists are documented; no qualified callable inventory for the selected route | separate provider evidence required |
| Anthropic Managed Agents | operator-owned agent version owns model configuration | not applicable to the selected managed-agent operation |
| llama.cpp owned lifecycle | serving-only operation returns an endpoint for another inference route | not applicable; attached route already owns catalogue |

This yields six definite solution-row conversions after implementation: Pi,
Kimi local server, Gemini Live, both OpenAI solutions, and xAI. The two ACP
solutions can expose negotiated model options without being mislabeled as
standalone catalogue drivers.

## Runtime Lifecycle Gap

The portable catalogue request carries a host-monotonic deadline but no
independent consumer cancellation control. Dropping the returned future cannot
claim cancellation because it cannot await owned cleanup. Pi can therefore
prove deadline-triggered process stop and join now; portable catalogue
cancellation needs a later shared runtime contract before any adapter claims
it.

Provider references:

- [Kimi `0.28.1` model catalogue route](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.28.1/packages/kap-server/src/routes/modelCatalog.ts)
- [Kimi `0.29.0` model catalogue route](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.0/packages/kap-server/src/routes/modelCatalog.ts)
- [ACP session config options](https://agentclientprotocol.com/announcements/session-config-options-stabilized)
- [Gemini Models API](https://ai.google.dev/api/models)
- [OpenAI Models API](https://platform.openai.com/docs/api-reference/models)
- [xAI Models API](https://docs.x.ai/developers/rest-api-reference/inference/models)
- [Qwen configured model providers](https://qwenlm.github.io/qwen-code-docs/en/users/configuration/model-providers/)
- [Alibaba regional model list](https://help.aliyun.com/en/model-studio/models)

## Promotion

- Contract 020 distinguishes standalone catalogue observations from
  session-negotiated model options and serving-only non-applicability.
- Architecture records Pi catalogue discovery as a separate prepared operation.
- Roadmap g02.021 sequences Pi first, Kimi local second, ACP negotiated options
  separately, then shared hosted-provider catalogue branches.
