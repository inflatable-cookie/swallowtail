# 210 Kimi Code Headless Reasoning-Effort Evidence

Status: promoted; empty deliver-now
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Card: g04.063 / 176

## Question

Which exact `kimi-code.headless` executable versions, selected models, and
reasoning-effort values can be dispatched through an adapter-owned process-
local surface without silent fallback, ambient override, config mutation, or
thought disclosure?

## Decision

No. Research 210 admits an empty deliver-now set. No typed reasoning-effort
binding is admitted on `kimi-code.headless`.

Exact `@moonshot-ai/kimi-code@0.38.0` documents `KIMI_MODEL_THINKING_EFFORT`
and per-model `support_efforts` / `default_effort`, and selected headless
renderer/options/`run-prompt` source remains byte-identical to `0.37.2` through
`0.38.0` (Research 179). The candidate transport is process-local environment
override after config resolution. That lead does not survive the no-substitution,
precedence, selected-model agreement, and confirmation gates required for cards
177-178.

Cards 177-178 stay blocked. No headless reasoning-effort feature ships.

## Method And Boundary

Official Kimi Code configuration, environment-variable, and command
documentation plus exact public GitHub source for `@moonshot-ai/kimi-code@0.38.0`
were inspected on 2026-08-25. Decisive `agent-core` thinking and env-resolution
blobs were fetched from commit `0999454bdcb5ddd98f39bffee434dcf0a810f394`.
Official npm `@moonshot-ai/kimi-code@0.38.0` was downloaded to a disposable
directory for tarball identity only. No Kimi install, executable launch, OAuth,
credential capture, provider prompt, or paid inference was used.

Production route evidence is the selected default v1 headless path (`run-prompt`
unless `KIMI_CODE_LEGACY_FLAG` is truthy). Experimental v2 print remains
unselected (Research 179). Sibling Kimi ACP effort evidence (Research 207) and
plan-mode evidence (Research 208) do not qualify headless transport or
confirmation.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [Configuration files](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.38.0/docs/en/configuration/config-files.md) | `thinking`, `support_efforts`, `default_effort`, fallback warning | 2026-08-25 | `4b9d6e66f08c3a824be8c8bcf8bdf755fb6bf07969edbcdb3225327c22005d67` |
| [Environment variables](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.38.0/docs/en/configuration/env-vars.md) | `KIMI_MODEL_THINKING_EFFORT` semantics and precedence | 2026-08-25 | tool-normalized body from official raw |
| [kimi command](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.38.0/docs/en/reference/kimi-command.md) | headless argv; stream-json thinking absent from JSONL | 2026-08-25 | official raw fetch |
| npm `@moonshot-ai/kimi-code@0.38.0` tarball | package identity | 2026-08-25 | `d5c047dbfbbdfddf8d20030327e723ea9121af66260983a8556124580d64b549` |
| npm integrity | identity cross-check | 2026-08-25 | `sha512-O/z6sfjFdoDPPeTnoXzdsJ2U8IqP6K2gD3LsT+Nu8BAlHwdhCjdCQFkFTjIbLBun+aZT6x81ha5FiFt7trEilg==` |
| tag `@moonshot-ai/kimi-code@0.38.0` → commit `0999454bdcb5ddd98f39bffee434dcf0a810f394` | selected source tree | 2026-08-25 | annotated tag `488fe6bb311959227c8c2602e12486e48f8b5446` |
| `packages/agent-core/src/agent/config/thinking.ts` @ `0.38.0` | resolve, normalize, default, support checks | 2026-08-25 | `2286a08371e696e2b4400b4e733b87feec878353049b20b8b5cbe34716dbe7d0` |
| `packages/agent-core/test/agent/config/thinking.test.ts` @ `0.38.0` | kimi-protocol fallback and always_thinking clamp | 2026-08-25 | `2bd6df4496384b8c1d7f2be25679caaecaf7e1aeb20aee97b903e7bf27418266` |
| `packages/acp-adapter/src/model-catalog.ts` @ `0.38.0` | catalogue projection reference; not headless session surface | 2026-08-25 | `ca27ae18254c9b7f3a2f0c3c2e3687563f21297fcad9e44a8f70b1644ce92629` |
| Research 179 `0.38.0` identity | headless renderer/options/`run-prompt` byte-identical to `0.37.2` | 2026-08-21 | frozen fixture corpus |
| Research 207 ACP effort | contrast: ACP confirmation path | 2026-08-25 | promoted sibling-route only |

`thinking.ts` matches Research 207's cited blob through `0.29.0..=0.38.0`.
Headless does not add a second thinking resolver on the selected v1 print path.

## Candidate Transport

| Fact | Exact finding |
| --- | --- |
| Process-local key | `KIMI_MODEL_THINKING_EFFORT` |
| Parse point | `resolveKimiEnvThinkingEffort` in bundled `agent-core` after `resolveThinkingEffort` |
| Applies when | Kimi provider identity and resolved effort ≠ `off` |
| Wire effect | Replaces operational effort; intentionally bypasses declared `support_efforts` |
| Does not apply when | Resolved effort is `off`, including `thinking.enabled === false` in ambient user config |
| CLI flag | none on `-p` / `--output-format stream-json` |
| Config mutation | none; env is process-local |
| Headless confirmation | none comparable to ACP `session/set_config_option` or Qwen `set_effort` |

