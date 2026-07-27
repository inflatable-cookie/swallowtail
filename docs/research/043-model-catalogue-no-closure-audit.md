# 043 Model Catalogue No-Closure Audit

Status: promoted
Owner: Tom
Date: 2026-07-27

## Question

Can every solution previously marked `model_catalog = No` report models
through its selected public interface, and does Swallowtail expose every
machine-readable source that actually exists?

## Method

The audit started from every `No` in the 21-row solution matrix. It checked
exact qualified harness tags, current maintained harness sources, official
provider API references, and the selected Swallowtail route boundary. It then
implemented each callable source or assigned a more exact non-catalogue
classification.

No live credential, account, provider request, model invocation, deployment,
or paid operation was used. Provider responses are represented by bounded
deterministic corpora and decoders.

## Correction To Research 042

Research 042 was too conservative for Qwen Code and Alibaba Model Studio.

Qwen Code `0.19.11` and current `0.21.0` expose
`get_available_models` through the documented stream-JSON control plane. The
initialize response advertises `can_get_available_models`; the request returns
model ids, labels, capabilities, and context-window sizes. This is a
machine-readable headless interface, not merely the interactive `/model`
command.

Alibaba Cloud Model Studio added an official deployable-model API on
2026-06-06. `GET /api/v1/deployments/models` lists base or custom deployment
candidates with bounded pagination. It is a cloud-control-plane catalogue,
not proof that a model is invocable through the Conversations route.

Research 042 remains the historical evidence that opened the lane. This
research supersedes only its Qwen and Alibaba dispositions and its resulting
counts.

## Final Disposition

| Former `No` solution | Upstream capability | Swallowtail result |
| --- | --- | --- |
| Alibaba Model Studio Conversations | official paginated deployable-model API | separate typed cloud-control-plane catalogue branch |
| Anthropic Managed Agents | model is operator-owned agent configuration | `Not applicable` to the selected operation |
| Claude Agent ACP | caller supplies the allowed model set | `Caller-supplied`, not discovery |
| Gemini CLI ACP | authorized session advertises available model options | bounded negotiated session evidence |
| Gemini Live | official paginated Gemini Models API | separate typed hosted catalogue branch |
| Kimi Code ACP | authorized session advertises a model selector | bounded negotiated session evidence |
| Kimi local server | exact authenticated `GET /api/v1/models` | attached typed catalogue operation |
| llama.cpp owned lifecycle | serving start returns one selected route endpoint | `Not applicable`; attached route owns catalogue |
| OpenAI background Responses | official `GET /v1/models` | shared typed OpenAI provider catalogue branch |
| OpenAI Realtime | official `GET /v1/models` | shared typed OpenAI provider catalogue branch |
| Pi RPC | exact `get_available_models` | ephemeral provider-suppressed catalogue operation |
| Qwen Code headless | stream-JSON `get_available_models` control request | ephemeral safe-mode catalogue operation |
| xAI Responses WebSocket | official `/v1/language-models` | separate typed hosted catalogue branch |

## Runtime And Authority Shape

Standalone catalogues use `ModelCatalogDriver`. ACP options use optional
`NegotiatedSessionModelOptions` on an already-authorized session handle.
Caller-supplied allowlists remain caller evidence. Serving-only and
operator-configured operation shapes remain explicitly not applicable.

Qwen catalogue preparation starts one joined safe-mode stream-JSON process,
performs control initialization, verifies the advertised catalogue
capability, requests models, projects bounded public fields, then closes and
joins the child. Safe mode is a harness posture, not an operating-system
sandbox claim.

Alibaba catalogue preparation binds its own configured instance, international
endpoint audience, API-key access profile, facade revision, and catalogue
plan. It traverses bounded base and custom pages and deduplicates reported
model names. It shares neither operation identity nor invocation authority
with Conversations.

The OpenAI, Gemini, and xAI catalogue branches bind separate immutable
catalogue plans and public provider endpoints. A listed model does not imply
Realtime, Live, background, WebSocket, or any other transport compatibility.

## Matrix Result

The matrix now contains:

- 16 `Yes`
- 2 `Session-negotiated`
- 2 `Not applicable`
- 1 `Caller-supplied`
- 0 `No`

Every selected solution with a qualified machine-readable model source now
has a Swallowtail path. The remaining five classifications describe the
actual interface semantics; they are not missing adapter work.

## Evidence

- [Qwen Code `0.19.11` system control controller](https://github.com/QwenLM/qwen-code/blob/v0.19.11/packages/cli/src/nonInteractive/control/controllers/systemController.ts)
- [Qwen Code `0.21.0` system control controller](https://github.com/QwenLM/qwen-code/blob/v0.21.0/packages/cli/src/nonInteractive/control/controllers/systemController.ts)
- [Qwen Code headless mode](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
- [Alibaba Cloud list deployable models API](https://www.alibabacloud.com/help/en/model-studio/list-deployable-models-api)
- [Gemini Models API](https://ai.google.dev/api/models)
- [OpenAI Models API](https://developers.openai.com/api/reference/resources/models)
- [xAI Models API](https://docs.x.ai/developers/rest-api-reference/inference/models)
- [Kimi `0.28.1` model catalogue route](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.28.1/packages/kap-server/src/routes/modelCatalog.ts)
- [Kimi `0.29.0` model catalogue route](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.0/packages/kap-server/src/routes/modelCatalog.ts)
- [ACP session configuration options](https://agentclientprotocol.com/announcements/session-config-options-stabilized)

## Promotion

- Contract 020 now covers harness-native safe mode, bounded negotiated session
  options, deployable-candidate scope, and hosted/control-plane conformance.
- Architecture records every realized catalogue branch and the remaining
  non-catalogue classifications.
- Roadmap g02.021 and cards 068-070 close the implementation and audit lane.
