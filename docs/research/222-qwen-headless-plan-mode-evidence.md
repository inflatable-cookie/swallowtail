# 222 Qwen Headless Plan-Mode Evidence

Status: promoted
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Card: g04.075 / 207

## Question

Do exact maintained Qwen Code points `0.21.15`, `0.22.0`, or `0.22.1` apply
`--approval-mode plan` to every selected `qwen.headless` child with behavior
equivalent to portable `HarnessMode::Plan`, without provider work, ambient
configuration mutation, or authority widening?

## Method And Boundary

Official npm metadata, GitHub source at the frozen gitHeads, current official
docs, and the existing Qwen fixtures were inspected on 2026-08-26. Qwen was
not installed onto the host. No login, account, catalogue, prompt, tool
execution, paid work, or ambient config mutation was used. Host PATH still has
no `qwen`.

The selected route remains `qwen.headless` (driver `swallowtail.qwen.headless`,
axis `qwen-code.package`). Current argv is `--safe-mode --approval-mode default`
plus stream-json output, explicit `--exclude-tools`, model, 60-second wall
time, and adapter-held turn/tool budgets. `--core-tools` is present on the
current argv but is ignored under `--safe-mode` at every named point; that is
existing route truth, not a Plan claim.

Current official pages corroborate. The exact packages are the finding.

## Frozen Sources

Identities match Research 173 and 216.

