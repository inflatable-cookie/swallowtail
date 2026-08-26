# 220 Cline Headless Plan-Mode Evidence

Status: promoted
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Card: g04.073 / 201

## Question

Which exact `cline.headless` value, if any, can Swallowtail bind on qualified
package `3.0.55` as portable `HarnessMode::Plan` through one fixed process
argument without treating provider Plan behavior as isolation, inventing an
effective JSON observation, or letting the model widen itself to Act during
the selected one-prompt run?

## Method And Boundary

Official Cline CLI documentation and exact `cline@3.0.55` artifacts were
inspected on 2026-08-26 in a disposable directory. Cline was not installed onto
the host, no platform binary was downloaded or executed, and no login,
credential, account, catalogue, or `--json` prompt was used. Host PATH still
has no `cline`.

The selected route remains `cline.headless` (behavior
`cline.headless.stdio-json-v1`, argv `cline --json --auto-approve false` plus
one prompt operand and `-c`). It sits on axis `cline.package` `3.0.55` and
selects no provider or model. `cline.acp` is cited only as the ACP early-return
contrast.

Current official pages are leads. The exact package is the finding.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| npm `cline@3.0.55` metadata | wrapper identity; integrity unchanged from Research 146/147/190 | 2026-08-26 | metadata `3ea4bd2d8eea7eca997a94e014047dedf9d14bb4dc35b3aff7038cde86a6abd3`; integrity `sha512-3JQ5vPl8/BIyTShTpn0VDX59lbzFhGxRwYqZmAVLOPcX83ETaHbgH2fc9iJYv1pB2ChIpXQEXmFHowgz8F1GgQ==` |
| [CLI reference](https://docs.cline.bot/cli/cli-reference) | current official `-p, --plan` help line | 2026-08-26 | `08a0c4d7fa418f89ecc1624c42377d337349536a05051f6024e7418ecf8c5a48` |
| `apps/cli/package.json` | `@cline/cli@3.0.55` at the tagged commit | 2026-08-26 | `669d6f7b553bdeb6131894cd55cb81f191c03e767d5186a5ff71d0be181e9064` |
| `apps/cli/README.md` | byte-identical to the published wrapper README | 2026-08-26 | `94c3c1b240b5e8a971cc806876dde2e893c78c8cc0085efe23c961e113c197ad` |
| `apps/cli/src/commands/program.ts` | commander `-p/--plan`, hidden `--act/--yolo/--zen`, `commanderToParsedArgs` | 2026-08-26 | `392401e42a77120d80b852eed43b81a8f24f1b2fe1d399a2a952181583692aec` |
| `apps/cli/src/utils/helpers.ts` | pre-commander argv normalize; `parseArgs` | 2026-08-26 | `61330c996391129868de943cd04d093532c4e4bee865f4b0b44aca521666488e` |
| `apps/cli/src/utils/startup-settings.ts` | explicit CLI mode wins persisted `planActMode` | 2026-08-26 | `ca5868e8d82202d9f57e215ca1f3b857c968bc4e361761a4321131780993fcf7` |
| `apps/cli/src/main.ts` | ACP early return; headless config.mode and system prompt | 2026-08-26 | `1b7a0a5b680aa6f3f736826c449f64e9a62dbe7b57e0b55cbc74379d45f37274` |
| `apps/cli/src/acp/index.ts` | ACP entry takes auto-approve only | 2026-08-26 | `8cbcf4c04bb9b01ccc877c661210346380732a32b796ac8335663b5d141adbb7` |
| `apps/cli/src/runtime/run-agent.ts` | one-shot `interactive: false`; verbose-gated `run_start.mode` | 2026-08-26 | `34d6dfc8e7324ad91f6e6a9daa032d11aeed36fbd9e1e790af3d9f3c828de5e6` |
| `apps/cli/src/runtime/prompt.ts` | `resolveSystemPrompt({ mode })`; default switch-tool prompt | 2026-08-26 | `95481c14e59a60ef8995c66b5ba94ca664178dcd706d6ecee8c0fff736ce818a` |
| `apps/cli/src/runtime/run-interactive.ts` | sole `switch_to_act_mode` registration and auto-continue | 2026-08-26 | `6e26e0d97ab4dbfca24cf463328d9bafbb1ca508aa2f46a25b4c83ee65d16b6f` |
| `apps/cli/src/runtime/interactive/mode.ts` | Plan-to-Act tool and continuation prompt | 2026-08-26 | `53ef8ba28190c3836597d5127447cf9a925bf05eb71a88d8da20933109aab844` |
| `sdk/packages/shared/src/prompt/cline.ts` | `MODE_TAG_INSTRUCTIONS` / `PLAN_MODE_INSTRUCTIONS` | 2026-08-26 | `33e078f5618344aa249c7b7c226e5f30e72690b59bd0b08f4397efa116406646` |
| `sdk/packages/shared/src/prompt/format.ts` | `<user_input mode="plan\|act\|yolo">` | 2026-08-26 | `1b78fabfafa0508ee34d03691ca1f16e5e231b7069fd7e68776b10759cdf1976` |
| `sdk/packages/core/src/extensions/tools/presets.ts` | plan preset: editor off, bash/search/web/spawn/teams on | 2026-08-26 | `fb86bc1acff114963a9214137eda66758819b7f7176d11fc7bd3a9b322903ce7` |
| `sdk/packages/core/src/extensions/tools/command-guard.ts` | plan-mode `run_commands` blacklist | 2026-08-26 | `f56d4deb3e5ba8ea0def770c50ebfa2387d70fe36087765153238746da527132` |
| `sdk/packages/core/src/extensions/tools/command-guard-extension.ts` | `beforeTool` guard registered for plan sessions | 2026-08-26 | `f3c75bed0e8505e525f73f95e45d26c1d2cd001e84eb9d50bf99035904078ba5` |
| `sdk/packages/core/src/runtime/orchestration/runtime-builder.ts` | preset + guard + delegated-agent inheritance | 2026-08-26 | `d7204b81e55b60b5472c28fe29cff7b888e73779984c1db76e90438cb7791a03` |
| `sdk/packages/core/src/runtime/host/local-runtime-host.ts` | turn input stamped with session mode | 2026-08-26 | `61fce1f2f9b9061b86721f48a7a1786fa869fc96b566c4cd18fc3379c0ed0743` |

Source paths are read at GitHub commit
`ad442cbb6a81d21773ceabc1398ea5eb58170718`, the commit Research 146, 147, and
190 already froze for tag `cli-v3.0.55` (annotated
`c238103e631d492b97bf9e63b060390f1bb8a8a6`). `apps/cli/package.json` there is
`@cline/cli@3.0.55`. `apps/cli/README.md` remains byte-identical to the
published wrapper README (`94c3c1b2…`). Wrapper integrity is unchanged from
Research 146.

HTML digests identify retrieved documentation bodies. They are not a
compatibility guarantee. The platform executable was not extracted, so every
finding below is source-level for the exact tagged package point.

## Parser

`-p` / `--plan` is a boolean commander option. It takes no value. Canonical
spelling for this lane is `--plan`.

`commanderToParsedArgs` resolves mode as:

```ts
mode: opts.plan ? "plan" : opts.yolo ? "yolo" : opts.zen ? "zen" : "act",
modeExplicitlySet: !!(opts.plan || opts.act || opts.yolo || opts.zen),
```

`--plan` wins over hidden `--yolo` / `--zen` when more than one mode flag is
present. `--act` is hidden and only marks `modeExplicitlySet`; without `--plan`
the ternary still yields `"act"`. `--plan` is not last-wins across different
mode flags.

`--plan` does not consume the positional prompt. `--json`, `--auto-approve
false`, `-c`, and the prompt operand remain independently parsed. Invalid
non-boolean `--plan=<value>` is a commander parse concern, not a portable
input. This lane emits only the canonical boolean `--plan`.

Omission leaves `modeExplicitlySet` false and parsed mode `"act"`.

## Precedence And Application

`resolveStartupMode` applies explicit CLI first, then persisted
`settings.planActMode`, then the parsed default:

```ts
if (args.modeExplicitlySet) {
    return args.mode;
}
return settings.planActMode ?? args.mode;
```

`--plan` therefore wins every ambient `planActMode` that could otherwise choose
Act or restore a prior TUI setting. CLI `--plan` does not write `planActMode`;
`saveProviderSettings` persists provider/model/key only.

ACP returns before this resolution:

```ts
if (args.acpMode) {
    await runAcpMode({ autoApproveTools: args.autoApproveOverride === true });
    return;
}
```

`cline --json --acp` is still ACP. Root `--plan` is discarded on that path.
This lane is headless-only.

On the selected JSON path, `effectiveMode` is copied into `config.mode` and
`resolveSystemPrompt({ mode: effectiveMode })`. `runAgent` starts the session
with `interactive: false` and that config. The runtime builder then:

- maps `mode === "plan"` to the `plan` tool preset
- registers `createPlanModeCommandGuardExtension` when tools are enabled
- stamps user input with `formatModePrompt(..., session.config.mode)`

`main.test.ts` proves persisted `planActMode: "plan"` reaches `runAgent` on a
single-prompt run, and that an explicit `--act` beats that persistence. The
same `effectiveMode` field is what `--plan` sets.

## Plan Behavior

Exact Plan on this package is provider behavior, not isolation.

System prompt: `buildClineSystemPrompt` appends `MODE_TAG_INSTRUCTIONS` for
every mode and `PLAN_MODE_INSTRUCTIONS` when `mode === "plan"`. The headless
caller does not pass `planModeSwitchTool: false`, so the prompt tells the model
to use `switch_to_act_mode`. That tool is not registered on `runAgent`.

Tool preset `plan`: read/search/bash/web/skills/ask on; editor and apply-patch
off; spawn agents and teams on. Delegated agents inherit `normalized.mode` and
the same plan-mode command-guard extension.

Command guard: a `beforeTool` blacklist on `run_commands` only. It rejects
common file-editing command words, some git/package-manager subcommands, and
output redirection. The source states it is not a shell interpreter and will
not catch every mutation (`python -c "open(..., 'w')"`, quoted `bash -c`).
Blocked calls `skip` with a tool error; they do not stop the run. This is not
filesystem, network, shell, process, sandbox, or descendant containment.

MCP tools from ambient settings still load unless `disableMcpSettingsTools`.
`--auto-approve false`, read-only working-resource policy, `Ambient`, and
`AmbientHost` remain independent.

## Plan-To-Act

`switch_to_act_mode` and `sendTurnWithActModeContinuation` live in
`run-interactive.ts` only. `runAgent` does not attach `extraTools`, does not
rebuild the session for Act, and does not send the synthetic continuation
prompt. The selected one-prompt JSON operation cannot transition to Act
through that machinery. A model call to the missing tool is an ordinary
unknown-tool error, not a mode change.

Interactive Plan-to-Act, TUI toggles, queued/steered turns, and reusable
sessions stay out of this route.

## Output And Observation

`run_start` includes `mode` only inside `printModelProviderInfo`, which
`runAgent` calls under `if (config.verbose)`. The selected argv does not pass
`--verbose`. No effective-value observation is synthesized from an unselected
verbose envelope. `run_result` reports `finishReason`, text, usage, and model;
it does not report mode.

Requested / planned / dispatched states are Swallowtail-owned. Parser-accepted
and applied states are source-visible. Observed effective mode is withheld.

## Omission, Lifecycle, And Binding Seam

Omission retains exact current argv
`cline --json --auto-approve false -c <cwd> <prompt>` and leaves mode to
provider default plus persisted `planActMode`. That ambient default is
existing route truth, not a new Plan claim.

Selected Plan is immutable for the one child: argv is fixed at spawn; the
headless path does not rebuild mode. Canonical placement is `--plan` after
`--auto-approve false` and before `-c <cwd> <prompt>`.

Existing one-run activity, malformed JSON, terminal, cancellation, host
deadline, failure, provider retention, and joined cleanup do not depend on
mode. CLI `--timeout`, `--id`, `--yolo`, `--zen`, `--acp`, and
`--auto-approve true` stay unselected.

Production `ClineHeadlessRunProfileInput` currently has no harness-mode field.
`ClineHeadlessDriver` validation currently rejects any
`request.policy().harness_mode()`. The smallest binding is optional portable
`HarnessMode::Plan` on that input, advertised as `HarnessModeSelection(Plan)`
on exact `3.0.55`, copied onto immutable plan/evidence/policy, and emitted as
canonical `--plan`. No behavior revision is required:
`cline.headless.stdio-json-v1` is unchanged.

## Route And Value Dispositions

| Route | Value | CLI parse | Applied by the exact route | Disposition | Reason |
| --- | --- | --- | --- | --- | --- |
| `cline.headless` | portable `HarnessMode::Plan` via `--plan` | accepted | yes: config.mode, prompt, user-input tag, plan preset, command guard; no Act switch on this path | deliver now | complete fixed-argument Plan posture on the selected JSON child |
| `cline.headless` | omitted | n/a | existing argv and provider-default / persisted mode | retain existing behavior | not implicit Plan |
| `cline.headless` | provider `act` / `--act` | accepted | default / explicit Act | withheld | not a public Swallowtail value |
| `cline.headless` | provider `yolo` / `--yolo` | accepted | yolo preset and auto-approve true | withheld | hidden flag; auto-approve true stays out |
| `cline.headless` | provider `zen` / `--zen` | accepted | hub dispatch | withheld | not this route |
| `cline.acp` | `--plan` | accepted then discarded | no | not applicable | ACP returns before `effectiveMode` |
| any other package point | all | n/a | n/a | evidence-gated | no `UnverifiedNewer` inheritance; currentness stays standing |

## Contract 034 Decision

`HarnessMode::Plan` needs one exact portable posture that the qualified route
dispatches as a fixed process argument, keeps for the bounded run, and does
not confuse with isolation or permission.

Exact `3.0.55` headless `--plan` does that. Explicit selection wins ambient
settings, reaches the one-run config, and is enforced by the plan tool preset
plus a pre-approval command guard. The selected JSON child cannot widen itself
to Act. Contract 034 already says Plan is behavioral, not read-only access.
Remaining bash/MCP/spawn surfaces and the command blacklist are provider Plan
behavior. They are not a Swallowtail containment claim and do not fail
equivalence.

Observation stays withheld: the selected wire does not report mode without
unselected `--verbose`. ACP, thinking, model, timeout, auto-approve true, and
runtime Plan-to-Act stay out.

## Behavior Revision And Compatibility

No behavior revision is needed or proposed. `cline.headless.stdio-json-v1` and
the `cline.package` `3.0.55` qualified point are unchanged. Contract 029
currentness stays in its standing lane; this record qualifies no newer point.

## Promotion

Research 220 promotes one deliver-now row:

| Segment | Portable value | Dispatch | Observation | Lifecycle |
| --- | --- | --- | --- | --- |
| exact `cline.headless` `3.0.55` | `HarnessMode::Plan` | canonical `--plan` before `-c <cwd> <prompt>` | withheld | one child; no Plan-to-Act |

Cards 202-203 may bind that row through prepared input, capability,
immutable plan/evidence, driver validation, and argv. Omission must keep the
current argv. Plan grants no permission, tool, filesystem, network, sandbox,
shell, process, descendant, model, or account authority.
