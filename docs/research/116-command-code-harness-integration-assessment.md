# 116 Command Code Harness Integration Assessment

Status: promoted
Owner: Tom
Date: 2026-08-09

## Question

Does Command Code expose a stable enough machine boundary for a Swallowtail
adapter family, and which surfaces should be first-class versus deferred?

## Method

Sources:

- public docs: site, CLI reference, headless, permissions, Provider API, mods /
  AgentEvent catalog, custom agents
- installed global npm package `command-code@1.15.1` on the qualification host
- read-only local probes: `--version`, `--help`, `--list-models`,
  `status --json`, headless `-p --output-format json`
- isolated temporary workspaces; `--no-session`, `--no-skills`,
  `--skip-onboarding`, `--no-auto-update`, `--trust`
- two authenticated plan-mode completions after subscription credits were
  available: no-tool reply, and one `read_file` tool turn
- earlier pre-credit probes for exit `10` failure shape and preflight exits

Account username, session ids, trace ids, prompts, tool inputs/results, and
raw thinking text remain private capture data and are not copied here.

## Installed Artifact

| Fact | Value |
| --- | --- |
| Version | `1.15.1` |
| Launchers | `command-code`, `cmd`, `cmdc` → same `dist/index.mjs` |
| Preferred bin | `command-code` |
| Package path | `~/.local/lib/node_modules/command-code` |
| License | `UNLICENSED` |
| Engines | Node `>=22` (host Node `v22.23.2`) |
| Entrypoint SHA-256 | `157feefa0140e78f060ef2c1f9c50d10de702196ea229dc73db6bbcc39a0bcbb` (`dist/index.mjs`, 1836 B launcher) |
| Payload SHA-256 | `c1b62bc030128ee41b22f950c78be5132bdf8e306f8b4ff889ef5a8935471c5c` (`dist/cli.mjs`, 2,359,072 B) |
| package.json SHA-256 | `3fce9812137c0523e6c6a1331a0d0d36cc842a1d8654bf5352314a5cb0608377` |

`status --json` keys: `authenticated`, `version`, `model`, `context_window`,
`user`. Catalogue help lists 52 models. Probe model:
`deepseek/deepseek-v4-flash`.

No ACP or app-server subcommand. Bare `acp` / `app-server` fall into
interactive mode and require a TTY.

## Surfaces

| Surface | Machine boundary | Swallowtail role |
| --- | --- | --- |
| Headless `-p` + `--output-format json` | Owned process; NDJSON events + final result | Primary installed-harness route |
| Interactive CLI | Human TUI / slash commands | Not a Swallowtail driver |
| Provider API | Hosted OpenAI/Anthropic-compatible HTTP | Separate direct/hosted route candidate |
| Mods / hooks / taste / skills | Product extension and learning | Out of portable vocabulary |

## Selected Invocation

```text
command-code -p --output-format json \
  --permission-mode plan \
  --skip-onboarding --no-session --no-auto-update --trust --no-skills \
  --max-turns <N> \
  -m <exact-model>
```

Proven:

- stdin prompt (preferred) and `-p` argv both work
- `--plan` alias works
- `--permission-mode dont-ask` accepted though `--help` omits it
- `--verbose` prints `session: <id>` on stderr
- `--no-auto-update` available; bind it on the qualified route
- effort is model-specific (`low` rejected for the probe model; supported:
  `high`, `max`)
- unknown model and unsupported effort fail preflight: exit `1`, empty stdout

## Stream Evidence

### Success, no tools

Exit `0`. 51 events + 1 result. `finalText` was the requested one-word reply.
`stopReason=end_turn`. Non-zero usage on the result line.

Event types observed:

`run_start`, `turn_start`, `message_start`, `model_request_start`,
`model_trace`, `thinking_start`, `thinking_delta`, `thinking_end`,
`text_delta`, `message_update`, `model_request_end`, `message_end`,
`turn_end`, `run_end`.

### Success, read tool

Exit `0`. Two turns. Tool lifecycle observed:

| Event | Correlation / payload notes |
| --- | --- |
| `tool_queued` | `toolCallId`, `toolName=read_file`, `input` object |
| `tool_running` | same ids; `description` may be null |
| `tool_completed` | same ids; `result` list |

First `model_request_end.stopReason` was `tool_calls`; second was `stop`.
`turn_end.hadToolCalls` was `true` then `false`. `message_update` /
`message_end` content blocks used types `thinking`, `tool_use`, and `text`.

### Credit failure (pre-subscription)

Exit `10`. Short stream through `run_error` + `run_end` +
`result.subtype=error`. Usage zeros. Documents the portable credit-failure
path.

### Shared result / usage shape

Success result keys: `type`, `subtype`, `sessionId`, `stopReason`, `usage`,
`durationMs`, `finalText`.

Usage keys everywhere observed: `inputTokens`, `outputTokens`,
`cacheReadTokens`, `cacheWriteTokens`.

`run_end.result` still carries private `nextState` (messages, cwd, git). Do
not ingest into diagnostics, activity, or fixtures. Project bounded labels and
opaque ids only.

## Route Decision

Command Code qualifies for a dedicated installed-harness package and route:

- package: `swallowtail-adapter-command-code`
- family: `command-code`
- route: `command-code.headless`
- driver: `swallowtail.command-code.headless`
- transport: owned process; NDJSON AgentEvent + result
- version axis: `command-code.npm`
- first qualified point: exact `1.15.1` with payload digest above

Do not start with Provider API, mods/taste, TUI scraping, or an ACP driver.

## First Production Subset

One bounded structured run:

- host-approved `command-code` at exact `1.15.1`
- `-p --output-format json`
- explicit model; effort only when supported for that model
- `--permission-mode plan` for read-only; `dont-ask` when writes remain denied
  by policy; `--yolo` only under explicit write/shell authority
- `--skip-onboarding --no-auto-update --trust`
- `--no-session` for structured runs that must not retain transcripts
- stdin prompt preferred
- project: run/turn/message lifecycle, thinking deltas, text deltas, tool
  queue/run/complete, usage, terminal result
- ignore unknown event types
- never project `run_end.result.nextState` or tool input/result bodies into
  stable diagnostics
- map exit `10` and `result.subtype=error` credit failures portably

## Deferred

- resume / continue as interactive or retained-session roles
- session catalogue and import / export
- subagent topology control (observation only if later stream proves it)
- MCP / skills / worktree management
- Provider API direct route

## Contract Fit

No new provider-neutral contract required. Contracts 005, 032–033, 039–041,
044–045, and 051 already govern the selected surface. Implementation must bind
activity-affecting options (`--output-format json`, permission mode, model,
session retention) as immutable prepared evidence per Contract 044.

Contract 036 still requires architecture/package review before the new package
enters the workspace release set.

## Recommendation

**Promote Command Code into the installed-harness backlog** as
`command-code.headless` at exact `1.15.1`. Evidence is sufficient for a
bounded structured-run driver with thinking, text, tool activity, usage, and
typed failures.

Next planning move: compile a g03 (or backlog) adapter tranche after the
operator confirms sequencing against the current evidence-gate inventory.
Freeze redacted fixtures from the success captures during that tranche; do not
commit private transcripts into research.
