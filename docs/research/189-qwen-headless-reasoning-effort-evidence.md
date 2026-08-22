# 189 Qwen Headless Reasoning Effort Evidence

Status: complete; deliver-now subset admitted
Owner: Tom
Date: 2026-08-22
Card: g04 batch 113

## Question

Does exact official Qwen Code `0.21.15` expose at least one exact
model-qualified reasoning-effort value through an operation-private headless
transport that Contract 040 can bind without clamp, default substitution,
ambient configuration mutation, or a Contract 033 host-scoped lease?

## Decision

Yes, but only for the exact package point `0.21.15`, provider
`alibaba-modelstudio`, models `qwen3.8-max` and `qwen3.8-max-preview`, and
canonical values `low`, `medium`, `high`, `xhigh`, and `max`. This is a
feature-local exact-version gate. It does not widen the existing
`qwen-code.headless.v0.21.0-catalogue-filter` compatibility segment or claim
the same reasoning mapping for `0.21.0..=0.21.14`.

The selected transport is the private `stream-json` control exchange:

1. Start the existing bounded child with `--input-format stream-json`; all
   other route arguments and the approved environment remain unchanged.
2. Send `control_request/initialize` and require `can_set_effort: true` plus a
   bounded child session id.
3. Send `control_request/set_effort` with the canonical value and require a
   `set_effort` response whose `effort` equals the requested value,
   `applied` is `true`, and `override` is `null`.
4. Send the user message as a `user` stream record, then close stdin.

The control acknowledgement is Qwen Code runtime/config acceptance. It is not
a claim that the downstream provider accepted or effectively allocated the
requested reasoning budget.

## Official Surface

