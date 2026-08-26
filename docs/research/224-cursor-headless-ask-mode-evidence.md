# 224 Cursor Headless Ask-Mode Evidence

Status: promoted
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Roadmap: [g04.077 Cursor Headless Ask Mode](../roadmaps/g04/077-cursor-headless-ask-mode.md)
Card: [213 Cursor Headless Ask-Mode Evidence](../roadmaps/g04/batch-cards/213-cursor-headless-ask-mode-evidence.md)

## Question

Can any exact qualified `cursor-agent.headless` build bind one closed
adapter-local Ask selection for `ResourceAccess::Read` through canonical
`--mode ask`, with exact precedence and behavioral truth, no ambient widening,
no writable authority, and no provider work?

## Required Outcome

Promote one exact deliver-now table or an explicit empty set. Separate
requested, prepared, dispatched, parser-accepted, applied, effective, and
observed state.

## Method And Boundary

Evidence was frozen on 2026-08-26 from exact darwin-arm64 artifacts for the
four qualified calendar/build pairs, isolated local `--version`/`--help`
parser cases, webpack modules extracted from those bundles, the host
`cli-config.json` read without mutation, and current official public
documentation as corroboration only.

No Cursor install, host-binary replacement, login, account inspection,
authenticated catalogue, provider prompt, tool execution, paid work, ambient
`cli-config.json` mutation, or live model run.

Selected operation remains `cursor-agent.headless`, driver
`swallowtail.cursor-agent.headless`, axis `cursor-agent.release-date`, exact
pairs `2026.07.01-41b2de7`, `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, and
`2026.08.11-e8db854`. Current argv is `--print --output-format stream-json
--model <exact> --trust` plus `--mode plan` for `Read`. Isolation is
`AmbientHost`. Configuration is `Ambient`. Prompt content stays on stdin.

Official archives were downloaded into a `mktemp -d` directory and extracted
there. Host `2026.08.04-aaa8809` was observed in place and not rewritten.
Parser cases used an isolated `HOME` and `CURSOR_CONFIG_DIR` under that
directory; the provider config written during those cases landed in the
isolated directory and `~/.cursor/cli-config.json` was not modified.

Linux and Windows artifacts were not retrieved. The adapter, fixtures, guide,
and matrices were inspected and not changed. No production claim, public API,
shared contract, or Contract 029 window movement follows.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| [CLI overview markdown](https://cursor.com/docs/cli/overview.md) | Agent/Plan/Ask table; Ask is "Read-only exploration without making changes"; `--mode=ask` | 2026-08-26 | SHA-256 `c56ae1d766654ca892b2cfb513a2f7f5ef0082f107d225fb4319fa9f20109e85` (3341 bytes) |
| [CLI parameters markdown](https://cursor.com/docs/cli/reference/parameters.md) | `--mode <mode>`: `plan` or `ask`; agent is the default when no mode is specified | 2026-08-26 | SHA-256 `963d15eb1edf8407dfcda3d3334e23f89c6aa3593a7827b9fe53dc1c83112989` (13910 bytes) |
| [CLI configuration markdown](https://cursor.com/docs/cli/reference/configuration.md) | persisted config surface; no agent-mode key | 2026-08-26 | SHA-256 `d6921fd7a44cf73c0e09063d42aeebd01695fdc1848505df515b573dbf8d579d` (6643 bytes) |
| official `2026.07.01-41b2de7` darwin-arm64 archive | exact artifact | 2026-08-26 | SHA-256 `48cbf291c2e28d81b79fa0dcbf18ab50bf4ac7772d0e9ab0948ecbd5f5a29158` |
| official `2026.07.23-e383d2b` darwin-arm64 archive | exact artifact | 2026-08-26 | SHA-256 `f2eb25851f2079dcdf0558a816e06c402d187abfca93255d35167020439ebbf2` |
| official `2026.08.11-e8db854` darwin-arm64 archive | exact artifact | 2026-08-26 | SHA-256 `46044d6d7bcbd7b49a0cf1cd01aa4ca79aaa2ea5f2c7a32965fc0ebe29841790` |
| host `2026.08.04-aaa8809` | exact installed specimen | 2026-08-26 | runtime `index.js` SHA-256 `65cb83494b6134b1b1c78139f24ac77d12943d3ba2e540d24e45eef17ee10bef` |
| host `~/.cursor/cli-config.json` | ambient config read; `sandbox.mode` `disabled`; no agent-mode key | 2026-08-26 | read only; mtime unchanged |

Archive hashes match Research 183 and 223. Host `index.js` matches Research
135. The overview and configuration markdown digests match Research 223
unchanged. Current docs do not qualify a named build.

## Artifact Identity

| Version | Source | `index.js` SHA-256 | wrapper SHA-256 | `--help` SHA-256 |
| --- | --- | --- | --- | --- |
| `2026.07.01-41b2de7` | official darwin-arm64 archive | `b974679f9f421360ac6f063ba54d5032b75230a58f28fc5c3017de5ff481f230` | `eed61c5224668c9236334c4c68936a16aecc37374b592f59e31eb50433817831` | `28a810a84859c50339dddd12e38b05986ec5cab1b4e36590b391fa366abd438f` |
| `2026.07.23-e383d2b` | official darwin-arm64 archive | `39a3fbb76b3382d2ffa82f6a158f292ae4fe0ba06162795dcbbacde325ca9853` | `eed61c5224668c9236334c4c68936a16aecc37374b592f59e31eb50433817831` | `a4d110d02ffc7938a1c7d3d75e743388c9e09aed713d8da318874048520d31e5` |
| `2026.08.04-aaa8809` | installed host | `65cb83494b6134b1b1c78139f24ac77d12943d3ba2e540d24e45eef17ee10bef` | `eed61c5224668c9236334c4c68936a16aecc37374b592f59e31eb50433817831` | `f06c0cfd979a6b076db5fb30408735eb85f8b32bdfeae8085cce9ab59fb6e502` |
| `2026.08.11-e8db854` | official darwin-arm64 archive | `6aceb24b7c7ecddb1993946ebb18a7dd4d025842e6efda955eb0c13255b1e5f0` | `eed61c5224668c9236334c4c68936a16aecc37374b592f59e31eb50433817831` | `f06c0cfd979a6b076db5fb30408735eb85f8b32bdfeae8085cce9ab59fb6e502` |

Every identity reproduces Research 077, 135, 183, and 223. Isolated
`--version` returns the exact pair on all four. The `2026.08.04-aaa8809` and
`2026.08.11-e8db854` help surfaces remain byte-identical.

## Parser And Placement

All four exact `--help` surfaces publish the same two options:

```text
--mode <mode>                Start in the given execution mode. plan:
                             read-only/planning (analyze, propose plans, no
                             edits). ask: Q&A style for explanations and
                             questions (read-only). (choices: "plan", "ask")
