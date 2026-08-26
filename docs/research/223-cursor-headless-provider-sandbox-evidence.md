# 223 Cursor Headless Provider-Sandbox Evidence

Status: promoted; evidence stop
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Card: g04.076 / 210

## Question

Can any exact qualified `cursor-agent.headless` build/platform/access row bind
`--sandbox enabled` as `HarnessIsolation::ProviderEnforced` with a complete
preflight-bound native boundary and no ambient widening, silent outside-
sandbox execution, fallback, provider work, or authority expansion?

## Method And Boundary

Evidence was frozen on 2026-08-26 from exact darwin-arm64 artifacts for the
four qualified calendar/build pairs, isolated local `--version`/`--help`
parser cases, webpack modules extracted from those bundles, the colocated
`cursorsandbox` helper `--help`/strings, and current official public
documentation as corroboration only.

No Cursor install, host-binary replacement, login, account inspection,
authenticated catalogue, provider prompt, tool execution, paid work, ambient
`cli-config.json`/`sandbox.json` mutation, or live model run.

Selected operation remains `cursor-agent.headless`, driver
`swallowtail.cursor-agent.headless`, axis `cursor-agent.release-date`, exact
pairs `2026.07.01-41b2de7`, `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, and
`2026.08.11-e8db854`. Current argv is `--print --output-format stream-json
--model <exact> --trust` plus `--mode plan` for `Read`. Isolation is
`AmbientHost`. Configuration is `Ambient`. Prompt content stays on stdin.

Official archives were downloaded into a disposable directory and extracted
there. Host `2026.08.04-aaa8809` was observed in place and not rewritten.
Parser cases used an empty isolated `HOME`/`CURSOR_CONFIG_DIR` and restored
the original `HOME` afterwards.

Linux and Windows artifacts were not retrieved. Current Run Modes docs may
describe Landlock/Bubblewrap/WSL2; those pages cannot backport a platform
backend to these darwin-arm64 bundles.

The adapter, fixtures, guide, and matrices were inspected and not changed.
No production claim, public API, shared contract, or Contract 029 window
movement follows.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| [CLI overview markdown](https://cursor.com/docs/cli/overview.md) | `--sandbox <mode>` (`enabled`/`disabled`); settings persist | 2026-08-26 | SHA-256 `c56ae1d766654ca892b2cfb513a2f7f5ef0082f107d225fb4319fa9f20109e85` (3341 bytes) |
| [Run Modes markdown](https://cursor.com/docs/agent/security/run-modes.md) | command-scoped sandbox; approval escape; ambient `sandbox.json`; platform backends | 2026-08-26 | SHA-256 `556b501c270e1f906eb30930e4d7d52782a1019dd872420cb1f305edfe416206` (18010 bytes) |
| [CLI configuration markdown](https://cursor.com/docs/cli/reference/configuration.md) | `sandbox.mode`, `sandbox.networkAccess`; CLI overrides config | 2026-08-26 | SHA-256 `d6921fd7a44cf73c0e09063d42aeebd01695fdc1848505df515b573dbf8d579d` (6643 bytes) |
| [sandbox.json markdown](https://cursor.com/docs/reference/sandbox.md) | extra paths, `insecure_none`, network allow/deny, merge order | 2026-08-26 | SHA-256 `b2cc6bccb34dc9438e303e6b5a1964235699e8197d28150d1b97f38be7207672` (7367 bytes) |
| [Agent sandboxing blog](https://cursor.com/blog/agent-sandboxing) | Seatbelt/Landlock/WSL2 engineering; command sandbox | 2026-08-26 | SHA-256 `0d05d8b48e64a8c00fda9322de06e5daec60eab87da585d57f4195ed86a1fa2a` (196710-byte HTML) |
| official `2026.07.01-41b2de7` darwin-arm64 archive | exact artifact | 2026-08-26 | SHA-256 `48cbf291c2e28d81b79fa0dcbf18ab50bf4ac7772d0e9ab0948ecbd5f5a29158` |
| official `2026.07.23-e383d2b` darwin-arm64 archive | exact artifact | 2026-08-26 | SHA-256 `f2eb25851f2079dcdf0558a816e06c402d187abfca93255d35167020439ebbf2` |
| official `2026.08.11-e8db854` darwin-arm64 archive | exact artifact | 2026-08-26 | SHA-256 `46044d6d7bcbd7b49a0cf1cd01aa4ca79aaa2ea5f2c7a32965fc0ebe29841790` |
| host `2026.08.04-aaa8809` | exact installed specimen | 2026-08-26 | runtime `index.js` SHA-256 `65cb83494b6134b1b1c78139f24ac77d12943d3ba2e540d24e45eef17ee10bef` |

Archive hashes match Research 183. Host `index.js` matches Research 135.
Wrapper `cursor-agent` remains SHA-256
`eed61c5224668c9236334c4c68936a16aecc37374b592f59e31eb50433817831` on all four
builds. Markdown exports are the digestable docs corpus. The blog HTML is
corroboration only. Current docs do not qualify a named build.

## Artifact Identity

| Version | Source | `index.js` SHA-256 | `cursorsandbox` SHA-256 | `--help` SHA-256 |
| --- | --- | --- | --- | --- |
| `2026.07.01-41b2de7` | official darwin-arm64 archive | `b974679f9f421360ac6f063ba54d5032b75230a58f28fc5c3017de5ff481f230` | `d430c34223b7f8d538111205733f6a9937efd693f1ed766e6436ca2042c2115d` | `28a810a84859c50339dddd12e38b05986ec5cab1b4e36590b391fa366abd438f` |
| `2026.07.23-e383d2b` | official darwin-arm64 archive | `39a3fbb76b3382d2ffa82f6a158f292ae4fe0ba06162795dcbbacde325ca9853` | `9826ccf01148f2e8aa4ad74519ab8ad6b39022bbf6ae4b696c0b77038122f374` | `a4d110d02ffc7938a1c7d3d75e743388c9e09aed713d8da318874048520d31e5` |
| `2026.08.04-aaa8809` | installed host | `65cb83494b6134b1b1c78139f24ac77d12943d3ba2e540d24e45eef17ee10bef` | `36f4c7ae9357dba5181ee51f3126b68564558d3aaedc92f98f9a29582fff9f98` | `f06c0cfd979a6b076db5fb30408735eb85f8b32bdfeae8085cce9ab59fb6e502` |
| `2026.08.11-e8db854` | official darwin-arm64 archive | `6aceb24b7c7ecddb1993946ebb18a7dd4d025842e6efda955eb0c13255b1e5f0` | `2e37b098fc7cdd4edff8f5676047ea02081d22c644dfb9c86dd5d5bb907fe15f` | `f06c0cfd979a6b076db5fb30408735eb85f8b32bdfeae8085cce9ab59fb6e502` |

`cursorsandbox --help` on every extracted helper and the host helper:

```text
Sandboxing helper for Everysphere shell-exec
...
--preflight-only   Only perform sandbox preflight (no exec); exits 0 on success, 2 if unsupported
```

Host `/usr/bin/sandbox-exec` exists and is executable. That is a machine fact,
not a Swallowtail preflight binding.

## Parser And Placement

All four exact `--help` surfaces publish the same option:

```text
--sandbox <mode>             Explicitly enable or disable sandbox mode
                             (overrides config) (choices: "enabled",
                             "disabled")
