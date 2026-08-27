# 240 Cline ACP Plan-Mode Evidence

Status: promoted
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.085 / 241; delivered g04.086 / 242-243

## Question

Which exact `cline.acp` `3.0.55` value and lifecycle rows, if any, can bind
caller-selected Plan mode through the ACP path with pre-prompt application and
selected-value confirmation?

## Decision

Promote one deliver-now row. Exact `cline.acp` `3.0.55` advertises Plan on
`session/new`, accepts caller selection through `session/set_config_option`
`{ configId: "mode", value: "plan" }` before the first `session/prompt`,
confirms with response `configOptions` `mode.currentValue === "plan"`, and
applies that mode when `ensureSessionManager` builds the first runtime config.

Root `--plan` stays sibling headless evidence only (Research 220). It is
discarded by the ACP early-return and is not this row.

## Method And Boundary

Official Cline ACP and Plan/Act documentation plus exact `cline@3.0.55` /
GitHub `cli-v3.0.55` ACP sources were inspected on 2026-08-27 in a disposable
directory. No Cline install, platform binary extraction, login, credential,
account, catalogue, `initialize`, or provider prompt was used. Host PATH still
has no `cline`.

The selected route remains `cline.acp` (ACP v1 stdio, argv `cline --acp`,
caller working resource, observational permissions, no auto-approve). It sits
on axis `cline.package` `3.0.55`. Research 146 freezes identity. Research 220
is cited only as the headless `--plan` / ACP early-return contrast.