Official command documentation: stream-json does not write thinking content to
JSONL; tool progress and resume hints stay on stderr. That supports omission of
public thought material but is not a broader privacy claim.

## Precedence And Fallback (Frozen)

`resolveThinkingEffort` precedence:

1. explicit requested/session override when provided;
2. `thinking.enabled === false` → `off`;
3. configured `[thinking].effort` when set;
4. else `defaultThinkingEffortFor(model)`.

Then `normalizeThinkingEffortForModel` on Kimi protocol (`kimiProtocol =
true`):

- unsupported concrete values and legacy `on` → model `default_effort` or
  middle `support_efforts` entry;
- boolean models without `support_efforts` → `on`;
- `always_thinking` models cannot resolve to `off`; configured concrete effort
  may still be honored when present.

After that chain, `resolveKimiEnvThinkingEffort` reads `process.env` and, only
while effort ≠ `off`, replaces the operational effort with the env value
(bypassing `support_efforts`).

Official configuration documentation states that for Kimi models an unsupported
configured effort may fall back to the model's `default_effort` during model
resolution. That is a hard fallback path on the config-resolution arm, before
the env override is considered.

## Why No Deliver-Now Row Survives

| Gate | Finding |
| --- | --- |
| No-substitution | For non-`always_thinking` models, configured or normalized effort can fall back to `default_effort` before env override; `thinking.enabled = false` forces `off` and disables env entirely |
| Ambient shadowing | Route uses `HarnessConfigurationPosture::Ambient`; adapter does not read or mutate user `config.toml`; cannot fail closed when ambient thinking is disabled or carries a conflicting `[thinking].effort` |
| Selected-model agreement | Headless has no session-open catalogue snapshot (contrast ACP Research 207); `support_efforts` / `default_effort` live in user-managed model tables and may be rewritten by managed refresh unless manually overridden |
| Confirmation | No headless control exchange proves accepted or effective effort before the first prompt; env override is not echoed in stream-json output |
| Inherited environment | Ambient host may supply `KIMI_MODEL_THINKING_EFFORT`; child-only binding precedence against inherited values is not proved on the Swallowtail process surface |
| Transport ownership | `ProcessRequest` carries opaque `EnvironmentRef` values resolved by the host; adapter does not set raw `KIMI_MODEL_THINKING_EFFORT` without a host contract extension |

Illustrative official model metadata (configuration example, not a live
catalogue snapshot):

| Model alias (example) | `always_thinking` | Example `support_efforts` | Example `default_effort` |
| --- | --- | --- | --- |
| `kimi-code/k3` | yes | `low`, `high`, `max` | `max` |
| `kimi-code/kimi-for-coding` | yes | absent (boolean) | n/a |

Even for `always_thinking` effort-capable examples, the absence of pre-spawn
confirmation and mutable managed metadata without a headless snapshot fails the
milestone's fail-closed and selected-model agreement requirements.

## Claim Strength Available Without Live Provider

| Layer | Headless availability |
| --- | --- |
| Planned | could record adapter intent only if binding existed |
| Dispatched | could record child `EnvironmentRef` list only if binding existed |
| Accepted | no deterministic headless surface |
| Effective | no deterministic headless surface |
| Observed | no; stream-json excludes thinking content from JSONL |

Contrast Research 189 (Qwen `set_effort` with `applied: true`) and Research
207 (ACP `session/set_config_option` response `currentValue`).

## Deliver-Now Set

Empty. No executable-version / selected-model / portable-value row meets all
gates: exact process-local transport without silent fallback or ambient
override, selected-model agreement at preparation, and fail-closed behavior
before child creation.

## Withheld Surfaces (Not Empty-Set Reasons Alone)

| Surface | Disposition |
| --- | --- |
| `KIMI_MODEL_*` model synthesis channel | separate temporary model surface; not the existing `--model <alias>` headless route |
| `[thinking].effort` in user config | ambient configuration; out of scope for adapter mutation |
| ACP / local-server / Python `kimi-cli` | sibling routes; Research 207/208 do not qualify headless |
| `on` portable value | normalizes to `default_effort` on Kimi protocol before env override |
| `off` on `always_thinking` models | clamped back to model default; not a faithful off claim |
| Experimental v2 print | unselected; `KIMI_CODE_EXPERIMENTAL_FLAG` outside qualified headless guarantee |

## Continuation

Cards 177-178 remain blocked. Orchestrator should reassess the remaining
per-route feature inventory. Kimi headless reasoning effort may be revisited only
if Kimi exposes a headless-local confirmation transport or Swallowtail gains a
bounded, fail-closed child-environment contract that proves effort agreement
before spawn without user-config mutation.