```

Isolated parser cases, identical on every qualified binary:

| Case | Exit | Result |
| --- | --- | --- |
| `--help` | 0 | option listed; no default value |
| `--sandbox enabled --help` | 0 | parses |
| `--sandbox disabled --help` | 0 | parses |
| `--print --output-format stream-json --sandbox enabled --help` | 0 | parses in print position |
| `--sandbox enabled --print --help` | 0 | parses before print |
| `--sandbox --help` | 1 | `argument '--help' is invalid. Allowed choices are enabled, disabled.` |
| `--sandbox foo --help` | 1 | `argument 'foo' is invalid` |
| `--sandbox '' --help` | 1 | `argument '' is invalid` |
| `--sandbox ENABLED --help` | 1 | case-sensitive; `ENABLED` invalid |
| `--sandbox enabled --sandbox disabled --help` | 0 | repeats parse; last-versus-first application not observed |

`./src/cli.ts` registers the option with commander `.choices(["enabled",
"disabled"])` and no default. `./src/run-agent.tsx` copies it as
`sandboxOverride: o.sandbox`. There is no `--sandbox` alias, no boolean flag
on the agent command, and no `enabled|disabled` env annotation on this option.

A hidden `agent sandbox enable|disable|reset|run` command family exists. It
persists `sandbox.mode` or executes a debug command. It is not the headless
argv.

Omission sends no `--sandbox` token. That is the current production argv.

## Mode Resolution

Exact `./src/shared/autorun-mode.ts` resolver `PT`:

```text
sandboxOverride ?? config.sandbox.mode ?? (cliSandboxDefaultEnabled ? "enabled" : "disabled")
```

`cursor-config` defaults `sandbox.mode` to `"disabled"` and
`sandbox.networkAccess` to `"user_config_with_defaults"`. Server
`cliSandboxDefaultEnabled` defaults false when backend services are skipped.

CLI override beats persisted config. Persisted config still exists under
`Ambient` posture: `~/.cursor/cli-config.json`, project files, and team
controls remain live host surfaces. Feature gate
`composer_sandbox_settings_visible` plus `sandbox_force_disable_win32` can
force the effective mode to `"disabled"` even when the CLI override is
`"enabled"`. Team `sandboxingDisabled` is a further overlay.

`--sandbox enabled` is therefore requested/dispatched/parser-accepted. It is
not by itself backend-active, enforced, or effective.

## Platform And Backend

Darwin support in these bundles is local binary presence, not a Seatbelt
preflight:

- `isSandboxSupported` requires `/usr/bin/sandbox-exec` `X_OK`
- then `isSandboxHelperSupported` requires the colocated `cursorsandbox` path
  to exist
- success logs `"darwin platform, binary available, sandbox supported!"`
- `cursorsandbox --preflight-only` (exit 0 vs 2) is not invoked by that check

`./src/index.tsx` configures the helper if found and only debug-traces if
missing. CLI startup does not fail closed on a missing helper.

Print mode then:

```text
if print && enabled && !(featureGate && supported):
  exit 1 "Sandbox mode is enabled but not available on this system."
