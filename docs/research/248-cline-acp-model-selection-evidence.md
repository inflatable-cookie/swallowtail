# 248 Cline ACP Model-Selection Evidence

Status: promoted
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Card: g04.088 / 251

## Question

Which exact `cline.acp` `3.0.55` provider/model and lifecycle rows, if any,
can bind model selection with closed membership, route agreement, pre-prompt
selection, exact confirmation, persistence/restoration, failure, and omission?

## Decision

Promote an honest empty deliver-now set. Exact `cline.acp` `3.0.55` exposes the
same ACP config-option protocol pattern Research 240 used for Plan
(`session/new` advertisement plus `session/set_config_option` with returned
`configOptions`), and it advertises provider/model pickers. That proves the
protocol seam, not a closed provider/model row.

No row closes membership, route agreement, and pre-effect rejection together.
Research 221 headless `-m/--model` conclusions are contrast only and are not
promoted onto ACP.

## Method And Boundary

Official Cline ACP documentation plus exact `cline@3.0.55` /
GitHub `cli-v3.0.55` ACP and LLM sources were inspected on 2026-08-28 in a
disposable directory. No Cline install, platform binary extraction, login,
credential, account, catalogue request, `initialize`, provider prompt, or
ambient settings mutation was used. Host PATH still has no `cline`.

The selected route remains `cline.acp` (ACP v1 stdio, argv `cline --acp`,
caller working resource, observational permissions, no auto-approve). It sits
on axis `cline.package` `3.0.55`. Research 146 freezes identity. Research 240
proves the Plan config-option pattern. Research 221 is headless contrast only.

