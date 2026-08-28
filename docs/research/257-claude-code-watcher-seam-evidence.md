# 257 Claude Code Watcher Seam Evidence

Status: promoted
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Card: g05.003 / 007

## Question

Can exact qualified Claude Code headless carry one operation-private watcher
skill and tool channel, then block early completion and return active-watcher
state to the same `-p` model turn?

## Decision

Yes, as a closed candidate mechanism under an explicit composition delta. Not
as today's production argv.

Research 257 admits a complete route-mechanism table for
`claude-code.headless` in `2.1.220..=2.1.241`. The seam is:

- `--bare` to drop ambient hooks, skills, and MCP discovery
- `--mcp-config <json-or-file>` plus `--strict-mcp-config` for one private
  server, or the current empty object for omission
- `--settings` inline/file JSON carrying a `hooks.Stop` handler
- `--add-dir <operation-private-root>` with `.claude/skills/<name>/SKILL.md`
  (not `~/.claude/skills` and not the consumer project `.claude/skills`)
- `--include-hook-events` when stream-json must observe hook lifecycle

Stop can block with `decision: "block"` / `reason`, or continue with
`hookSpecificOutput.additionalContext`. Both return control inside the same
`-p` conversation before a successful terminal result. Anti-loop uses
`stop_hook_active` and an 8-consecutive-block cap
(`CLAUDE_CODE_STOP_HOOK_BLOCK_CAP`). Claude-native `background_tasks` stay
distinct from Contract 059 host watchers.

This does **not** bind production. Current Swallowtail still passes
`--mcp-config {"mcpServers":{}}` + `--strict-mcp-config` without `--bare`,
Stop hooks, skill root, or hook events. Empty strict MCP omission stays exact.
Cards 010-011 remain gated on host registry plus live acceptance, not on an
empty mechanism set.

## Boundary

Official docs and exact npm/native artifacts for published
`2.1.220..=2.1.241` points. No login, credentials, provider prompt, paid work,
host install/update, ambient host configuration mutation, watcher process, or
production command change. Disposable probe `HOME` only.

## Method

2026-08-28. Host `claude` was not installed or replaced. Official markdown for
hooks, headless, MCP, skills, settings, and CLI reference was retrieved and
digested. Wrapper `@anthropic-ai/claude-code` and platform
`@anthropic-ai/claude-code-darwin-arm64` packages were fetched for endpoints
`2.1.220` and `2.1.241`, plus mid-window natives `2.1.234` and `2.1.238`.
Native binaries were string-inspected. Prompt-free empty-print probes under
`env -i` + throwaway `HOME`/`cwd` proved argv acceptance before auth.

Probe terminal:

```text
claude [flags] -p --output-format stream-json ... </dev/null
→ ec 1
→ Error: Input must be provided either through stdin or as a prompt argument when using --print
```