--plan                       Start in plan mode (shorthand for --mode=plan).
                             (default: false)
```

`./src/cli.ts` registers the option with commander
`.choices(["plan","ask"])` and no default, then `.option("--plan", "...", !1)`.
There is no `--ask` flag, no `--mode` alias, and no env annotation on this
option.

Isolated parser cases, identical exit status and diagnostic on every qualified
binary:

| Case | Exit | Result |
| --- | --- | --- |
| `--help` | 0 | both options listed; `--mode` has no default |
| `--mode ask --help` | 0 | parses |
| `--mode plan --help` | 0 | parses |
| `--mode=ask --help` | 0 | parses |
| `--mode=plan --help` | 0 | parses |
| `--mode agent --help` | 1 | `argument 'agent' is invalid. Allowed choices are plan, ask.` |
| `--mode ASK --help` | 1 | case-sensitive; `ASK` invalid |
| `--mode Ask --help` | 1 | case-sensitive; `Ask` invalid |
| `--mode '' --help` | 1 | `argument '' is invalid` |
| `--mode --help` | 1 | `--help` consumed as the value and rejected |
| `--mode ask,plan --help` | 1 | no list form |
| `--mode ask --mode plan --help` | 0 | repeats parse |
| `--mode plan --mode ask --help` | 0 | repeats parse |
| `--plan --mode ask --help` | 0 | parses |
| `--mode ask --plan --help` | 0 | parses |
| `--print --output-format stream-json --model gpt-5 --trust --mode ask --help` | 0 | parses in the exact production position |
| `--mode ask --print --help` | 0 | parses before print |
| `agent --mode ask --help` | 0 | parses on the explicit subcommand |

`agent`, the default Cursor posture, is not a `--mode` value. Only `plan` and
`ask` reach the selected headless path. Repeats parse; last-versus-first
application was not observed without a run.

## Precedence

Exact `./src/commands/chat.ts` on all four builds:

```text
o.plan || "plan" === o.mode ? mode = "plan"
                            : "ask" === o.mode && (mode = "ask")