Current official pages are leads. The exact package is the finding.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| npm `cline@3.0.55` metadata | wrapper identity unchanged from Research 146/221/240 | 2026-08-28 | integrity `sha512-3JQ5vPl8/BIyTShTpn0VDX59lbzFhGxRwYqZmAVLOPcX83ETaHbgH2fc9iJYv1pB2ChIpXQEXmFHowgz8F1GgQ==`; shasum `88172d7b7ace564811185858da42b8f62a56751b` |
| [ACP](https://docs.cline.bot/usage/acp.md) | official model/provider picker and env-pin lead | 2026-08-28 | `f421629eae0d4ae5b451d0085b8395f14ce1a103ba54774a275e09a1e7099086` |
| [CLI reference](https://docs.cline.bot/cli/cli-reference.md) | `--acp` coexistence lead | 2026-08-28 | `c39fb3197cb72491da81cb74c1e01cf15f60b3cde8218162e7510a30cce0f72f` |
| `apps/cli/src/main.ts` | ACP early-return discards root `-m`/`-P` and skips headless settings write | 2026-08-28 | `1b7a0a5b680aa6f3f736826c449f64e9a62dbe7b57e0b55cbc74379d45f37274` |
| `apps/cli/src/acp/index.ts` | ACP entry takes auto-approve only | 2026-08-28 | `8cbcf4c04bb9b01ccc877c661210346380732a32b796ac8335663b5d141adbb7` |
| `apps/cli/src/acp/acpAgent.ts` | newSession models/configOptions; setSessionConfigOption provider/model; buildConfig | 2026-08-28 | `248092d41e330ef1898f98b99d35c6713574a7b9305d95601177c07e64db9e71` |
| `apps/cli/src/acp/auth.ts` | ACP auth-method ids double as selectable providers | 2026-08-28 | `ad46c0b5cd79e561de5bbbb0ef8e513339f73618b2ce733c3d88448c0023024d` |
| `apps/cli/src/acp/session-updates.ts` | `config_option_update` observation | 2026-08-28 | `e7f8beca5101fcdfc8079025980b479918c4959fcc34728452ed94f4e1e18fe0` |
| `apps/cli/src/acp/session-load.ts` | load path; no per-session provider/model restore on fresh connection | 2026-08-28 | `90bf351aaab8a1dd37b89df609b195af914b6a8e0a70a5cfa167a3a1e37dea38` |
| `sdk/packages/llms/src/providers/model-registry.ts` | `getModelsForProvider` built-in + custom merge; no selection gate | 2026-08-28 | `186005c71e1a6a891c1503a4901858dcb5007e5c7491e81ee15ec93ea2c8f6d3` |
| `sdk/packages/llms/src/providers/builtins.ts` | `cline` models from OpenRouter-generated catalogue plus Vercel aliases | 2026-08-28 | `94f9324f9d0f51efaef589370d4a802ddb282d878b4191babc9f49ebb301b98e` |
| `sdk/packages/llms/src/providers/model-facts.ts` | explicit accommodation of user-typed unlisted ids | 2026-08-28 | `a3af38a5d3761e0555777c2efd663b0b503e34eb5abec047baac67c4666645d1` |
| `sdk/packages/core/src/runtime/host/local-runtime-host.ts` | `updateSessionModel` applies requested id into session config/manifest | 2026-08-28 | `61fce1f2f9b9061b86721f48a7a1786fa869fc96b566c4cd18fc3379c0ed0743` |
| Frozen fixture `cline-acp-3.0.55/` | initialize / session-new / protocol baseline | 2026-08-18 | workspace |
| Frozen evidence `model-selection-evidence.json` | closed ACP model seam dispositions | 2026-08-28 | workspace |
| Prepared guide `cline-acp-prepared-integration.md` | no model route; Plan only | 2026-08-28 | workspace |
| Research 146 / 221 / 240 | identity, headless contrast, Plan protocol pattern | 2026-08-28 | promoted siblings |

Source paths are read at GitHub commit
`ad442cbb6a81d21773ceabc1398ea5eb58170718`, the commit Research 146, 221, and
240 already froze for tag `cli-v3.0.55`. Wrapper integrity is unchanged.

HTML digests identify retrieved documentation bodies. They are not a
compatibility guarantee. Findings below are source-level for the exact tagged
package point.

## ACP Protocol Surface

Official ACP docs advertise model and provider selection from the client
picker, plus `CLINE_PROVIDER` / `CLINE_MODEL` env pins. Exact `AcpAgent`
implements both.

| Frame | Model-relevant fields | Selection seam |
| --- | --- | --- |
| `initialize` result | auth methods only; no models | none |
| `session/new` result | `models.availableModels`, `models.currentModelId`, `configOptions` `provider`/`model` | snapshot only; defaults from env/auth/`cline` |
| `session/set_config_option` `provider` | request value must be an ACP auth-method id; tears down manager; re-resolves model | selected-value confirmation via rebuilt options |
| `session/set_config_option` `model` | request value assigned verbatim; no membership check | selected-value confirmation via rebuilt options |
| `unstable/set_session_model` | request `modelId`; response `{}` | applies ACP session state; empty RPC confirmation |
| `session/prompt` terminal | `stopReason` only | no model field |
| `session/load` (fresh connection) | rebuilds provider/model from same defaults as `newSession` | no restored prior selection |
| `session/update` | `config_option_update` | observation side channel |

Root argv `-m` / `-P` never reaches ACP setup:

```ts
if (args.acpMode) {
    await runAcpMode({ autoApproveTools: args.autoApproveOverride === true });
    return;
}
```

That early return also skips the headless `saveProviderSettings` path Research
221 closed against. ACP model/provider `set_config_option` updates in-memory
`SessionState` only. Do not import the headless durable-write stop onto this
seam.

## Membership, Route Agreement, Confirmation

Truth layers stay distinct:

| Layer | Exact finding |
| --- | --- |
| Advertisement | `session/new` returns `models` plus select options `provider` and `model` |
| Provider option set | closed to ACP auth methods `cline`, `cline-pass`, `openai-codex` |
| Model option set | `Llms.getModelsForProvider(providerId)`; for `cline`, OpenRouter-generated catalogue plus Vercel aliases |
| Default provider | `CLINE_PROVIDER ?? authResult?.providerId ?? "cline"` |
| Default model | `CLINE_MODEL` if present in catalogue, else provider default, else first catalogue id, else `""` |
| Request | ACP `session/set_config_option` `{ configId: "model"\|"provider", value }` |
| Compatibility path | `unstable/set_session_model` updates the same `session.currentModelId` but returns `{}` |
| Acceptance | unknown `provider` rejects; unknown/foreign `model` does **not** reject |
| Confirmation | set-config response `configOptions[id].currentValue`; empty set-model body is insufficient |
| Pre-prompt application | first `ensureSessionManager` copies `session.currentModelId` / provider into `buildConfig` |
| Effectiveness | runtime host stores requested `modelId`; unlisted ids remain accommodated downstream |
| Persistence | provider/model are not persisted per ACP session across fresh connections |
| Observation | optional `config_option_update`; no provider-effective selected-value field on prompt terminal |

Provider ambient identity is not fixed by the prepared
`cline_local_account_access_profile`. Authenticated `session/new` membership is
account- and catalogue-shaped. The lane forbids login, account inspection, and
live catalogue access, so available model ids stay
`provider-catalog-private` in the frozen fixture.

Contract 020: catalogue presence proves no `ModelRoute`. Promoting any exact
OpenRouter-generated id set as Swallowtail membership would be a catalogue
claim, not a closed route row. Contract 033 is not the stop on this ACP
set-config path; membership and route agreement already fail independently.

## Deliver-Now Table

| Route | Provider | Model | Operation | Lifecycle | Selection seam | Membership | Confirmation | Persistence | Omission | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `cline.acp` `3.0.55` | ambient/`cline` | any advertised catalogue id | `session/set_config_option` `model` | new, before first prompt | present | open / catalogue-derived | echo `currentValue` only | session-private; fresh load resets | retain ambient ACP | no |
| `cline.acp` `3.0.55` | caller-selected auth-method id | any | `session/set_config_option` `provider` then `model` | new | present | provider ids closed; models open | echo `currentValue` | session-private; fresh load resets | retain ambient ACP | no |
| `cline.acp` `3.0.55` | any | unlisted / foreign id | `session/set_config_option` `model` | new | present | no pre-effect rejection | echoes the foreign value | session-private | retain ambient ACP | no |
| `cline.acp` `3.0.55` | any | any | `unstable/set_session_model` | new | present | open | empty RPC body | session-private | retain ambient ACP | no |
| `cline.acp` `3.0.55` | any | any | root `-m` / `-P` | spawn | discarded by ACP early-return | n/a | none | none | retain ambient ACP | no |
| `cline.acp` `3.0.55` | any | prior selection | `session/load` fresh connection | load/resume | none | ambient defaults rebuilt | none | no restore | retain ambient ACP | no |
| `cline.headless` `3.0.55` | any | `-m` | headless argv | one-run | sibling Research 221 | open; durable write | request echo | shared settings write | sibling only | no |

No row is deliver-now. Honest empty set.

## Lifecycle And Omission

| Lifecycle | Disposition | Reason |
| --- | --- | --- |
| new session, select before first prompt | protocol present; membership open | set-config can run before `ensureSessionManager`, but ids are not closed |
| follow-up prompts on that manager | retains requested id in session config | still not a closed membership row |
| set model after manager started | `updateSessionModel` mutates live config | still open membership; not a Swallowtail binding |
| `session/load` on a fresh connection | no restoration | source comment: provider/model not persisted per session |
| fresh replacement session | new negotiation | new `session/new` rebuilds ambient defaults |
| omission | retain existing frames | no model request; guide and production stay without a model route |
| unsupported config id | reject before prompt | `Unknown config option` |
| unsupported provider id | reject before prompt | `Unknown provider` |
| unsupported / foreign model id | accepted | no invalid-model path on ACP set-config |

Omission sends no model or provider request and keeps current initialize /
`session/new` / Plan-optional / prompt frames.

## Production Audit

No production change is authorized or needed. Current prepared ACP binds Plan
only. `ClineSessionProfileInput` has no model route. The guide states model
selection needs a separate card. Fixtures already mark
`availableModels` / `currentModelId` as provider-private. Matrices and API
baseline stay unchanged.

## Contrast With Research 221 And 240

| Fact | Headless 221 | ACP Plan 240 | This ACP model lane |
| --- | --- | --- | --- |
| Selection request | root `-m` / `-P` | `set_config_option` `mode` | `set_config_option` `model`/`provider` |
| Membership | open; no validation | closed `plan\|act` | open for model; provider limited to auth ids |
| Confirmation | `run_result.model` request echo | `mode.currentValue === plan` | `model.currentValue` echo without membership gate |
| Durable shared settings write | unavoidable `saveProviderSettings` | not used for mode | not used for model/provider set-config |
| Deliver-now | empty | one Plan row | empty |

Plan proves the config-option confirmation pattern. Model does not inherit
Plan's closed membership.

## Promotion

Research 248 promotes no deliver-now `cline.acp` provider/model row. The
deliver-now set is empty.

Card 251 must not bind a `ModelRoute`. The Cline adapter, ACP guide, route and
feature matrices, and unreleased package API baseline are unchanged. The only
corpus addition is `model-selection-evidence.json`, which freezes seam
dispositions and adds no capability, claim, or route.

A later lane may reopen only with an exact package point that:

1. fixes provider identity from route or access facts without caller provider
   selection or ambient auth drift
2. closes model membership and rejects foreign ids before provider effects
3. keeps pre-prompt selected-value confirmation without turning catalogue
   observation into a `ModelRoute`
4. proves lifecycle, failure, and omission without login, live catalogue, or
   unauthorized durable mutation

Until then, `cline.acp` stays without model-selection binding. Headless
`-m/--model`, caller provider selection, thinking, currentness, and release
remain out of scope.