| Source | Use | Retrieved | Digest |
| --- | --- | --- | --- |
| npm `@qwen-code/qwen-code@0.21.15` | package identity | 2026-08-26 | integrity `sha512-f4ER/SRVLpwhcqzuytK3Qeq8bG9HnVhv7f7wsf3cpE/AkRfzKSvaeURnW7s7zI3nWkEqA7DM6njSLYS2s6DWDg==`; gitHead `5dce2515a778f9cf2013168962b4fbc3454636e3`; tarball SHA-256 `8d405b065888b7000a6989d99c2d79257cd8f9f5b68e9078fb76484527351b9a` |
| npm `@qwen-code/qwen-code@0.22.0` | package identity | 2026-08-26 | integrity `sha512-y66e3+gVso86miKbp1vc81cJ/RGx/OKvVlFGpMX09tFS3jvQyEmqa4VPYAMx/++04glRGIYMyv98pipoMMN1Qg==`; gitHead `1c3a385d9bc83e0b2a1ce5a24454ce1d090595fb`; tarball SHA-256 `c0ae0ad006c4dd8b69ebe1705d13bb57d37d1c808dcb891c5bfcde91e66670c2` |
| npm `@qwen-code/qwen-code@0.22.1` | package identity | 2026-08-26 | integrity `sha512-sDki8GaxUA7eEbo1SQNd15TXiP22CMmOpUmfKeDvl+vmyw5sMwX5XJunQ8R4zReRV8z+HIaqqK5u28UX807lhw==`; gitHead `2755dbe1399f94e53e24377d2e21fa86ce923529`; tarball SHA-256 `1108f84ad96f9582c7513f4d83fde2e015b54d0b32239943b1c4ce4044a0f998` |
| [Approval Mode docs](https://qwenlm.github.io/qwen-code-docs/en/users/features/approval-mode/) | current official Plan/Default/Auto-Edit/Auto/YOLO table | 2026-08-26 | corroboration only |
| [Headless docs](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/) | `--approval-mode` and `--safe-mode` still apply together | 2026-08-26 | corroboration only |
| `packages/cli/src/config/config.ts` @ `0.21.15` | parser, precedence, safe-mode, exclude-tools | 2026-08-26 | git `8babd54e3fac09519d584f305a1d5e098c504867`; SHA-256 `a195b3a8782eab208559620c0c24649a6e685c48559aafdcb725738fb3f27042` |
| `packages/cli/src/config/config.ts` @ `0.22.0` | same parser family as `0.21.15` | 2026-08-26 | git `f667e4a24a1bfa85c2af61735b8ac126e8afdc5b`; SHA-256 `3b324ce5e917d196a6d26418176d07a91996f74c7042b3774d156a7a531ef16a` |
| `packages/cli/src/config/config.ts` @ `0.22.1` | shared `APPROVAL_MODES` parser | 2026-08-26 | git `6379739b11763be1b3bb63564600c0be312a7246`; SHA-256 `31fbb32cc6635fafe587338b07092f8c6eac4a66643fbb60c2c26093f92acb65` |
| `packages/core/src/config/approval-mode.ts` | enum `plan\|default\|auto-edit\|auto\|yolo` | 2026-08-26 | `0.21.15` git `1697eade3b0e966dc788e9a20f68203074bcd703`; `0.22.1` git `103535c542cf3f9038dcc174818d898803a27cf4` (adds `ApprovalModeValue`) |
| `packages/core/src/core/permissionFlow.ts` | `isPlanModeBlocked` | 2026-08-26 | `0.21.15` git `a1a9cafdafa861ef72e31fbf646a81e9fadda317`; `0.22.1` git `f2cb3dbb6569212ddc1edd856d3918c2bd3fc41c` |
| `packages/core/src/core/plan-mode-shell-policy.ts` | Plan shell classification | 2026-08-26 | git `799ce4b87e01cdc0cb8ccf696f0cb39fc91cfa9b`; byte-identical `0.21.15` through `0.22.1` |
| `packages/core/src/tools/exitPlanMode.ts` | Plan-to-execute tool | 2026-08-26 | git `1174d6fac2ef401b0672b55c0d28cb01f82098ed`; byte-identical `0.21.15` through `0.22.1` |
| `packages/core/src/config/config.ts` tool registration | headless skips `exit_plan_mode` | 2026-08-26 | `supportsUserInteraction = resolveInteractionMode(this) !== 'headless'` |
| `packages/core/src/core/prompts.ts` | `resolveInteractionMode`; Plan reminder | 2026-08-26 | `0.22.1` git `9627b6e87731296df36d242091ef2d61a3f1334a` |
| `packages/cli/src/nonInteractive/control/controllers/permissionController.ts` | `can_use_tool` / `set_permission_mode` | 2026-08-26 | `0.22.1` git `3652b634c794ebf34f642cf572ba24aafd064fe6` |
| `packages/cli/src/ui/commands/planCommand.ts` | `/plan` interactive only | 2026-08-26 | git `8839cd8097a97b5bc1b4da14f3eca40f72c85da6` |
| `packages/cli/src/ui/commands/approvalModeCommand.ts` | `/approval-mode` interactive only | 2026-08-26 | git `e5aa44d1043eed96c7f9bffc2594d6f661dd2dfe` |
| fixtures `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-v0.19.11/success.jsonl` | `session_start.permission_mode` | existing | applied wire field; current decoder requires `"default"` |

Source paths are read at the npm gitHeads already frozen by Research 173 and
216. HTML docs are leads. Every finding below is source-level for those
exact points.

## Parser

Canonical spelling is `--approval-mode plan` as two argv tokens, matching the
current `--approval-mode default` placement.

`ApprovalMode.PLAN = 'plan'`. Shared values are
`plan|default|auto-edit|auto|yolo`. `0.21.15` and `0.22.0` parse with an
explicit switch and yargs `choices` of that list. `0.22.1` uses
`APPROVAL_MODES.find` after `trim().toLowerCase()`, with aliases
`auto_edit|autoedit` only for Auto-Edit. `plan` has no alias.

Invalid, empty, and unknown values throw
`Invalid approval mode: ${value}. Valid values are: plan, default, auto-edit, auto, yolo`.
`--yolo` together with `--approval-mode` is a parse error; the documented
replacement is `--approval-mode=yolo`, which this lane does not emit.

Omission is not implicit Plan. With `--safe-mode` and no `--approval-mode`,
construction selects `ApprovalMode.DEFAULT`. Swallowtail currently emits
explicit `default` and must keep those exact bytes when Plan is omitted.

## Precedence And Application

Construction order:

1. `argv.approvalMode` if present
2. else `argv.yolo` → YOLO
3. else settings `tools.approvalMode` when not bare/safe
4. else DEFAULT under `--safe-mode` / `--bare`
5. else AUTO

CLI `--approval-mode plan` therefore wins settings and is not shadowed by
`--safe-mode`. Official headless docs state the same: safe-mode disables
settings-sourced approval overrides; `--yolo` and `--approval-mode` still
take effect.

Untrusted folders force non-DEFAULT, non-PLAN modes back to DEFAULT. PLAN
survives that gate.

`--resume` restores conversation messages. `Config.approvalMode` is taken from
this child's argv, not from the transcript. Every structured run, reasoning
child, first turn, later `--resume` turn, and fresh replacement must re-emit
the same `--approval-mode` value or fail before spawn.

## Plan Behavior

Exact Plan is provider behavior, not isolation or permission.

Scheduler L5 `isPlanModeBlocked` denies any tool whose confirmation type is
not `info`, except `exit_plan_mode`, `ask_user_question`, and
`enter_plan_mode`. That is the applied Plan posture: non-read-only tools error
with `Tool blocked by plan mode` and `EXECUTION_DENIED`. Shell has a separate
read-only classifier; mutating or unknown shell is blocked or requires a
one-off confirmation that does not exit Plan.

`getPlanModeSystemReminder()` injects a system reminder that Plan is active
and state-modifying tools must not run. Prompt text is not the proof; the
scheduler block is.

`--exclude-tools` still merges into deny under `--safe-mode`. `--core-tools`
does not: safe-mode prints `⚠ Safe mode: --core-tools flag is ignored` and
clears the whitelist. Swallowtail's write/process denylist therefore remains
the selected tool filter. Plan does not replace it and does not restore
`--core-tools`.

`--safe-mode` continues to drop ambient MCP, hooks, skills, extensions,
settings allow/deny, and settings-sourced approval. Plan does not reopen those
surfaces.

## Plan-To-Execute

`exit_plan_mode` can set AUTO_EDIT, DEFAULT, or restore `prePlanMode` after
confirmation. That tool is the widening seam.

Registration:

```ts
const supportsUserInteraction = resolveInteractionMode(this) !== 'headless';
if (!this.sdkMode && (supportsUserInteraction || options?.forSubAgent)) {
  await registerLazy(ToolNames.EXIT_PLAN_MODE, ...);
}
```

`resolveInteractionMode` returns `headless` for `--input-format text`
non-interactive children, `acp` for `--input-format stream-json`.

Text-input structured runs and text-input turns therefore do not register
`exit_plan_mode`. A model call is an unknown-tool error, not a mode change.

Reasoning-control children use stream-json input, so `exit_plan_mode` is
registered. `requiresUserInteraction()` is true in PLAN. Without a host
`can_use_tool` allow, `PermissionController` cancels with "The host could not
present plan-exit approval" and execute returns "Remaining in plan mode."
Swallowtail's control client sends only `initialize` and `set_effort`. It
does not answer `can_use_tool` or send `set_permission_mode`. Timeout or
ignored host request cancels; it does not proceed.

`/plan` and `/approval-mode` declare `supportedModes: ['interactive']`. They
are not this route.

`set_permission_mode` is a host control request. It is a later consumer
operation, not child self-widening. This lane must not send it.

## Output And Observation

Frozen `session_start` records include `permission_mode`. The current decoder
requires `"default"`. That field is applied argv mode on the selected
stream-json wire, not an invented observation.

Requested / planned / dispatched states are Swallowtail-owned. Parser-accepted
and applied states are source-visible. When Plan is selected, the decoder must
require `permission_mode` `"plan"`; omission must keep `"default"`. No other
stream event is treated as effective-mode proof.

## Omission, Lifecycle, And Binding Seam

Omission retains exact current `--approval-mode default` bytes on every child
shape, including reasoning-control and `--resume`.

Selected Plan replaces only that value with canonical `plan`. It does not add
`--yolo`, change `--safe-mode`, change exclude-tools, mutate ambient settings,
or alter model, reasoning, budgets, session, terminal, cancellation, or
cleanup.

Production `QwenRunProfileInput` / `QwenSessionProfileInput` currently have no
harness-mode field. Session open currently rejects any
`request.options().harness_mode()`. Run validation currently ignores
harness-mode. The smallest binding is optional portable `HarnessMode::Plan` on
those inputs, advertised as `HarnessModeSelection(Plan)` only on exact
`0.21.15`, `0.22.0`, and `0.22.1`, copied onto immutable plan/evidence/policy,
and emitted as `--approval-mode plan`. Unsupported versions and non-Plan
values reject before process work.

No behavior revision is required:
`qwen-code.headless.v0.21.15-reasoning-control` stays the selected mapped
revision. Reasoning remains exact `0.21.15`. Budgets remain exact `0.21.15`.
Plan composes with those rows only where they already admit; `0.22.0..=0.22.1`
Plan does not extend reasoning or budgets.

## Route And Value Dispositions

| Route | Value | CLI parse | Applied by the exact route | Disposition | Reason |
| --- | --- | --- | --- | --- | --- |
| `qwen.headless` `0.21.15`, `0.22.0`, `0.22.1` | portable `HarnessMode::Plan` via `--approval-mode plan` | accepted | yes: Config.approvalMode PLAN, scheduler block, Plan reminder; text-input children omit `exit_plan_mode`; stream-json children cannot complete plan-exit without host approval | deliver now | complete fixed-argument Plan posture on every selected child |
| `qwen.headless` those points | omitted | n/a | existing `--approval-mode default` | retain existing behavior | not implicit Plan |
| `qwen.headless` those points | `default\|auto-edit\|auto\|yolo` as public values | accepted | provider modes | withheld | not portable `HarnessMode::Plan` |
| `qwen.headless` `0.19.11..=0.20.1`, `0.21.0..=0.21.14` | Plan | n/a for this lane | not traced as a named point | not applicable | lane froze only `0.21.15`, `0.22.0`, `0.22.1` |
| later `UnverifiedNewer` | all | n/a | n/a | evidence-gated | no inheritance; currentness stays standing |

## Contract 034 Decision

`HarnessMode::Plan` needs one exact portable posture that the qualified route
dispatches as a fixed process argument, keeps for each bounded child, and does
not confuse with isolation or permission.

Exact `0.21.15`, `0.22.0`, and `0.22.1` headless `--approval-mode plan` does
that. CLI selection wins ambient settings and safe-mode defaults, reaches
`Config.approvalMode`, and is enforced by `isPlanModeBlocked`. Text-input
children cannot register `exit_plan_mode`. Stream-json reasoning children
cannot complete plan-exit without a host operation this route does not send.
Contract 034 already says Plan is behavioral, not read-only access. Remaining
info-type tools, `--exclude-tools`, and `--safe-mode` are independent. They
are not a Swallowtail containment claim and do not fail equivalence.

Applied `session_start.permission_mode` is observed on the selected wire.
`auto-edit|auto|yolo`, `/plan`, ACP, and `set_permission_mode` stay out.

## Behavior Revision And Compatibility

No behavior revision is needed or proposed. Keep
`qwen-code.headless.v0.21.15-reasoning-control`. Do not extend reasoning or
budgets past exact `0.21.15`. Contract 029 currentness stays in its standing
lane; this record qualifies no newer point.

## Promotion

Research 222 promotes one deliver-now row covering three exact package points:

| Segment | Portable value | Dispatch | Observation | Lifecycle |
| --- | --- | --- | --- | --- |
| exact `qwen.headless` `0.21.15`, `0.22.0`, `0.22.1` | `HarnessMode::Plan` | canonical `--approval-mode plan` in place of `default` | `session_start.permission_mode` `"plan"` | every child reapplies argv; no completed plan-exit on this route |

Cards 208-209 may bind that row through prepared input, capability, immutable
plan/evidence, driver validation, and argv. Omission must keep `--approval-mode
default`. Plan grants no permission, tool, filesystem, network, sandbox,
shell, process, descendant, model, or account authority.
