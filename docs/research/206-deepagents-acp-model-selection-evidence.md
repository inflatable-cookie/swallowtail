# 206 Deep Agents ACP Model Selection Evidence

Status: promoted
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Card: g04.059 / 164

## Question

Can exact `deepagents-acp@0.1.25` bind one typed `provider:model` selection at
server start, agree it with the prepared host-owned provider-key access
profile, and reject invalid, unsupported, or unauthenticated values without
fallback while preserving the current ACP session authority?

## Method And Boundary

Authorized evidence frozen on 2026-08-24:

- current official LangChain Deep Agents ACP and Models pages
- exact public npm `deepagents-acp@0.1.25` tarball (reverified against
  Research 157)
- exact public npm `deepagents@1.12.4` runtime dependency named by that
  package
- Research 153 / 157 / 159
- existing secret-free fixtures under
  `crates/swallowtail-adapter-deepagents/tests/fixtures/deepagents-acp-0.1.25/`
- secret-free local reimplementation of the exact `0.1.25` `normalizeArgs` /
  `parseArgs` path (no ACP server execution)

No install, `npx`, ACP server execution, login, host-key inspection,
credential capture/materialization, provider prompt, external inference
request, or paid work. No production code in card 164.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [ACP docs HTML](https://docs.langchain.com/oss/javascript/deepagents/acp) | `--model` / `-m`; default `"claude-sonnet-4-5-20250929"`; env keys; no ACP model field | 2026-08-24 | `925725bea66b88e5c90aca085fda469cdba0767de1bb1beff7f9fa7c9a10e933` |
| [Models docs HTML](https://docs.langchain.com/oss/javascript/deepagents/models) | current `provider:model` lead; many providers; live catalogue language | 2026-08-24 | `20cb1d6dfa90dd23a19754606276aa2da650914af77d9558f9cf9f1bccf365c5` |
| npm `deepagents-acp@0.1.25` tarball | exact package identity | 2026-08-24 | `6a56fa60e985a0681217cd20b1e21c0f7782fb10ebed6728f2865346ba137141` |
| npm integrity | matches Research 157 | 2026-08-24 | `sha512-5S6Rpd74vV3YKVxAEqQkXKek+y1ChTpL0D2xf+WLaAYneJQZ9haZ4lPgjPy2VvszqErVsSr+T5tq8vdjuAWShQ==` |
| `package/package.json` | bin → `dist/cli.js`; dep `deepagents` `1.12.4` | 2026-08-24 | `cddb5563aafc9fc22e67760c2ac906187c69e83d1ed73c36ae13db04c35cdb5e` |
| `package/dist/cli.js` | CLI parser, default, server start, createAgent, initialize, session/new, `/status` | 2026-08-24 | `68b7d6cb31d181a399f623a4c6486892bf7d408aec61cac0a3ea9e033baa2319` |
| npm `deepagents@1.12.4` tarball | `createDeepAgent` model default and `initChatModel` path | 2026-08-24 | `cea1b79d542a9ee695d103d4371b5265b83eac157cbbef8101c431f7062654ec` |
| `deepagents@1.12.4` `package.json` | peer/runtime dependency identity | 2026-08-24 | `c8e0779d4097894f460a0126c1e45e21d4b3bc6a456c8b7d8823c8930f1586ba` |
| `deepagents@1.12.4` `dist/langsmith-DRyafCNe.js` | string model → harness profile / `initChatModel`; bare `claude*` Anthropic heuristic | 2026-08-24 | `d17461ce79202cb8d3470b98949afeea6adfc7acdb016d2cd11220379fb501aa` |
| fixture `identity.json` | `pass_model_flag: false` | frozen | `ab112df1f7a721f661f6fe0fbf472d3d10a7c9663e72f55aead22315ff5d0ac3` |
| fixture `protocol.json` | model flag unmapped; argv empty | frozen | `8e82b396696943ddc6726d1294043f2ab312e3fd9bc34789953bb6f364db7364` |
| fixture `initialize.json` | no model field | frozen | `702edcf6d1fbab5baf1c3d1563342eb409538f2d56d58371dc770c24da047f7a` |
| fixture `session-new.json` | no model field; auth gate text only | frozen | `b11b24466ccf3ade1715ac69766580bb8b1ae8c1795c099c60bf00f4c44d9082` |
| fixture `negative-cases.json` | `model-flag-unmapped` still negative | frozen | `440403b278c155681d14c2c4ef09d7e88bce16c4361dccba6274522d9436ea07` |

Current official docs are leads only. Exact package digests match Research
157. Docs HTML digests differ from Research 157's earlier ACP page digest;
treat them as current, not exact `0.1.25`.

## Advertisement Versus Exact `0.1.25`

Official ACP docs advertise `--model <model>` / `-m` with default
`"claude-sonnet-4-5-20250929"`. Official Models docs advertise
`provider:model` and list many providers beyond Anthropic/OpenAI.

Exact `deepagents-acp@0.1.25` CLI:

| Behavior | Exact source |
| --- | --- |
| Flag forms | `--model`, `-m`; `normalizeArgs` also accepts `--model=value` and `"--model value"` as one token |
| Default when omitted | `"claude-sonnet-4-5-20250929"` (bare id, no provider prefix) |
| Missing value (`--model` alone) | silently retains the default |
| Empty string value | falsy under `if (nextArg)`; silently retains the default |
| Whitespace | preserved; not trimmed |
| Repeat flags | last value wins |
| Validation | none — any next token is accepted, including a following flag token |
| Provider grammar | not parsed by the CLI |
| Auth methods | env-var Anthropic and OpenAI plus generic setup; `authenticate` is a no-op |
| Spawn → model construction | process starts with config stored; `createAgent` / `createDeepAgent` run at `session/new` |

Exact `deepagents@1.12.4` `createDeepAgent`:

| Behavior | Exact source |
| --- | --- |
| Library default when `model` omitted | `"anthropic:claude-sonnet-4-6"` |
| CLI path | always passes the CLI string, so the library default is not used on the ACP CLI path |
| String handling | `initChatModel(model)` after spawn; harness-profile lookup by string |
| Anthropic heuristic | bare strings starting with `claude` count as Anthropic for caching middleware |

CLI default and library default disagree. Omission with empty argv still
selects the CLI default inside the child; Swallowtail currently owns no
upstream-default-model claim for that path.

## Requested / Planned / Dispatched / Accepted / Effective / Observed

| State | Exact truth |
| --- | --- |
| Requested | caller-prepared value, if any |
| Planned / dispatched | would be one `--model <string>` argv pair; today no model tokens |
| Accepted | no ACP initialize / `session/new` field returns the model |
| Effective | constructed only at `createAgent` during `session/new` via `initChatModel`; not visible on the ACP wire |
| Observed | `/status` slash text echoes `config.model`; Swallowtail does not send slash commands and does not treat that text as confirmation |

Initialize returns `agentInfo.name` / `agentInfo.version`, capabilities, and
auth methods. `session/new` returns `sessionId` and modes. Neither includes a
model id. Fixture initialize and session-new specimens match that gap.
`negotiated_model_options` remains absent on the production route.

## Access Agreement

Current preparation uses one generic local unauthenticated
`deepagents_provider_api_key_access_profile` with audience
`deepagents.provider-api-key`. It does not name Anthropic versus OpenAI, does
not encode which host key exists, and forbids credential references.

Exact auth advertisement covers both Anthropic and OpenAI env keys. Current
Models docs lead many additional providers. Provider agreement therefore
cannot be checked before spawn under the existing profile without inspecting
host environment contents or inferring from provider failure — both forbidden.

Missing/wrong key currently maps provider error text to
`swallowtail.deepagents.acp.host_auth_required`. That is not selected-provider
agreement and must not become a fallback trigger.

## Invalid Selection And Fallback

Exact CLI evidence does **not** prove fail-closed invalid selection before
spawn:

1. `--model` / `-m` with no value silently keeps the default
2. empty string silently keeps the default
3. arbitrary strings, including other flags as values, are accepted
4. model construction and provider-package/key failures happen after spawn at
   `session/new`
5. no closed provider/model allowlist exists in `deepagents-acp@0.1.25`
6. no ACP confirmation field can catch drift before accepted output

A dispatch-only claim requires exact proof that the CLI cannot substitute or
fall back. The silent default-on-missing-value path and post-spawn
construction block that claim under existing contracts.

## Lifecycle Disposition

| Lifecycle | Disposition |
| --- | --- |
| Omission / empty argv | retain current no-extra-argv path; do not acquire an upstream default-model claim |
| Explicit `--model <value>` | advertised; not deliver-now — access agreement, fail-closed invalid, and confirmation unproved |
| Turn reuse on one child | selection would be immutable if delivered; blocked by empty set |
| Cancellation / terminal / close | join owned turn, connection, task, process, working resource; unchanged; no provider-state deletion claim |
| Fresh working-state restoration | context-losing new child; may reassert only if a later non-empty set admits it |
| Load / resume | unmapped; unchanged |

## Provider / Model / Access Disposition

| Provider lead | Model form | Access agreement before spawn | Fail-closed invalid | ACP confirmation | Deliver-now |
| --- | --- | --- | --- | --- | --- |
| Anthropic (docs / bare `claude*` heuristic) | bare id or `anthropic:…` | **no** — generic profile cannot prove Anthropic-only host key | **no** — CLI accepts any string; construction post-spawn | **no** | no |
| OpenAI | `openai:…` | **no** — same generic profile | no | no | no |
| Other docs providers (`google_genai`, `openrouter`, `ollama`, …) | `provider:model` | **no** — outside named auth methods and profile | no | no | no |
| Omission | CLI internal default only | n/a | n/a | n/a | n/a — omission is not a selection row |

No row is deliver-now. The empty set rests on authorized evidence:

1. generic host-owned access profile cannot prove provider agreement before
   spawn without env inspection
2. CLI has no bounded provider/model domain and silently retains the default
   when `--model` lacks a usable value
3. model construction / auth failure occur after spawn at `session/new`
4. initialize / `session/new` expose no model confirmation field; `/status`
   text is not Swallowtail confirmation

It is not because `--model` is undocumented. Current docs advertise it. Exact
`0.1.25` still cannot meet the deliver-now gates.

## Application, Failure, And Revision Posture

No adapter-local provider/model type, prepared input, plan constraint, request
member, argv emission, confirmation check, guide claim, matrix claim,
contract, or configured-instance revision is proposed. Omission keeps current
production behavior: empty argv, `pass_model_flag: false`,
`model-flag-unmapped` negative corpus.

Requested, planned, dispatched, accepted, effective, and observed remain
distinct. Docs can request. Acceptance requires pre-spawn access agreement,
fail-closed invalid handling, and either exact confirmation or an honest
dispatch-only basis. Those remain unproved on exact `0.1.25`.

## Promotion

Research 206 promotes an empty deliver-now set.

Cards 165-166 stay blocked. A later lane may reopen this family only when
exact `0.1.25` or a newly qualified version shows:

1. provider/model selection that agrees with explicit prepared access evidence
   before spawn without inspecting key bytes
2. fail-closed invalid / missing-value / unsupported-provider handling with no
   silent default substitution
3. an exact ACP confirmation field, or a bounded dispatch-only claim with
   source proof against fallback
4. no shared contract or credential-authority expansion

Until then, `deepagents.acp` continues without `--model`.