```

That is a presence gate, not proof that seatbelt/landlock applied to a
command, and not a harness-process boundary.

Linux Landlock/Bubblewrap and Windows WSL2 are named by current docs and by
`cursorsandbox` strings (`--run-linux-inner`, `linux_network_mode`). They are
not bound from these darwin-arm64 artifacts. Calendar gaps and
`UnverifiedNewer` dates do not inherit Darwin presence.

## Filesystem, Network, Subprocess

`cursorsandbox` is a shell-exec helper. `shell-exec` `spawnInSandbox` wraps
non-`insecure_none` policies by spawning that helper with a JSON policy.
Missing policy resolves to `{type:"insecure_none"}`.

Policy types in exact source: `workspace_readwrite`, `workspace_readonly`,
`insecure_none`. User and project `sandbox.json` load through
`loadSandboxPolicyFromFileOrUndefined`. Merge union extra paths, can set
network `default: "allow"`, and can disable the sandbox with
`insecure_none`. Team-admin and hardcoded rules layer on top; local files
can still add `additionalReadwritePaths` and open network.

`allowlistEscalated` is an `insecure_none` field. Commands that cannot run
under the sandbox are a classified escape, not process containment.

File, MCP, fetch, and browser tools are not this helper. Read/Write/Edit
paths are local-exec, not `cursorsandbox`. `.cursorignore` can feed
`ignoreMapping` for sandboxed shell, which is a tool-visibility rule, not
harness isolation. `--mode plan`, `--trust`, and `Read|ReadWrite` stay
independent.

Network defaults are policy-driven. `networkAllowAllPolicy` /
`networkDisabledPolicy` exist. Config `networkAccess` values include
`user_config_only`, `user_config_with_defaults`, and `allow_all`. Swallowtail
does not currently bind any of those facts before spawn.

## Approval, Print Mode, And Escape

Print-mode decision provider:

```text
print ? (headlessAutoApprove ? AlwaysApprove : AlwaysDeny) : AutorunAware
```

Always-deny is `requestApproval() => {approved:false}`. Always-approve is the
`--force`/`--yolo` path after team Run Everything is allowed. Current
Swallowtail argv never sends `--force`, `--yolo`, or `--auto-review`.

Without force, an outside-sandbox shell that needs approval is denied, not
silently executed. Denial is not containment of the `cursor-agent` process or
its non-shell tools. With force, print mode auto-approves, which is an
explicit out-of-scope escape.

Interactive autorun/allowlist/auto-review can still run or prompt for
unsandboxed commands. This route is print-only; that interactive path is not
the selected invocation, but it shows the sandbox is a command classifier on
top of approval, not a process boundary.

## Observation

Research 077's qualified stream is `system/init`, assistant, thinking,
tool-call, result, optional usage. No sandbox-backend, policy-type, or
`CURSOR_SANDBOX` field is a required terminal event. `CURSOR_SANDBOX` is
injected into sandboxed child processes. Swallowtail does not observe child
environment or helper preflight.

Argv, parser acceptance, helper presence, and a successful shell command do
not prove enforcement.

## Production Audit

`CursorHeadlessRunProfileInput` has no isolation member.
`prepare_run` always sets `HarnessIsolation::AmbientHost` on requirements and
policy. `headless_command::arguments` never emits `--sandbox`.
`headless_validation` and the ACP driver reject any isolation other than
`AmbientHost`. Tests assert `--sandbox` is absent for both `Read` and
`ReadWrite`.

The guide already says the prepared path does not request optional
sandboxing. The route matrix says optional sandbox is not selected.
Architecture already records force flags and implicit sandboxing as unselected.

Omission therefore retains exact no-flag argv, `AmbientHost`, ambient
configuration, durable provider state without management, and the current
lifecycle. Binding `ProviderEnforced` would need every Contract 023 native
fact in immutable preflight. Exact evidence does not supply that set.

## Classification

| Build | Platform | Access | Value | Parser | Native harness boundary | Ambient widening frozen | Observation | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `2026.07.01-41b2de7` | darwin-arm64 | `Read` | `--sandbox enabled` | yes | no; shell-exec helper only | no | no | no |
| `2026.07.01-41b2de7` | darwin-arm64 | `ReadWrite` | `--sandbox enabled` | yes | no | no | no | no |
| `2026.07.23-e383d2b` | darwin-arm64 | `Read` / `ReadWrite` | `--sandbox enabled` | yes | no | no | no | no |
| `2026.08.04-aaa8809` | darwin-arm64 | `Read` / `ReadWrite` | `--sandbox enabled` | yes | no | no | no | no |
| `2026.08.11-e8db854` | darwin-arm64 | `Read` / `ReadWrite` | `--sandbox enabled` | yes | no | no | no | no |
| any qualified | Linux / Windows | any | `--sandbox enabled` | help exists on darwin bundles only | unbound | n/a | n/a | no |
| any | any | any | `--sandbox disabled` | yes | out of scope | n/a | n/a | no |
| any | any | any | omitted | n/a | current `AmbientHost` | ambient config unchanged | n/a | not a sandbox row |
| calendar gaps / `UnverifiedNewer` | any | any | any | no inheritance | n/a | n/a | n/a | no |

No row is deliver-now. The empty set is because exact source proves a
command-scoped helper whose boundary still depends on ambient
`sandbox.json`/config/team/feature-gate state, whose Darwin "supported" check
is binary presence, and whose print-mode deny path is approval rather than
process containment. It is not because `--sandbox enabled` is missing from
the parser.

## Promotion

Research 223 promotes an empty deliver-now set.

Cards 211-212 stay blocked. A later lane may reopen this family only when an
exact qualified build/platform/profile can bind every Contract 023 native
fact before spawn: helper/backend activation, filesystem/network/subprocess
policy, no ambient path/network widening, no outside-sandbox execution or
silent approval, and an observable enforcement signal, without login, account
inspection, provider prompt, tool execution, paid work, or ambient config
mutation.

`--sandbox disabled`, raw sandbox strings, network/path policy selection,
host isolation, `--force`/`--yolo`/`--auto-review`, approval exchange, MCP,
plugins, cloud workers, worktree mode, and sibling Cursor routes remain out.