```

`--plan` wins over `--mode ask`. Swallowtail never sends `--plan`, so this is
recorded, not relied on.

Persisted configuration holds no agent-mode key. The exact `cursor-config`
default object carries `display.mode`, `approvalMode`, and `sandbox.mode`
only; the host `cli-config.json` matches that shape. No environment variable
selects the mode. Team and feature-gate overlays reach approval, sandbox, and
Run Everything, not `--mode`.

Without `--resume`, `--continue`, or `--new-session-id`, session start is
`{kind:"new"}`. A fresh conversation carries no prior mode metadata, so
omission resolves to Agent and `--mode plan` resolves to Plan exactly as the
current route claims.

`--mode` therefore has no ambient competitor for *selection*. That is the one
dimension where Ask is stronger than the Research 223 sandbox family.

## Applied Behavior

Exact `./src/run-agent.tsx` on all four builds:

```text
const m = o.mode
if (m && ["plan","ask"].includes(m)) {
  agentStore.setMetadata("mode", m === "ask" ? "search" : m)
}
```

Ask is stored as `"search"`. Exact `./src/utils/interaction-utils.ts` maps
metadata to the wire enum:

```text
"plan"   -> AgentMode.PLAN
"search" -> AgentMode.ASK
"debug"  -> AgentMode.DEBUG
default  -> AgentMode.AGENT
```

Exact `./src/headless.ts` attaches that value to the outbound message:

```text
new UserMessage({ text, selectedContext, messageId, mode: cT(getMetadata("mode")) })
```

So `--mode ask` deterministically becomes `AgentMode.ASK` on the request sent
to the Cursor backend. That is applied local state, not observed behavior.

Mode is immutable for a print run. Every other `setMetadata("mode", ...)`
writer in the bundles is an interactive surface (`ui.tsx`, `prompt-bar.tsx`,
`use-slash-commands.ts`) or ACP (`acp/agent-session.ts`), none of which runs on
this route. A model-initiated switch is refused: headless answers
`switchModeRequestQuery` with `switchModeRejected(id, "Switch mode requires
approval")`, and `askQuestionInteractionQuery` with `"Questions skipped in
headless mode"`.

## Read-Only Boundary

`getIsAskMode` has exactly one consumer in the exact bundles. Its wiring is
`getIsAskMode: () => "search" === agentStore.getMetadata("mode")`, and
`./src/shared/resources.ts` uses it only to pick a shell-exec sandbox policy
type:

```text
policy = approvalMode === "unrestricted" ? {type:"insecure_none"}
       : sandboxAvailable && !teamSandboxingDisabled
         ? (isAskMode ? {type:"workspace_readonly", ...}
                      : {type:"workspace_readwrite", ...})
         : {type:"insecure_none"}
```

Exact `./src/shared/autorun-mode.ts`:

```text
sandboxAvailable = sandboxFeatureGateEnabled
                && isSandboxSupported()
                && "enabled" === (sandboxOverride ?? config.sandbox.mode
                                   ?? (cliSandboxDefaultEnabled ? "enabled" : "disabled"))
```

Swallowtail sends no `--sandbox`. Config default and host `cli-config.json`
both hold `sandbox.mode: "disabled"`, and `cliSandboxDefaultEnabled` is false
when backend services are skipped. `sandboxAvailable` is therefore false and
the policy is `insecure_none` — the same value Ask, Plan, and Agent all
produce on this route today.

Nothing else in the exact bundles gates on Ask. `AgentMode.ASK` appears only in
the mapping module and the subagent host adapter. There is no read-only tool
registry, no edit/write tool exclusion, and no local refusal keyed to mode.
Tool exclusion is the separate `--exclude-tools` mechanism validated against
the `ToolCall` proto field list.

Three consequences follow, and all three bound the claim rather than the
dispatch:

- On the exact production argv, `--mode ask` has no local read-only effect.
  Read-only is backend behavior on an unobservable server.
- Where Ask does have a local effect, it is ambient. Turning on
  `sandbox.mode` in `~/.cursor/cli-config.json`, a project file, or a team
  overlay changes what Ask means locally, and Swallowtail binds none of that
  before spawn.
- Nothing in the CLI rejects `--mode ask` against a writable working
  resource; the write and edit tools stay registered. The exact `/ask`
  slash-command string ("Toggle ask mode (Q&A, read-only; no edits or command
  execution)") and the current docs row ("Read-only exploration without making
  changes") are labels in the artifact and on a mutable page, not an enforced
  boundary in this source.

Swallowtail therefore may not claim a locally enforced read-only boundary from
Ask, and the binding admits Ask only alongside `ResourceAccess::Read`, where
read-only intent already comes from declared working-resource authority. The
last point is a fact about the native CLI, not about this route: Swallowtail's
preflight rejects Ask with `ReadWrite` before any process work.

## Access, Model Parameters, And Defaults

`--mode` is independent of `--model`. Every Research 183 deliver-now tuple
parses with `--mode ask` in the exact production position on all four builds:
`claude-opus-4-8[context=1m]`, `[effort=high]`, `[fast=false]`,
`[context=1m,effort=high,fast=false]`, `claude-opus-5[context=300k]`,
`claude-opus-5[effort=high]`, and `composer-2.5[fast=false]`. Rendering stays
one `--model` argument; the parameter support set is unchanged.

Current defaults are unaffected by this research. `ResourceAccess::Read`
dispatches exactly one `--mode plan`; `ResourceAccess::ReadWrite` omits
`--mode`; both keep `--trust`, the explicit model, `Ambient` configuration,
`AmbientHost` isolation, durable retention, and the one-child lifecycle.

## Observation

Qualified stream JSON reports no mode. The `system/init` event emits a constant
`permissionMode: "default"` and carries `apiKeySource`, `cwd`, `session_id`,
and `model` only. Research 077's qualified stream — `system/init`, assistant,
thinking, tool-call, result, optional usage — contains no requested, selected,
applied, or effective mode field. The stored `agentMode` metadata reaches
telemetry, not stdout.

Absence of edits in a transcript is not mode evidence.

## Production Audit At Card 213

Before card 214, `CursorHeadlessRunProfileInput` had no mode member.
`headless_command::arguments(model, access)` was the entire mode surface: it
appended `--mode plan` when `access == ResourceAccess::Read` and nothing
otherwise. `CursorPreparedHeadlessRun::low_level_driver` returned
`CursorHeadlessDriver::new(environment)`, and `headless_validation::validate`
derived access from the plan's `WorkingResource` capability constraint. There
was no adapter-local carrier between prepared state and the driver.

Card 214 added exactly that carrier: adapter-private driver state plus a
fail-closed check against the prepared plan's access and exact release. No
portable plan change was needed. See **Production Binding** below.

Existing tests already asserted `--mode plan` for `Read`, no `--mode` for
`ReadWrite`, and one `--model` argument; card 215 extends them rather than
replacing them. The guide claimed dispatch only — "Read authority selects
Cursor plan mode" — and made no read-only enforcement claim. Nothing in the
prior surface overclaimed, and nothing needed correction.

## Claim Strength

Ask is admitted at qualified dispatch and application. Effective and observed
mode are explicitly withheld.

| State | Ask on the four exact builds |
| --- | --- |
| requested | `CursorHeadlessReadMode::Ask` on a `Read` profile input |
| prepared | resolved and frozen at preparation; rejects read-write authority and any release that is not exactly qualified |
| dispatched | exactly one canonical `--mode ask` token in the existing argv position |
| parser-accepted | yes; exact, case-sensitive, closed to `plan` and `ask` |
| applied | yes; store metadata `"search"` and `AgentMode.ASK` on the outbound `UserMessage`, immutable for a print run |
| effective | withheld; backend behavior, not in these artifacts |
| observed | withheld; stream JSON emits no mode field |

## Classification

| Build | Access | Value | Parser | Selection immutable | Applied state | Effective / observed | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `2026.07.01-41b2de7` | `Read` | `--mode ask` | yes | yes | metadata `"search"`; `AgentMode.ASK` | withheld | **yes**, dispatch and application only |
| `2026.07.23-e383d2b` | `Read` | `--mode ask` | yes | yes | metadata `"search"`; `AgentMode.ASK` | withheld | **yes**, dispatch and application only |
| `2026.08.04-aaa8809` | `Read` | `--mode ask` | yes | yes | metadata `"search"`; `AgentMode.ASK` | withheld | **yes**, dispatch and application only |
| `2026.08.11-e8db854` | `Read` | `--mode ask` | yes | yes | metadata `"search"`; `AgentMode.ASK` | withheld | **yes**, dispatch and application only |
| any qualified | `ReadWrite` | `--mode ask` | yes | yes | n/a | n/a | no; Swallowtail rejects before process work |
| any qualified | any | `--plan` with `--mode ask` | yes | `--plan` wins | n/a | n/a | no; Swallowtail never sends `--plan` |
| any qualified | any | `--mode agent` / `ASK` / `''` / list | no | n/a | n/a | n/a | no |
| any qualified | `Read` | `--mode plan` | yes | yes | metadata `"plan"`; `AgentMode.PLAN` | withheld | current production; unchanged |
| any qualified | `ReadWrite` | omitted | n/a | n/a | Agent default | n/a | current production; unchanged |
| calendar gaps / `UnverifiedNewer` | any | `--mode ask` | no inheritance | n/a | n/a | n/a | no |

Four deliver-now rows. Every one is `ResourceAccess::Read` on an exactly
qualified build, at dispatch and application strength only.

## Why This Tier And Not More

The admitted claim stops where the artifacts stop.

`getIsAskMode` has one consumer, and it selects a shell-exec sandbox policy
type — `workspace_readonly` instead of `workspace_readwrite` — gated on
`sandboxAvailable`. This route sends no `--sandbox` and both the default and
host configs hold `sandbox.mode: "disabled"`, so that branch is inert here;
where it is not inert, ambient `cli-config.json`, project state, team
overlays, and feature gates decide it. No tool registry, approval path, or
write refusal keys on Ask.

Ask therefore must not be documented, tested, or sold as a locally enforced
read-only boundary. Read-only intent on this route continues to come from the
`ResourceAccess::Read` working-resource authority the consumer declares, which
is exactly why the binding admits Ask only alongside it.

Two facts are separate and both hold. The native CLI does not reject
`--mode ask` against a writable working resource; Swallowtail does, before any
process work. That is a Swallowtail preflight guarantee about what this route
dispatches, not a claim about what Cursor would tolerate.

The inert-or-ambient sandbox path bounds the claim. It does not erase the
exact dispatch and application behavior the four builds prove, which is the
same tier at which the route already binds `--mode plan` and at which
Research 183 binds Cursor-local model parameters without effective-value
confirmation.

## Production Binding

`CursorHeadlessReadMode` is the closed adapter-local type. It carries `Plan`
and `Ask` and no raw string, and it is not portable `HarnessMode`.

- `CursorHeadlessRunProfileInput::new` is unchanged: `Read` resolves to
  `--mode plan`, `ReadWrite` resolves to no mode.
- `with_read_mode` is the only selection path. It rejects any selection on
  `ReadWrite` with
  `swallowtail.cursor.headless.read_mode_access_rejected`.
- `prepare_run` rejects `Ask` on a release that is not exactly qualified with
  `swallowtail.cursor.headless.ask_mode_unqualified`. `UnverifiedNewer` does
  not inherit Ask; the existing behavior revision is unchanged because all
  four qualified builds share identical Ask semantics.
- The resolved mode is stored on `CursorPreparedHeadlessRun`, readable through
  `read_mode`, and passed to `CursorHeadlessDriver::with_read_mode`. It cannot
  drift after preparation.
- The low-level driver re-validates access and exact release before process
  work and never falls back to Plan or Agent after a rejected selection.
- `HarnessIsolation::AmbientHost`, `HarnessConfigurationPosture::Ambient`,
  `ProviderRetentionPolicy::DurableAllowed`, working-resource authority,
  `--trust`, model rendering, activity, usage, cancellation, deadline,
  terminal, and joined cleanup are untouched.

## Promotion

Research 224 promotes four deliver-now rows: `--mode ask` with
`ResourceAccess::Read` on exact `2026.07.01-41b2de7`, `2026.07.23-e383d2b`,
`2026.08.04-aaa8809`, and `2026.08.11-e8db854`, at qualified dispatch and
application only.

Effective and observed mode remain withheld. A later lane may raise the claim
only when an exact qualified build proves a local Ask boundary independent of
ambient sandbox, approval, team, and feature-gate state, or when a qualified
observation channel reports applied or effective mode, without login, account
inspection, provider prompt, tool execution, paid work, or ambient config
mutation.

Portable `HarnessMode::Ask`, raw mode strings, `--plan`, Agent selection,
`--force`, `--yolo`, `--auto-review`, sandboxing, approvals, tool policy,
Cursor ACP and catalogue work, and sibling Cursor routes remain out.