Current official pages are leads. The exact package is the finding.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| npm `cline@3.0.55` metadata | wrapper identity; integrity unchanged from Research 146/220 | 2026-08-27 | metadata `3ea4bd2d8eea7eca997a94e014047dedf9d14bb4dc35b3aff7038cde86a6abd3`; integrity `sha512-3JQ5vPl8/BIyTShTpn0VDX59lbzFhGxRwYqZmAVLOPcX83ETaHbgH2fc9iJYv1pB2ChIpXQEXmFHowgz8F1GgQ==` |
| [ACP](https://docs.cline.bot/usage/acp.md) | official Plan/Act client-mode selector lead | 2026-08-27 | `f421629eae0d4ae5b451d0085b8395f14ce1a103ba54774a275e09a1e7099086` |
| [Plan & Act Mode](https://docs.cline.bot/core-workflows/plan-and-act.md) | product Plan/Act behavior lead | 2026-08-27 | `8213ccb40b5a5db1b0ca23b61ac78e54fb9931503af8df05e4eb410e93e2d23e` |
| [CLI reference](https://docs.cline.bot/cli/cli-reference.md) | `--acp` / `--plan` coexistence lead | 2026-08-27 | `c39fb3197cb72491da81cb74c1e01cf15f60b3cde8218162e7510a30cce0f72f` |
| `apps/cli/src/main.ts` | ACP early-return discards root mode argv | 2026-08-27 | `1b7a0a5b680aa6f3f736826c449f64e9a62dbe7b57e0b55cbc74379d45f37274` |
| `apps/cli/src/acp/index.ts` | ACP entry takes auto-approve only | 2026-08-27 | `8cbcf4c04bb9b01ccc877c661210346380732a32b796ac8335663b5d141adbb7` |
| `apps/cli/src/acp/acpAgent.ts` | modes, setSessionMode, setSessionConfigOption, buildConfig | 2026-08-27 | `248092d41e330ef1898f98b99d35c6713574a7b9305d95601177c07e64db9e71` |
| `apps/cli/src/acp/session-updates.ts` | `current_mode_update` / `config_option_update` | 2026-08-27 | `e7f8beca5101fcdfc8079025980b479918c4959fcc34728452ed94f4e1e18fe0` |
| `apps/cli/src/acp/session-load.ts` | load replay; Act continuation filter only | 2026-08-27 | `90bf351aaab8a1dd37b89df609b195af914b6a8e0a70a5cfa167a3a1e37dea38` |
| `apps/cli/src/session/session.ts` | `createCliCore`; no Plan-to-Act extra tool | 2026-08-27 | `2782c2cdf2edf743d6095192d798a3f151c3a673f16a63e2f0c311e909e93078` |
| `apps/cli/src/runtime/prompt.ts` | `resolveSystemPrompt({ mode })`; no `planModeSwitchTool: false` | 2026-08-27 | `95481c14e59a60ef8995c66b5ba94ca664178dcd706d6ecee8c0fff736ce818a` |
| `apps/cli/src/runtime/interactive/mode.ts` | Plan-to-Act tool lives on TUI interactive path only | 2026-08-27 | `53ef8ba28190c3836597d5127447cf9a925bf05eb71a88d8da20933109aab844` |
| `sdk/packages/shared/src/prompt/cline.ts` | `PLAN_MODE_INSTRUCTIONS`; default switch-tool prompt | 2026-08-27 | `33e078f5618344aa249c7b7c226e5f30e72690b59bd0b08f4397efa116406646` |
| `sdk/packages/core/src/runtime/orchestration/runtime-builder.ts` | plan preset + command-guard extension | 2026-08-27 | `d7204b81e55b60b5472c28fe29cff7b888e73779984c1db76e90438cb7791a03` |
| `sdk/packages/core/src/extensions/tools/presets.ts` | plan tool preset | 2026-08-27 | `fb86bc1acff114963a9214137eda66758819b7f7176d11fc7bd3a9b322903ce7` |
| Frozen fixture `cline-acp-3.0.55/` | initialize / session-new / protocol baseline | 2026-08-18 | workspace |
| Frozen evidence `plan-mode-evidence.json` | closed ACP Plan selection/confirm shapes | 2026-08-27 | workspace |
| Prepared guide `cline-acp-prepared-integration.md` | current omission; plan/act not yet bound | 2026-08-27 | workspace |
| Adapter audit `driver/validation.rs` | current open rejects `harness_mode` | 2026-08-27 | workspace |
| Research 146 / 220 | identity and headless `--plan` contrast | 2026-08-27 | promoted siblings |

Source paths are read at GitHub commit
`ad442cbb6a81d21773ceabc1398ea5eb58170718`, the commit Research 146 and 220
already froze for tag `cli-v3.0.55`. Wrapper integrity is unchanged.

HTML digests identify retrieved documentation bodies. They are not a
compatibility guarantee. Findings below are source-level for the exact tagged
package point.

## ACP Protocol Surface

Official ACP docs advertise Plan/Act switching from the client mode selector.
Exact `AcpAgent` implements both `setSessionMode` and `setSessionConfigOption`.

| Frame | Mode-relevant fields | Selection seam |
| --- | --- | --- |
| `initialize` result | agent identity / capabilities; no modes | none |
| `session/new` result | `modes.availableModes` (`plan`,`act`), `currentModeId: "act"`, `configOptions` including `mode` | snapshot only; default Act |
| `session/set_mode` | request `modeId`; response `{}`; emits `current_mode_update` | applies ACP session state; empty RPC confirmation |
| `session/set_config_option` `mode` | request value `plan\|act`; response rebuilt `configOptions`; emits mode + config updates | selected-value confirmation |
| `session/prompt` terminal | `stopReason` only | no mode field |
| `session/load` (fresh connection) | rebuilds session with `currentMode: "act"` | no restored Plan |
| `session/update` | `current_mode_update`, `config_option_update` | observation / confirm side channels |

`initialize` does not own Plan. Root argv `--plan` never reaches ACP setup:

```ts
if (args.acpMode) {
    await runAcpMode({ autoApproveTools: args.autoApproveOverride === true });
    return;
}
```

## Selection, Application, Confirmation

Truth layers stay distinct:

| Layer | Exact finding |
| --- | --- |
| Advertisement | `session/new` returns `plan`/`act` modes and `buildModeConfigOption("act")` |
| Request | ACP `session/set_config_option` `{ configId: "mode", value: "plan" }` |
| Compatibility path | `session/set_mode` `{ modeId: "plan" }` updates the same `session.currentMode` but returns `{}` |
| Acceptance | unknown mode / unknown configId / unknown session → throw / `invalid_params` before prompt |
| Pre-prompt application | first `ensureSessionManager` calls `buildConfig(session)` with `mode: session.currentMode` and `resolveSystemPrompt({ mode })`, then `sessionManager.start({ ..., interactive: true, config })` |
| Effectiveness | runtime builder maps `mode === "plan"` to the plan tool preset and plan command-guard extension; turns stamp `formatModePrompt(..., session.config.mode)` |
| Observation | set-config response `mode.currentValue`; optional `current_mode_update` |

The Swallowtail request is `session/set_config_option`, not `session/set_mode`.
Only set-config returns the refreshed snapshot whose `mode.currentValue` must
equal `plan` before readiness. Matching Kimi Research 208, empty
`setSessionMode` success is insufficient confirmation by itself.

## Lifecycle And Stickiness

| Lifecycle | Disposition | Reason |
| --- | --- | --- |
| new session, select before first prompt | deliver now | `session.currentMode` is copied into runtime config at first `ensureSessionManager` |
| follow-up prompts on that manager | retain selected Plan | later `send({ prompt })` uses baked `session.config.mode`; no Plan-to-Act extra tool is registered on the ACP `createCliCore` path |
| set mode after the manager already started | withheld | `setSessionMode` / set-config update ACP `SessionState` and notifications, but do not rebuild `sessionManager` config or system prompt |
| `session/load` on a fresh connection | no Plan restoration | hard-defaults `currentMode: "act"`; no client redeclaration of prior Plan |
| live in-connection load | returns current ACP `session.currentMode` | still not a Swallowtail load/resume binding; load stays unmapped |
| fresh replacement session | new negotiation | new `session/new` defaults to Act again |
| omission | retain existing frames | no mode request; default Act; no Plan claim |

Unsupported / foreign mode values reject on the set call before
`session/prompt`. Omission sends no mode request and keeps current initialize /
`session/new` / prompt frames.

## Plan Behavior On ACP

Exact Plan on this package is provider behavior, not isolation. Research 220's
behavioral inventory still applies once `config.mode === "plan"`:

- system prompt appends plan-mode instructions (default text mentions
  `switch_to_act_mode` because ACP does not pass `planModeSwitchTool: false`)
- plan tool preset: editor off; search/bash/web/spawn surfaces on
- plan command-guard blacklist on `run_commands` only; not filesystem, network,
  shell, process, sandbox, or descendant containment
- MCP tools from ambient settings may still load

ACP starts the manager with `interactive: true`, but `createCliCore` does not
attach the TUI `switch_to_act_mode` `extraTools` registration from
`run-interactive.ts`. A model call to that missing tool is an ordinary
unknown-tool error, not a mode change. Interactive Plan-to-Act, TUI toggles,
and ambient `planActMode` persistence stay out of this deliver-now row.
`newSession` hardcodes default mode `"act"` and does not read persisted
`planActMode`.

Permissions remain observational `allow_once` / `allow_always` /
`reject_once`. Plan does not widen auto-approve. Swallowtail still must not
select `allow_always`. Working resource stays `session/new` `cwd`.

## Route And Value Dispositions

| Route | Value | ACP parse | Applied by the exact route | Disposition | Reason |
| --- | --- | --- | --- | --- | --- |
| `cline.acp` | portable `HarnessMode::Plan` via config `mode=plan` | accepted | yes, when set before first prompt | deliver now | confirmed ACP session option with pre-prompt application |
| `cline.acp` | omitted | n/a | default Act on `session/new` | retain existing behavior | not implicit Plan |
| `cline.acp` | provider `act` | accepted | default / explicit Act | withheld | not a public Swallowtail value |
| `cline.acp` | `session/set_mode` alone | accepted | same state field; empty RPC body | withheld as Swallowtail request | use set-config confirmation |
| `cline.acp` | post-start mode change | accepted then diverges | ACP state updates; runtime config sticky | withheld | effective-value mismatch risk |
| `cline.acp` | `session/load` Plan restore | n/a | fresh load defaults Act | not applicable | no restored Plan binding |
| `cline.acp` | root `--plan` | accepted then discarded | no | not applicable | Research 220 ACP early-return |
| `cline.headless` | `--plan` | accepted | yes on JSON child | sibling only | Research 220; do not promote onto ACP |
| any other package point | all | n/a | n/a | evidence-gated | no `UnverifiedNewer` inheritance |

## Contract 034 Decision

Contract 034 already authorizes mapping `HarnessMode::Plan` through an ACP mode
when qualified behavior is equivalent. Exact `3.0.55` ACP exposes the
negotiation sequence on new sessions:

1. `session/new` advertises selectable `plan`
2. caller sends one set-config mode request
3. response confirms `currentValue: "plan"`
4. first prompt builds runtime config from that stored mode

That is equivalent portable Plan posture to Research 220's headless row for
behavior (plan preset + command guard; no registered Act switch on this path),
carried on an ACP session option instead of a process argument. It remains
behavioral Plan, not read-only access, sandboxing, permission, or containment.

Production binds optional `ClineSessionProfileInput::with_harness_mode(Plan)`
on exact `3.0.55` and requires set-config confirmation before readiness.
Omission still sends no mode request. No behavior revision is required:
`cline.acp` ACP v1 stdio identity stays unchanged.

## Deliver-Now Table

| Segment | Portable value | Snapshot gate | Set / confirm gate | Lifecycle |
| --- | --- | --- | --- | --- |
| exact `cline.acp` `3.0.55` | `HarnessMode::Plan` | `session/new` `mode` select advertises exact `plan` | one `session/set_config_option` `{ configId: "mode", value: "plan" }`; response `mode.currentValue` must equal `plan`; must complete before first `session/prompt` | new-session only |

## Omission And Permissions

Omission retains exact current frames: `cline --acp`, initialize, `session/new`
with default Act, one bounded text `session/prompt`, observational permissions,
and joined cleanup. No Plan claim.

Plan grants no permission, tool, filesystem, network, sandbox, shell, process,
descendant, model, account, auto-approve, load, or resume authority.

## Behavior Revision And Compatibility

No behavior revision is needed or proposed. The `cline.package` `3.0.55`
qualified point is unchanged. Contract 029 currentness stays in its standing
lane; this record qualifies no newer point.

## Promotion

Research 240 promotes one deliver-now row. Card 241 is complete. Cards 242-243
bind `HarnessModeSelection(Plan)` on exact `3.0.55` ACP, negotiate after
`session/new`, require set-config confirmation, apply before first prompt, and
keep omission on the current wire. Shared inventory, programme, indexes, and
Next Task remain orchestrator-owned after merge.

## Delivery Evidence

Worker delivery under g04.086 cards 242-243:

- prepared `ClineSessionProfileInput::with_harness_mode(Plan)` binds capability,
  plan, evidence, and `SessionOptions` agreement
- driver requires unique `session/new` modes/`configOptions` plan membership,
  one correlated `session/set_config_option`, and `mode.currentValue = plan`
  before returning a usable handle
- deterministic fixtures cover positive dispatch, omission, missing/ambiguous/
  malformed/rejected/mismatched confirmation, permission non-widening, and
  fresh context-losing replacement renegotiation
- guide, route/feature matrices, example, package API baseline, and changelog
  claim only the exact requested/dispatched/confirmed Plan row

## Evidence

- [plan-mode-evidence.json](../../crates/swallowtail-adapter-cline/tests/fixtures/cline-acp-3.0.55/plan-mode-evidence.json)