Invalid `--mcp-config '{bad'` rejects as missing file path, confirming the
parser treats non-JSON tokens as paths while valid JSON objects are accepted
inline.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| [Hooks](https://code.claude.com/docs/en/hooks) | Stop / StopFailure, decision control, anti-loop, fail-open paths | 2026-08-28 | SHA-256 `b888a5e6ef09dcca31216c749ef471d2c3c270b28e3f9ac1d714562b0cc61809` |
| [Headless](https://code.claude.com/docs/en/headless) | `--bare`, MCP wait, background Bash grace, SIGTERM/SessionEnd | 2026-08-28 | SHA-256 `02f132525d009deae88590647bcac872eb2e5d6cca39c4ba3008a0f810db8314` |
| [MCP](https://code.claude.com/docs/en/mcp) | `--mcp-config`, `--strict-mcp-config`, scopes | 2026-08-28 | SHA-256 `12e0db40002a402e5a1770a94237841fdd1714524681616f5f4867b54fdc0c0e` |
| [Skills](https://code.claude.com/docs/en/skills) | `--add-dir` skill load; bare still loads add-dir skills | 2026-08-28 | SHA-256 `8f2e206b4e74fbd8323028456041bcdd4b8fb5cbf91d4c05f08e5f558c3c6bd5` |
| [Settings](https://code.claude.com/docs/en/settings) | `--settings` session-only JSON; hooks merge across files | 2026-08-28 | SHA-256 `a6f10afd9d41fffa2b14e1f113ef641ea6fbe9b319c114bb986efc3e186de009` |
| [CLI reference](https://code.claude.com/docs/en/cli-reference) | flag membership | 2026-08-28 | SHA-256 `8d4526b2fca256e91d92bff4bdb305012c84c9289ec2d59c771cee73152f00e8` |
| `@anthropic-ai/claude-code@2.1.220` / `2.1.241` wrappers | endpoint identity | 2026-08-28 | `df330874…` / `752252ff…` (match Research 202/249) |
| darwin-arm64 natives `2.1.220`, `2.1.234`, `2.1.238`, `2.1.241` | Stop/MCP/hook strings + empty-print | 2026-08-28 | digests match prior Claude corpora |
| `claude-code-2.1.241/headless-watcher-seam.json` | sanitized specimen | 2026-08-28 | asserted below |

Help digests match Research 202: `fcd5b455…` at `2.1.220`, `71ad650f…` at
`2.1.241`.

## Mechanism Table

| # | Mechanism | Admitted | Exact transport | Truth closed here |
| --- | --- | --- | --- | --- |
| 1 | Private MCP | yes | `--mcp-config` inline JSON or file + `--strict-mcp-config` | parsed + dispatched argv; omission = empty servers object |
| 2 | Instruction asset | yes | op-private `--add-dir` `.claude/skills/…/SKILL.md` under `--bare`; optional `--append-system-prompt` | transport only; live skill attachment not observed |
| 3 | Pre-terminal Stop | yes | `--settings` `hooks.Stop`; stream via `--include-hook-events` | docs + binary symbols at window endpoints and mid points |
| 4 | Same-turn re-entry | yes | `decision: "block"` + `reason`, or `additionalContext`; exit 2 → stderr reason | docs + package anti-loop; live continuation not run |
| 5 | Anti-loop | yes | `stop_hook_active`; override after 8 consecutive blocks; `CLAUDE_CODE_STOP_HOOK_BLOCK_CAP` | docs + binary |
| 6 | Failure / cleanup | yes | StopFailure; MCP/hook fail-open on timeout/missing server; SIGTERM → SessionEnd only, exit 143; Bash bg ~5s grace | docs |
| 7 | Native bg ≠ host watcher | yes | `background_tasks` / session crons / Bash grace vs host MCP+registry | docs + Contract 059 separation |

## Truth Layers

| Layer | Finding |
| --- | --- |
| Requested | Contract 059 watcher opt-in on Claude headless |
| Parsed | `--mcp-config`, `--strict-mcp-config`, `--settings`, `--bare`, `--add-dir`, `--include-hook-events` accepted at empty-print on `2.1.220` and `2.1.241` |
| Configured | candidate composition is session argv/settings/add-dir; not user/project persistent skill or settings writes |
| Dispatched | current production still dispatches empty strict MCP without bare/Stop/skill |
| Applied | Stop block/continue and skill attachment require a live turn; not observed |
| Model-visible | docs: `reason` / `additionalContext` return to Claude in the same conversation |
| Blocking | Stop before successful terminal; StopFailure has no decision control |
| Terminal | successful Stop allow → result/exit; SIGTERM → no result, SessionEnd, 143 |
| Cleanup | host owns watcher join; Claude-native bg tasks follow provider grace/wait rules |

## Version Milestones

| Milestone | Floor | Note |
| --- | --- | --- |
| Stop anti-loop symbols | ≤2.1.220 | present at every probed native in-window |
| `mcp_server_errors` in `system/init` | 2.1.219+ | skip detection for bad `--mcp-config` entries |
| pending MCP wait before first `-p` turn | 2.1.221+ | 30s `MCP_TIMEOUT` default |
| strict project-approval wait skip | 2.1.246+ | above ceiling; `--bare` avoids loading project `.mcp.json` |

`2.1.230` was never published.

## Why Not An Empty Set

Stop is not observation-only: official hooks docs and package strings show
block/continue into the same conversation, with an explicit consecutive-block
cap. MCP and hooks do not require shared persistent configuration when using
`--mcp-config`/`--strict-mcp-config` and session `--settings` under `--bare`.
Instruction delivery can use an operation-private `--add-dir` skill root
without writing user or project skill folders.

Live same-turn re-entry was intentionally not run. That is a card 010
acceptance gate, not a mechanism absence.

## Why Not Production Binding

Hooks merge across settings levels. Without `--bare`, current
`--setting-sources user,project,local` can load ambient Stop hooks and skills.
The admitted seam therefore requires the bare + explicit injection composition
above. Changing production argv is out of this card's scope.

## Omission

No watcher opt-in ⇒ keep `--mcp-config {"mcpServers":{}}` +
`--strict-mcp-config`, and do not add `--bare` Stop/`--settings`/`--add-dir`
watcher assets. That preserves today's empty strict MCP behavior.

## Fixture

`crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-watcher-seam.json`

## Non-Claims

- No production command, MCP server, skill injection, or hook wiring.
- No claim that Claude-native background Bash/subagents are Contract 059
  watchers.
- No live provider proof of model-visible Stop re-entry.
- No merge or card 010 start.

## References

- [Contract 059](../contracts/059-operation-scoped-process-watchers.md)
- [Research 255](255-production-harness-skill-and-watcher-surface-census.md)
- [Research 202](202-claude-code-2-1-241-identity.md)
- [g05.003 card 007](../roadmaps/g05/batch-cards/007-claude-code-watcher-seam-evidence.md)