The [official headless documentation](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
documents prompt/stdin operation, `stream-json`, and `--resume`, but does not
document a public `--effort` or reasoning flag. The [official settings
documentation](https://qwenlm.github.io/qwen-code-docs/en/users/configuration/settings/)
documents `model.reasoningEffort` with the five named values and interactive
`/effort`; it also states that providers map or clamp the value and that an
unset value uses the model/provider default. That settings/UI surface is not
used by this mapping.

## Exact Package Evidence

Evidence was inspected from the disposable official npm tarball and matching
official GitHub source. No package was installed onto the host, and no live
Qwen process, account, credential, catalogue, or prompt was used.

- Package: `@qwen-code/qwen-code@0.21.15`
- npm integrity: `sha512-f4ER/SRVLpwhcqzuytK3Qeq8bG9HnVhv7f7wsf3cpE/AkRfzKSvaeURnW7s7zI3nWkEqA7DM6njSLYS2s6DWDg==`
- npm tarball SHA-256: `8d405b065888b7000a6989d99c2d79257cd8f9f5b68e9078fb76484527351b9a`
- `package.json` SHA-256: `dc7b0c825626dd3f6d8cecbdced167bad83e5529e3ce958597ed8691569f5711`
- GitHub source commit: `5dce2515a778f9cf2013168962b4fbc3454636e3`
- Existing route fixture: `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.21.15/`

The decisive source paths are:

- `packages/core/src/core/reasoning-effort.ts`: canonical tiers,
  normalization, and clamp helper
- `packages/core/src/config/config.ts`: in-memory `setReasoningEffort`,
  override detection, and provider-default fallback when unset
- `packages/cli/src/nonInteractive/types.ts`: private `initialize.effort` and
  `set_effort` control request types
- `packages/cli/src/nonInteractive/control/controllers/systemController.ts`:
  exact `set_effort` application and `applied`/`override` response
- `packages/cli/src/nonInteractive/session.ts`: initialize-before-user
  stream lifecycle and synchronous control-response handling
- `packages/core/src/core/openaiContentGenerator/provider/dashscope.ts`:
  model-qualified wire mapping
- `packages/core/src/core/openaiContentGenerator/provider/dashscope.test.ts`:
  exact pass-through tests for both admitted models and all five values

## Precedence, Mutation, And Lifetime

`/effort` applies the value and persists `model.reasoningEffort` through the
settings adapter. The headless mapping does not call that command or any
settings write. The private `set_effort` request updates the live in-memory
config for the current child. Qwen Code reports a higher-priority provider
knob through `override` and returns `applied: false`; Swallowtail rejects that
child before sending the user message. No synthetic home, project tree, or
configuration root is created.

The selected value is sent once per structured-run child, once before every
first or resumed interactive turn child, and again for a fresh context-losing
replacement because that replacement is a new child. No value is inferred from
response text, retained settings, provider session state, or model defaults.

The package also accepts upstream aliases such as `med`, `maximum`, and
separator variants in its generic normalizer and has a generic clamp helper.
Those are not public Swallowtail inputs. Swallowtail admits only the five
canonical `ReasoningMode` values and rejects any response that substitutes a
different tier.

## Model And Value Dispositions

| Exact package/provider/model row | Value | Disposition | Reason |
| --- | --- | --- | --- |
| `0.21.15` / `alibaba-modelstudio` / `qwen3.8-max` | `low`, `medium`, `high`, `xhigh`, `max` | deliver-now | DashScope sends each tier as flat `reasoning_effort`; source tests assert exact pass-through. |
| `0.21.15` / `alibaba-modelstudio` / `qwen3.8-max-preview` | `low`, `medium`, `high`, `xhigh`, `max` | deliver-now | Same exact tiered-wire branch and source coverage. |
| `0.21.15` / other provider or unqualified provider | all | withheld | The qualified wire evidence is DashScope-specific. |
| `0.21.15` / legacy or other Qwen model, including `qwen3-coder-plus` | all | withheld | The source maps these models to boolean `enable_thinking`, which cannot represent one exact five-tier selection. |
| Any admitted model | upstream aliases (`med`, `maximum`, separator variants) | withheld/invalid | Generic upstream normalization is not exposed as a portable mapping. |
| Any package point other than exact `0.21.15` | all | evidence-gated | No retroactive mapping across the existing compatibility segment. |
| Any model/value with reasoning omitted | not applicable | retain existing behavior | Existing text-stdin path and request shape remain unchanged. |
| Interactive `/effort` or ambient `model.reasoningEffort` | all | not applicable | Settings/UI persistence is outside the operation-private headless transport. |
| `initialize.effort` before config initialization | all | withheld transport candidate | The exact lifecycle can report before the live content generator is ready; post-initialize `set_effort` is deterministic. |

## Contract And Truth Boundary

The route binds the selected `ReasoningSelection` constraint into prepared
input, preflight plan, request policy/session options, prepared evidence, and
the driver. The transport is operation-private under Contract 040 and keeps
the existing Contract 033 Ambient posture without a lease gap or configuration
mutation.

The implementation proves requested, planned, dispatched, and Qwen-control
accepted values. It does not claim provider-effective or response-observed
reasoning. Deterministic fixtures cover both models, all five values, a
substitution/override rejection, structured runs, first turns, resumed turns,
and preserved fresh-replacement preparation.

## Behavior Revision And Compatibility

The existing compatibility claim remains unchanged:
`qwen-code.headless.v0.21.0-catalogue-filter` covers the route's established
headless stream behavior. The reasoning mapping is a private feature boundary
named `qwen-code.headless.v0.21.15-reasoning-control` in this evidence record,
gated by exact version equality. It is not a new general compatibility segment
and is not claimed for `0.21.0..=0.21.14`.

## Validation

- `effigy validate:focused swallowtail-adapter-qwen` — 45 tests passed
- exact-package source and npm identity rechecked from disposable extraction
- no install, login, credential, account, catalogue, or provider prompt
- shared architecture, route/feature matrices, programme/front doors, indexes,
  changelog, currentness claim, and `packages.txt` were not edited
