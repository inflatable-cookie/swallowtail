# 208 Kimi Code ACP Plan-Mode Evidence

Status: promoted
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Card: g04.061 / 170

## Question

Which exact qualified Kimi Code ACP versions advertise a selectable `plan`
session mode, apply it with manual permission, and return effective
confirmation before new-session readiness without widening isolation or
attachment mutation authority?

## Method And Boundary

Official Kimi ACP method coverage and exact public GitHub / npm identities for
`@moonshot-ai/kimi-code` were inspected on 2026-08-25. Decisive ACP adapter,
node-sdk, agent-core, and test blobs were fetched from the peeled commits of
tags `@moonshot-ai/kimi-code@0.28.0`, `@0.28.1`, `@0.29.0`, `@0.31.1`, and
`@0.38.0`, and hashed across every listed qualified ACP point in exact `0.28.1`
plus `0.29.0..=0.38.0`. No Kimi install, executable launch, OAuth/login
mutation, credential or account inspection, provider prompt, external
inference, or paid work was used.

Production route evidence is the selected `@moonshot-ai/acp-adapter` path
already frozen by Research 006/086/165/179/207. The sibling
`@moonshot-ai/acp-server` package (agent-core-v2) remains unselected
experimental surface and is cited only as a non-authority contrast.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [kimi acp](https://www.kimi.com/code/docs/en/kimi-code-cli/reference/kimi-acp.html) | official method-coverage lead: `session/set_config_option` is the unified model / thinking / mode dispatcher; `session/set_mode` is a compatibility path | 2026-08-25 | `5ad1333c4934181a6c2e67461e4f1dad76cea47c8ec36dff9dca440ba1b72f64` |
| [Configuration files](https://www.kimi.com/code/docs/en/kimi-code-cli/configuration/config-files.html) | official config lead; no ACP mode-row construction | 2026-08-25 | `4b9d6e66f08c3a824be8c8bcf8bdf755fb6bf07969edbcdb3225327c22005d67` |
| npm `@moonshot-ai/kimi-code@0.28.1` | first qualified ACP identity | 2026-08-25 | integrity `sha512-1+GqFBdY6N0O6YBqNuclaoUY2jtKVQSKPikDBAMxF633AuB4emuSsMxDyh2KCnINH7f4ceeUdQhIjKunbS6GDA==` |
| npm `@moonshot-ai/kimi-code@0.29.0` | first declared-effort identity; mode path already complete | 2026-08-25 | integrity `sha512-cDwEubXkFAch4DsRq/Zp1RCcnkhn8+lC4fwstWmlEK62X5qgIRAeGdp8INAponIGP2ljUfcB6dU36fsAuqlumg==` |
| npm `@moonshot-ai/kimi-code@0.38.0` | current qualified ceiling; matches Research 179 | 2026-08-25 | integrity `sha512-O/z6sfjFdoDPPeTnoXzdsJ2U8IqP6K2gD3LsT+Nu8BAlHwdhCjdCQFkFTjIbLBun+aZT6x81ha5FiFt7trEilg==` |
| tag `@moonshot-ai/kimi-code@0.28.0` → commit `a05228c67122c8233dc87226ce0ca7414780b680` | immediately preceding published tag; ACP mode blobs already match `0.28.1` | 2026-08-25 | annotated tag `495cded13d17ea4ad9753ba5506c63f959a68fd9` |
| tag `@moonshot-ai/kimi-code@0.28.1` → commit `efacf0452d46f5dbd67499eabc053869495d5213` | first qualified complete plan-mode milestone | 2026-08-25 | annotated tag `0032545b65f95c139ecba5a48ba1b911844e1ffe` |
| tag `@moonshot-ai/kimi-code@0.29.0` → commit `8bf5bacba9e524c38fb808c0122070037ead25a8` | maintained-range floor | 2026-08-25 | annotated tag `03c34eefa49513e6216390a9773326077a37f414` |
| tag `@moonshot-ai/kimi-code@0.38.0` → commit `0999454bdcb5ddd98f39bffee434dcf0a810f394` | current ceiling; Research 179 identity | 2026-08-25 | annotated tag `488fe6bb311959227c8c2602e12486e48f8b5446` |
| `packages/acp-adapter/src/modes.ts` @ `0.28.0..=0.38.0` | four-mode registry and `plan` → `setPlanMode(true)` + `setPermission('manual')` | 2026-08-25 | `f37dda20476f75c1aa0ffbe08b71eaac94a198079684e5b2b3c09edbf1d7f6d9` |
| `packages/acp-adapter/src/config-options.ts` @ `0.28.1` | `buildModeOption`; always appended | 2026-08-25 | `804e588478aa922326b7ef7f3076975a1afa5684fe5d7bd733200518f19c15e1` |
| `packages/acp-adapter/src/config-options.ts` @ `0.29.0..=0.38.0` | same `buildModeOption`; thinking rows expanded | 2026-08-25 | `4fb35fb760a868dff6ec0b212d050904eb48435701c42e86e3e192552b8d4567` |
| `packages/acp-adapter/src/session.ts` @ `0.28.1` | `setMode`; unknown id rejects before SDK | 2026-08-25 | `a4e30a1c7e8a5ed1a3c09be4077c8aa28901333141e21016c1a5b457b878b6da` |
| `packages/acp-adapter/src/session.ts` @ `0.29.0..=0.38.0` | byte-identical `setMode` to `0.28.1`; thinking path differs | 2026-08-25 | `1e4fe3cfd52b29cd4a3210099678cd486550b9cf012f078c40f66dbc0ff11e97` |
| `packages/acp-adapter/src/server.ts` @ `0.28.1` | `setSessionConfigOption` mode arm; load/resume `currentModeId = default` | 2026-08-25 | `d7ea7a3da9cab909306f34f788a1999a1ad742fe2e2310435ce5353116180957` |
| `packages/acp-adapter/src/server.ts` @ `0.29.0..=0.31.0` | mode arm identical; thinking effort rename | 2026-08-25 | `46fece2ed9c523dcfcda68087624ee796370751b5e8baba78312caa29cd68184` |
| `packages/acp-adapter/src/server.ts` @ `0.31.1..=0.38.0` | mode arm identical to `0.29.0` | 2026-08-25 | `b108fd4a66bcea09d9e0f35b1bb975f118dcb27276f6401f425da7df8ed3aa14` |
| `packages/acp-adapter/test/set-session-config-option.test.ts` @ `0.29.0..=0.38.0` | `configId="mode"` + four rows; SDK calls + `config_option_update` `currentValue` | 2026-08-25 | `31a25b8b5286ea9f195c6837de7b3401c5af577885ec0e6b1db710b3a24c1bd7` |
| `packages/acp-adapter/test/session-control.test.ts` @ `0.28.1..=0.38.0` | `session/set_mode` same mapping; unknown `modeId` before SDK | 2026-08-25 | `dacb00c2703dbd9c5af60d774a7d412db30c4f879e6486d45182b4166ec4947d` |
| `packages/acp-adapter/test/config-options.test.ts` @ `0.29.0..=0.38.0` | locked row order `default → plan → auto → yolo`; mode always present | 2026-08-25 | `cfc1608481cb5b3353f61cb0f74e5efad7bcfe5d105abe1d863ce0d5ec7811fb` |
| `packages/node-sdk/src/session.ts` @ `0.28.1` | `setPlanMode(boolean)` and `setPermission` with no fallback | 2026-08-25 | `6553f476fdc9d0baf0503cb11a779efc3a8d420055cd6d46c15d495cbc52bcc3` |
| `packages/node-sdk/src/session.ts` @ `0.38.0` | same `setPlanMode` / `setPermission` functions | 2026-08-25 | `379660a68e6114ded4fb63b910953f9282b64e09472687683f2d5e87aa7ba837` |
| `packages/node-sdk/src/rpc.ts` @ `0.28.1` | `setPlanMode(true)` → `enterPlan`; `false` → `cancelPlan` | 2026-08-25 | `2462e986cb89470615493655bb05a017e34f79a6d27197f647f9541a92e9705c` |
| `packages/node-sdk/src/rpc.ts` @ `0.38.0` | same `setPlanMode` mapping | 2026-08-25 | `301e92ad60b74169aa7673b296862e92fa249fd8a6b1894e7c0a7b5dd87c644e` |
| `packages/agent-core/src/agent/injection/plan-mode.ts` @ `0.28.0..=0.38.0` | plan-mode reminder; read-only except plan file; Bash follows permission mode | 2026-08-25 | `fc0f6e82102d88175f719829627c0df7ed90fa5a41496004c73fac8dcca0374c` |
| `packages/agent-core/src/agent/permission/policies/plan-mode-guard-deny.ts` @ `0.28.0..=0.38.0` | denies non-plan-file writes, `TaskStop`, cron mutation | 2026-08-25 | `01b5c86ffe1f7b7b4943f8c3f51e086e20193a5da01d1928be6d8f90d0abd939` |
| `packages/agent-core/src/agent/permission/policies/plan-mode-tool-approve.ts` @ `0.38.0` | approves `EnterPlanMode`, plan-file writes, `ExitPlanMode` | 2026-08-25 | `97b45eed12d345e13f45d57f4b999e6d1ed4ecfd12320971c185f0465270d14b` |

Official HTML is a lead only. Exact adapter / node-sdk / agent-core blobs own
the deliver-now claims. npm `latest` remained `0.38.0` during this probe.

`modes.ts` and `session-control.test.ts` are byte-identical at every checked
qualified point from `0.28.1` through `0.38.0`. `setMode` in `session.ts` is
byte-identical between `0.28.1` and `0.29.0..=0.38.0`. The `server.ts` mode
arm of `setSessionConfigOption` is identical across `0.28.1`, `0.29.0`, and
`0.31.1..=0.38.0`; the `0.31.1` server digest change is the thinking-effort
path already frozen by Research 207.

## Version Floor

| Boundary | Mode option | Complete `plan` set / confirm path |
| --- | --- | --- |
| published `0.28.0` | yes; same `modes.ts` / `buildModeOption` as `0.28.1` | yes, but outside the qualified ACP window |
| exact `0.28.1` | yes | yes |
| exact `0.29.0..=0.38.0` | yes; builder digest changes are thinking-only | yes |

First qualified complete plan-mode milestone is exact `0.28.1`. Immediately
preceding published tag `0.28.0` already carries the same ACP mode registry
and builder, but it is not a qualified ACP version. No qualified ACP point
lacks the mode path. No maintained-range split is required for plan mode.

## Option Construction

Exact `buildModeOption(currentModeId)`:

- `type: 'select'`, `id: 'mode'`, `name: 'Mode'`, `category: 'mode'`
- rows from `ACP_MODES` in locked order `default → plan → auto → yolo`
- row values are the ids; names/descriptions are display only
- `currentValue` is the adapter `AcpModeId`
- the mode option is always appended after model and optional thinking

Exact new-session / load / resume snapshots pass `DEFAULT_MODE_ID`
(`'default'`). Server comments freeze that load/resume always advertise
`currentModeId = default` because the SDK does not persist ACP mode across
runs.

Display labels (`Default`, `Plan`, `Auto`, `YOLO`) and the provider
description "Read-only planning; no tool execution" are not selection keys
and are not Swallowtail isolation claims.

## Selection, Application, Confirmation

Truth layers stay distinct:

| Layer | Exact finding |
| --- | --- |
| Advertisement | session-open `mode` select rows from `ACP_MODES` |
| Request | ACP `session/set_config_option` `{ configId: 'mode', value: 'plan' }` |
| Dispatch | `AcpSession.setMode` after `isAcpModeId` |
| SDK application | `setPlanMode(true)` then `setPermission('manual')`; node-sdk maps true to `enterPlan`, false to `cancelPlan` |
| Acceptance | unknown mode id / unknown `configId` / unknown session id → JSON-RPC `invalid_params` before SDK |
| Effectiveness | adapter stores `currentModeId` only after both SDK calls succeed; `setSessionConfigOption` then rebuilds `configOptions` from that stored id |
| Observation | response snapshot and `config_option_update` `currentValue` |

`session/set_mode` is the same `setMode` dispatcher but returns void. The
Swallowtail request is `session/set_config_option`, matching the existing
reasoning path, because only that method returns the refreshed snapshot.

Exact `acpModeToToggles`:

- `'default'` → `setPlanMode(false)` + `setPermission('manual')`
- `'plan'` → `setPlanMode(true)` + `setPermission('manual')`
- `'auto'` → `setPlanMode(false)` + `setPermission('auto')`
- `'yolo'` → `setPlanMode(false)` + `setPermission('yolo')`

No remap, nearest-value fallback, or permission substitution on the `plan`
arm. Adapter tests prove `plan` calls `setPlanMode(true)` and
`setPermission('manual')` and that the emitted `config_option_update`
`currentValue` equals `plan`. The RPC response is built by the same
`buildSessionConfigOptions(..., currentModeId)` after `setMode` returns.
Mode-specific tests assert the notification; the handler source makes the
response snapshot the Swallowtail confirmation channel.

`currentModeId` is request-echo after successful SDK calls, not
`getStatus().planMode` readback. `/status` can observe `status.planMode`,
but that is not the set-config confirmation channel. Swallowtail still
requires response `currentValue = plan` before readiness. Dispatch without
that snapshot is not confirmation.

Partial SDK failure: if `setPlanMode` throws, `setPermission` is skipped and
no snapshot is stored. If `setPlanMode` succeeds and `setPermission` throws,
`currentModeId` is not updated and no success response is returned. The
provider may have entered plan via `enterPlan`. Swallowtail fails closed and
joins already-allocated attachment work. No first prompt.

## Failure And Foreign-Value Disposition

| Case | Provider / adapter disposition | Swallowtail deliver-now disposition |
| --- | --- | --- |
| omitted harness mode | no mode request | unchanged wire |
| snapshot `plan` row | selectable | admit `HarnessMode::Plan` |
| snapshot `default\|auto\|yolo` | coexist; selectable by provider clients | coexist; never public |
| unknown / foreign mode value | `invalid_params` before SDK | unsupported / fail closed |
| missing / duplicate / malformed `mode` option | invalid option shape | malformed |
| wrong category / non-select | not produced by exact builder | malformed |
| missing confirmation or drifted `currentValue` | client-visible mismatch | fail closed |
| `session/set_mode` | same SDK mapping; empty success | do not use as the Swallowtail request |
| load / resume / import / recovery | adapter resets `currentModeId` to `default`; no client redeclaration path | no harness-mode mutation; reject before host effects |
| disconnect / deadline / cancel / cleanup after allocation | unchanged host lifecycle | join existing abort |
| UnverifiedNewer | not in this qualified claim | latest qualified mapping may be attempted; drift fails closed |

Provider-only rows must coexist so a valid `default|plan|auto|yolo` snapshot
is not treated as malformed. They never become public selections.

## Reasoning Composition

Plan mode and reasoning are separate `configId` arms. Each needs its own
plan constraint, snapshot membership, one set request, and effective
confirmation. One confirmation never proves the other.

Request order: keep the existing reasoning request first when reasoning is
present, then send the plan-mode request. Plan membership is checked against
the current snapshot immediately before that mode request: `session/new` when
reasoning is omitted, otherwise the reasoning confirmation snapshot. Exact
Kimi `session/set_config_option` rebuilds the full `configOptions` list, so
the pre-reasoning session-open snapshot is not current after thinking
succeeds. Omission of plan sends no mode request and preserves the current
reasoning-only wire. Omission of reasoning sends only the mode request.

Every Research 207-admitted reasoning value applicable to the exact version
composes:

- exact `0.28.1`: `off|on|low|medium|high` under legacy-select
- exact `0.29.0..=0.38.0`: `off|on|low|medium|high|xhigh|max` when the current
  thinking snapshot advertises the exact value

`setThinking` does not call `setPlanMode`. `setMode` does not call
`setThinking`. Joined failure: if the second request fails after the first
succeeded, abort the attachment before the first prompt.

## Isolation, Permission, And Access

`plan` keeps permission `manual`. It does not grant `auto` or `yolo`.
Agent-core plan-mode policy is prompt injection plus tool-policy: writes
outside the plan file, `TaskStop`, and cron mutation are denied; Bash still
follows the current permission mode. That is not process or filesystem
containment.

The route remains `AmbientHost` with delegated membership OAuth. Manual
permission and ambient isolation stay independent claims. Plan mode is not
reported as sandboxing.

## Compatibility

| Segment | Behavior revision | `HarnessMode::Plan` |
| --- | --- | --- |
| exact `0.28.1` | `kimi.acp.reasoning.legacy-select-v1` | yes |
| exact `0.29.0..=0.38.0` | `kimi.acp.reasoning.declared-effort-v2` | yes |
| later stable | visible `UnverifiedNewer` | does not inherit this qualified plan-mode claim |

No new behavior revision or maintained-segment split is required. Advertise
`HarnessModeSelection(Plan)` on both existing qualified ACP revisions.

## Deliver-Now Table

| Version range | Value | Snapshot gate | Set / confirm gate | Lifecycle |
| --- | --- | --- | --- | --- |
| exact `0.28.1` | `HarnessMode::Plan` | current `mode` select advertises exact `plan` | one `session/set_config_option` `{ configId: 'mode', value: 'plan' }`; response `currentValue` must equal `plan` | new-session only |
| exact `0.29.0..=0.38.0` | `HarnessMode::Plan` | same | same | new-session only |

Non-goals retained: public `default|auto|yolo`, generic configuration,
display-label translation, aliases, fallback, permission widening, load /
resume / import / recovery mutation, headless / local-server / Platform
routes, live OAuth / prompt work, and `UnverifiedNewer` inheritance.

## Promotion Gate

Promoted with a non-empty exact deliver-now set. Cards 171-172 may bind and
accept only the rows above under existing Contracts 012/017/023/029/034/037/
040/041/052. No shared contract or runtime change is required.
